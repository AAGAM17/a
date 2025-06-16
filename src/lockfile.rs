use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Default)]
pub struct Lockfile {
    dependencies: HashMap<String, String>,
}

pub fn add_dependency(pkg: &str, version: &str) -> anyhow::Result<()> {
    let mut lock = read_or_new()?;
    lock.dependencies.insert(pkg.to_string(), version.to_string());
    save_lockfile(&lock)
}

pub fn remove_dependency(pkg: &str) -> anyhow::Result<()> {
    let mut lock = read_or_new()?;
    lock.dependencies.remove(pkg);
    save_lockfile(&lock)
}

pub fn read_dependencies() -> anyhow::Result<HashMap<String, String>> {
    let lock = read_or_new()?;
    Ok(lock.dependencies)
}

fn read_or_new() -> anyhow::Result<Lockfile> {
    if let Ok(content) = fs::read_to_string("a.lock") {
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(Lockfile::default())
    }
}

fn save_lockfile(lock: &Lockfile) -> anyhow::Result<()> {
    let content = serde_json::to_string_pretty(lock)?;
    fs::write("a.lock", content)?;
    Ok(())
}
