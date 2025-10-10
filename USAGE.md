# RSIAD - Usage Guide

Complete guide for running RSIAD on all supported platforms.

## Table of Contents

- [Quick Start](#quick-start)
- [macOS](#macos)
- [Linux](#linux)
- [Windows](#windows)
- [Mobile (iOS/Android)](#mobile-iosandroid)
- [Command Line Interface](#command-line-interface)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

---

## Quick Start

RSIAD has three interfaces:

1. **Desktop GUI** (egui) - Native app for macOS/Linux/Windows
2. **Web Interface** - Mobile-friendly, works on any device
3. **Command Line** - For automation and scripting

---

## macOS

### Option 1: Run the App Bundle (Easiest)

```bash
# Double-click RSIAD.app in Finder
# OR use terminal:
open /Applications/RSIAD.app
```

**First time installation:**
```bash
# Build and install
./build-macos-app.sh
cp -r dist/RSIAD.app /Applications/

# Launch
open /Applications/RSIAD.app
```

If macOS says "can't be opened because it's from an unidentified developer":
- Right-click → Open → Click "Open" in dialog
- OR: `xattr -cr /Applications/RSIAD.app`

### Option 2: Run Standalone GUI Binary

```bash
# Build
cargo build --release --bin rsiad-gui

# Run
./target/release/rsiad-gui
```

### Option 3: Run Web Interface

```bash
# Build
cargo build --release --bin rsiad-web --features web-server

# Run
./target/release/rsiad-web

# Open browser to http://localhost:3000
```

### Option 4: Command Line

```bash
# Build
cargo build --release --bin rsiad

# Run with default settings (Baritone, Triads)
./target/release/rsiad

# Save to MP3
./target/release/rsiad --save output.mp3

# Custom range and exercise
./target/release/rsiad -f C3 -t C5 -e scales --duration 1.0
```

---

## Linux

### Prerequisites

Install ALSA development libraries:

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install libasound2-dev pkg-config

# Fedora/RHEL
sudo dnf install alsa-lib-devel

# Arch Linux
sudo pacman -S alsa-lib
```

### Option 1: Desktop GUI (Recommended)

```bash
# Clone and build
git clone https://github.com/ioma8/rsiad
cd rsiad
cargo build --release --bin rsiad-gui

# Run
./target/release/rsiad-gui
```

### Option 2: Web Interface

```bash
# Build
cargo build --release --bin rsiad-web --features web-server

# Run
./target/release/rsiad-web

# Access at http://localhost:3000
```

### Option 3: Command Line

```bash
# Build
cargo build --release --bin rsiad

# Run
./target/release/rsiad

# For available options
./target/release/rsiad --help
```

### Create Desktop Entry (Optional)

```bash
# Create desktop entry
cat > ~/.local/share/applications/rsiad.desktop << EOF
[Desktop Entry]
Name=RSIAD Vocal Warmup
Comment=Vocal warmup exercise generator
Exec=/path/to/rsiad/target/release/rsiad-gui
Icon=audio-x-generic
Terminal=false
Type=Application
Categories=Audio;Music;Education;
EOF

# Update desktop database
update-desktop-database ~/.local/share/applications/
```

---

## Windows

### Prerequisites

Install Rust from [rustup.rs](https://rustup.rs/)

### Option 1: Desktop GUI

```powershell
# Clone and build
git clone https://github.com/ioma8/rsiad
cd rsiad
cargo build --release --bin rsiad-gui

# Run
.\target\release\rsiad-gui.exe
```

### Option 2: Web Interface

```powershell
# Build
cargo build --release --bin rsiad-web --features web-server

# Run
.\target\release\rsiad-web.exe

# Open browser to http://localhost:3000
```

### Option 3: Command Line

```powershell
# Build
cargo build --release --bin rsiad

# Run
.\target\release\rsiad.exe

# With options
.\target\release\rsiad.exe --range soprano --save output.mp3
```

### Create Desktop Shortcut

1. Right-click on `target\release\rsiad-gui.exe`
2. Send to → Desktop (create shortcut)
3. Rename to "RSIAD Vocal Warmup"

---

## Mobile (iOS/Android)

### Option 1: Progressive Web App (PWA) - No Installation Required

**On Desktop:**
```bash
# Start the server
./target/release/rsiad-web

# Find your local IP
# macOS/Linux:
ifconfig | grep "inet " | grep -v 127.0.0.1
# Windows:
ipconfig
```

**On Mobile Device:**

1. Open browser (Safari on iOS, Chrome on Android)
2. Navigate to `http://YOUR_IP:3000`
3. **iOS**: Tap Share → "Add to Home Screen"
4. **Android**: Tap menu (⋮) → "Add to Home screen"

The app icon will appear on your home screen!

### Option 2: Deploy Server to Cloud

Deploy the server and access from anywhere:

```bash
# Build for Linux server
cargo build --release --target x86_64-unknown-linux-gnu --bin rsiad-web --features web-server

# Upload to server
scp target/x86_64-unknown-linux-gnu/release/rsiad-web user@server:/opt/rsiad/
scp UprightPianoKW-small-bright-20190703.sf2 user@server:/opt/rsiad/

# Run on server
ssh user@server
cd /opt/rsiad
./rsiad-web

# Access from mobile at http://your-server:3000
```

### Option 3: Native Mobile Apps (Advanced)

See [BUILD_MOBILE.md](BUILD_MOBILE.md) for Capacitor/Tauri setup.

---

## Command Line Interface

### Basic Usage

```bash
# Default: Baritone range, Triads, 0.8s notes, realtime playback
rsiad

# Save to MP3 instead of playing
rsiad --save output.mp3

# Different exercise type
rsiad --exercise scales
rsiad --exercise octaves
rsiad -e triads

# Different vocal range preset
rsiad --range soprano
rsiad --range bass
rsiad --range tenor

# Custom range (MIDI keys)
rsiad --from C3 --to C5
rsiad -f 48 -t 72

# Adjust note duration
rsiad --duration 1.2
rsiad -d 0.5

# Combine options
rsiad -e scales -r alto --duration 1.0 --save alto-scales.mp3
```

### Available Options

```
Options:
  -e, --exercise <TYPE>      Exercise type: triads, scales, octaves [default: triads]
  -r, --range <RANGE>        Vocal range: bass, baritone, tenor, alto, mezzo, soprano
  -f, --from <NOTE>          Starting note (e.g., C3 or MIDI key 48)
  -t, --to <NOTE>            Ending note (e.g., C5 or MIDI key 72)
  -d, --duration <SECONDS>   Note duration in seconds [default: 0.8]
  -s, --save <PATH>          Save to MP3 file instead of playing
  -h, --help                 Print help
  -V, --version              Print version
```

### Vocal Range Presets

| Range         | MIDI Range | Note Range |
|---------------|------------|------------|
| Bass          | 40-64      | E2-E4      |
| Baritone      | 57-81      | A2-A4      |
| Tenor         | 60-84      | C3-C5      |
| Alto          | 65-89      | F3-F5      |
| Mezzo-Soprano | 69-93      | A3-A5      |
| Soprano       | 72-96      | C4-C6      |

### Examples

```bash
# Practice triads in baritone range
rsiad

# Save soprano scales to file
rsiad -e scales -r soprano -s soprano-scales.mp3

# Custom range with slow notes
rsiad -f G2 -t G4 -d 1.5

# Quick octave jumps for tenor
rsiad -e octaves -r tenor -d 0.6

# Full major scales through alto range
rsiad -e scales -r alto --save alto-practice.mp3
```

---

## Configuration

### Soundfont File

RSIAD requires a SoundFont (.sf2) file for audio synthesis.

**Default location:** `UprightPianoKW-small-bright-20190703.sf2` (in current directory)

**To use a different soundfont:**

```bash
# Set environment variable
export SOUNDFONT_PATH=/path/to/your/soundfont.sf2

# Then run
rsiad-gui
# or
rsiad-web
# or
rsiad
```

**Download soundfonts:**
- [FluidR3_GM.sf2](https://member.keymusician.com/Member/FluidR3_GM/index.html)
- [MuseScore soundfonts](https://musescore.org/en/handbook/3/soundfonts-and-sfz-files)
- [FreePats project](https://freepats.zenvoid.org/)

### Desktop GUI Settings

The GUI automatically saves:
- Last used soundfont path
- Exercise preferences
- Window size/position

Settings stored in:
- **macOS**: `~/Library/Application Support/rsiad/`
- **Linux**: `~/.config/rsiad/`
- **Windows**: `%APPDATA%\rsiad\`

### Web Server Configuration

```bash
# Change port
PORT=8080 ./target/release/rsiad-web

# Set soundfont path
SOUNDFONT_PATH=/path/to/font.sf2 ./target/release/rsiad-web

# Both
SOUNDFONT_PATH=./my.sf2 PORT=8080 ./target/release/rsiad-web
```

---

## Troubleshooting

### macOS

**"Can't be opened because it's from an unidentified developer"**
```bash
xattr -cr /Applications/RSIAD.app
```

**"Soundfont not found"**
```bash
# Make sure soundfont is in the same directory
ls -la UprightPianoKW-small-bright-20190703.sf2

# Or set path
export SOUNDFONT_PATH=/full/path/to/soundfont.sf2
```

**"No audio output"**
- Check System Preferences → Sound → Output
- Try restarting the application
- Check if other apps can play audio

### Linux

**"ALSA lib errors"**
```bash
# Install ALSA development files
sudo apt-get install libasound2-dev

# Rebuild
cargo clean
cargo build --release
```

**"StreamConfigNotSupported"**
```bash
# Check available audio devices
aplay -l

# Try different sample rate in code or use different device
```

**"Permission denied for audio device"**
```bash
# Add user to audio group
sudo usermod -a -G audio $USER

# Log out and back in
```

### Windows

**"VCRUNTIME140.dll not found"**
- Install [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)

**"Failed to initialize audio"**
- Check Windows Sound settings
- Update audio drivers
- Try running as administrator

### Web Interface

**"Connection refused"**
```bash
# Make sure server is running
./target/release/rsiad-web

# Check if port 3000 is available
lsof -i :3000  # macOS/Linux
netstat -an | findstr :3000  # Windows
```

**"Mobile device can't connect"**
```bash
# Find your IP
ifconfig  # macOS/Linux
ipconfig  # Windows

# Make sure devices are on same network
# Use IP instead of localhost: http://192.168.1.x:3000
```

**"Soundfont file too large for mobile"**
- Use a smaller soundfont file
- Deploy server to cloud instead of bundling
- Use server-client architecture (recommended)

### Build Errors

**"fluidlite not found"**
```bash
# Make sure you have C compiler
# macOS:
xcode-select --install

# Linux:
sudo apt-get install build-essential

# Windows:
# Install Visual Studio Build Tools
```

**"cargo build fails"**
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

---

## Performance Tips

### Faster Startup

```bash
# Desktop GUI: Pre-load soundfont
# The GUI caches soundfont after first load

# CLI: Use environment variable
export SOUNDFONT_PATH=/path/to/soundfont.sf2
```

### Reduce CPU Usage

```bash
# Use longer note durations
rsiad --duration 1.0

# Or save to file instead of realtime
rsiad --save output.mp3
```

### Smaller File Sizes

```bash
# Use smaller soundfont files
# SoundFont file size affects startup time only
# Generated MP3s are always compressed efficiently
```

---

## Getting Help

```bash
# CLI help
rsiad --help

# Check version
rsiad --version

# Web server info
curl http://localhost:3000/api/health
```

**Documentation:**
- [README.md](README.md) - Project overview
- [BUILD_APPLE.md](BUILD_APPLE.md) - macOS/iOS builds
- [BUILD_MOBILE.md](BUILD_MOBILE.md) - Mobile deployment
- [GUI_README.md](GUI_README.md) - Desktop GUI guide

**Support:**
- GitHub Issues: [https://github.com/ioma8/rsiad/issues](https://github.com/ioma8/rsiad/issues)
- Discussions: [https://github.com/ioma8/rsiad/discussions](https://github.com/ioma8/rsiad/discussions)

---

## Quick Reference

### Build All Versions

```bash
# Desktop GUI (macOS/Linux/Windows)
cargo build --release --bin rsiad-gui

# Web server (Mobile-friendly)
cargo build --release --bin rsiad-web --features web-server

# CLI tool
cargo build --release --bin rsiad

# macOS App Bundle
./build-macos-app.sh
```

### Run All Versions

```bash
# Desktop GUI
./target/release/rsiad-gui

# Web server
./target/release/rsiad-web
# Then open http://localhost:3000

# CLI
./target/release/rsiad --help

# macOS App
open dist/RSIAD.app
```

### Common Tasks

```bash
# Generate exercise for practice
rsiad -e triads -r baritone

# Create MP3 for offline practice
rsiad -e scales -r tenor -s tenor-scales.mp3

# Run web server for mobile devices
rsiad-web

# Quick test with custom range
rsiad -f C2 -t C4 -d 0.5
```

---

**Enjoy your vocal warmup practice! 🎵**
