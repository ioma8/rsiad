# RSIAD - Quick Start Guide

## 🎯 Choose Your Interface

### 🖥️ Desktop GUI (Best for Desktop Users)

**macOS:**
```bash
open /Applications/RSIAD.app
```

**Linux/Windows:**
```bash
./target/release/rsiad-gui
```

Features: Beautiful UI, all settings, instant feedback

---

### 🌐 Web Interface (Best for Mobile)

```bash
# Start server
./target/release/rsiad-web

# Access from:
# Desktop: http://localhost:3000
# Mobile: http://YOUR_IP:3000
```

Features: Works on any device, mobile-optimized, PWA support

---

### ⌨️ Command Line (Best for Automation)

```bash
# Quick practice
./target/release/rsiad

# Save MP3
./target/release/rsiad -e scales -r tenor -s practice.mp3
```

Features: Fast, scriptable, automation-friendly

---

## 📱 Mobile Access

### On Your Phone/Tablet:

1. **Start server on computer:**
   ```bash
   ./target/release/rsiad-web
   ```

2. **Find your IP address:**
   ```bash
   ifconfig | grep "inet "  # macOS/Linux
   ipconfig                 # Windows
   ```

3. **Open on mobile:**
   - iOS Safari: `http://YOUR_IP:3000`
   - Android Chrome: `http://YOUR_IP:3000`

4. **Add to Home Screen:**
   - iOS: Share → "Add to Home Screen"
   - Android: Menu → "Add to Home screen"

Now you have a native-like app! 📱

---

## 🎵 Common Tasks

### Practice Vocal Range
```bash
# Desktop GUI: Open app, select range, click Play
# CLI:
./target/release/rsiad -r baritone
```

### Create Practice MP3
```bash
# Desktop GUI: Select settings, click "Save MP3"
# CLI:
./target/release/rsiad -e triads -r soprano -s soprano.mp3
```

### Custom Exercise
```bash
# Desktop GUI: Use custom range inputs
# CLI:
./target/release/rsiad -f C2 -t G4 -d 1.0 -e scales
```

---

## 📖 Full Documentation

- **[USAGE.md](USAGE.md)** - Complete usage guide
- **[BUILD.md](BUILD.md)** - Building from source
- **[README.md](README.md)** - Project overview

---

## 🆘 Need Help?

**Desktop GUI not opening?**
- macOS: `xattr -cr /Applications/RSIAD.app`
- Check soundfont file is present

**Web server won't start?**
- Check port 3000 is not in use: `lsof -i :3000`
- Try different port: `PORT=8080 ./target/release/rsiad-web`

**CLI no audio?**
- Check audio output settings
- Verify soundfont path: `export SOUNDFONT_PATH=/path/to/file.sf2`

**More help:** [USAGE.md](USAGE.md) → Troubleshooting section

---

**Ready to practice! 🎤**
