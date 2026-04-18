# TETR.IO Optimizer - Absurd Desktop Performance

A Tauri-based desktop wrapper that runs TETR.IO with performance optimizations your regular browser doesn't have. Basically it's TETR.IO, but faster, lighter, and without those annoying stutters.

## What is this?

You know TETR.IO, that online Tetris game everyone plays in the browser? Well, this project is an attempt to make a desktop app that runs the game directly, but with a bunch of tricks under the hood to make everything smoother.

## Features

- **Native GPU Acceleration:** Specific configurations for Mesa drivers (AMD/Intel) on Linux. Your graphics card actually works.
- **Tauri 2.0 Engine:** Uses less RAM and CPU than the official client or an open browser. It's magic, but with code.
- **Cross-Platform Build:** Runs on Linux and generates Windows executable directly from Arch. No VM, no headaches.

## Which version to download?

We have two main branches now. The project evolved and the structure lets you choose between stability and raw performance:

### Main Version (Stability) - **RECOMMENDED FOR MOST PEOPLE**
This is the version where the magic happens in a controlled way. It uses all optimizations but keeps FPS limited to 60Hz to avoid issues.

- **Focus:** Optimized performance with guaranteed stability.
- **Optimizations:** GPU acceleration, V-Sync bypass, DOM optimizations.
- **For who:** Everyone who wants to play without stutters but no surprises.

### Performance-Uncapped Version (Turbo)
This version is our "turbo mode". Removes all FPS limits and disables everything possible for maximum performance.

- **Focus:** Unlimited FPS for high refresh rate monitors (144Hz+).
- **Optimizations:** Everything from Main + `--disable-frame-rate-limit` + aggressive `vblank_mode=0`.
- **For who:** People with gaming monitors who want every possible frame.

**Tip:** If you don't know what refresh rate is or don't have a monitor above 60Hz, stick with Main - it's a sure thing.

## How to download

To download the application, visit our releases page:

**[Click here to see all versions (Releases)](https://github.com/siraprem/TetrioOptimizer/releases)**

**Golden tip:** Choose between Main (stable) or Performance-Uncapped (turbo) and be happy. If in doubt, go with Main.

## How to run and build

### Prerequisites
- Rust (latest stable version)
- Node.js 20+
- Tauri CLI (`npm install -g @tauri-apps/cli`)

### Build on Linux (for Linux)
```bash
# Get the code
git clone https://github.com/siraprem/TetrioOptimizer.git
cd TetrioOptimizer

# Install dependencies
npm install

# Normal build (Main branch)
npm run tauri build

# Or if you want the turbo version:
git checkout performance-uncapped
npm run tauri build
```

### Build for Windows (directly from Arch)
```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Install Mingw linker (on Arch)
sudo pacman -S mingw-w64-gcc

# Cross-compile build
npx tauri build --target x86_64-pc-windows-gnu
```

The Windows executable will appear at `target/x86_64-pc-windows-gnu/release/tetrio-optimizer.exe`.

## AI-Augmented Development

TETR.IO Optimizer uses an AI-Augmented development workflow. This allows me, as a developer, to focus on architecture and user experience, while using advanced language models to optimize implementation, diagnose performance issues, and accelerate resolution of platform-specific bugs.

## Project structure

```
TetrioOptimizer/
├── src-tauri/          # Tauri Rust code (where the magic happens)
├── public/             # Optimization scripts that get injected
├── .cargo/             # Cross-compile configurations
├── package.json        # Node dependencies
└── README.md           # This file here
```

## License

**MIT** - basically you can do whatever you want with the code, just don't sue me if something breaks, I recommend reading LICENSE.md

## Legal notice

**This is an unofficial project.** TETR.IO belongs to the original creators. This app has no affiliation with the TETR.IO team, it's just a wrapper made by a fan for fans. Please don't strike me, I love the game.

If you can, support osk (the game dev) on the official site! The guy maintains the servers with ads and donations.
