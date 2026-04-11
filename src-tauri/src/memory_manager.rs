use std::process::Command;
use tracing::{info, warn};

pub struct MemoryManager;

impl MemoryManager {
    pub fn new() -> Self {
        Self
    }
    
    pub fn clean_cache() -> Result<String, String> {
        info!("Cleaning WebView cache...");
        
        // Platform-specific cache cleaning
        #[cfg(target_os = "windows")]
        {
            Self::clean_windows_cache()?;
        }
        
        #[cfg(target_os = "linux")]
        {
            Self::clean_linux_cache()?;
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::clean_macos_cache()?;
        }
        
        // Clear DNS cache
        Self::clear_dns_cache()?;
        
        Ok("Cache cleaned successfully".to_string())
    }
    
    #[cfg(target_os = "windows")]
    fn clean_windows_cache() -> Result<(), String> {
        use std::env;
        
        // Clear WebView2 cache
        let local_app_data = env::var("LOCALAPPDATA")
            .map_err(|e| format!("Failed to get LOCALAPPDATA: {}", e))?;
        
        let webview_cache = format!("{}\\Microsoft\\Edge\\User Data\\Default\\Cache", local_app_data);
        
        if std::path::Path::new(&webview_cache).exists() {
            match std::fs::remove_dir_all(&webview_cache) {
                Ok(_) => info!("Cleared WebView2 cache at: {}", webview_cache),
                Err(e) => warn!("Failed to clear WebView2 cache: {}", e),
            }
        }
        
        // Run disk cleanup
        Command::new("cleanmgr")
            .args(["/sagerun:1"])
            .output()
            .map_err(|e| format!("Failed to run cleanmgr: {}", e))?;
            
        Ok(())
    }
    
    #[cfg(target_os = "linux")]
    fn clean_linux_cache() -> Result<(), String> {
        // Clear WebKit cache
        let cache_dirs = [
            format!("{}/.cache/tetrio-optimizer", std::env::var("HOME").unwrap_or_default()),
            format!("{}/.cache/webkit", std::env::var("HOME").unwrap_or_default()),
            format!("{}/.cache/org.webkit", std::env::var("HOME").unwrap_or_default()),
        ];
        
        for dir in &cache_dirs {
            if std::path::Path::new(dir).exists() {
                match std::fs::remove_dir_all(dir) {
                    Ok(_) => info!("Cleared cache at: {}", dir),
                    Err(e) => warn!("Failed to clear cache at {}: {}", dir, e),
                }
            }
        }
        
        // Clear system cache
        Command::new("sync")
            .output()
            .map_err(|e| format!("Failed to sync: {}", e))?;
            
        Command::new("echo")
            .args(["3", ">", "/proc/sys/vm/drop_caches"])
            .output()
            .map_err(|e| format!("Failed to drop caches: {}", e))?;
            
        Ok(())
    }
    
    #[cfg(target_os = "macos")]
    fn clean_macos_cache() -> Result<(), String> {
        // Clear WebKit cache on macOS
        let cache_dir = format!("{}/Library/Caches/WebKit", std::env::var("HOME").unwrap_or_default());
        
        if std::path::Path::new(&cache_dir).exists() {
            match std::fs::remove_dir_all(&cache_dir) {
                Ok(_) => info!("Cleared WebKit cache at: {}", cache_dir),
                Err(e) => warn!("Failed to clear WebKit cache: {}", e),
            }
        }
        
        Ok(())
    }
    
    fn clear_dns_cache() -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            Command::new("ipconfig")
                .args(["/flushdns"])
                .output()
                .map_err(|e| format!("Failed to flush DNS: {}", e))?;
        }
        
        #[cfg(target_os = "linux")]
        {
            // Try systemd-resolve first
            let _ = Command::new("systemd-resolve")
                .args(["--flush-caches"])
                .output();
                
            // Try nscd
            let _ = Command::new("nscd")
                .args(["-i", "hosts"])
                .output();
        }
        
        #[cfg(target_os = "macos")]
        {
            Command::new("dscacheutil")
                .args(["-flushcache"])
                .output()
                .map_err(|e| format!("Failed to flush DNS cache: {}", e))?;
        }
        
        Ok(())
    }
    
    pub fn get_memory_usage() -> Result<u64, String> {
        use sysinfo::System;
        
        let mut sys = System::new_all();
        sys.refresh_all();
        
        Ok(sys.used_memory())
    }
}