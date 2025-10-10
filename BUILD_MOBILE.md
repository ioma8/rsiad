# Building RSIAD for Mobile (iOS/Android)

## Overview

RSIAD provides a **web-based GUI** that can be packaged for mobile platforms. The web interface works on any platform and can be packaged as native mobile apps using Capacitor or Tauri.

## Architecture

```
┌─────────────┐
│  Web UI     │  HTML/CSS/JS (works everywhere)
│  (Browser)  │
└──────┬──────┘
       │ HTTP/REST API
┌──────▼──────┐
│ Rust Server │  Audio synthesis, MP3 generation
│ (rsiad-web) │  (requires soundfont file)
└─────────────┘
```

## Quick Start - Web Interface

### 1. Run the Web Server

```bash
# Build the web server
cargo build --release --bin rsiad-web --features web-server

# Run it
./target/release/rsiad-web

# Open in browser
open http://localhost:3000
```

The web UI works on:
- ✅ Desktop browsers (Chrome, Firefox, Safari, Edge)
- ✅ Mobile browsers (iOS Safari, Android Chrome)
- ✅ Tablets
- ✅ Progressive Web App (PWA)

## Mobile App Packaging Options

### Option 1: Progressive Web App (PWA) - Easiest

No build required! Just add to home screen:

**iOS:**
1. Open `http://yourserver:3000` in Safari
2. Tap Share button
3. Select "Add to Home Screen"
4. App icon appears on home screen

**Android:**
1. Open `http://yourserver:3000` in Chrome
2. Tap menu (⋮)
3. Select "Add to Home screen"
4. App icon appears on home screen

### Option 2: Capacitor (Recommended for Native Apps)

Capacitor wraps the web app in a native container.

#### Prerequisites

```bash
# Install Node.js and npm
brew install node

# Install Capacitor CLI
npm install -g @capacitor/cli
```

#### Setup Capacitor Project

```bash
# Create capacitor project
mkdir rsiad-mobile
cd rsiad-mobile

npm init -y
npm install @capacitor/core @capacitor/cli

# Initialize Capacitor
npx cap init RSIAD com.rsiad.vocal dev.rsiad.vocal

# Add platforms
npx cap add ios
npx cap add android
```

#### Configure for iOS

```bash
# Copy web files
cp -r ../static/* www/

# Update capacitor.config.json to point to server
cat > capacitor.config.json << 'EOF'
{
  "appId": "com.rsiad.vocal",
  "appName": "RSIAD",
  "webDir": "www",
  "server": {
    "url": "http://localhost:3000",
    "cleartext": true
  }
}
EOF

# Open in Xcode
npx cap open ios
```

Then in Xcode:
1. Set signing team
2. Build and run on device/simulator

#### Configure for Android

```bash
# Open in Android Studio
npx cap open android
```

Then in Android Studio:
1. Build and run on device/emulator

### Option 3: Tauri Mobile (Rust-based)

Tauri v2 supports mobile, but requires more setup.

```bash
# Install Tauri CLI
cargo install tauri-cli --version "^2.0.0-beta"

# Create Tauri project
cargo tauri init

# Add mobile support
cargo tauri android init
cargo tauri ios init

# Build
cargo tauri android build
cargo tauri ios build
```

## Server Deployment Options

### Local Server (Development)

Run the server on your development machine:

```bash
./target/release/rsiad-web
# Access at http://localhost:3000
```

### Self-Hosted Server

Deploy to a VPS or cloud server:

```bash
# Build for Linux
cargo build --release --target x86_64-unknown-linux-gnu --bin rsiad-web --features web-server

# Copy to server
scp target/x86_64-unknown-linux-gnu/release/rsiad-web user@server:/opt/rsiad/
scp UprightPianoKW-small-bright-20190703.sf2 user@server:/opt/rsiad/

# Run as service (systemd)
sudo systemctl enable rsiad-web
sudo systemctl start rsiad-web
```

### Cloud Deployment

Deploy to cloud platforms:
- **Fly.io**: `fly deploy`
- **Railway**: Connect GitHub repo
- **AWS**: Deploy to EC2/ECS
- **Google Cloud**: Deploy to Cloud Run
- **Azure**: Deploy to App Service

## Current Limitations & Solutions

### ⚠️ Soundfont File Access

