use crate::player::Player;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize)]
pub enum ToneRange {
    Bass,
    Baritone,
    Tenor,
    Alto,
    MezzoSoprano,
    Soprano,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize)]
pub enum ExerciseType {
    Triads,
    Scales,
    Octaves,
}

/// Exercise pattern generators
pub struct ExercisePatterns;

impl ExercisePatterns {
    pub fn major_chord(key: u8) -> Vec<u8> {
        vec![key, key + 4, key + 7] // Root, Major 3rd, Perfect 5th
    }
    
    pub fn major_scale_to_fifth(key: u8) -> Vec<u8> {
        vec![key, key + 2, key + 4, key + 5, key + 7] // Root, 2nd, 3rd, 4th, 5th
    }
    
    pub fn octave_chord(key: u8) -> Vec<u8> {
        vec![key, key + 12] // Root and octave
    }
}

/// Individual exercise implementations
pub fn play_triad(player: &mut dyn Player, key: u8, note_duration: f64) -> bool {
    let chord = ExercisePatterns::major_chord(key);
    let triad = vec![chord[0], chord[1], chord[2], chord[1], chord[0]];
    for &key in &triad {
        if !player.play_note(key, note_duration) {
            return false; // Stopped
        }
    }
    if !player.wait(note_duration) {
        return false; // Stopped
    }
    player.play_chord(&chord, note_duration * 2.0)
}

pub fn play_scale(player: &mut dyn Player, key: u8, note_duration: f64) -> bool {
    let scale = ExercisePatterns::major_scale_to_fifth(key);
    // Play up: Root, 2nd, 3rd, 4th, 5th
    for &note in &scale {
        if !player.play_note(note, note_duration * 0.5) {
            return false; // Stopped
        }
    }
    // Play down: 4th, 3rd, 2nd, Root
    for &note in scale[0..4].iter().rev() {
        if note == scale[0] {
            if !player.play_note(note, note_duration) {
                return false; // Stopped
            }
        } else {
            if !player.play_note(note, note_duration * 0.5) {
                return false; // Stopped
            }
        }
    }
    if !player.wait(note_duration) {
        return false; // Stopped
    }
    // Play the full chord (root, 3rd, 5th)
    let chord = vec![scale[0], scale[2], scale[4]];
    player.play_chord(&chord, note_duration * 2.0)
}

pub fn play_octave(player: &mut dyn Player, key: u8, note_duration: f64) -> bool {
    let octave_notes = ExercisePatterns::octave_chord(key);
    // Play root note
    if !player.play_note(octave_notes[0], note_duration) {
        return false; // Stopped
    }
    // Play octave note
    if !player.play_note(octave_notes[1], note_duration) {
        return false; // Stopped
    }
    // Play both together as chord
    player.play_chord(&octave_notes, note_duration * 2.0)
}

/// Exercise range implementations
pub fn play_triads_from(player: &mut dyn Player, key_from: u8, key_to: u8, note_duration: f64) -> bool {
    // Validate that we have enough range for a triad (needs 7 semitones)
    if key_to < key_from || key_to - key_from < 7 {
        eprintln!("Warning: Invalid key range for triads: {} to {}", key_from, key_to);
        return false;
    }
    
    for i in key_from..=(key_to - 7) {
        if !play_triad(player, i, note_duration) {
            return false; // Stopped
        }
    }
    true
}

pub fn play_scales_from(player: &mut dyn Player, key_from: u8, key_to: u8, note_duration: f64) -> bool {
    // Validate that we have enough range for a scale to 5th (needs 7 semitones)
    if key_to < key_from || key_to - key_from < 7 {
        eprintln!("Warning: Invalid key range for scales: {} to {}", key_from, key_to);
        return false;
    }
    
    for i in key_from..=(key_to - 7) {
        if !play_scale(player, i, note_duration) {
            return false; // Stopped
        }
    }
    true
}

pub fn play_octaves_from(player: &mut dyn Player, key_from: u8, key_to: u8, note_duration: f64) -> bool {
    // Validate that we have enough range for octaves (needs 12 semitones)
    if key_to < key_from || key_to - key_from < 12 {
        eprintln!("Warning: Invalid key range for octaves: {} to {}", key_from, key_to);
        return false;
    }
    
    for i in key_from..=(key_to - 12) {
        if !play_octave(player, i, note_duration) {
            return false; // Stopped
        }
    }
    true
}

/// Main exercise dispatcher
pub fn play_exercises_from(
    player: &mut dyn Player,
    exercise_type: ExerciseType,
    key_from: u8,
    key_to: u8,
    note_duration: f64,
) -> bool {
    match exercise_type {
        ExerciseType::Triads => play_triads_from(player, key_from, key_to, note_duration),
        ExerciseType::Scales => play_scales_from(player, key_from, key_to, note_duration),
        ExerciseType::Octaves => play_octaves_from(player, key_from, key_to, note_duration),
    }
}
