use std::path::{Path, PathBuf};

use log::debug;
use package_json_schema::PackageJson;

use crate::package_manager::fs;

#[derive(Clone, Debug, Default)]
pub struct Project {
    package_json: Option<PackageJson>,
    package_json_path: PathBuf,
    node_modules_path: PathBuf,
}

impl Project {
    pub fn new(path: &Path) -> Self {
        Self {
            package_json: None,
            package_json_path: path.join(crate::constants::PACKAGE_JSON_NAME),
            node_modules_path: path.join(crate::constants::NODE_MODULES_NAME),
        }
    }

    pub fn find_nearest(cwd: Option<&Path>) -> Option<Self> {
        debug!("find nearest project start...");

        for ref dir in fs::walk_up(cwd) {
            let project = Self::new(dir);

            if project.is_project_dir() {
                debug!("find nearest project end, found: {:?}", dir);

                return Some(project);
            }
        }

        debug!("find nearest project end, not found");
        None
    }

    pub fn list_bins(&self) -> Option<Vec<String>> {
        debug!("list bins start...");

        if !fs::is_dir_with_cache(&self.node_modules_path) {
            debug!("list bins end, not found");
            return None;
        }

        let mut bins = vec![];
        for (_, file_type, file_name) in fs::read_dir_with_cache(&self.node_modules_path)? {
            let is_file = file_type.is_some_and(|f| f.is_file());
            if is_file {
                bins.push(file_name.to_string_lossy().to_string());
            }
        }

        debug!("list bins end, bins: {:?}", bins);
        Some(bins)
    }

    pub fn list_scripts(&mut self, prefix: Option<&str>) -> Option<Vec<(String, String)>> {
        debug!("list scripts start...");

        let pkg = self.parse_package_json()?;

        let scripts = pkg
            .scripts
            .as_ref()?
            .into_iter()
            .filter_map(|(k, v)| v.as_ref().map(|v| (k.to_owned(), v.to_owned())))
            .filter(|(k, _)| match prefix {
                Some(prefix) => k.starts_with(prefix),
                None => true,
            })
            .collect();

        debug!("list scripts end, scripts: {:?}", scripts);
        Some(scripts)
    }

    pub fn list_dependencies(&mut self, prefix: Option<&str>) -> Option<Vec<(String, String)>> {
        debug!("list dependencies start...");

        let pkg = self.parse_package_json()?;

        let deps = pkg
            .dependencies
            .as_ref()
            .into_iter()
            .flatten()
            .map(|(name, _)| (name.to_owned(), "dependencies".to_owned()));
        let dev_deps = pkg
            .dev_dependencies
            .as_ref()
            .into_iter()
            .flatten()
            .map(|(name, _)| (name.to_owned(), "devDependencies".to_owned()));
        let iter = deps.chain(dev_deps);

        let deps = match prefix {
            Some(prefix) => iter.filter(|(k, _)| k.starts_with(prefix)).collect(),
            None => iter.collect(),
        };

        debug!("list dependencies end, dependencies: {:?}", deps);
        Some(deps)
    }

    pub fn resolve_package_manager_by_dev_engines(&mut self) -> Option<(String, String)> {
        debug!("resolve devEngines packageManager field start...");

        let pkg = self.parse_package_json()?;

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

        let pkg = self.parse_package_json()?;

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

    fn parse_package_json(&mut self) -> Option<&PackageJson> {
        debug!("parse package.json start...");

        if self.package_json.is_some() {
            debug!("parse package.json end, already parsed");
            return self.package_json.as_ref();
        }

        let content = fs::read_with_cache(&self.package_json_path)?;
        debug!("content: {:?}", content);

        self.package_json = content.try_into().ok();

        debug!(
            "parse package.json end, package_json: {:?}",
            self.package_json
        );
        self.package_json.as_ref()
    }
}