**Problem**: Mobile apps can't access arbitrary .sf2 files from filesystem.

**Solutions**:
1. **Bundle soundfont in app** - Include .sf2 in app bundle (increases app size)
2. **Remote server** - Keep server on desktop/cloud, mobile is just UI
3. **Alternative synthesis** - Replace fluidlite with mobile-compatible synth

### ⚠️ MP3 Generation on Mobile

**Problem**: File system access is restricted on mobile.

**Solutions**:
1. **Download via browser** - Return MP3 as HTTP response for download
2. **Share functionality** - Use native share sheet
3. **Cloud storage** - Save to iCloud/Google Drive

### ⚠️ Real-time Audio

**Problem**: cpal has limited mobile audio support.

**Solutions**:
1. **Web Audio API** - Generate audio in browser (requires JS rewrite)
2. **Server-side only** - Only MP3 generation, no realtime
3. **Native audio** - Use platform-specific audio APIs

## Recommended Architecture

For best mobile experience:

```
Mobile App (Capacitor)
    ↓ HTTP
Server (Desktop/Cloud)
    ↓
Soundfont + Audio Engine
```

**Benefits**:
- Works on any mobile device
- No complex native dependencies
- Easy updates (server-side only)
- Smaller app size

## Building for Production

### 1. Build Web Server

```bash
cargo build --release --bin rsiad-web --features web-server
```

### 2. Package Mobile Apps

```bash
# iOS
cd rsiad-mobile
npx cap sync ios
npx cap open ios
# Build in Xcode

# Android
npx cap sync android
npx cap open android
# Build in Android Studio
```

### 3. Distribution

**iOS**:
- TestFlight for beta testing
- App Store for public release

**Android**:
- Google Play Console for release
- APK for direct installation

## File Structure

```
rsiad/
├── src/
│   ├── web_server.rs          # Web server with REST API
│   └── ...
├── static/
│   └── index.html             # Mobile-friendly web UI
├── target/release/
│   └── rsiad-web              # Server binary
└── rsiad-mobile/              # Capacitor project (separate)
    ├── ios/                   # iOS app
    ├── android/               # Android app
    └── www/                   # Web files
```

## Features

✅ **Web UI Features**:
- Exercise type selection (Triads, Scales, Octaves)
- Vocal range presets + custom ranges
- Adjustable note duration
- Real-time playback
- MP3 generation
- Mobile-responsive design
- Dark theme (Catppuccin-inspired)
- Touch-friendly controls

✅ **Server Features**:
- REST API for all operations
- Health check endpoint
- CORS enabled for mobile
- Efficient audio synthesis
- MP3 encoding

## Development

### Testing Locally

```bash
# Terminal 1: Run server
./target/release/rsiad-web

# Terminal 2: Test API
curl -X POST http://localhost:3000/api/exercise \
  -H "Content-Type: application/json" \
  -d '{"exercise_type":"triads","key_from":57,"key_to":81,"note_duration":0.8,"realtime":true}'

# Browser: Open http://localhost:3000
```

### Testing on Mobile Device

```bash
# Find your local IP
ifconfig | grep "inet " | grep -v 127.0.0.1

# Run server on all interfaces
RUST_LOG=debug ./target/release/rsiad-web

# Open on mobile: http://YOUR_IP:3000
```

## Troubleshooting

### CORS Issues

If mobile app can't connect to server, ensure CORS is enabled (already configured).

### Soundfont Not Found

Set environment variable:
```bash
export SOUNDFONT_PATH=/path/to/soundfont.sf2
./target/release/rsiad-web
```

### Mobile Browser Issues

- iOS Safari: May require HTTPS for microphone/audio features
- Android Chrome: Works best for web audio

## Future Enhancements

- [ ] Native mobile audio synthesis
- [ ] Bundle soundfonts in mobile apps
- [ ] Offline mode with service workers
- [ ] Cloud storage integration (iCloud, Google Drive)
- [ ] Social sharing of exercises
- [ ] Exercise history/favorites
- [ ] Multiple soundfont support

## Summary

Current status:
- ✅ Web server working
- ✅ Mobile-responsive web UI
- ✅ REST API functional
- ⚠️ Requires server running (desktop/cloud)
- ⚠️ Native mobile builds need Capacitor setup

Best approach: Use Capacitor to wrap web UI, server runs separately.
