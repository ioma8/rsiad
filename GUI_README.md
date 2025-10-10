# RSIAD GUI - Vocal Warmup Exercise Generator

A minimalist, user-friendly graphical interface for generating vocal warmup exercises.

## Features

### 🎵 Exercise Types
- **Triads**: Root → 3rd → 5th → 3rd → Root → Chord
- **Scales**: Major scale patterns up to the 5th
- **Octaves**: Root → Octave → Both together

### 🎤 Vocal Ranges
Preset ranges for different voice types:
- Bass (E2-E4)
- Baritone (A2-A4)
- Tenor (C3-C5)
- Alto (F3-F5)
- Mezzo-Soprano (A3-A5)
- Soprano (C4-C6)

Or use **Custom Range** for precise control (e.g., "C3" to "G4")

### ⚙️ Controls

**Soundfont Selection**
- Browse button to select your .sf2/.sf3 soundfont file
- Default: UprightPianoKW-small-bright-20190703.sf2

**Note Duration**
- Adjustable slider from 0.1s to 2.0s
- Default: 0.8s

**Output Options**
- **▶ Play**: Realtime audio playback
- **💾 Save MP3**: Generate and save as MP3 file
- **⏹ Stop**: Stop current playback/generation

### 📊 Status Display
- Real-time status indicator (Ready/Playing/Generating)
- Key range preview with MIDI numbers
- Estimated duration calculation
- Range validation warnings

## Running the GUI

```bash
# Build and run
cargo run --release --bin rsiad-gui

# Or run the compiled binary
./target/release/rsiad-gui
```

## Usage Tips

1. **Select your exercise type** - Choose between Triads, Scales, or Octaves
2. **Set your vocal range** - Use presets or custom range
3. **Adjust note duration** - Shorter for faster practice, longer for careful exercises
4. **Choose action**:
   - Click **Play** for immediate practice
   - Click **Save MP3** to create a file for later use
5. **Stop anytime** - Click Stop to interrupt playback

## Interface Layout

```
┌─────────────────────────────────────┐
│  🎵 Vocal Warmup Exercise          │
├─────────────────────────────────────┤
│  Soundfont: [path] [Browse]        │
├─────────────────────────────────────┤
│  Exercise: ◉ Triads ○ Scales        │
│                                      │
│  Vocal Range: □ Custom              │
│    [Baritone ▼]                     │
│                                      │
│  Note Duration: [━━●━━━] 0.8s       │
├─────────────────────────────────────┤
│  Save to: [output.mp3] [Browse]     │
│                                      │
│  [▶ Play] [💾 Save MP3] [⏹ Stop]   │
├─────────────────────────────────────┤
│  Status: Ready                       │
│  Key range: A2 to A4 (MIDI 57-81)   │
│  Estimated duration: 43.2s (9 keys) │
└─────────────────────────────────────┘
```

## Requirements

- Soundfont file (.sf2 or .sf3)
- Audio output device (for realtime playback)

## Technical Details

- Built with **egui** for cross-platform GUI
- Uses **fluidlite** for audio synthesis
- MP3 encoding via **LAME**
- Non-blocking audio playback in separate thread
- Real-time status updates
