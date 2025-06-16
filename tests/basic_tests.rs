#[cfg(test)]
mod tests {
    use crate::cache;
    use crate::lockfile;
    use crate::manifest;

    #[test]
    fn test_lockfile_read_write() {
        // Create a new lockfile entry
        lockfile::add_dependency("test-package", "1.0.0").unwrap();
        
        // Read it back
        let deps = lockfile::read_dependencies().unwrap();
        
        // Check if it's there
        assert!(deps.contains_key("test-package"));
        assert_eq!(deps.get("test-package").unwrap(), "1.0.0");
        
        // Clean up
        lockfile::remove_dependency("test-package").unwrap();
        let deps_after = lockfile::read_dependencies().unwrap();
        assert!(!deps_after.contains_key("test-package"));
    }

    #[test]
    fn test_manifest_read_write() {
        // Create a test manifest
        let test_name = "test-project";
        manifest::init(test_name).unwrap();
        
        // Read it back
        let manifest = manifest::read_manifest().unwrap();
        
        // Check values
        assert_eq!(manifest.name, test_name);
        assert_eq!(manifest.version, "0.1.0");
        
        // Clean up by overwriting
        let mut manifest = manifest::read_manifest().unwrap();
        manifest.name = "cleanup".to_string();
        manifest::write_manifest(&manifest).unwrap();
    }

    #[test]
    fn test_cache_dirs() {
        // Ensure cache directories exist
        cache::ensure_cache_dirs().unwrap();
        
        // Check if they were created
        let cache_dir = cache::get_cache_dir();
        assert!(cache_dir.exists());
        assert!(cache_dir.join("packages").exists());
        assert!(cache_dir.join("metadata").exists());
    }
}
