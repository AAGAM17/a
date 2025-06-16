use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Serialize, Deserialize, Default)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
    pub scripts: HashMap<String, String>,
}

pub fn read_manifest() -> Result<Manifest> {
    let manifest_path = Path::new("a.json");
    
    if !manifest_path.exists() {
        return Ok(Manifest::default());
    }
    
    let content = fs::read_to_string(manifest_path)?;
    let manifest: Manifest = serde_json::from_str(&content)?;
    Ok(manifest)
}

pub fn write_manifest(manifest: &Manifest) -> Result<()> {
    let content = serde_json::to_string_pretty(manifest)?;
    fs::write("a.json", content)?;
    Ok(())
}

pub fn init(name: &str) -> Result<()> {
    // Create a new manifest with basic details
    let manifest = Manifest {
        name: name.to_string(),
        version: "0.1.0".to_string(),
        description: Some("Package created with a package manager".to_string()),
        author: None,
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
        scripts: HashMap::new(),
    };
    
    write_manifest(&manifest)
}

pub fn add_dependency(pkg: &str, version: &str, is_dev: bool) -> Result<()> {
    let mut manifest = read_manifest()?;
    
    if is_dev {
        manifest.dev_dependencies.insert(pkg.to_string(), version.to_string());
    } else {
        manifest.dependencies.insert(pkg.to_string(), version.to_string());
    }
    
    write_manifest(&manifest)
}

pub fn remove_dependency(pkg: &str, is_dev: bool) -> Result<()> {
    let mut manifest = read_manifest()?;
    
    if is_dev {
        manifest.dev_dependencies.remove(pkg);
    } else {
        manifest.dependencies.remove(pkg);
    }
    
    write_manifest(&manifest)
}
