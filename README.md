# TETR.IO Optimizer

A Tauri application that loads TETR.IO in a webview and injects performance optimization scripts.

## 🚀 Build & Release

This project uses GitHub Actions for automated builds. When you push a tag starting with `v` (e.g., `v1.0.0`), the workflow will:

1. **Build for Windows** (MSVC) - Creates `.msi` and `.exe` installers
2. **Build for Linux** - Creates `.deb` and `.AppImage` packages
3. **Create GitHub Release** with all artifacts

### Prerequisites for Local Development

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (v20+)
# Install Tauri CLI
npm install @tauri-apps/cli

# Install system dependencies
# Ubuntu/Debian:
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

### Local Build Commands

```bash
# Development
npm run tauri dev

# Build for current platform
npm run tauri build

# Build for specific target
npm run tauri build -- --target x86_64-pc-windows-msvc
```

## 📦 Distribution

The application is distributed as:
- **Windows**: `.msi` installer (with WebView2 bootstrapper)
- **Windows**: `.exe` installer (NSIS)
- **Linux**: `.deb` package
- **Linux**: `.AppImage` portable

## 🔧 Windows WebView2 Requirements

The Windows installer includes a WebView2 bootstrapper. If users encounter WebView2 issues, refer to [docs/WINDOWS_TROUBLESHOOTING.md](docs/WINDOWS_TROUBLESHOOTING.md) for troubleshooting steps.

## 📄 License

MIT