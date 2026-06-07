use std::{path::Path, str::FromStr};

use log::debug;

use crate::PackageManager;
use crate::package_manager::{Project, fs};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DetectionStrategy {
    Lockfile,
    PackageManagerField,
    DevEnginesField,
    #[allow(dead_code)]
    InstallMetadata,
}

impl DetectionStrategy {
    fn default() -> &'static [Self] {
        use DetectionStrategy::*;

        &[Lockfile, DevEnginesField, PackageManagerField]
    }
}

pub fn detect_package_manager(
    cwd: Option<&Path>,
    strategies: Option<&[DetectionStrategy]>,
) -> Option<PackageManager> {
    debug!("package manager detection start...");

    let strategies = strategies.unwrap_or_else(|| DetectionStrategy::default());
    debug!("strategies: {:?}", strategies);

    for dir in fs::walk_up(cwd) {
        debug!("directory: {:?}", dir);

        if let Some(pm) = detect_with_strategies(&dir, strategies) {
            debug!("package manager detection end, package_manager: {:?}", pm);
            return Some(pm);
        }
    }

    debug!("package manager detection end, not determined");
    None
}

fn detect_with_strategies(dir: &Path, strategies: &[DetectionStrategy]) -> Option<PackageManager> {
    use DetectionStrategy::*;
    use PackageManager::*;

    for strategy in strategies {
        match strategy {
            Lockfile => {
                if let Some(pm) = detect_by_lockfile(dir) {
                    if pm != Yarn {
                        return Some(pm);
                    }

                    debug!("yarn further detection start...");
                    let partial_strategies: Vec<_> = strategies
                        .iter()
                        .filter(|strategy| {
                            **strategy == DevEnginesField || **strategy == PackageManagerField
                        })
                        .copied()
                        .collect();
                    debug!("partial_strategies: {:?}", partial_strategies);

                    if let Some(pm) = detect_with_strategies(dir, &partial_strategies) {
                        debug!("yarn further detection end, package_manager: {:?}", pm);
                        return Some(pm);
                    } else {
                        debug!(
                            "yarn further detection end, not determined, fallback to yarn classic"
                        );
                        return Some(Yarn);
                    }
                }
            }
            PackageManagerField => {
                if let Some(pm) = detect_by_package_manager_field(dir) {
                    return Some(pm);
                }
            }
            DevEnginesField => {
                if let Some(pm) = detect_by_dev_engines_field(dir) {
                    return Some(pm);
                }
            }
            InstallMetadata => {
                if let Some(pm) = detect_by_install_metadata(dir) {
                    return Some(pm);
                }
            }
        };
    }

    None
}

fn detect_by_lockfile(dir: &Path) -> Option<PackageManager> {
    use PackageManager::*;

    debug!("by lockfile start...");

    const LOCKFILES: [(&str, PackageManager); 7] = [
        ("bun.lock", Bun),
        ("bun.lockb", Bun),
        ("pnpm-lock.yaml", Pnpm),
        ("pnpm-workspace.yaml", Pnpm),
        ("yarn.lock", Yarn), // NOTE: This will also match Yarn Berry, should be checked further if needed
        ("package-lock.json", Npm),
        ("npm-shrinkwrap.json", Npm),
    ];

    let found = LOCKFILES
        .into_iter()
        .find(|(file, _)| fs::is_file_with_cache(&dir.join(file)));
    if let Some((file, pm)) = found {
        debug!(
            "by lockfile end, lockfile: {:?}, package_manager: {:?}",
            file, pm
        );
        Some(pm)
    } else {
        debug!("by lockfile end, not determined");
        None
    }
}

fn detect_by_package_manager_field(dir: &Path) -> Option<PackageManager> {
    debug!("by packageManager field start...");

    let mut project = Project::new(dir);
    let (name, version) = project.resolve_package_manager_by_field()?;
    let pm = determine_package_manager(&name, &version);

    debug!("by packageManager field end, package_manager: {:?}", pm);
    pm
}

fn detect_by_dev_engines_field(dir: &Path) -> Option<PackageManager> {
    debug!("by devEngines field start...");

    let mut project = Project::new(dir);
    let (name, version) = project.resolve_package_manager_by_dev_engines()?;
    let pm = determine_package_manager(&name, &version);

    debug!("by devEngines field end, package_manager: {:?}", pm);
    pm
}

fn detect_by_install_metadata(dir: &Path) -> Option<PackageManager> {
    use PackageManager::*;

    debug!("by install metadata start...");

    const INSTALL_METADATA_FILES: [(&str, PackageManager); 8] = [
        ("node_modules/.pnpm/", Pnpm),
        ("node_modules/.yarn-state.yml", YarnBerry),
        ("node_modules/.yarn_integrity", Yarn),
        ("node_modules/.package-lock.json", Npm),
        (".pnp.cjs", YarnBerry),
        (".pnp.js", YarnBerry),
        ("bun.lock", Bun),
        ("bun.lockb", Bun),
    ];

    let found = INSTALL_METADATA_FILES.into_iter().find(|(meta, _)| {
        let path = dir.join(meta);
        if meta.ends_with("/") {
            fs::is_dir_with_cache(&path)
        } else {
            fs::is_file_with_cache(&path)
        }
    });
    if let Some((meta, pm)) = found {
        debug!(
            "by install metadata end, metadata: {:?}, package_manager: {:?}",
            meta, pm
        );
        Some(pm)
    } else {
        debug!("by install metadata end, not determined");
        None
    }
}

