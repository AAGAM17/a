use crate::registry;
use crate::lockfile;
use crate::manifest;
use colored::*;
use std::fs;
use std::process::Command;

pub async fn add(package: &str, dev: bool) -> anyhow::Result<()> {
    println!("{}", format!("🔍 Resolving {} ...", package).blue());
    let version = registry::get_latest_version(package).await?;
    println!("{}", format!("📦 Adding {}@{}", package, version).green());

    // Update manifest
    manifest::add_dependency(package, &version, dev)?;
    
    // Update lockfile
    lockfile::add_dependency(package, &version)?;
    
    // Immediately install the package
    registry::download_and_unpack(package, &version).await?;
    
    println!("{}", format!("✅ Added {}@{} to {}", package, version, 
             if dev { "devDependencies" } else { "dependencies" }).green());
    Ok(())
}

pub async fn install() -> anyhow::Result<()> {
    println!("{}", "🔑 Reading lockfile...".blue());
    let deps = lockfile::read_dependencies()?;
    
    if deps.is_empty() {
        println!("{}", "No dependencies to install.".yellow());
        return Ok(());
    }
    
    // Convert the dependencies to a format suitable for parallel installation
    let packages: Vec<(String, String)> = deps.into_iter().collect();
    
    // Determine a reasonable number of parallel downloads based on system
    let parallel_count = std::cmp::min(num_cpus::get() * 2, 8); // Reasonable default
    
    println!("{}", format!("🚀 Installing {} packages with {} parallel downloads", 
             packages.len(), parallel_count).blue());
    
    // Install packages in parallel
    crate::cache::install_packages_concurrently(packages, parallel_count).await?;
    
    println!("{}", "✅ All dependencies installed!".green());
    Ok(())
}

pub async fn remove(package: &str, dev: bool) -> anyhow::Result<()> {
    println!("{}", format!("❌ Removing {} ...", package).red());
    
    // Update manifest
    manifest::remove_dependency(package, dev)?;
    
    // Update lockfile
    lockfile::remove_dependency(package)?;
    
    // Remove from node_modules
    let pkg_dir = format!("node_modules/{}", package);
    if fs::metadata(&pkg_dir).is_ok() {
        fs::remove_dir_all(pkg_dir)?;
    }
    
    println!("{}", format!("✅ Removed {}", package).green());
    Ok(())
}

pub async fn search(query: &str) -> anyhow::Result<()> {
    println!("{}", format!("🔎 Searching for '{}' ...", query).blue());
    let results = registry::search_package(query).await?;
    for pkg in results {
        println!("📦 {}", pkg);
    }
    Ok(())
}

pub async fn init(name: Option<&str>) -> anyhow::Result<()> {
    let pkg_name = match name {
        Some(n) => n.to_string(),
        None => {
            // Use current directory name
            let path = std::env::current_dir()?;
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("my-package")
                .to_string()
        }
    };
    
    println!("{}", format!("📝 Creating a new package: {}", pkg_name).blue());
    manifest::init(&pkg_name)?;
    
    println!("{}", "✅ Created a.json file".green());
    Ok(())
}

pub async fn run_script(script_name: &str) -> anyhow::Result<()> {
    let manifest = manifest::read_manifest()?;
    
    if let Some(script) = manifest.scripts.get(script_name) {
        println!("{}", format!("▶️ Running script: {}", script_name).blue());
        
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", script])
                .output()?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(script)
                .output()?
        };
        
        println!("{}", String::from_utf8_lossy(&output.stdout));
        
        if !output.status.success() {
            println!("{}", format!("❌ Script '{}' failed with exit code: {:?}", 
                     script_name, output.status.code()).red());
            return Err(anyhow::anyhow!("Script execution failed"));
        }
        
        println!("{}", format!("✅ Script '{}' completed successfully", script_name).green());
    } else {
        return Err(anyhow::anyhow!("Script '{}' not found in a.json", script_name));
    }
    
    Ok(())
}

