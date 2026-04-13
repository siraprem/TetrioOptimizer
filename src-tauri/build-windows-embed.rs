// Script para tentar embutir DLL no executável Windows
// Nota: Esta é uma abordagem complexa e pode não funcionar com cross-compiling GNU

fn main() {
    // Esta é uma abordagem experimental para embutir DLLs
    // No Windows, poderíamos usar recursos do executável
    
    println!("cargo:rerun-if-changed=WebView2Loader.dll");
    
    #[cfg(target_os = "windows")]
    {
        // Tentar embutir a DLL como recurso
        // Isso requer a DLL estar presente durante a compilação
        if std::path::Path::new("WebView2Loader.dll").exists() {
            println!("cargo:rustc-link-arg=/EMBED:WebView2Loader.dll");
        } else {
            println!("cargo:warning=WebView2Loader.dll não encontrado para embedding");
            println!("cargo:warning=O executável funcionará apenas se a DLL estiver no PATH");
        }
    }
}