fn determine_package_manager(name: &str, version: &str) -> Option<PackageManager> {
    debug!("determine package manager start...");

    use PackageManager::*;

    let pm = PackageManager::from_str(name).ok()?;
    if pm != Yarn {
        debug!("determine package manager end, package_manager: {:?}", pm);

        return Some(pm);
    }

    let major = version
        .split(".")
        .next()?
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .ok();
    debug!("yarn major version: {:?}", major);

    if major.is_some_and(|major| major > 1) {
        debug!(
            "determine package manager end, package_manager: {:?}",
            YarnBerry
        );
        Some(YarnBerry)
    } else {
        debug!("determine package manager end, package_manager: {:?}", Yarn);
        Some(Yarn)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env, fs,
        path::PathBuf,
        process,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::DetectionStrategy;

    fn fixture_dir(category: &str, case: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(category)
            .join(case)
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or_default();
        env::temp_dir().join(format!("{prefix}-{}-{nanos}", process::id()))
    }

    fn copy_dir_all(src: &PathBuf, dst: &PathBuf) {
        fs::create_dir_all(dst).expect("create destination directory");

        let entries = fs::read_dir(src).expect("read source directory");
        for entry in entries {
            let entry = entry.expect("read source entry");
            let from = entry.path();
            let to = dst.join(entry.file_name());

            if from.is_dir() {
                copy_dir_all(&from, &to);
            } else {
                fs::copy(&from, &to).expect("copy fixture file");
            }
        }
    }

    fn detect_package_manager_as_name(
        category: &str,
        case: &str,
        strategies: Option<&[DetectionStrategy]>,
    ) -> Option<String> {
        let src = fixture_dir(category, case);
        let cwd = unique_temp_dir(&format!("unpm-package-manager-fixture-{category}-{case}"));
        copy_dir_all(&src, &cwd);

        let result = super::detect_package_manager(Some(&cwd), strategies)
            .map(|package_manager| package_manager.to_string());
        let _ = fs::remove_dir_all(cwd);

        result
    }

    #[test]
    fn detect_lockfile_fixtures() {
        let cases = [
            ("bun", Some("bun")),
            ("npm", Some("npm")),
            ("pnpm", Some("pnpm")),
            ("yarn", Some("yarn")),
            ("yarn-berry", Some("yarn")),
            ("unknown", None),
        ];

        for (case, expected) in cases {
            let actual = detect_package_manager_as_name("lockfile", case, None);
            assert_eq!(actual.as_deref(), expected, "lockfile fixture `{case}`");
        }
    }

    #[test]
    fn detect_package_manager_field_fixtures() {
        let cases = [
            ("bun", Some("bun")),
            ("npm", Some("npm")),
            ("pnpm", Some("pnpm")),
            ("pnpm-version-range", Some("pnpm")),
            ("yarn", Some("yarn")),
            ("yarn-berry", Some("yarn-berry")),
            ("unknown", None),
        ];

        for (case, expected) in cases {
            let actual = detect_package_manager_as_name("packager", case, None);
            assert_eq!(actual.as_deref(), expected, "packager fixture `{case}`");
        }
    }

    #[test]
    fn detect_dev_engines_fixtures() {
        let cases = [
            ("bun", Some("bun")),
            ("npm", Some("npm")),
            ("pnpm", Some("pnpm")),
            ("pnpm-version-range", Some("pnpm")),
            ("yarn", Some("yarn")),
            ("yarn-berry", Some("yarn-berry")),
            ("unknown", None),
        ];

        for (case, expected) in cases {
            let actual = detect_package_manager_as_name("dev-engines", case, None);
            assert_eq!(actual.as_deref(), expected, "dev-engines fixture `{case}`");
        }
    }

    #[test]
    fn detect_install_metadata_fixtures() {
        let cases = [
            ("bun", Some("bun")),
            ("npm", Some("npm")),
            ("pnpm", Some("pnpm")),
            ("yarn", Some("yarn")),
            ("yarn-berry", Some("yarn-berry")),
            ("yarn-berry-pnp-v2", Some("yarn-berry")),
            ("yarn-berry-pnp-v3", Some("yarn-berry")),
            ("unknown", None),
        ];

        let strategies = vec![
            DetectionStrategy::InstallMetadata,
            DetectionStrategy::Lockfile,
            DetectionStrategy::PackageManagerField,
            DetectionStrategy::DevEnginesField,
        ];

        for (case, expected) in cases {
            let actual =
                detect_package_manager_as_name("install-metadata", case, Some(&strategies));
            assert_eq!(
                actual.as_deref(),
                expected,
                "install-metadata fixture `{case}`"
            );
        }
    }
}
