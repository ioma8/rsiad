use crate::audio::convert_wav_to_mp3;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use once_cell::sync::Lazy;
use fluidlite::{Settings, Synth};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::Mutex;
use std::fs::File;

const WAV_OUTPUT_PATH: &str = "output.wav";
const SAMPLE_RATE: u32 = 44100;

// Global stop flag for playback control
static STOP_PLAYBACK: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

pub fn stop_playback() {
    STOP_PLAYBACK.store(true, Ordering::Relaxed);
}

pub fn reset_stop_flag() {
    STOP_PLAYBACK.store(false, Ordering::Relaxed);
}

fn should_stop() -> bool {
    STOP_PLAYBACK.load(Ordering::Relaxed)
}

pub trait Player {
    fn play_note(&mut self, key: u8, duration: f64) -> bool; // Returns false if stopped
    fn play_chord(&mut self, keys: &[u8], duration: f64) -> bool; // Returns false if stopped
    fn load_soundfont(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn wait(&mut self, duration: f64) -> bool; // Returns false if stopped
    fn finalize(self: Box<Self>);
}

pub struct RealtimePlayer {
    synth: Arc<Mutex<Synth>>,
    _stream: cpal::Stream,
}

impl Player for RealtimePlayer {
    fn play_note(&mut self, key: u8, duration: f64) -> bool {
        if should_stop() {
            return false;
        }
        
        let synth = self.synth.lock().unwrap();
        if synth.note_on(0, key as u32, 127).is_err() {
            eprintln!("Warning: Failed to send note_on for key {}", key);
        }
        drop(synth);
        
        if !self.wait(duration) {
            // Send note off even if stopped to avoid hanging notes
            let synth = self.synth.lock().unwrap();
            let _ = synth.note_off(0, key as u32);
            return false;
        }
        
        let synth = self.synth.lock().unwrap();
        if synth.note_off(0, key as u32).is_err() {
            eprintln!("Warning: Failed to send note_off for key {}", key);
        }
        true
    }

    fn play_chord(&mut self, keys: &[u8], duration: f64) -> bool {
        if should_stop() {
            return false;
        }
        
        let synth = self.synth.lock().unwrap();
        for &key in keys {
            if synth.note_on(0, key as u32, 127).is_err() {
                eprintln!("Warning: Failed to send note_on for key {} in chord", key);
            }
        }
        drop(synth);
        
        let result = self.wait(duration);
        
        // Always send note off events to avoid hanging notes
        let synth = self.synth.lock().unwrap();
        for &key in keys {
            let _ = synth.note_off(0, key as u32);
        }
        
        result
    }

    fn load_soundfont(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Loading Soundfont: {}", path);
        let synth = self.synth.lock().unwrap();
        synth.sfload(path, true)
            .map_err(|_| format!("Failed to load soundfont from: {}", path))?;
        println!("Loaded");
        Ok(())
    }

    fn wait(&mut self, duration: f64) -> bool {
        // Break up the sleep into smaller chunks to check for stop more frequently
        let chunk_duration = Duration::from_millis(50); // Check every 50ms
        let total_duration = Duration::from_secs_f64(duration);
        let mut elapsed = Duration::ZERO;
        
        while elapsed < total_duration {
            if should_stop() {
                return false;
            }
            
            let remaining = total_duration - elapsed;
            let sleep_time = if remaining < chunk_duration {
                remaining
            } else {
                chunk_duration
            };
            
            spin_sleep::sleep(sleep_time);
            elapsed += sleep_time;
        }
        
        true
    }

    fn finalize(self: Box<Self>) {}
}

pub struct FilePlayer {
    synth: Synth,
    save_path: String,
    wav_writer: hound::WavWriter<std::io::BufWriter<File>>,
    buffer: Vec<i16>, // Reusable buffer
}

impl Player for FilePlayer {
    fn play_note(&mut self, key: u8, duration: f64) -> bool {
        // File generation doesn't support stopping mid-process, always returns true
        if self.synth.note_on(0, key as u32, 127).is_err() {
            eprintln!("Warning: Failed to send note_on for key {}", key);
        }
        self.wait(duration);
        let _ = self.synth.note_off(0, key as u32);
        true
    }

    fn play_chord(&mut self, keys: &[u8], duration: f64) -> bool {
        // File generation doesn't support stopping mid-process, always returns true
        for &key in keys {
            if self.synth.note_on(0, key as u32, 127).is_err() {
                eprintln!("Warning: Failed to send note_on for key {} in chord", key);
            }
        }
        self.wait(duration);
        for &key in keys {
            let _ = self.synth.note_off(0, key as u32);
        }
        true
    }

    fn load_soundfont(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Loading Soundfont: {}", path);
        self.synth.sfload(path, true)
            .map_err(|_| format!("Failed to load soundfont from: {}", path))?;
        println!("Loaded");
        Ok(())
    }

