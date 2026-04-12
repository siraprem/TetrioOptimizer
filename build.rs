fn main() {
    // Build script vazio - Tauri 2.0 não precisa mais disso
    println!("cargo:rerun-if-changed=tauri.conf.json");
}