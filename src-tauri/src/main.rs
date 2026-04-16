use tauri::Manager;

fn main() {
    #[cfg(target_os = "linux")]
    {
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "0");
        std::env::set_var("WEBKIT_FORCE_COMPOSITING_MODE", "1");
    }

    tauri::Builder::default()
        .setup(|app| {
            // Obter a janela principal que já existe
            let _window = app
                .get_webview_window("main")
                .expect("no main window found");

            // Injetar script após um delay
            let window_clone = _window.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(3));
                let _ = window_clone.eval(include_str!("../../public/optimizer.js"));
                println!("[TETR.IO Optimizer] ✅ Performance Scripts Injected");
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
