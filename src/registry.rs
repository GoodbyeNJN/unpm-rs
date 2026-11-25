use anyhow::{Context, Result, bail};
use jiff::Timestamp;
use reqwest::Url;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
struct NpmSearchResponse {
    objects: Vec<NpmSearchObject>,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct NpmSearchObject {
    package: NpmSearchPackage,
    updated: Timestamp,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct NpmSearchPackage {
    name: String,
    description: String,
    version: String,
}

#[derive(Clone, Debug, Default)]
pub struct PackageInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub updated: String,
}

impl From<&NpmSearchObject> for PackageInfo {
    fn from(obj: &NpmSearchObject) -> Self {
        PackageInfo {
            name: obj.package.name.to_owned(),
            description: obj.package.description.to_owned(),
            version: obj.package.version.to_owned(),
            updated: obj.updated.strftime("%Y-%m-%d").to_string(),
        }
    }
}

pub fn search_npm_registry(base_url: &str, query: Option<&str>) -> Result<Vec<PackageInfo>> {
    let query = match query {
        Some(query) if query.len() >= 2 => query,
        Some(_) => bail!("search query must be at least 2 characters"),
        None => bail!("search query is required"),
    };

    let url =
        build_url(base_url, query, Some(20), None).context("failed to parse npm registry URL")?;
    let body = reqwest::blocking::get(url)
        .and_then(|response| response.text())
        .context("failed to search npm registry")?;
    let res: NpmSearchResponse =
        serde_json::from_str(&body).context("failed to parse npm search response")?;
    let pkgs = res.objects.iter().map(PackageInfo::from).collect();

    Ok(pkgs)
}

// fn into_search_error<T: Into<Error>>(err: T) -> Error {
//     anyhow!(
//         "failed to search npm registry\n       {} {}",
//         cstr!("<red,bold>cause:</>"),
//         err.into()
//     )
// }

fn build_url(base_url: &str, query: &str, size: Option<i32>, from: Option<i32>) -> Result<Url> {
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
