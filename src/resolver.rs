use anyhow::Result;
use semver::{Version, VersionReq};
use crate::registry;
use std::collections::HashMap;

pub async fn resolve_version(pkg: &str, req: &str) -> Result<String> {
    if req == "latest" {
        return registry::get_latest_version(pkg).await;
    }
    
    let req = VersionReq::parse(req)?;
    let url = format!("https://registry.npmjs.org/{}", pkg);
    let resp: registry::NpmResponse = reqwest::Client::new().get(url).send().await?.json().await?;
    
    // Find all versions that match the requirement
    let mut versions = Vec::new();
    for (ver_str, _) in resp.versions.iter() {
        if let Ok(version) = Version::parse(ver_str) {
            if req.matches(&version) {
                versions.push(version);
            }
        }
    }
    
    // Sort versions and get the latest matching one
    if versions.is_empty() {
        anyhow::bail!("No version found that matches requirement: {}", req);
    }
    
    versions.sort();
    Ok(versions.last().unwrap().to_string())
}

pub async fn resolve_dependencies(deps: &HashMap<String, String>) -> Result<HashMap<String, String>> {
    let mut resolved = HashMap::new();
    
    for (pkg, req) in deps {
        let version = resolve_version(pkg, req).await?;
        resolved.insert(pkg.clone(), version);
    }
    
    Ok(resolved)
}
