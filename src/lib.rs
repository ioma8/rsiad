//! RSIAD - Rust Vocal Warmup Exercise Library
//! 
//! This library provides audio synthesis capabilities for generating vocal warmup exercises.

pub mod player;
pub mod exercises;
pub mod audio;

// Re-export key types for the public API
pub use exercises::{ExerciseType, ToneRange};
pub use player::{Player, PlayerType, PlayerFactory, stop_playback, reset_stop_flag};
pub use audio::{note_to_key, note_string_to_key, get_tone_range, convert_wav_to_mp3};

/// Main library interface for vocal warmup exercises
pub struct VocalExerciseEngine {
    soundfont_path: String,
}

/// Configuration for exercise generation
#[derive(Debug, Clone)]
pub struct ExerciseConfig {
    pub exercise_type: ExerciseType,
    pub key_range: (u8, u8),
    pub note_duration: f64,
    pub vocal_range: Option<ToneRange>,
}

/// Output mode for exercises
#[derive(Debug, Clone)]
pub enum OutputMode {
    Realtime,
    File { path: String },
}

/// Results from exercise generation
#[derive(Debug)]
pub struct ExerciseResult {
    pub duration_seconds: f64,
    pub notes_played: usize,
    pub key_range: (u8, u8),
}

impl VocalExerciseEngine {
    /// Create a new vocal exercise engine with the specified soundfont
    pub fn new(soundfont_path: impl Into<String>) -> Self {
        Self {
            soundfont_path: soundfont_path.into(),
        }
    }
    
    /// Generate and play/save a vocal exercise
    pub fn generate_exercise(
        &self, 
        config: ExerciseConfig, 
        output: OutputMode
    ) -> Result<ExerciseResult, Box<dyn std::error::Error>> {
        let mut player = match output {
            OutputMode::Realtime => PlayerFactory::create_realtime_player()?,
            OutputMode::File { path } => PlayerFactory::create_file_player(path)?,
        };
        
        // Load soundfont
        player.as_mut().load_soundfont(&self.soundfont_path)?;
        
        // Calculate exercise metrics
        let (key_from, key_to) = config.key_range;
        let range_size = match config.exercise_type {
            ExerciseType::Triads | ExerciseType::Scales => (key_to - key_from).saturating_sub(7),
            ExerciseType::Octaves => (key_to - key_from).saturating_sub(12),
        };
        
        // Reset stop flag before starting
        player::reset_stop_flag();
        
        // Play the exercise
        let completed = exercises::play_exercises_from(
            player.as_mut(),
            config.exercise_type,
            key_from,
            key_to,
            config.note_duration,
        );
        
        // Finalize
        player.finalize();
        
        if !completed {
            return Err("Exercise was stopped by user".into());
        }
        
        Ok(ExerciseResult {
            duration_seconds: (range_size as f64 + 1.0) * config.note_duration * 6.0, // Rough estimate
            notes_played: (range_size as usize + 1) * 6, // Rough estimate  
            key_range: (key_from, key_to),
        })
    }
    
    /// Get MIDI key range for a vocal range
    pub fn get_vocal_range_keys(range: ToneRange) -> (u8, u8) {
        get_tone_range(Some(range))
    }
    
    /// Convert note name and octave to MIDI key
    pub fn note_to_key(note: &str, octave: u8) -> u8 {
        note_to_key(note, octave)
    }
    
    /// Convert note string (e.g., "C4") to MIDI key
    pub fn note_string_to_key(note_string: &str) -> u8 {
        note_string_to_key(note_string)
    }
}
