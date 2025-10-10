# 🎶 RSIAD - Vocal Warmup Exercise Generator 🎶

[![CI](https://github.com/ioma8/rsiad/actions/workflows/ci.yml/badge.svg)](https://github.com/ioma8/rsiad/actions/workflows/ci.yml)

A professional vocal warmup tool for singers, especially for opera and classical training. 🎤 Generates musical exercises (triads, scales, octaves) with high-quality synthesized piano accompaniment.

## ✨ Features

- **Three Interface Options**:
  - 🖥️ **Desktop GUI** - Native app with beautiful Catppuccin theme
  - 🌐 **Web Interface** - Mobile-friendly, works on any device
  - ⌨️ **Command Line** - For automation and scripting

- **Three Exercise Types**:
  - 🎵 **Triads** - Root → 3rd → 5th → 3rd → Root → Chord
  - 🎼 **Scales** - Major scale to fifth and back with chord
  - 🎹 **Octaves** - Octave jumps with both notes together

- **Six Vocal Range Presets**:
  - Bass (E2-E4), Baritone (A2-A4), Tenor (C3-C5)
  - Alto (F3-F5), Mezzo-Soprano (A3-A5), Soprano (C4-C6)

- **Cross-Platform**:
  - ✅ macOS (native .app bundle)
  - ✅ Linux (desktop app)
  - ✅ Windows (desktop app)
  - ✅ iOS (web/PWA)
  - ✅ Android (web/PWA)

## 🚀 Quick Start

### Desktop GUI (Recommended)

**macOS:**
```bash
# Build and install
./build-macos-app.sh
open dist/RSIAD.app
```

**Linux/Windows:**
```bash
cargo build --release --bin rsiad-gui
./target/release/rsiad-gui
```

### Web Interface (Mobile-Friendly)

```bash
# Start web server
cargo build --release --bin rsiad-web --features web-server
./target/release/rsiad-web

# Open browser to http://localhost:3000
```

### Command Line

```bash
# Build
cargo build --release --bin rsiad

# Run with default settings (Baritone, Triads)
./target/release/rsiad

# Save to MP3
./target/release/rsiad --save output.mp3

# Custom exercise
./target/release/rsiad -e scales -r soprano -d 1.0
```

## 📖 Documentation

- **[USAGE.md](USAGE.md)** - Complete usage guide for all platforms
- **[BUILD.md](BUILD.md)** - Building from source for all platforms
- **[BUILD_APPLE.md](BUILD_APPLE.md)** - macOS/iOS specific builds
- **[BUILD_MOBILE.md](BUILD_MOBILE.md)** - Mobile deployment (iOS/Android)
- **[GUI_README.md](GUI_README.md)** - Desktop GUI guide

## 📥 Downloads

Pre-built binaries and pre-generated MP3 exercises are available from [GitHub Releases](https://github.com/ioma8/rsiad/releases).

## 💻 Platform Support

| Platform | Desktop GUI | Web Interface | CLI |
|----------|-------------|---------------|-----|
| macOS    | ✅ Native App | ✅ | ✅ |
| Linux    | ✅ Native App | ✅ | ✅ |
| Windows  | ✅ Native App | ✅ | ✅ |
| iOS      | 🌐 Web/PWA | ✅ | ❌ |
| Android  | 🌐 Web/PWA | ✅ | ❌ |

## ⌨️ CLI Quick Reference

```bash
# Exercise types
rsiad -e triads    # Triads (default)
rsiad -e scales    # Major scales
rsiad -e octaves   # Octave jumps

# Vocal ranges
rsiad -r bass      # E2-E4
rsiad -r baritone  # A2-A4 (default)
rsiad -r tenor     # C3-C5
rsiad -r alto      # F3-F5
rsiad -r mezzo     # A3-A5
rsiad -r soprano   # C4-C6

# Custom range
rsiad -f C3 -t C5  # Custom MIDI range

# Duration
rsiad -d 1.2       # 1.2 seconds per note

# Save to file
rsiad -s output.mp3

# Combine options
rsiad -e scales -r alto -d 1.0 -s alto-scales.mp3
```

## 🏗️ Building

### Prerequisites

- Rust 1.70+ ([Install](https://rustup.rs/))
- C compiler (for fluidlite)
- Linux only: ALSA development libraries

### Build All Versions

```bash
# Clone repository
git clone https://github.com/ioma8/rsiad
cd rsiad

# Build desktop GUI
cargo build --release --bin rsiad-gui

# Build web server
cargo build --release --bin rsiad-web --features web-server

# Build CLI
cargo build --release --bin rsiad

# macOS: Build .app bundle
./build-macos-app.sh
```

See [BUILD.md](BUILD.md) for detailed instructions.

## 🎵 How to Use

1. **Desktop GUI**: Launch the app, select exercise type, range, and press Play or Save MP3
2. **Web Interface**: Open browser, configure settings, play or download
3. **CLI**: Run from terminal with desired options

All three interfaces provide the same functionality with different user experiences.

## 🛠️ Architecture

RSIAD is a library-first design with three frontends:

```
┌─────────────────────────────────────┐
│         rsiad (library)             │
│  - VocalExerciseEngine              │
│  - Audio synthesis (fluidlite)      │
│  - MP3 encoding                     │
└─────────┬───────────────────────────┘
          │
    ┌─────┴──────┐
    │            │
┌───▼────┐  ┌───▼────┐  ┌───▼────┐
│  CLI   │  │  GUI   │  │  Web   │
│ (rsiad)│  │ (egui) │  │ (axum) │
└────────┘  └────────┘  └────────┘
```

## 📦 What's Included

- **src/lib.rs** - Core library with VocalExerciseEngine API
- **src/main.rs** - Command-line interface
- **src/gui.rs** - Desktop GUI (egui)
- **src/web_server.rs** - Web server with REST API
- **static/index.html** - Mobile-responsive web UI
- **Build scripts** - Platform-specific build automation

## 🤝 Contributing

Contributions welcome! Please open an issue or pull request.

## 📄 License

This project is open source. See LICENSE file for details.

## 🙏 Credits

- Audio synthesis: [fluidlite](https://github.com/divideconcept/FluidLite)
- MP3 encoding: [mp3lame-encoder](https://crates.io/crates/mp3lame-encoder)
- Desktop GUI: [egui](https://github.com/emilk/egui)
- Web server: [axum](https://github.com/tokio-rs/axum)
- Theme: [Catppuccin](https://github.com/catppuccin/catppuccin)

## 📞 Support

- GitHub Issues: [Report bugs or request features](https://github.com/ioma8/rsiad/issues)
- Discussions: [Ask questions or share ideas](https://github.com/ioma8/rsiad/discussions)

---

**Enjoy your vocal warmup practice! 🎵**