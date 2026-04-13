use tauri::{WebviewUrl, WebviewWindowBuilder};

fn main() {
    println!("[TETR.IO Optimizer] Starting...");
    
    tauri::Builder::default()
        .setup(|app| {
            let window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://tetr.io".parse().unwrap()),
            )
            .title("TETR.IO Optimizer")
            .inner_size(1280.0, 720.0)
            .min_inner_size(800.0, 600.0)
            .build()?;
            
            println!("[TETR.IO Optimizer] ✅ Main window created!");
            
            // Inject the optimizer script after the window loads
            let window_ = window.clone();
            window.on_webview_ready(move |_| {
                println!("[TETR.IO Optimizer] TETR.IO should now be loading...");
                
                // Wait a bit for the page to load, then inject our script
                std::thread::sleep(std::time::Duration::from_millis(2000));
                
                if let Err(e) = window_.eval(include_str!("../adblocker.js")) {
                    eprintln!("[TETR.IO Optimizer] Failed to inject script: {}", e);
                } else {
                    println!("[TETR.IO Optimizer] ✅ Optimizer script injected!");
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}