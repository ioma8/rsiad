use rsiad::{VocalExerciseEngine, ExerciseConfig, OutputMode, ExerciseType, ToneRange};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the engine
    let engine = VocalExerciseEngine::new("UprightPianoKW-small-bright-20190703.sf2");
    
    // Configure a triad exercise for soprano range
    let config = ExerciseConfig {
        exercise_type: ExerciseType::Triads,
        key_range: VocalExerciseEngine::get_vocal_range_keys(ToneRange::Soprano),
        note_duration: 0.5,
        vocal_range: Some(ToneRange::Soprano),
    };
    
    // Generate as MP3 file
    let output = OutputMode::File { 
        path: "soprano_triads_library.mp3".to_string() 
    };
    
    println!("Generating soprano triads using library API...");
    let result = engine.generate_exercise(config, output)?;
    
    println!("Generated exercise: {:?}", result);
    println!("File saved as: soprano_triads_library.mp3");
    
    Ok(())
}
