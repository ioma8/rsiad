use crate::render::XSynthRender;
use crate::audio::convert_wav_to_mp3;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use once_cell::sync::Lazy;
use xsynth_core::{
    channel::{ChannelAudioEvent, ChannelConfigEvent, ChannelEvent},
    channel_group::SynthEvent,
    soundfont::{SampleSoundfont, SoundfontBase},
    AudioStreamParams,
};
use xsynth_realtime::{RealtimeEventSender, RealtimeSynth, ThreadCount, XSynthRealtimeConfig};

const SF_PATH: &str = "UprightPianoKW-small-bright-20190703.sf2";
const WAV_OUTPUT_PATH: &str = "output.wav";

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
    fn load_soundfont(&mut self, params: AudioStreamParams);
    fn wait(&mut self, duration: f64) -> bool; // Returns false if stopped
    fn finalize(self: Box<Self>);
}

pub struct RealtimePlayer {
    sender: RealtimeEventSender,
    _synth: RealtimeSynth,
}

impl Player for RealtimePlayer {
    fn play_note(&mut self, key: u8, duration: f64) -> bool {
        if should_stop() {
            return false;
        }
        
        self.sender.send_event(SynthEvent::Channel(
            0,
            ChannelEvent::Audio(ChannelAudioEvent::NoteOn { key, vel: 127 }),
        ));
        
        if !self.wait(duration) {
            // Send note off even if stopped to avoid hanging notes
            self.sender.send_event(SynthEvent::Channel(
                0,
                ChannelEvent::Audio(ChannelAudioEvent::NoteOff { key }),
            ));
            return false;
        }
        
        self.sender.send_event(SynthEvent::Channel(
            0,
            ChannelEvent::Audio(ChannelAudioEvent::NoteOff { key }),
        ));
        true
    }

    fn play_chord(&mut self, keys: &[u8], duration: f64) -> bool {
        if should_stop() {
            return false;
        }
        
        for &key in keys {
            self.sender.send_event(SynthEvent::Channel(
                0,
                ChannelEvent::Audio(ChannelAudioEvent::NoteOn { key, vel: 127 }),
            ));
        }
        
        let result = self.wait(duration);
        
        // Always send note off events to avoid hanging notes
        for &key in keys {
            self.sender.send_event(SynthEvent::Channel(
                0,
                ChannelEvent::Audio(ChannelAudioEvent::NoteOff { key }),
            ));
        }
        
        result
    }

    fn load_soundfont(&mut self, params: AudioStreamParams) {
        println!("Loading Soundfont");
        let soundfonts: Vec<Arc<dyn SoundfontBase>> = vec![Arc::new(
            SampleSoundfont::new(SF_PATH, params, Default::default()).unwrap(),
        )];
        println!("Loaded");

        self.sender
            .send_event(SynthEvent::AllChannels(ChannelEvent::Config(
                ChannelConfigEvent::SetSoundfonts(soundfonts),
            )));
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
    synth: XSynthRender,
    save_path: String,
}

impl Player for FilePlayer {
    fn play_note(&mut self, key: u8, duration: f64) -> bool {
        // File generation doesn't support stopping mid-process, always returns true
        self.synth.send_event(SynthEvent::Channel(
            0,
            ChannelEvent::Audio(ChannelAudioEvent::NoteOn { key, vel: 127 }),
        ));
        self.wait(duration);
        self.synth.send_event(SynthEvent::Channel(
            0,
            ChannelEvent::Audio(ChannelAudioEvent::NoteOff { key }),
        ));
        true
    }

    fn play_chord(&mut self, keys: &[u8], duration: f64) -> bool {
        // File generation doesn't support stopping mid-process, always returns true
        for &key in keys {
            self.synth.send_event(SynthEvent::Channel(
                0,
                ChannelEvent::Audio(ChannelAudioEvent::NoteOn { key, vel: 127 }),
            ));
        }
        self.wait(duration);
        for &key in keys {
            self.synth.send_event(SynthEvent::Channel(
                0,
                ChannelEvent::Audio(ChannelAudioEvent::NoteOff { key }),
            ));
        }
        true
    }

    fn load_soundfont(&mut self, params: AudioStreamParams) {
        println!("Loading Soundfont");
        let soundfonts: Vec<Arc<dyn SoundfontBase>> = vec![Arc::new(
            SampleSoundfont::new(SF_PATH, params, Default::default()).unwrap(),
        )];
        println!("Loaded");

        self.synth
            .send_event(SynthEvent::AllChannels(ChannelEvent::Config(
                ChannelConfigEvent::SetSoundfonts(soundfonts),
            )));
    }

    fn wait(&mut self, duration: f64) -> bool {
        // File generation processes audio in chunks, not real-time
        self.synth.render_batch(duration);
        true
    }

    fn finalize(self: Box<Self>) {
        self.synth.finalize();
        println!("Converting to MP3...");
        convert_wav_to_mp3(WAV_OUTPUT_PATH, &self.save_path).unwrap();
        println!("Done!");
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
        let config = XSynthRealtimeConfig {
            multithreading: ThreadCount::Auto,
            render_window_ms: 50.0,
            ..Default::default()
        };
        let synth = RealtimeSynth::open_with_default_output(config);
        let sender = synth.get_sender_ref().clone();
        let player = RealtimePlayer {
            sender,
            _synth: synth,
        };
        Ok(PlayerType::Realtime(Box::new(player)))
    }
    
    pub fn create_file_player(output_path: String) -> Result<PlayerType, Box<dyn std::error::Error>> {
        let synth = XSynthRender::new(Default::default(), WAV_OUTPUT_PATH.into());
        let player = FilePlayer { 
            synth, 
            save_path: output_path 
        };
        Ok(PlayerType::File(Box::new(player)))
    }
}
