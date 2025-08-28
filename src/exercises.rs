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
pub fn play_triad(player: &mut dyn Player, key: u8, note_duration: f64) {
    let chord = ExercisePatterns::major_chord(key);
    let triad = vec![chord[0], chord[1], chord[2], chord[1], chord[0]];
    for &key in &triad {
        player.play_note(key, note_duration);
    }
    player.wait(note_duration);
    player.play_chord(&chord, note_duration * 2.0);
}

pub fn play_scale(player: &mut dyn Player, key: u8, note_duration: f64) {
    let scale = ExercisePatterns::major_scale_to_fifth(key);
    // Play up: Root, 2nd, 3rd, 4th, 5th
    for &note in &scale {
        player.play_note(note, note_duration * 0.5);
    }
    // Play down: 4th, 3rd, 2nd, Root
    for &note in scale[0..4].iter().rev() {
        if note == scale[0] {
            player.play_note(note, note_duration);
        } else {
            player.play_note(note, note_duration * 0.5);
        }
    }
    player.wait(note_duration);
    // Play the full chord (root, 3rd, 5th)
    let chord = vec![scale[0], scale[2], scale[4]];
    player.play_chord(&chord, note_duration * 2.0);
}

pub fn play_octave(player: &mut dyn Player, key: u8, note_duration: f64) {
    let octave_notes = ExercisePatterns::octave_chord(key);
    // Play root note
    player.play_note(octave_notes[0], note_duration);
    // Play octave note
    player.play_note(octave_notes[1], note_duration);
    // Play both together as chord
    player.play_chord(&octave_notes, note_duration * 2.0);
}

/// Exercise range implementations
pub fn play_triads_from(player: &mut dyn Player, key_from: u8, key_to: u8, note_duration: f64) {
    for i in key_from..=(key_to - 7) {
        play_triad(player, i, note_duration);
    }
}

pub fn play_scales_from(player: &mut dyn Player, key_from: u8, key_to: u8, note_duration: f64) {
    for i in key_from..=(key_to - 7) {
        play_scale(player, i, note_duration);
    }
}

pub fn play_octaves_from(player: &mut dyn Player, key_from: u8, key_to: u8, note_duration: f64) {
    for i in key_from..=(key_to - 12) {
        play_octave(player, i, note_duration);
    }
}

/// Main exercise dispatcher
pub fn play_exercises_from(
    player: &mut dyn Player,
    exercise_type: ExerciseType,
    key_from: u8,
    key_to: u8,
    note_duration: f64,
) {
    match exercise_type {
        ExerciseType::Triads => play_triads_from(player, key_from, key_to, note_duration),
        ExerciseType::Scales => play_scales_from(player, key_from, key_to, note_duration),
        ExerciseType::Octaves => play_octaves_from(player, key_from, key_to, note_duration),
    }
}
