use futures::stream::{self, StreamExt};
use std::path::PathBuf;
use dirs::home_dir;
use colored::*;
use std::fs;
use anyhow::Result;

// Get package cache directory
pub fn get_cache_dir() -> PathBuf {
    home_dir().unwrap_or_default().join(".a-pm-cache")
}

// Create cache folders if they don't exist
pub fn ensure_cache_dirs() -> Result<()> {
    let cache_dir = get_cache_dir();
    fs::create_dir_all(&cache_dir)?;
    fs::create_dir_all(cache_dir.join("packages"))?;
    fs::create_dir_all(cache_dir.join("metadata"))?;
    Ok(())
}

// Check if package is cached
pub fn is_package_cached(pkg: &str, version: &str) -> bool {
    let pkg_path = get_cache_dir()
        .join("packages")
        .join(format!("{}-{}.tgz", pkg.replace("/", "-"), version));
    pkg_path.exists()
}

// Perform concurrent installations
pub async fn install_packages_concurrently(
    packages: Vec<(String, String)>,
    max_concurrent: usize,
) -> Result<()> {
    println!("{}", format!("🚀 Installing {} packages...", packages.len()).blue());

    let results = stream::iter(packages)
        .map(|(pkg, ver)| async move {
            let result = crate::registry::download_and_unpack(&pkg, &ver).await;
            (pkg, ver, result)
        })
        .buffer_unordered(max_concurrent)
        .collect::<Vec<_>>()
        .await;

    let mut success_count = 0;
    let mut failure_count = 0;

    for (pkg, ver, result) in results {
        match result {
            Ok(_) => {
                success_count += 1;
                println!("{}", format!("✅ Installed {}@{}", pkg, ver).green());
            }
            Err(e) => {
                failure_count += 1;
                println!("{}", format!("❌ Failed to install {}@{}: {}", pkg, ver, e).red());
            }
        }
    }

    println!(
        "{}",
        format!(
            "📊 Summary: {} succeeded, {} failed",
            success_count, failure_count
        )
        .blue()
    );

    if failure_count > 0 {
        return Err(anyhow::anyhow!(
            "{} packages failed to install",
            failure_count
        ));
    }

    Ok(())
}

// Clean cache that's older than a specific time
pub fn clean_cache(days_old: u64) -> Result<()> {
    println!("{}", format!("🧹 Cleaning cache older than {} days...", days_old).blue());
    
    let cache_dir = get_cache_dir();
    let now = std::time::SystemTime::now();
    let seconds_threshold = days_old * 24 * 60 * 60;
    
    let mut removed_count = 0;
    let mut total_size = 0;
    
    // Clean packages
    let packages_dir = cache_dir.join("packages");
    if packages_dir.exists() {
        for entry in fs::read_dir(packages_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = now.duration_since(modified) {
                        if duration.as_secs() > seconds_threshold {
                            total_size += metadata.len();
                            fs::remove_file(&path)?;
                            removed_count += 1;
                        }
                    }
                }
            }
        }
    }
    
    // Clean metadata
    let metadata_dir = cache_dir.join("metadata");
    if metadata_dir.exists() {
        for entry in fs::read_dir(metadata_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(duration) = now.duration_since(modified) {
                        if duration.as_secs() > seconds_threshold {
                            total_size += metadata.len();
                            fs::remove_file(&path)?;
                            removed_count += 1;
                        }
                    }
                }
            }
        }
    }
    
    println!("{}", format!("🧹 Removed {} files ({} MB)", 
             removed_count, total_size / 1024 / 1024).green());
    
    Ok(())
}