pub async fn update(pkg: Option<&str>) -> anyhow::Result<()> {
    if let Some(package) = pkg {
        // Update specific package
        println!("{}", format!("🔄 Updating {} ...", package).blue());
        let version = registry::get_latest_version(package).await?;
        
        // Update lockfile
        lockfile::add_dependency(package, &version)?;
        
        // Update the package
        registry::download_and_unpack(package, &version).await?;
        
        println!("{}", format!("✅ Updated {}@{}", package, version).green());
    } else {
        // Update all packages
        println!("{}", "🔄 Updating all dependencies...".blue());
        
        // Read from manifest
        let manifest = manifest::read_manifest()?;
        
        for (pkg, _) in &manifest.dependencies {
            let version = registry::get_latest_version(pkg).await?;
            println!("{}", format!("� Updating {}@{}", pkg, version).yellow());
            
            // Update lockfile
            lockfile::add_dependency(pkg, &version)?;
            
            // Update the package
            registry::download_and_unpack(pkg, &version).await?;
        }
        
        println!("{}", "✅ All dependencies updated!".green());
    }
    
    Ok(())
}

pub async fn list() -> anyhow::Result<()> {
    let manifest = manifest::read_manifest()?;
    let lockfile = lockfile::read_dependencies()?;
    
    println!("�📦 Dependencies:");
    if manifest.dependencies.is_empty() {
        println!("   No dependencies");
    } else {
        for (pkg, ver_req) in &manifest.dependencies {
            let ver_req_clone = ver_req.clone();
            let resolved = lockfile.get(pkg).unwrap_or(&ver_req_clone);
            println!("   {} {} (locked at {})", "➤".blue(), pkg, resolved);
        }
    }
    
    println!("\n🔧 Dev Dependencies:");
    if manifest.dev_dependencies.is_empty() {
        println!("   No dev dependencies");
    } else {
        for (pkg, ver_req) in &manifest.dev_dependencies {
            let ver_req_clone = ver_req.clone();
            let resolved = lockfile.get(pkg).unwrap_or(&ver_req_clone);
            println!("   {} {} (locked at {})", "➤".blue(), pkg, resolved);
        }
    }
    
    Ok(())
}

pub async fn list_cache() -> anyhow::Result<()> {
    let cache_dir = crate::cache::get_cache_dir();
    let packages_dir = cache_dir.join("packages");
    // We'll keep this variable for future expansion but mark it as unused
    let _metadata_dir = cache_dir.join("metadata");
    
    println!("{}", "📦 Cached Packages:".blue());
    
    if packages_dir.exists() {
        let mut total_size = 0;
        let mut count = 0;
        
        for entry in fs::read_dir(packages_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Ok(metadata) = fs::metadata(&path) {
                let size_kb = metadata.len() / 1024;
                let modified = metadata.modified()?.elapsed()?.as_secs() / 86400; // days
                
                println!("  {} {} ({} KB, {} days old)", "➤".yellow(), 
                         path.file_name().unwrap().to_string_lossy(),
                         size_kb,
                         modified);
                         
                total_size += metadata.len();
                count += 1;
            }
        }
        
        println!("\n{}", format!("Total: {} packages, {:.2} MB", 
                 count, total_size as f64 / 1024.0 / 1024.0).green());
    } else {
        println!("  No cached packages found");
    }
    
    Ok(())
}

pub async fn clear_cache() -> anyhow::Result<()> {
    let cache_dir = crate::cache::get_cache_dir();
    
    println!("{}", "🧹 Clearing entire cache...".blue());
    
    if cache_dir.exists() {
        fs::remove_dir_all(&cache_dir)?;
        fs::create_dir_all(&cache_dir)?;
        fs::create_dir_all(cache_dir.join("packages"))?;
        fs::create_dir_all(cache_dir.join("metadata"))?;
        
        println!("{}", "✅ Cache cleared successfully".green());
    } else {
        println!("{}", "Cache directory does not exist".yellow());
    }
    
    Ok(())
}
