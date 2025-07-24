mod config;
mod render;
mod writer;

use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;

use crate::render::XSynthRender;
use mp3lame_encoder::{Builder, DualPcm, FlushNoGap, Quality};
use xsynth_core::{
    channel::{ChannelAudioEvent, ChannelConfigEvent, ChannelEvent, ControlEvent},
    channel_group::SynthEvent,
    soundfont::{SampleSoundfont, SoundfontBase},
    AudioStreamParams,
};
use xsynth_realtime::{RealtimeEventSender, RealtimeSynth};

const SF_PATH: &str = "Yamaha_C3_Grand_Piano.sf2";
const WAV_OUTPUT_PATH: &str = "output.wav";
const MP3_OUTPUT_PATH: &str = "output.mp3";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Whether to save the output to an MP3 file
    #[arg(short, long)]
    save: bool,
}

trait Player {
    fn play_note(&mut self, key: u8, duration: f64, pan: f32);
    fn play_chord(&mut self, keys: &[u8], duration: f64);
    fn load_soundfont(&mut self, params: AudioStreamParams);
    fn wait(&mut self, duration: f64);
    fn finalize(self: Box<Self>);
}

struct RealtimePlayer {
    sender: RealtimeEventSender,
}

impl Player for RealtimePlayer {
    fn play_note(&mut self, key: u8, duration: f64, pan: f32) {
        let pan_value = ((pan + 1.0) / 2.0 * 127.0) as u8;
        self.sender.send_event(SynthEvent::Channel(
            0,
            ChannelEvent::Audio(ChannelAudioEvent::Control(ControlEvent::Raw(10, pan_value))),
        ));
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

struct FilePlayer {
    synth: XSynthRender,
}

impl Player for FilePlayer {
    fn play_note(&mut self, key: u8, duration: f64, pan: f32) {
        let pan_value = ((pan + 1.0) / 2.0 * 127.0) as u8;
        self.synth.send_event(SynthEvent::Channel(
            0,
            ChannelEvent::Audio(ChannelAudioEvent::Control(ControlEvent::Raw(10, pan_value))),
        ));
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
        convert_wav_to_mp3(WAV_OUTPUT_PATH, MP3_OUTPUT_PATH).unwrap();
        println!("Done!");
    }
}

fn main() {
    let args = Args::parse();

    let (mut player, params): (Box<dyn Player>, AudioStreamParams) = if args.save {
        let synth = XSynthRender::new(Default::default(), WAV_OUTPUT_PATH.into());
        let params = synth.get_params();
        (Box::new(FilePlayer { synth }), params)
    } else {
        let synth = RealtimeSynth::open_with_all_defaults();
        let params = synth.stream_params();
        let sender = synth.get_sender_ref().clone();
        (Box::new(RealtimePlayer { sender }), params)
    };

    player.load_soundfont(params);

    play_triads_from(player.as_mut(), note_to_key("G", 2), note_to_key("G", 3));

    player.finalize();
}

fn note_to_key(note: &str, octave: u8) -> u8 {
    let base_key = match note {
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
        _ => panic!("Invalid note: {}", note),
    };
    base_key + octave * 12
}

fn get_major_chord(key: u8) -> Vec<u8> {
    vec![key, key + 4, key + 7] // Root, Major 3rd, Perfect 5th
}

fn play_triad(player: &mut dyn Player, key: u8) {
    let chord = get_major_chord(key);
    let triad = vec![chord[0], chord[1], chord[2], chord[1], chord[0]];
    let mut pan = -0.5;
    for &key in &triad {
        player.play_note(key, 0.7, pan);
        pan *= -1.0;
    }
    player.wait(0.7);
    player.play_chord(&chord, 1.4);
}

fn play_triads_from(player: &mut dyn Player, key_from: u8, key_to: u8) {
    for i in key_from..=key_to {
        play_triad(player, i);
    }
}

fn convert_wav_to_mp3(wav_path: &str, mp3_path: &str) -> Result<(), std::io::Error> {
    let mut wav_file = File::open(wav_path)?;
    let mut wav_data = Vec::new();
    wav_file.read_to_end(&mut wav_data)?;

    let mut mp3_file = File::create(mp3_path)?;

    let wav = hound::WavReader::new(&wav_data[..]).unwrap();
    let mut samples = wav.into_samples::<f32>();
    let mut pcm_left = Vec::new();
    let mut pcm_right = Vec::new();

    while let (Some(left), Some(right)) = (samples.next(), samples.next()) {
        pcm_left.push((left.unwrap() * std::i16::MAX as f32) as i16);
        pcm_right.push((right.unwrap() * std::i16::MAX as f32) as i16);
    }

    let mut encoder = Builder::new().expect("Create LAME builder");
    encoder.set_num_channels(2).unwrap();
    encoder.set_sample_rate(44100).unwrap();
    encoder.set_quality(Quality::Best).unwrap();
    let mut encoder = encoder.build().expect("To create LAME encoder");

    let input = DualPcm {
        left: &pcm_left,
        right: &pcm_right,
    };

    let mut mp3_buffer = Vec::new();
    mp3_buffer.resize(
        mp3lame_encoder::max_required_buffer_size(pcm_left.len()),
        0,
    );
    let mut mp3_buffer_uninit = unsafe {
        std::mem::transmute::<&mut [u8], &mut [std::mem::MaybeUninit<u8>]>(&mut mp3_buffer)
    };

    let encoded_size = encoder.encode(input, &mut mp3_buffer_uninit).unwrap();
    mp3_buffer.truncate(encoded_size);

    let mut final_mp3_buffer = Vec::new();
    final_mp3_buffer.resize(7200, 0);
    let mut final_mp3_buffer_uninit = unsafe {
        std::mem::transmute::<&mut [u8], &mut [std::mem::MaybeUninit<u8>]>(&mut final_mp3_buffer)
    };
    let encoded_size = encoder
        .flush::<FlushNoGap>(&mut final_mp3_buffer_uninit)
        .unwrap();
    final_mp3_buffer.truncate(encoded_size);
    mp3_buffer.extend_from_slice(&final_mp3_buffer);

    std::io::Write::write_all(&mut mp3_file, &mp3_buffer)?;

    Ok(())
}