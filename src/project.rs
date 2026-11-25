use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use color_print::cformat;
use log::debug;
use package_json_schema::PackageJson;

use crate::{NODE_MODULES_BIN_NAME, NODE_MODULES_NAME, PACKAGE_JSON_NAME, fs};

#[derive(Clone, Debug, Default)]
pub struct Project {
    package_json: Option<PackageJson>,
    package_json_path: PathBuf,
    node_modules_bin_path: PathBuf,
}

impl Project {
    pub fn new(path: &Path) -> Self {
        Self {
            package_json: None,
            package_json_path: path.join(PACKAGE_JSON_NAME),
            node_modules_bin_path: path.join(NODE_MODULES_NAME).join(NODE_MODULES_BIN_NAME),
        }
    }

    pub fn find_nearest(cwd: Option<&Path>) -> Result<Self> {
        debug!("find nearest project start...");

        let mut start_dir = None;
        for dir in fs::walk_up(cwd) {
            if start_dir.is_none() {
                start_dir = Some(dir.to_owned());
            }

            let project = Self::new(&dir);

            if project.is_project_dir() {
                debug!("find nearest project end, found: {:?}", dir);

                return Ok(project);
            }
        }

        debug!("find nearest project end, not found");
        bail!(
            "no package.json file found in or above path: '{}'",
            cformat!("<yellow>{}</>", start_dir.unwrap().display())
        )
    }

    pub fn list_bins(&self) -> Result<Vec<String>> {
        debug!("list bins start...");

        let entries = fs::read_dir_with_cache(&self.node_modules_bin_path).with_context(|| {
            format!(
                "failed to read directory: '{}'",
                cformat!("<yellow>{}</>", self.node_modules_bin_path.display())
            )
        })?;

        let mut bins = vec![];
        for (_, file_type, file_name) in entries {
            if file_type.is_file() {
                bins.push(file_name.to_string_lossy().to_string());
            }
        }

        debug!("list bins end, bins: {:?}", bins);
        Ok(bins)
    }

    pub fn list_scripts(&mut self, prefix: Option<&str>) -> Result<Vec<(String, String)>> {
        debug!("list scripts start...");

        let pkg = self.parse_package_json()?;

        let scripts = pkg
            .scripts
            .as_ref()
            .into_iter()
            .flatten()
            .filter_map(|(k, v)| {
                let Some(v) = v.as_ref() else {
                    return None;
                };

                match prefix {
                    Some(prefix) if k.starts_with(prefix) => Some((k.to_owned(), v.to_owned())),
                    None => Some((k.to_owned(), v.to_owned())),
                    _ => None,
                }
            })
            .collect();

        debug!("list scripts end, scripts: {:?}", scripts);
        Ok(scripts)
    }

    pub fn list_dependencies(
        &mut self,
        prefix: Option<&str>,
    ) -> Result<Vec<(String, &'static str)>> {
        debug!("list dependencies start...");

        let pkg = self.parse_package_json()?;

        let prod_deps = pkg
            .dependencies
            .as_ref()
            .into_iter()
            .flatten()
            .map(|(name, _)| (name.to_owned(), "dependencies"));
        let dev_deps = pkg
            .dev_dependencies
            .as_ref()
            .into_iter()
            .flatten()
            .map(|(name, _)| (name.to_owned(), "devDependencies"));
        let iter = prod_deps.chain(dev_deps);

        let deps = match prefix {
            Some(prefix) => iter.filter(|(k, _)| k.starts_with(prefix)).collect(),
            None => iter.collect(),
        };

        debug!("list dependencies end, dependencies: {:?}", deps);
        Ok(deps)
    }

    pub fn resolve_package_manager_by_dev_engines(&mut self) -> Option<(String, String)> {
        debug!("resolve devEngines packageManager field start...");

        let pkg = self.parse_package_json().ok()?;

        let dev_engines = pkg.other.as_ref()?.get("devEngines")?.as_object()?;
        debug!("dev_engines: {:?}", dev_engines);

        let package_manager = dev_engines.get("packageManager")?.as_object()?;
        debug!("package_manager: {:?}", package_manager);

        let name = package_manager.get("name")?.as_str()?;
        let version = package_manager
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        debug!(
            "resolve devEngines packageManager field end, name: {:?}, version: {:?}",
            name, version
        );
        Some((name.to_owned(), version.to_owned()))
    }

    pub fn resolve_package_manager_by_field(&mut self) -> Option<(String, String)> {
        debug!("resolve packageManager field start...");

        let pkg = self.parse_package_json().ok()?;

        let package_manager = pkg.package_manager.as_ref()?;
        debug!("package_manager: {:?}", package_manager);

        let package_manager = package_manager.strip_prefix("^").unwrap_or(package_manager);
        let (name, version) = package_manager
            .split_once("@")
            .unwrap_or((package_manager, ""));

        debug!(
            "resolve packageManager field end, name: {:?}, version: {:?}",
            name, version
        );
        Some((name.to_owned(), version.to_owned()))
    }

    fn is_project_dir(&self) -> bool {
        fs::is_file_with_cache(&self.package_json_path)
    }

    fn parse_package_json(&mut self) -> Result<&PackageJson> {
        debug!("parse package.json start...");

        if self.package_json.is_none() {
            let content = fs::read_with_cache(&self.package_json_path).with_context(|| {
                format!(
                    "failed to read package.json file: '{}'",
                    cformat!("<yellow>{}</>", self.package_json_path.display())
                )
            })?;
            if let Ok(package_json) = content.try_into() {
                self.package_json = Some(package_json);
            } else {
                bail!(
                    "invalid package.json file: '{}'",
                    cformat!("<yellow>{}</>", self.package_json_path.display())
                )
            }
        }

        Ok(self.package_json.as_ref().unwrap())
    }
}
