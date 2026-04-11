#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::sync::Mutex;

use tauri::{Manager, WindowEvent};
use tracing::{info, warn, error};

mod memory_manager;
mod webgl_optimizer;

use memory_manager::MemoryManager;
use webgl_optimizer::WebGLOptimizer;

#[tauri::command]
fn get_system_info() -> Result<String, String> {
    use sysinfo::System;
    
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let info = format!(
        "System: {} {}\nCPU: {} cores\nMemory: {}/{} MB",
        System::name().unwrap_or_else(|| "Unknown".to_string()),
        System::os_version().unwrap_or_else(|| "Unknown".to_string()),
        sys.cpus().len(),
        sys.used_memory() / 1024 / 1024,
        sys.total_memory() / 1024 / 1024,
    );
    
    Ok(info)
}

#[tauri::command]
fn optimize_webgl(flags: Vec<String>) -> Result<Vec<String>, String> {
    let optimizer = WebGLOptimizer::new();
    optimizer.apply_flags(flags)
}

#[tauri::command]
fn clean_memory_cache() -> Result<String, String> {
    MemoryManager::clean_cache()
}

fn main() {
    // Set environment variable to stabilize WebKit on Linux
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "0");
    
    // Setup logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting TETR.IO Optimizer...");
    info!("WEBKIT_DISABLE_COMPOSITING_MODE=0 (Linux stability)");

    // Ensure data directory exists for persistent storage
    let data_dir = "./tetrio_data";
    if let Err(e) = std::fs::create_dir_all(data_dir) {
        warn!("Failed to create data directory: {}", e);
    } else {
        info!("Data directory ready: {}", data_dir);
    }

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            
            // Configure User-Agent via JavaScript injection
            // This is a workaround since Tauri 2.0 doesn't support userAgent in config
            window.eval(&format!(
                "Object.defineProperty(navigator, 'userAgent', {{ value: '{}', configurable: true }}); console.log('User-Agent set to Chrome 120');",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
            )).unwrap_or_else(|e| warn!("Failed to set User-Agent: {}", e));
            
            // Log startup info
            info!("TETR.IO Optimizer starting...");
            info!("User-Agent configured: Chrome 120.0.0.0");
            info!("Data directory: ./tetrio_data (cookies/localStorage will persist here)");
            
            // Initialize memory manager
            let memory_manager = Arc::new(Mutex::new(MemoryManager::new()));
            app.manage(memory_manager.clone());
            
            // Cache cleaning is now manual only via the clean_memory_cache command
            // No background tasks to prevent shutdown blocking
            
            // Apply stable WebGL optimizations (removed problematic flags for Linux)
            let optimizer = WebGLOptimizer::new();
            let flags = vec![
                // Removed: --disable-gpu-vsync (causes segfault)
                // Removed: --enable-webgl2-compute-context (causes segfault)
                "--enable-gpu-rasterization".to_string(), // Keep - working well
                // Removed: --enable-zero-copy (causes segfault)
                "--disable-software-rasterizer".to_string(),
                "--max-active-webgl-contexts=16".to_string(),
                "--disable-frame-rate-limit".to_string(),
            ];
            
            if let Err(e) = optimizer.apply_flags(flags) {
                warn!("Failed to apply some WebGL flags: {}", e);
            }
            
            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    info!("Window closing, forcing process exit...");
                    
                    // Allow the window to close
                    api.prevent_close();
                    
                    // Force exit the process immediately
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            optimize_webgl,
            clean_memory_cache
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
