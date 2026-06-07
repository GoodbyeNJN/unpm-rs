use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::Deserialize;

use crate::{BoxedError, Result};

#[derive(Clone, Debug, Default, Deserialize)]
struct NpmSearchResponse {
    objects: Vec<NpmSearchObject>,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct NpmSearchObject {
    package: Option<NpmSearchPackage>,
    updated: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct NpmSearchPackage {
    name: Option<String>,
    description: Option<String>,
    version: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct PackageInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub updated: String,
}

impl From<&NpmSearchObject> for Option<PackageInfo> {
    fn from(obj: &NpmSearchObject) -> Self {
        let pkg = obj.package.as_ref()?;
        let info = PackageInfo {
            name: pkg.name.to_owned()?,
            description: pkg.description.to_owned().unwrap_or_default(),
            version: pkg.version.to_owned().unwrap_or_default(),
            updated: obj
                .updated
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default(),
        };

        Some(info)
    }
}

pub fn search_npm_registry(query: &str, base_url: &str) -> Result<Vec<PackageInfo>, BoxedError> {
    let url = build_url()
        .base_url(base_url)
        .query(query)
        .size(20)
        .call()?;
    let body = reqwest::blocking::get(url)?.text()?;
    let res: NpmSearchResponse = serde_json::from_str(&body)?;

    let pkgs = res
        .objects
        .iter()
        .filter_map(|object| -> Option<_> { object.into() })
        .collect();

    Ok(pkgs)
}

#[bon::builder]
fn build_url(
    base_url: &str,
    query: &str,
    size: Option<i32>,
    from: Option<i32>,
) -> Result<Url, BoxedError> {
    let mut url = Url::parse(base_url)?;

    url.set_path("/-/v1/search");
    url.query_pairs_mut().append_pair("text", query);
    if let Some(size) = size {
        url.query_pairs_mut().append_pair("size", &size.to_string());
    }
    if let Some(from) = from {
        url.query_pairs_mut().append_pair("from", &from.to_string());
    }

    Ok(url)
}
