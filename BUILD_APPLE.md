# Building RSIAD for Apple Platforms

## macOS Builds

### Quick Build (Current Architecture)

Build for your current Mac:

```bash
./build-macos-app.sh
```

This creates:
- `dist/RSIAD.app` - macOS application bundle
- `dist/macos-binaries/rsiad-gui` - GUI standalone binary
- `dist/macos-binaries/rsiad` - CLI standalone binary

### Installing the App

```bash
# Copy to Applications folder
cp -r dist/RSIAD.app /Applications/

# Or open directly
open dist/RSIAD.app
```

### Manual Build

```bash
# Build GUI
cargo build --release --bin rsiad-gui

# Build CLI
cargo build --release --bin rsiad

# Binaries will be in:
# target/release/rsiad-gui
# target/release/rsiad
```

## Architecture Support

Currently built for:
- **Apple Silicon (ARM64)** - M1, M2, M3 Macs
- **Intel (x86_64)** - Intel-based Macs (requires additional setup)

### Universal Binary (ARM + Intel)

To create a universal binary that works on both Apple Silicon and Intel Macs:

1. Install Intel target:
```bash
rustup target add x86_64-apple-darwin
```

2. Build for both architectures:
```bash
cargo build --release --target aarch64-apple-darwin --bin rsiad-gui
cargo build --release --target x86_64-apple-darwin --bin rsiad-gui
```

3. Create universal binary:
```bash
lipo -create \
    target/aarch64-apple-darwin/release/rsiad-gui \
    target/x86_64-apple-darwin/release/rsiad-gui \
    -output rsiad-gui-universal
```

## iOS Builds

⚠️ **iOS builds are NOT currently supported** due to:

1. **Native Dependencies**: `fluidlite` requires C library compilation for iOS
2. **Audio System**: `cpal` has limited iOS audio support
3. **File System**: iOS sandboxing prevents direct .sf2 file access
4. **Code Signing**: Requires Apple Developer account and provisioning profiles

### iOS Future Support

To add iOS support, the following would be needed:

1. Replace `fluidlite` with an iOS-compatible audio synthesis library
2. Implement iOS-specific file picker and storage
3. Use `oboe` or native iOS audio APIs instead of `cpal`
4. Bundle soundfont files within the app
5. Handle App Store requirements and signing

## App Bundle Structure

```
RSIAD.app/
├── Contents/
│   ├── Info.plist          # App metadata
│   ├── MacOS/
│   │   └── rsiad-gui       # Executable
│   └── Resources/
│       └── (icons, etc.)
```

## Requirements

- **macOS**: 10.15 (Catalina) or later
- **Rust**: 1.70 or later
- **Dependencies**: Automatically handled by Cargo

## Distribution

### For End Users

Provide:
1. `RSIAD.app` - Double-click to install/run
2. `UprightPianoKW-small-bright-20190703.sf2` - Default soundfont

### For Developers

Clone and build:
```bash
git clone https://github.com/ioma8/rsiad
cd rsiad
cargo build --release --bin rsiad-gui
```

## Code Signing (Optional)

For distribution outside the App Store:

```bash
# Sign the app
codesign --force --deep --sign - dist/RSIAD.app

# Verify signature
codesign --verify --verbose dist/RSIAD.app
```

For App Store distribution, use your Developer ID:
```bash
codesign --force --deep --sign "Developer ID Application: Your Name" dist/RSIAD.app
```

## Troubleshooting

### "App can't be opened because it's from an unidentified developer"

Right-click the app and select "Open", then click "Open" in the dialog.

Or disable Gatekeeper temporarily:
```bash
xattr -cr dist/RSIAD.app
```

### Build Errors

Clean and rebuild:
```bash
cargo clean
cargo build --release
```

### Missing Dependencies

Update Rust and dependencies:
```bash
rustup update
cargo update
```

## Size Optimization

To reduce binary size:

1. Strip debug symbols:
```bash
strip target/release/rsiad-gui
```

2. Use build profile in `Cargo.toml`:
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Strip symbols
```

Current sizes:
- GUI: ~5.3 MB
- CLI: ~1.5 MB
- App Bundle: ~5.3 MB

## Platform-Specific Notes

### Apple Silicon (M1/M2/M3)
- Native ARM64 build
- Best performance
- Default target

### Intel Macs
- Requires x86_64 target
- Runs via Rosetta 2 on Apple Silicon
- Slightly larger binaries

### Universal Binary
- Contains both ARM64 and x86_64 code
- ~2x file size
- Works on all Macs
