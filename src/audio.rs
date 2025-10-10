use crate::exercises::ToneRange;
use std::fs::File;
use std::io::Read;
use mp3lame_encoder::{Builder, DualPcm, FlushNoGap, Quality};

/// Convert note string (e.g., "C4") to MIDI key number
pub fn note_string_to_key(note_string: &str) -> u8 {
    let note = note_string.trim_end_matches(char::is_numeric);
    let octave = note_string
        .chars()
        .last()
        .unwrap_or('0')
        .to_digit(10)
        .unwrap_or(0) as u8;
    note_to_key(note, octave)
}

/// Convert note name and octave to MIDI key number
pub fn note_to_key(note: &str, octave: u8) -> u8 {
    let base_key: u8 = match note {
        "C" => 24,
        "C#" | "Db" => 25,
        "D" => 26,
        "D#" | "Eb" => 27,
        "E" => 28,
        "F" => 29,
        "F#" | "Gb" => 30,
        "G" => 31,
        "G#" | "Ab" => 32,
        "A" => 33,
        "A#" | "Bb" => 34,
        "B" => 35,
        _ => {
            eprintln!("Warning: Invalid note '{}', defaulting to C", note);
            24 // Default to C
        }
    };
    base_key.saturating_add(octave.saturating_mul(12))
}

/// Get the MIDI key range for a vocal range
pub fn get_tone_range(range: Option<ToneRange>) -> (u8, u8) {
    match range {
        Some(ToneRange::Bass) => (note_string_to_key("E2"), note_string_to_key("E4")),
        Some(ToneRange::Baritone) => (note_string_to_key("A2"), note_string_to_key("A4")),
        Some(ToneRange::Tenor) => (note_string_to_key("C3"), note_string_to_key("C5")),
        Some(ToneRange::Alto) => (note_string_to_key("F3"), note_string_to_key("F5")),
        Some(ToneRange::MezzoSoprano) => (note_string_to_key("A3"), note_string_to_key("A5")),
        Some(ToneRange::Soprano) => (note_string_to_key("C4"), note_string_to_key("C6")),
        None => (note_string_to_key("A2"), note_string_to_key("A4")),
    }
}

/// Convert WAV file to MP3
pub fn convert_wav_to_mp3(wav_path: &str, mp3_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut wav_file = File::open(wav_path)
        .map_err(|e| format!("Failed to open WAV file '{}': {}", wav_path, e))?;
    let mut wav_data = Vec::new();
    wav_file.read_to_end(&mut wav_data)
        .map_err(|e| format!("Failed to read WAV file: {}", e))?;

    let mut mp3_file = File::create(mp3_path)
        .map_err(|e| format!("Failed to create MP3 file '{}': {}", mp3_path, e))?;

    let wav = hound::WavReader::new(&wav_data[..])
        .map_err(|e| format!("Failed to parse WAV data: {}", e))?;
    let mut samples = wav.into_samples::<i16>();
    let mut pcm_left = Vec::new();
    let mut pcm_right = Vec::new();

    while let (Some(left), Some(right)) = (samples.next(), samples.next()) {
        pcm_left.push(left.map_err(|e| format!("Failed to read sample: {}", e))?);
        pcm_right.push(right.map_err(|e| format!("Failed to read sample: {}", e))?);
    }

    if pcm_left.is_empty() {
        return Err("WAV file contains no audio data".into());
    }

    let mut encoder = Builder::new()
        .ok_or("Failed to create LAME encoder")?;
    encoder.set_num_channels(2)
        .map_err(|e| format!("Failed to set channel count: {:?}", e))?;
    encoder.set_sample_rate(44100)
        .map_err(|e| format!("Failed to set sample rate: {:?}", e))?;
    encoder.set_quality(Quality::Best)
        .map_err(|e| format!("Failed to set quality: {:?}", e))?;
    let mut encoder = encoder.build()
        .map_err(|e| format!("Failed to build LAME encoder: {:?}", e))?;

    let input = DualPcm {
        left: &pcm_left,
        right: &pcm_right,
    };

    let mut mp3_buffer = Vec::new();
    mp3_buffer.resize(mp3lame_encoder::max_required_buffer_size(pcm_left.len()), 0);
    
    // SAFETY: We're transmuting a slice of u8 to MaybeUninit<u8>, which is always safe
    // because MaybeUninit<u8> has the same layout as u8
    let mp3_buffer_uninit = unsafe {
        std::slice::from_raw_parts_mut(
            mp3_buffer.as_mut_ptr() as *mut std::mem::MaybeUninit<u8>,
            mp3_buffer.len()
        )
    };

    let encoded_size = encoder.encode(input, mp3_buffer_uninit)
        .map_err(|e| format!("Failed to encode MP3: {:?}", e))?;
    mp3_buffer.truncate(encoded_size);

    let mut final_mp3_buffer = Vec::new();
    final_mp3_buffer.resize(7200, 0);
    
    // SAFETY: Same as above
    let final_mp3_buffer_uninit = unsafe {
        std::slice::from_raw_parts_mut(
            final_mp3_buffer.as_mut_ptr() as *mut std::mem::MaybeUninit<u8>,
            final_mp3_buffer.len()
        )
    };
    
    let encoded_size = encoder
        .flush::<FlushNoGap>(final_mp3_buffer_uninit)
        .map_err(|e| format!("Failed to flush encoder: {:?}", e))?;
    final_mp3_buffer.truncate(encoded_size);
    mp3_buffer.extend_from_slice(&final_mp3_buffer);

    std::io::Write::write_all(&mut mp3_file, &mp3_buffer)
        .map_err(|e| format!("Failed to write MP3 data: {}", e))?;

    Ok(())
}
