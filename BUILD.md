# RSIAD - Build Guide

Complete guide for building RSIAD on all supported platforms.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Build](#quick-build)
- [Build by Platform](#build-by-platform)
  - [macOS](#macos)
  - [Linux](#linux)
  - [Windows](#windows)
  - [Mobile (iOS/Android)](#mobile-iosandroid)
- [Build Options](#build-options)
- [Cross-Compilation](#cross-compilation)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

### All Platforms

1. **Rust** (1.70 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Or download from [rustup.rs](https://rustup.rs/)

2. **Git**
   ```bash
   # macOS
   xcode-select --install
   
   # Linux
   sudo apt-get install git  # Ubuntu/Debian
   sudo dnf install git      # Fedora
   
   # Windows
   # Download from https://git-scm.com/
   ```

3. **C Compiler** (for fluidlite dependency)
   ```bash
   # macOS
   xcode-select --install
   
   # Linux
   sudo apt-get install build-essential  # Ubuntu/Debian
   sudo dnf groupinstall "Development Tools"  # Fedora
   
   # Windows
   # Install Visual Studio Build Tools from:
   # https://visualstudio.microsoft.com/downloads/
   ```

### Platform-Specific

**Linux:**
```bash
# ALSA development libraries (required for audio)
sudo apt-get install libasound2-dev pkg-config  # Ubuntu/Debian
sudo dnf install alsa-lib-devel                  # Fedora
sudo pacman -S alsa-lib                          # Arch
```

**Windows:**
- Visual Studio 2019 or later (for C++ build tools)
- Or Visual Studio Build Tools

---

## Quick Build

```bash
# Clone the repository
git clone https://github.com/ioma8/rsiad
cd rsiad

# Download soundfont (if not included)
# Place your .sf2 file in the project root

# Build everything (all three interfaces)
cargo build --release

# Build specific versions:
cargo build --release --bin rsiad          # CLI
cargo build --release --bin rsiad-gui      # Desktop GUI
cargo build --release --bin rsiad-web --features web-server  # Web server
```

**Binaries will be in:** `target/release/`

---

## Build by Platform

### macOS

#### Option 1: Build Desktop App Bundle (Recommended)

```bash
# Clone repository
git clone https://github.com/ioma8/rsiad
cd rsiad

# Build macOS .app bundle
chmod +x build-macos-app.sh
./build-macos-app.sh

# Install to Applications
cp -r dist/RSIAD.app /Applications/

# Run
open /Applications/RSIAD.app
```

**Result:**
- `dist/RSIAD.app` - macOS application bundle
- `dist/macos-binaries/rsiad-gui` - Standalone GUI
- `dist/macos-binaries/rsiad` - CLI tool

#### Option 2: Build Individual Binaries

```bash
# Desktop GUI
cargo build --release --bin rsiad-gui
./target/release/rsiad-gui

# Web server
cargo build --release --bin rsiad-web --features web-server
./target/release/rsiad-web

# CLI
cargo build --release --bin rsiad
./target/release/rsiad --help
```

#### Build for Different Architectures

```bash
# Your current architecture (Apple Silicon or Intel)
cargo build --release

# Universal binary (both ARM64 and x86_64) - Advanced
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Combine into universal binary
lipo -create \
    target/aarch64-apple-darwin/release/rsiad-gui \
    target/x86_64-apple-darwin/release/rsiad-gui \
    -output rsiad-gui-universal

# Verify
lipo -info rsiad-gui-universal
```

---

### Linux

#### Ubuntu/Debian

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libasound2-dev \
    git \
    curl

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/ioma8/rsiad
cd rsiad

# Build all versions
cargo build --release --bin rsiad           # CLI
cargo build --release --bin rsiad-gui       # Desktop GUI
cargo build --release --bin rsiad-web --features web-server  # Web

# Binaries in target/release/
./target/release/rsiad-gui
```

#### Fedora/RHEL/CentOS

```bash
# Install dependencies
sudo dnf groupinstall "Development Tools"
sudo dnf install -y \
    alsa-lib-devel \
    pkg-config \
    git

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/ioma8/rsiad
cd rsiad
cargo build --release
```

#### Arch Linux

```bash
# Install dependencies
sudo pacman -Sy base-devel alsa-lib git rust

# Clone and build
git clone https://github.com/ioma8/rsiad
cd rsiad
cargo build --release
```

#### Create Distribution Package

```bash
# Build release
cargo build --release

# Create package directory
mkdir -p rsiad-linux-x64
cp target/release/rsiad rsiad-linux-x64/
cp target/release/rsiad-gui rsiad-linux-x64/
cp target/release/rsiad-web rsiad-linux-x64/
cp UprightPianoKW-small-bright-20190703.sf2 rsiad-linux-x64/
cp README.md rsiad-linux-x64/

# Create tarball
tar -czf rsiad-linux-x64.tar.gz rsiad-linux-x64/

# Or create .deb package (advanced)
# See: https://github.com/mmstick/cargo-deb
cargo install cargo-deb
cargo deb
```

---

### Windows

#### Using Command Prompt

```cmd
# Install Rust from https://rustup.rs/
# Install Visual Studio Build Tools from:
# https://visualstudio.microsoft.com/downloads/

# Clone repository
git clone https://github.com/ioma8/rsiad
cd rsiad

# Build
cargo build --release

# Run
target\release\rsiad-gui.exe
target\release\rsiad-web.exe
target\release\rsiad.exe
```

#### Using PowerShell

```powershell
# Install Rust and Visual Studio Build Tools first

# Clone and build
git clone https://github.com/ioma8/rsiad
cd rsiad
cargo build --release

# Create distribution folder
New-Item -ItemType Directory -Force -Path dist\windows
Copy-Item target\release\rsiad.exe dist\windows\
Copy-Item target\release\rsiad-gui.exe dist\windows\
Copy-Item target\release\rsiad-web.exe dist\windows\
Copy-Item UprightPianoKW-small-bright-20190703.sf2 dist\windows\

# Compress
Compress-Archive -Path dist\windows -DestinationPath rsiad-windows-x64.zip
```

#### Build Installer (Advanced)

Using [WiX Toolset](https://wixtoolset.org/):

```powershell
# Install WiX
# Install cargo-wix
cargo install cargo-wix

# Create installer
cargo wix init
cargo wix
```

---

### Mobile (iOS/Android)

Mobile builds use the web interface. See [BUILD_MOBILE.md](BUILD_MOBILE.md) for details.

#### Quick Summary

**Build web server:**
```bash
cargo build --release --bin rsiad-web --features web-server
```

**Deploy options:**
1. Run locally, access via mobile browser
2. Deploy to cloud server
3. Package as PWA (Progressive Web App)
4. Wrap in Capacitor for native apps
5. Use Tauri Mobile for Rust-based native

See [BUILD_MOBILE.md](BUILD_MOBILE.md) for complete instructions.

---

## Build Options

### Build Profiles

```bash
# Debug build (faster compilation, larger binary, includes debug symbols)
cargo build

# Release build (optimized, smaller binary, no debug symbols)
cargo build --release

# Release with debug info (for profiling)
cargo build --release --profile release-with-debug
```

### Feature Flags

```bash
# Desktop GUI only (default)
cargo build --release --bin rsiad-gui

# Web server (includes axum, tokio, etc.)
cargo build --release --bin rsiad-web --features web-server

# All features
cargo build --release --all-features
```

### Specific Binaries

```bash
# CLI only
cargo build --release --bin rsiad

# GUI only  
cargo build --release --bin rsiad-gui

# Web server only
cargo build --release --bin rsiad-web --features web-server

# All binaries
cargo build --release
```

### Size Optimization

For smallest possible binaries, add to `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
strip = true         # Strip symbols
panic = "abort"      # Smaller panic handler
```

Then build:
```bash
cargo build --release
strip target/release/rsiad-gui  # Additional stripping (Unix)
```

### Static Linking (Linux)

For maximum portability:

```bash
# Install musl target
rustup target add x86_64-unknown-linux-musl

# Build with musl
cargo build --release --target x86_64-unknown-linux-musl

# Result: fully static binary with no dependencies
```

---

## Cross-Compilation

### macOS → Linux

```bash
# Install cross-compilation tools
brew install filosottile/musl-cross/musl-cross
rustup target add x86_64-unknown-linux-musl

# Add to ~/.cargo/config.toml:
cat >> ~/.cargo/config.toml << 'EOF'
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
EOF

# Build
cargo build --release --target x86_64-unknown-linux-musl
```

### Linux → macOS (Limited Support)

Cross-compiling from Linux to macOS is complex. Better options:
- Use CI/CD (GitHub Actions)
- Build on macOS VM
- Use [osxcross](https://github.com/tpoechtrager/osxcross)

### Using Docker for Cross-Compilation

```bash
# Create Dockerfile
cat > Dockerfile << 'EOF'
FROM rust:latest
RUN apt-get update && apt-get install -y libasound2-dev
WORKDIR /app
COPY . .
RUN cargo build --release
EOF

# Build
docker build -t rsiad-builder .
docker run --rm -v $(pwd)/target:/app/target rsiad-builder
```

### Using Cross Tool

```bash
# Install cross
cargo install cross

# Build for different targets
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-unknown-linux-gnu
cross build --release --target x86_64-pc-windows-gnu
```

---

## Build Scripts

### Build All Platforms Script

```bash
#!/bin/bash
# build-all.sh

set -e

echo "Building RSIAD for all platforms..."

# CLI
echo "Building CLI..."
cargo build --release --bin rsiad

# Desktop GUI
echo "Building Desktop GUI..."
cargo build --release --bin rsiad-gui

# Web Server
echo "Building Web Server..."
cargo build --release --bin rsiad-web --features web-server

# macOS App Bundle (if on macOS)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Building macOS App Bundle..."
    ./build-macos-app.sh
fi

echo "Build complete!"
echo "Binaries in target/release/"
ls -lh target/release/rsiad*
```

Make it executable:
```bash
chmod +x build-all.sh
./build-all.sh
```

---

## Continuous Integration

### GitHub Actions

The repository includes CI workflows:

- `.github/workflows/ci.yml` - Test on all platforms
- `.github/workflows/build-macos.yml` - Build macOS artifacts
- `.github/workflows/release.yml` - Create releases

### Local CI Testing

```bash
# Install act (local GitHub Actions runner)
brew install act  # macOS
# or download from https://github.com/nektos/act

# Run CI locally
act -j build
```

---

## Troubleshooting

### "fluidlite not found" or "bindgen failed"

**macOS:**
```bash
xcode-select --install
brew install llvm
```

**Linux:**
```bash
sudo apt-get install build-essential clang libclang-dev
```

**Windows:**
- Install Visual Studio Build Tools
- Install LLVM from https://releases.llvm.org/

### "ALSA not found" (Linux)

```bash
sudo apt-get install libasound2-dev pkg-config
```

### "linking with `cc` failed" (macOS)

```bash
xcode-select --install
sudo xcodebuild -license accept
```

### "could not compile `rsiad`"

```bash
# Clean and rebuild
cargo clean
cargo update
cargo build --release
```

### Build is Very Slow

```bash
# Use more parallel jobs
cargo build --release -j 8

# Or use sccache for caching
cargo install sccache
export RUSTC_WRAPPER=sccache
cargo build --release
```

### Out of Disk Space

```bash
# Clean build artifacts
cargo clean

# Clean all Cargo caches
cargo cache -a

# Or just registry
rm -rf ~/.cargo/registry
```

---

## Build Times

Approximate build times (release mode):

| Platform      | CLI    | GUI      | Web      | Total     |
|---------------|--------|----------|----------|-----------|
| macOS M1      | ~30s   | ~2min    | ~90s     | ~4min     |
| Linux (8 core)| ~45s   | ~3min    | ~2min    | ~6min     |
| Windows       | ~60s   | ~4min    | ~2.5min  | ~7min     |

First build takes longer (downloads dependencies). Subsequent builds are much faster.

---

## Build Artifacts

After building, you'll have:

```
target/release/
├── rsiad              # CLI tool (~1.5 MB)
├── rsiad-gui          # Desktop GUI (~5.3 MB)
└── rsiad-web          # Web server (~5.4 MB)

dist/  (if built macOS app)
├── RSIAD.app/         # macOS app bundle
└── macos-binaries/
    ├── rsiad
    └── rsiad-gui
```

---

## Verification

### Test Built Binaries

```bash
# Test CLI
./target/release/rsiad --version
./target/release/rsiad --help

# Test GUI (opens window)
./target/release/rsiad-gui

# Test web server
./target/release/rsiad-web &
curl http://localhost:3000/api/health
kill %1

# Test with exercise
./target/release/rsiad -e triads -r baritone --save test.mp3
ls -lh test.mp3
```

### Check Dependencies

```bash
# macOS
otool -L target/release/rsiad-gui

# Linux
ldd target/release/rsiad-gui

# Windows
dumpbin /dependents target\release\rsiad-gui.exe
```

---

## Distribution

### Prepare Release Package

```bash
# Create distribution directory
mkdir -p rsiad-v0.1.0

# Copy binaries
cp target/release/rsiad rsiad-v0.1.0/
cp target/release/rsiad-gui rsiad-v0.1.0/
cp target/release/rsiad-web rsiad-v0.1.0/

# Copy resources
cp UprightPianoKW-small-bright-20190703.sf2 rsiad-v0.1.0/
cp README.md rsiad-v0.1.0/
cp USAGE.md rsiad-v0.1.0/

# Create archive
tar -czf rsiad-v0.1.0-$(uname -s)-$(uname -m).tar.gz rsiad-v0.1.0/

# Or zip
zip -r rsiad-v0.1.0-$(uname -s)-$(uname -m).zip rsiad-v0.1.0/
```

### Code Signing (macOS)

```bash
# Ad-hoc signing (for local use)
codesign --force --deep --sign - target/release/rsiad-gui

# Developer ID signing (for distribution)
codesign --force --deep --sign "Developer ID Application: Your Name" target/release/rsiad-gui

# Verify
codesign --verify --verbose target/release/rsiad-gui
```

---

## Next Steps

After building:

1. **Read Usage Guide**: [USAGE.md](USAGE.md)
2. **Try Examples**: See `examples/` directory
3. **Configure**: Set up soundfonts and preferences
4. **Deploy**: Mobile, web, or distribute binaries

**Questions?** See [README.md](README.md) or open an issue on GitHub.

---

**Happy building! 🔨**
