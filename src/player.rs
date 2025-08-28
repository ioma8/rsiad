use crate::render::XSynthRender;
use crate::audio::convert_wav_to_mp3;
use std::sync::Arc;
use std::time::Duration;
use xsynth_core::{
    channel::{ChannelAudioEvent, ChannelConfigEvent, ChannelEvent},
    channel_group::SynthEvent,
    soundfont::{SampleSoundfont, SoundfontBase},
    AudioStreamParams,
};
use xsynth_realtime::{RealtimeEventSender, RealtimeSynth, ThreadCount, XSynthRealtimeConfig};

const SF_PATH: &str = "UprightPianoKW-small-bright-20190703.sf2";
const WAV_OUTPUT_PATH: &str = "output.wav";

pub trait Player {
    fn play_note(&mut self, key: u8, duration: f64);
    fn play_chord(&mut self, keys: &[u8], duration: f64);
    fn load_soundfont(&mut self, params: AudioStreamParams);
    fn wait(&mut self, duration: f64);
    fn finalize(self: Box<Self>);
}

pub struct RealtimePlayer {
    sender: RealtimeEventSender,
    _synth: RealtimeSynth,
}

impl Player for RealtimePlayer {
    fn play_note(&mut self, key: u8, duration: f64) {
        self.sender.send_event(SynthEvent::Channel(
            0,
            ChannelEvent::Audio(ChannelAudioEvent::NoteOn { key, vel: 127 }),
        ));
        self.wait(duration);
        self.sender.send_event(SynthEvent::Channel(
            0,
            ChannelEvent::Audio(ChannelAudioEvent::NoteOff { key }),
        ));
    }

    fn play_chord(&mut self, keys: &[u8], duration: f64) {
        for &key in keys {
            self.sender.send_event(SynthEvent::Channel(
                0,
                ChannelEvent::Audio(ChannelAudioEvent::NoteOn { key, vel: 127 }),
            ));
        }
        self.wait(duration);
        for &key in keys {
            self.sender.send_event(SynthEvent::Channel(
                0,
                ChannelEvent::Audio(ChannelAudioEvent::NoteOff { key }),
            ));
        }
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

    fn wait(&mut self, duration: f64) {
        spin_sleep::sleep(Duration::from_secs_f64(duration));
    }

    fn finalize(self: Box<Self>) {}
}

pub struct FilePlayer {
    synth: XSynthRender,
    save_path: String,
}

impl Player for FilePlayer {
    fn play_note(&mut self, key: u8, duration: f64) {
        self.synth.send_event(SynthEvent::Channel(
            0,
            ChannelEvent::Audio(ChannelAudioEvent::NoteOn { key, vel: 127 }),
        ));
        self.wait(duration);
        self.synth.send_event(SynthEvent::Channel(
            0,
            ChannelEvent::Audio(ChannelAudioEvent::NoteOff { key }),
        ));
    }

    fn play_chord(&mut self, keys: &[u8], duration: f64) {
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

    fn wait(&mut self, duration: f64) {
        self.synth.render_batch(duration);
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
