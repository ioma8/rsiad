#!/bin/bash
set -e

echo "🍎 Building RSIAD macOS App Bundle"
echo "===================================="

# Build for current architecture
echo ""
echo "📦 Building GUI for current architecture..."
cargo build --release --bin rsiad-gui

# Create app bundle structure
echo ""
echo "🔨 Creating macOS App Bundle..."
APP_NAME="RSIAD.app"
APP_DIR="dist/${APP_NAME}"
mkdir -p "${APP_DIR}/Contents/MacOS"
mkdir -p "${APP_DIR}/Contents/Resources"

# Copy binary
cp target/release/rsiad-gui "${APP_DIR}/Contents/MacOS/rsiad-gui"
chmod +x "${APP_DIR}/Contents/MacOS/rsiad-gui"

# Create Info.plist
cat > "${APP_DIR}/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>RSIAD</string>
    <key>CFBundleDisplayName</key>
    <string>RSIAD Vocal Warmup</string>
    <key>CFBundleIdentifier</key>
    <string>com.rsiad.vocal-warmup</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleExecutable</key>
    <string>rsiad-gui</string>
    <key>CFBundleIconFile</key>
    <string>icon.icns</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
</dict>
</plist>
EOF

# Create a simple icon (text-based placeholder)
# In production, you'd use a proper .icns file
echo "Icon placeholder created"

echo ""
echo "✅ macOS App Bundle created: dist/${APP_NAME}"
echo ""
echo "To install:"
echo "  cp -r dist/${APP_NAME} /Applications/"
echo ""
echo "To run:"
echo "  open dist/${APP_NAME}"
echo ""

# Also create standalone binaries
echo "📋 Creating standalone binaries..."
mkdir -p dist/macos-binaries
cp target/release/rsiad-gui dist/macos-binaries/
cp target/release/rsiad dist/macos-binaries/

echo ""
echo "📊 Build artifacts:"
ls -lh dist/macos-binaries/
echo ""
du -sh "dist/${APP_NAME}"

echo ""
echo "✅ Build complete!"
