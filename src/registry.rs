use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::io::Cursor;
use tar::Archive;
use flate2::read::GzDecoder;
use colored::Colorize;

#[derive(Deserialize, Debug)]
pub struct NpmDist {
    pub tarball: String,
    #[serde(default)]
    pub shasum: String,
    #[serde(default)]
    pub integrity: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct NpmVersion {
    pub dist: NpmDist,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub author: Option<serde_json::Value>, // Can be a string or object
}

#[derive(Deserialize, Debug)]
pub struct NpmResponse {
    #[serde(rename = "dist-tags")]
    pub dist_tags: std::collections::HashMap<String, String>,
    pub versions: std::collections::HashMap<String, NpmVersion>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

pub async fn get_latest_version(pkg: &str) -> anyhow::Result<String> {
    let url = format!("https://registry.npmjs.org/{}", pkg);
    
    // Handle large response with a timeout and retries
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    
    let resp: NpmResponse = client.get(url)
        .send().await?
        .json().await
        .map_err(|e| anyhow::anyhow!("Failed to parse registry response: {}", e))?;
    
    // Get the latest version from dist_tags, or return an error if "latest" not found
    resp.dist_tags.get("latest")
        .ok_or_else(|| anyhow::anyhow!("No 'latest' version found for package {}", pkg))
        .map(|v| v.clone())
}

pub async fn download_and_unpack(pkg: &str, version: &str) -> anyhow::Result<()> {
    // Form URLs and paths
    let url = format!("https://registry.npmjs.org/{}", pkg);
    let cache_dir = crate::cache::get_cache_dir();
    let package_cache = cache_dir.join("packages");
    let tarball_path = package_cache.join(format!("{}-{}.tgz", pkg.replace("/", "-"), version));
    
    // Create directory structure if it doesn't exist
    fs::create_dir_all(&package_cache)?;
    fs::create_dir_all("node_modules")?;
    
    // First check if we have a cached version
    let tarball_bytes = if tarball_path.exists() {
        println!("{}", format!("📦 Using cached version of {}@{}", pkg, version).blue());
        fs::read(&tarball_path)?
    } else {
        // If not in cache, download it
        println!("{}", format!("⬇️ Downloading {}@{}", pkg, version).yellow());
        
        // First get the full package info to find the tarball URL
        let resp: NpmResponse = Client::new().get(&url)
            .send().await?
            .json().await?;
        
        // Get the version data, or return an error if version not found
        let version_data = resp.versions.get(version)
            .ok_or_else(|| anyhow::anyhow!("Version {} not found for package {}", version, pkg))?;
            
        let tarball_url = &version_data.dist.tarball;
        let bytes = Client::new().get(tarball_url)
            .send().await?
            .bytes().await?;
        
        // Save to cache
        fs::write(&tarball_path, &bytes)?;
        bytes.to_vec()
    };
    
    // Create a properly normalized package path
    let pkg_path = std::path::Path::new("node_modules").join(pkg);
    let parent_dir = pkg_path.parent().unwrap_or_else(|| std::path::Path::new("node_modules"));
    
    // Make sure the parent directory exists
    fs::create_dir_all(parent_dir)?;
    
    // Remove existing package directory if it exists to avoid conflicts
    if pkg_path.exists() {
        // On Windows, attempting to remove a directory that's in use can fail
        // We'll attempt it and continue if it succeeds
        let _ = fs::remove_dir_all(&pkg_path);
    }
    
    // Create the package directory
    fs::create_dir_all(&pkg_path)?;
    
    // Unpack the tarball
    println!("{}", format!("📦 Unpacking {}@{}", pkg, version).green());
    let gz = GzDecoder::new(Cursor::new(tarball_bytes));
    let mut archive = Archive::new(gz);
    
    // Some packages use different folder structures, so we need to handle multiple cases
    let common_prefixes = ["package/", "package", "/"];
    
    // Get the entries to process
    let entries = match archive.entries() {
        Ok(entries) => entries,
        Err(e) => return Err(anyhow::anyhow!("Failed to read tarball entries: {}", e)),
    };
    
    // Process each entry in the tarball
    for entry in entries {
        let mut entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Warning: Failed to read entry in tarball: {}", e);
                continue;
            }
        };
        
        let path = match entry.path() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Warning: Failed to get path for entry: {}", e);
                continue;
            }
        };
        
        // Try to strip common prefixes that npm packages use
        let rel_path = common_prefixes.iter()
            .find_map(|prefix| path.strip_prefix(prefix).ok())
            .unwrap_or(&path);
            
        if rel_path.to_string_lossy().is_empty() {
            continue; // Skip empty paths
        }
        
        let target_path = pkg_path.join(rel_path);
        
        // Ensure the parent directory exists
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Unpack the file
        if let Err(e) = entry.unpack(&target_path) {
            eprintln!("Warning: Failed to unpack {}: {}", target_path.display(), e);
            // Continue unpacking other files rather than failing completely
        }
    }
    
    Ok(())
}

#[derive(Deserialize)]
pub struct NpmSearchResult {
    pub name: String,
}

#[derive(Deserialize)]
pub struct NpmSearchResponse {
    pub objects: Vec<NpmSearchObj>,
}

#[derive(Deserialize)]
pub struct NpmSearchObj {
    pub package: NpmSearchResult,
}

pub async fn search_package(query: &str) -> anyhow::Result<Vec<String>> {
    let url = format!("https://registry.npmjs.org/-/v1/search?text={}", query);
    
    // Set a timeout for the search request
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;
    
    let resp = client.get(url).send().await?;
    
    // Check response status
    if !resp.status().is_success() {
        return Err(anyhow::anyhow!("Search request failed with status: {}", resp.status()));
    }
    
    let search_resp: NpmSearchResponse = resp.json().await
        .map_err(|e| anyhow::anyhow!("Failed to parse search results: {}", e))?;
    
    Ok(search_resp.objects.into_iter().map(|o| o.package.name).collect())
}