    fn wait(&mut self, duration: f64) -> bool {
        // Calculate number of samples needed for this duration
        let num_samples = (SAMPLE_RATE as f64 * duration) as usize;
        let buffer_size = num_samples * 2; // Stereo
        
        // Resize buffer if needed
        if self.buffer.len() < buffer_size {
            self.buffer.resize(buffer_size, 0);
        }
        
        // Use only the portion we need
        let buffer_slice = &mut self.buffer[..buffer_size];
        
        if self.synth.write::<&mut [i16]>(buffer_slice).is_err() {
            eprintln!("Warning: Failed to generate audio samples");
            return true;
        }
        
        // Write to WAV file
        for sample in buffer_slice.iter() {
            if let Err(e) = self.wav_writer.write_sample(*sample) {
                eprintln!("Warning: Failed to write sample to WAV: {}", e);
            }
        }
        
        true
    }

    fn finalize(self: Box<Self>) {
        if let Err(e) = self.wav_writer.finalize() {
            eprintln!("Error finalizing WAV file: {}", e);
            return;
        }
        
        println!("Converting to MP3...");
        match convert_wav_to_mp3(WAV_OUTPUT_PATH, &self.save_path) {
            Ok(_) => {
                // Clean up WAV file after successful conversion
                if let Err(e) = std::fs::remove_file(WAV_OUTPUT_PATH) {
                    eprintln!("Warning: Failed to delete temporary WAV file: {}", e);
                }
                println!("Done!");
            }
            Err(e) => {
                eprintln!("Error converting to MP3: {}", e);
            }
        }
    }
}

pub enum PlayerType {
    Realtime(Box<RealtimePlayer>),
    File(Box<FilePlayer>),
}

impl PlayerType {
    pub fn as_mut(&mut self) -> &mut dyn Player {
        match self {
            PlayerType::Realtime(player) => player.as_mut(),
            PlayerType::File(player) => player.as_mut(),
        }
    }
    
    pub fn finalize(self) {
        match self {
            PlayerType::Realtime(player) => player.finalize(),
            PlayerType::File(player) => player.finalize(),
        }
    }
}

/// Factory for creating players
pub struct PlayerFactory;

impl PlayerFactory {
    pub fn create_realtime_player() -> Result<PlayerType, Box<dyn std::error::Error>> {
        let settings = Settings::new()
            .map_err(|_| "Failed to create FluidLite settings")?;
        let synth = Synth::new(settings)
            .map_err(|_| "Failed to create FluidLite synthesizer")?;
        
        let synth = Arc::new(Mutex::new(synth));
        let synth_clone = Arc::clone(&synth);
        
        // Set up audio output stream
        let host = cpal::default_host();
        let device = host.default_output_device()
            .ok_or("No output device available")?;
        
        // Get the default output config from the device
        let supported_config = device.default_output_config()?;
        let sample_rate = supported_config.sample_rate().0;
        let channels = supported_config.channels();
        
        // Set fluidlite to use the device's sample rate
        {
            let synth = synth.lock().unwrap();
            synth.set_sample_rate(sample_rate as f32);
        }
        
        let config = cpal::StreamConfig {
            channels,
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };
        
        // Build stream based on the sample format
        let stream = match supported_config.sample_format() {
            cpal::SampleFormat::I16 => {
                device.build_output_stream(
                    &config,
                    move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                        let synth = synth_clone.lock().unwrap();
                        // Don't panic in audio callback - just log and fill with silence
                        if synth.write::<&mut [i16]>(data).is_err() {
                            data.fill(0);
                        }
                    },
                    |err| eprintln!("Audio stream error: {}", err),
                    None,
                )?
            }
            cpal::SampleFormat::F32 => {
                device.build_output_stream(
                    &config,
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        let synth = synth_clone.lock().unwrap();
                        // Don't panic in audio callback - just log and fill with silence
                        if synth.write::<&mut [f32]>(data).is_err() {
                            data.fill(0.0);
                        }
                    },
                    |err| eprintln!("Audio stream error: {}", err),
                    None,
                )?
            }
            format => {
                return Err(format!("Unsupported sample format: {:?}", format).into());
            }
        };
        
        stream.play()?;
        
        let player = RealtimePlayer {
            synth,
            _stream: stream,
        };
        Ok(PlayerType::Realtime(Box::new(player)))
    }
    
    pub fn create_file_player(output_path: String) -> Result<PlayerType, Box<dyn std::error::Error>> {
        let settings = Settings::new()
            .map_err(|_| "Failed to create FluidLite settings")?;
        let synth = Synth::new(settings)
            .map_err(|_| "Failed to create FluidLite synthesizer")?;
        synth.set_sample_rate(SAMPLE_RATE as f32);
        
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: SAMPLE_RATE,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let wav_writer = hound::WavWriter::create(WAV_OUTPUT_PATH, spec)
            .map_err(|e| format!("Failed to create WAV file: {}", e))?;
        
        let player = FilePlayer { 
            synth, 
            save_path: output_path,
            wav_writer,
            buffer: Vec::with_capacity(SAMPLE_RATE as usize * 2), // Pre-allocate for ~1 second
        };
        Ok(PlayerType::File(Box::new(player)))
    }
}
