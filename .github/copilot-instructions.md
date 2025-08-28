# RSIAD - Rust Vocal Warmup Exercise Library

## Architecture Overview

RSIAD is a **library-first** audio synthesis tool that generates musical exercises for vocal warmup. The codebase uses a **dual-mode architecture** with a clean separation between library and CLI:

- **Library** (`src/lib.rs`): Core functionality exposed through `VocalExerciseEngine` API
- **CLI Binary** (`src/main.rs`): Command-line interface that consumes the library
- **Dual Audio Modes**: Realtime playback vs. file generation (WAV→MP3)

Both modes implement the `Player` trait enabling polymorphic audio handling across three exercise types: triads, scales, and octaves.

## Key Components

### Library Structure
- **`lib.rs`**: Main API with `VocalExerciseEngine`, `ExerciseConfig`, `OutputMode`
- **`exercises.rs`**: Exercise patterns (triads, scales, octaves) and `ExerciseType` enum
- **`player.rs`**: `Player` trait, `RealtimePlayer`, `FilePlayer`, `PlayerFactory`
- **`audio.rs`**: Note conversion utilities, vocal ranges, MP3 encoding
- **`render.rs`**: Custom file-based synthesizer wrapper around `xsynth-core`
- **`writer.rs`**: Threaded WAV file writer using `crossbeam-channel`
- **`config.rs`**: XSynth configuration with audio parameters

### CLI Integration
- **`main.rs`**: Minimal CLI that parses args and calls library
- **`examples/`**: Demonstrates programmatic library usage

## Development Workflow

### Build & Test Pattern
```bash
cargo check      # ALWAYS run after each change
cargo build      # For release builds
cargo run        # CLI interface
cargo run --example library_usage  # Library API demo
```

**Critical**: Use `cargo check` for validation during development.

## Library API Usage

### Basic Pattern
```rust
use rsiad::{VocalExerciseEngine, ExerciseConfig, OutputMode, ExerciseType, ToneRange};

let engine = VocalExerciseEngine::new("UprightPianoKW-small-bright-20190703.sf2");
let config = ExerciseConfig {
    exercise_type: ExerciseType::Triads,
    key_range: (60, 72), // C3 to C4
    note_duration: 0.8,
    vocal_range: Some(ToneRange::Baritone),
};
let result = engine.generate_exercise(config, OutputMode::File { path: "output.mp3".into() })?;
```

### Exercise Generation Patterns
Each exercise type has distinct musical patterns and range safety:

```rust
// Triads: Root → 3rd → 5th → 3rd → Root → Full Chord
ExercisePatterns::major_chord(key) // vec![key, key + 4, key + 7]
// Range: key_from..=(key_to - 7) // Ensures 5th fits

// Scales: Root → 2nd → 3rd → 4th → 5th → 4th → 3rd → 2nd → Root → Full Chord  
ExercisePatterns::major_scale_to_fifth(key) // vec![key, key + 2, key + 4, key + 5, key + 7]
// Range: key_from..=(key_to - 7) // Ensures 5th fits

// Octaves: Root → Octave → Both Together (Chord)
ExercisePatterns::octave_chord(key) // vec![key, key + 12]
// Range: key_from..=(key_to - 12) // Ensures octave fits
```

### Polymorphic Player System
```rust
// Central dispatcher pattern - add new exercises here
pub fn play_exercises_from(player: &mut dyn Player, exercise_type: ExerciseType, ...) {
    match exercise_type {
        ExerciseType::Triads => play_triads_from(...),
        ExerciseType::Scales => play_scales_from(...), 
        ExerciseType::Octaves => play_octaves_from(...),
    }
}
```

### Player Factory Pattern
```rust
let player = match output_mode {
    OutputMode::Realtime => PlayerFactory::create_realtime_player()?,
    OutputMode::File { path } => PlayerFactory::create_file_player(path)?,
};
```

## Project-Specific Patterns

### Note Handling Convention
```rust
// MIDI key calculation: base_key + octave * 12
// C2 = 24, C3 = 36, etc.
VocalExerciseEngine::note_to_key(note: &str, octave: u8) -> u8
VocalExerciseEngine::note_string_to_key(note_string: &str) -> u8  // "C4" -> 60
```

### Vocal Range Mappings
```rust
VocalExerciseEngine::get_vocal_range_keys(ToneRange::Baritone) // (A2, A4)
// Bass: E2-E4, Baritone: A2-A4, Tenor: C3-C5
// Alto: F3-F5, Mezzo-Soprano: A3-A5, Soprano: C4-C6
```

### Audio Processing Architecture
1. **Realtime**: `RealtimeEventSender` → `cpal` output stream
2. **File**: `ChannelGroup::read_samples()` → `AudioFileWriter` → background thread → WAV → MP3

### Error Handling Approach
- Library returns `Result<ExerciseResult, Box<dyn std::error::Error>>`
- Internal audio synthesis errors panic (development/debugging focus)
- File I/O operations use `?` operator with `std::io::Error`

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
Manual PCM buffer management in `audio::convert_wav_to_mp3()`:
1. WAV samples → separate left/right i16 vectors
2. `DualPcm` input → encode → flush → concatenate buffers

### Future GUI Integration
Library design enables simple GUI integration:
```rust
// GUI can directly use the library API
let engine = VocalExerciseEngine::new(soundfont_path);
let result = engine.generate_exercise(user_config, output_mode)?;
```

## CLI Usage Patterns
- Default: Realtime baritone range (A2-A4) with 0.8s notes, triads exercise
- File output: `--save filename.mp3` switches to file mode
- Range override: `--from C3 --to C5` or `--range soprano`
- Exercise types: `--exercise triads` (default), `--exercise scales` for major scale patterns, or `--exercise octaves` for octave intervals

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
