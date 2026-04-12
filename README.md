# TETR.IO Ultra Optimized Wrapper

A high-performance wrapper for TETR.IO with integrated adblocker, VSync configuration, and WebGL optimizations.

## ✨ Features

### 🛡️ Integrated Adblocker
- **Automatic ad removal**: Blocks and removes advertising elements from TETR.IO
- **Request blocking**: Intercepts and blocks requests to advertising domains
- **Dynamic detection**: Continuously monitors for new ad elements
- **Performance boost**: Reduces CPU/GPU usage by eliminating ad rendering

### ⚙️ VSync Configuration Wizard
- **Mandatory setup**: VSync configuration is required before game launch
- **Standard rates**: Predefined options (60, 75, 120, 144, 165, 240 Hz)
- **Custom values**: Support for custom refresh rates with safety warnings
- **Frame rate limiting**: Ensures stable performance matching selected VSync

### 🚀 Performance Optimizations
- **WebGL enhancements**: Optimized rendering pipeline for smoother gameplay
- **Memory management**: Intelligent cache cleaning and memory optimization
- **System compatibility**: Cross-platform stability improvements
- **User-Agent spoofing**: Ensures compatibility with TETR.IO

## 🚀 Installation

### Prerequisites
- Rust (latest stable)
- Node.js 18+ (for Tauri dependencies)
- System dependencies for Tauri (see [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

### Build from Source

```bash
# Clone the repository
git clone https://github.com/siraprem/TetrioOptimizer.git
cd TetrioOptimizer

# Install dependencies
npm install

# Build the application
cd src-tauri
cargo build --release
```

### Development

```bash
# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## 🎮 Usage

1. **Launch the application**
2. **VSync Configuration** (mandatory first step):
   - Select your monitor's refresh rate from the dropdown
   - Or enter a custom value (with caution)
   - Click "Confirm & Launch Game"

3. **Game Launch**:
   - The main TETR.IO window opens with optimal settings
   - Adblocker automatically activates
   - VSync is applied for smooth gameplay

4. **Performance Controls**:
   - Use system tray menu for quick actions
   - Access optimization settings
   - Monitor system performance

## 🔧 Technical Details

### Adblocker Implementation
- **Script injection**: JavaScript injected via Tauri's webview API
- **DOM monitoring**: MutationObserver detects and removes new ad elements
- **Request interception**: Overrides fetch() and XMLHttpRequest
- **Selective blocking**: Targets known ad domains and patterns

### VSync System
- **State management**: Rust backend stores VSync configuration
- **Frame limiting**: JavaScript-based frame rate control
- **Window coordination**: Setup window must complete before main window
- **Validation**: Input validation for safe custom values

### Performance Features
- **WebGL flags**: Optimized Chrome/WebKit rendering flags
- **Memory optimization**: Automatic and manual cache management
- **Platform-specific**: Linux stability improvements
- **Persistent storage**: Game data saved locally

## 📁 Project Structure

```
TetrioOptimizer/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs          # Main application logic
│   │   ├── memory_manager.rs # Memory optimization
│   │   └── webgl_optimizer.rs # WebGL enhancements
│   ├── adblocker.js         # Adblocker script
│   ├── setup.html           # VSync configuration UI
│   └── build.rs             # Build configuration
├── tauri.conf.json          # Tauri configuration
├── Cargo.toml              # Rust dependencies
├── package.json            # Node.js dependencies
└── README.md               # This file
```

## 🛠️ Configuration

### Customizing Adblocker
Edit `src-tauri/adblocker.js` to:
- Add/remove ad domains
- Modify CSS selectors for ad elements
- Adjust detection sensitivity

### VSync Settings
Modify `src-tauri/src/main.rs` to:
- Change default VSync options
- Adjust frame limiting behavior
- Modify validation rules

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This project is not affiliated with TETR.IO or osk. It is a third-party wrapper that provides additional features and optimizations. Use at your own risk.

## 🐛 Troubleshooting

### Common Issues

**Build fails on Linux:**
```bash
# Install required system dependencies
sudo apt-get update
sudo apt-get install libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**Adblocker not working:**
- Check browser console for errors
- Verify script injection in main.rs
- Update ad selectors in adblocker.js

**VSync configuration not saving:**
- Check file permissions for data directory
- Verify state management in AppState
- Ensure proper window event handling

### Debug Mode

Enable detailed logging:
```bash
RUST_LOG=debug npm run tauri dev
```

## 🔄 Changelog

### v0.2.0 (Current)
- Added integrated adblocker
- Implemented VSync configuration wizard
- Improved window management
- Enhanced performance optimizations
- Added custom VSync value support

### v0.1.0
- Initial release
- Basic TETR.IO wrapper
- WebGL optimizations
- Memory management
- Cross-platform support

## 📞 Support

For issues and feature requests, please use the [GitHub Issues](https://github.com/siraprem/TetrioOptimizer/issues) page.