use clap::Parser;
use rsiad::{
    VocalExerciseEngine, ExerciseConfig, OutputMode, 
    ExerciseType, ToneRange, get_tone_range, note_string_to_key
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// If set, saves the output to a file as mp3 instead of playing it in realtime
    #[arg(short, long)]
    save: Option<String>,
    /// Duration of the note in seconds
    #[arg(short, long, default_value_t = 0.8)]
    duration: f64,
    /// Starting key of the range
    #[arg(short, long)]
    from: Option<String>,
    /// Ending key of the range
    #[arg(short, long)]
    to: Option<String>,
    /// Tone range of the singer
    #[arg(short, long, value_enum)]
    range: Option<ToneRange>,
    /// Type of vocal exercise to generate
    #[arg(short, long, value_enum, default_value_t = ExerciseType::Triads)]
    exercise: ExerciseType,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let engine = VocalExerciseEngine::new("UprightPianoKW-small-bright-20190703.sf2");
    
    // Determine key range
    let (range_from, range_to) = get_tone_range(args.range);
    let key_from = if let Some(from) = &args.from {
        note_string_to_key(from)
    } else {
        range_from
    };
    let key_to = if let Some(to) = &args.to {
        note_string_to_key(to)
    } else {
        range_to
    };
    
    let config = ExerciseConfig {
        exercise_type: args.exercise,
        key_range: (key_from, key_to),
        note_duration: args.duration,
        vocal_range: args.range,
    };
    
    let output = if let Some(save_path) = args.save {
        OutputMode::File { path: save_path }
    } else {
        OutputMode::Realtime
    };
    
    println!(
        "Playing {} from {} to {}",
        match args.exercise {
            ExerciseType::Triads => "triads",
            ExerciseType::Scales => "scales",
            ExerciseType::Octaves => "octaves",
        },
        key_from,
        key_to
    );
    
    let result = engine.generate_exercise(config, output)?;
    
    println!("Exercise completed: {:?}", result);
    Ok(())
}
