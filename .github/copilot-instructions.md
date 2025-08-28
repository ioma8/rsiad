# RSIAD - Rust Vocal Warmup Exercises

## Architecture Overview

RSIAD is a command-line audio synthesis tool that generates musical exercises for vocal warmup. The codebase uses a **dual-mode architecture**:

- **Realtime Mode**: Uses `xsynth-realtime` with `RealtimePlayer` for immediate audio playback via system audio
- **File Mode**: Uses custom `XSynthRender` with `FilePlayer` for WAV generation + MP3 conversion

Both modes implement the `Player` trait (`play_note`, `play_chord`, `load_soundfont`, `wait`, `finalize`) enabling polymorphic audio handling across three exercise types: triads, scales, and octaves.

## Key Components

### Core Modules
- `main.rs`: CLI parsing, player selection, exercise generation with polymorphic `play_exercises_from()` dispatcher
- `render.rs`: Custom file-based synthesizer wrapper around `xsynth-core::ChannelGroup`
- `writer.rs`: Threaded WAV file writer using `crossbeam-channel`
- `config.rs`: XSynth configuration with audio parameters and envelope settings

### Critical Dependencies
- **xsynth-core/xsynth-realtime**: Software synthesizer engine - study [xsynth repository](https://github.com/BlackMIDIDevs/xsynth) for usage patterns
- **hound**: WAV file I/O
- **mp3lame-encoder**: WAV→MP3 conversion with manual PCM buffer handling
- **cpal**: Cross-platform audio output (realtime mode only)

## Development Workflow

### Build & Test Pattern
```bash
cargo check  # ALWAYS run after each change - never use cargo run during development
cargo build  # For release builds
```

**Critical**: Use `cargo check` for validation during development. Let users test with `cargo run` themselves.

### Audio File Requirements
- Soundfont file: `UprightPianoKW-small-bright-20190703.sf2` (hardcoded in `SF_PATH`)
- Output files: `output.wav` (intermediate) → user-specified MP3 (final)

## Project-Specific Patterns

### Note Handling Convention
```rust
// MIDI key calculation: base_key + octave * 12
// C2 = 24, C3 = 36, etc.
fn note_to_key(note: &str, octave: u8) -> u8
```

### Exercise Generation Patterns
Each exercise type has distinct musical patterns and range safety:

```rust
// Triads: Root → 3rd → 5th → 3rd → Root → Full Chord
fn get_major_chord(key: u8) -> Vec<u8> { vec![key, key + 4, key + 7] }
// Range: key_from..=(key_to - 7) // Ensures 5th fits

// Scales: Root → 2nd → 3rd → 4th → 5th → 4th → 3rd → 2nd → Root → Full Chord  
fn get_major_scale_to_fifth(key: u8) -> Vec<u8> { vec![key, key + 2, key + 4, key + 5, key + 7] }
// Range: key_from..=(key_to - 7) // Ensures 5th fits

// Octaves: Root → Octave → Both Together (Chord)
fn get_octave_chord(key: u8) -> Vec<u8> { vec![key, key + 12] }
// Range: key_from..=(key_to - 12) // Ensures octave fits
```

### Polymorphic Exercise System
```rust
// Central dispatcher pattern - add new exercises here
fn play_exercises_from(player: &mut dyn Player, exercise_type: ExerciseType, ...) {
    match exercise_type {
        ExerciseType::Triads => play_triads_from(...),
        ExerciseType::Scales => play_scales_from(...), 
        ExerciseType::Octaves => play_octaves_from(...),
    }
}
```

### Vocal Range Mappings
Predefined MIDI key ranges for each vocal type:
- Bass: E2-E4, Baritone: A2-A4, Tenor: C3-C5
- Alto: F3-F5, Mezzo-Soprano: A3-A5, Soprano: C4-C6

### Audio Processing Architecture
1. **Realtime**: `RealtimeEventSender` → `cpal` output stream
2. **File**: `ChannelGroup::read_samples()` → `AudioFileWriter` → background thread → WAV → MP3

### Error Handling Approach
- Minimal error handling with `.unwrap()` calls (CLI tool assumption)
- File I/O operations use `?` operator with `std::io::Error`
- Audio synthesis errors panic (development/debugging focus)

## Integration Points

### XSynth Event System
```rust
// All audio operations use SynthEvent message passing
SynthEvent::Channel(0, ChannelEvent::Audio(ChannelAudioEvent::NoteOn { key, vel: 127 }))
```

### Platform Dependencies
- Linux: Requires ALSA (`libasound2-dev`) - see CI workflow
- macOS/Windows: Uses CoreAudio/WASAPI via `cpal`

### MP3 Encoding Pipeline
Manual PCM buffer management required due to `mp3lame-encoder` API:
1. WAV samples → separate left/right i16 vectors
2. `DualPcm` input → encode → flush → concatenate buffers

### Release Automation
Release workflow generates pre-built MP3s for all vocal ranges:
```bash
# Pattern used in .github/workflows/release.yml
./target/release/rsiad --save <range>.mp3 --range <range>
```

## CLI Usage Patterns
- Default: Realtime baritone range (A2-A4) with 0.8s notes, triads exercise
- File output: `--save filename.mp3` switches to file mode
- Range override: `--from C3 --to C5` or `--range soprano`
- Exercise types: `--exercise triads` (default), `--exercise scales` for major scale patterns, or `--exercise octaves` for octave intervals
