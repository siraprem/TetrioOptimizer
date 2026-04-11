use std::collections::HashMap;
use tracing::{info, warn};

pub struct WebGLOptimizer {
    available_flags: HashMap<String, String>,
}

impl WebGLOptimizer {
    pub fn new() -> Self {
        let mut flags = HashMap::new();
        
        // WebGL optimization flags
        flags.insert("--disable-gpu-vsync".to_string(), "Disables VSync for unlimited FPS".to_string());
        flags.insert("--enable-webgl2-compute-context".to_string(), "Enables WebGL 2.0 Compute".to_string());
        flags.insert("--enable-gpu-rasterization".to_string(), "Uses GPU for rasterization".to_string());
        flags.insert("--enable-zero-copy".to_string(), "Enables zero-copy buffers".to_string());
        flags.insert("--disable-software-rasterizer".to_string(), "Disables software fallback".to_string());
        flags.insert("--max-active-webgl-contexts=16".to_string(), "Increases WebGL context limit".to_string());
        flags.insert("--disable-frame-rate-limit".to_string(), "Removes FPS cap".to_string());
        flags.insert("--enable-webgl-image-chromium".to_string(), "Optimizes WebGL image handling".to_string());
        flags.insert("--enable-features=VaapiVideoDecoder".to_string(), "Hardware video decoding".to_string());
        flags.insert("--ignore-gpu-blocklist".to_string(), "Uses all GPU features".to_string());
        flags.insert("--enable-gpu-benchmarking".to_string(), "Enables GPU benchmarking".to_string());
        flags.insert("--disable-gpu-driver-bug-workarounds".to_string(), "Disables driver workarounds".to_string());
        flags.insert("--force_high_performance_gpu".to_string(), "Forces high-performance GPU".to_string());
        flags.insert("--disable-low-end-device-mode".to_string(), "Disables low-end optimizations".to_string());
        flags.insert("--enable-hardware-overlays".to_string(), "Enables hardware overlays".to_string());
        
        // TETR.IO specific optimizations
        flags.insert("--canvas-oop-rasterization".to_string(), "Canvas out-of-process rasterization".to_string());
        flags.insert("--num-raster-threads=4".to_string(), "Increases raster threads".to_string());
        flags.insert("--enable-parallel-downloading".to_string(), "Parallel downloading".to_string());
        flags.insert("--disable-background-timer-throttling".to_string(), "No background throttling".to_string());
        
        Self {
            available_flags: flags,
        }
    }
    
    pub fn apply_flags(&self, flags: Vec<String>) -> Result<Vec<String>, String> {
        let mut applied = Vec::new();
        let mut failed = Vec::new();
        
        for flag in flags {
            if self.available_flags.contains_key(&flag) {
                info!("Applying WebGL flag: {}", flag);
                applied.push(flag.clone());
                
                // In a real implementation, these would be passed to the WebView
                // For now, we just log them
                match flag.as_str() {
                    "--disable-gpu-vsync" => {
                        info!("VSync disabled - Unlimited FPS enabled");
                    }
                    "--enable-webgl2-compute-context" => {
                        info!("WebGL 2.0 Compute context enabled");
                    }
                    "--enable-gpu-rasterization" => {
                        info!("GPU rasterization enabled");
                    }
                    "--enable-zero-copy" => {
                        info!("Zero-copy buffers enabled");
                    }
                    _ => {
                        info!("Flag applied: {}", flag);
                    }
                }
            } else {
                warn!("Unknown WebGL flag: {}", flag);
                failed.push(flag);
            }
        }
        
        if !failed.is_empty() {
            Err(format!("Failed to apply flags: {:?}", failed))
        } else {
            Ok(applied)
        }
    }
    
    pub fn get_recommended_flags(&self) -> Vec<String> {
        vec![
            "--disable-gpu-vsync".to_string(),
            "--enable-webgl2-compute-context".to_string(),
            "--enable-gpu-rasterization".to_string(),
            "--enable-zero-copy".to_string(),
            "--disable-software-rasterizer".to_string(),
            "--max-active-webgl-contexts=16".to_string(),
            "--disable-frame-rate-limit".to_string(),
            "--enable-webgl-image-chromium".to_string(),
            "--ignore-gpu-blocklist".to_string(),
            "--force_high_performance_gpu".to_string(),
            "--canvas-oop-rasterization".to_string(),
            "--num-raster-threads=4".to_string(),
        ]
    }
    
    pub fn get_flag_description(&self, flag: &str) -> Option<&String> {
        self.available_flags.get(flag)
    }
    
    pub fn list_all_flags(&self) -> Vec<(String, String)> {
        self.available_flags
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}