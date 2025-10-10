#!/bin/bash
set -e

echo "🍎 Building RSIAD for Apple Platforms"
echo "======================================"

# Create output directory
mkdir -p dist/apple

# Build for macOS ARM64 (Apple Silicon)
echo ""
echo "📦 Building for macOS ARM64 (Apple Silicon)..."
cargo build --release --target aarch64-apple-darwin --bin rsiad-gui
cargo build --release --target aarch64-apple-darwin --bin rsiad

# Build for macOS x86_64 (Intel)
echo ""
echo "📦 Building for macOS x86_64 (Intel)..."
cargo build --release --target x86_64-apple-darwin --bin rsiad-gui
cargo build --release --target x86_64-apple-darwin --bin rsiad

# Create Universal binaries (ARM + Intel)
echo ""
echo "🔨 Creating Universal macOS binaries..."
lipo -create \
    target/aarch64-apple-darwin/release/rsiad-gui \
    target/x86_64-apple-darwin/release/rsiad-gui \
    -output dist/apple/rsiad-gui-universal

lipo -create \
    target/aarch64-apple-darwin/release/rsiad \
    target/x86_64-apple-darwin/release/rsiad \
    -output dist/apple/rsiad-universal

chmod +x dist/apple/rsiad-gui-universal
chmod +x dist/apple/rsiad-universal

# Verify universal binaries
echo ""
echo "✅ Universal binaries created:"
lipo -info dist/apple/rsiad-gui-universal
lipo -info dist/apple/rsiad-universal

# Copy individual builds
echo ""
echo "📋 Copying individual platform builds..."
cp target/aarch64-apple-darwin/release/rsiad-gui dist/apple/rsiad-gui-arm64
cp target/aarch64-apple-darwin/release/rsiad dist/apple/rsiad-arm64
cp target/x86_64-apple-darwin/release/rsiad-gui dist/apple/rsiad-gui-x86_64
cp target/x86_64-apple-darwin/release/rsiad dist/apple/rsiad-x86_64

# Show sizes
echo ""
echo "📊 Build sizes:"
ls -lh dist/apple/

echo ""
echo "✅ macOS builds complete!"
echo ""
echo "Available builds in dist/apple/:"
echo "  - rsiad-gui-universal (ARM64 + x86_64)"
echo "  - rsiad-universal (ARM64 + x86_64)"
echo "  - rsiad-gui-arm64 (Apple Silicon only)"
echo "  - rsiad-gui-x86_64 (Intel only)"
echo "  - rsiad-arm64 (Apple Silicon CLI)"
echo "  - rsiad-x86_64 (Intel CLI)"
echo ""
echo "Note: iOS build requires additional setup and is not included."
echo "      The fluidlite dependency and file system access make iOS"
echo "      builds non-trivial without significant refactoring."
