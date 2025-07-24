mod config;
mod render;
mod writer;

use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;

use crate::render::XSynthRender;
use mp3lame_encoder::{Builder, DualPcm, FlushNoGap, Quality};
use xsynth_core::{
    channel::{ChannelAudioEvent, ChannelConfigEvent, ChannelEvent},
    channel_group::SynthEvent,
    soundfont::{SampleSoundfont, SoundfontBase},
    AudioStreamParams,
};
use xsynth_realtime::{RealtimeEventSender, RealtimeSynth};

//const SF_PATH: &str = "Yamaha_C3_Grand_Piano.sf2";
const SF_PATH: &str = "UprightPianoKW-small-bright-20190703.sf2";
const WAV_OUTPUT_PATH: &str = "output.wav";

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ToneRange {
    Bass,
    Baritone,
    Tenor,
    Alto,
    MezzoSoprano,
    Soprano,
}

fn get_tone_range(range: Option<ToneRange>) -> (u8, u8) {
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// If set, saves the output to a file as mp3 instead of playing it in realtime
    #[arg(short, long)]
    save: Option<String>,
    /// Duration of the note in seconds
    #[arg(short, long, default_value_t = 0.7)]
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
}

trait Player {
    fn play_note(&mut self, key: u8, duration: f64);
    fn play_chord(&mut self, keys: &[u8], duration: f64);
    fn load_soundfont(&mut self, params: AudioStreamParams);
    fn wait(&mut self, duration: f64);
    fn finalize(self: Box<Self>);
}

struct RealtimePlayer {
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

struct FilePlayer {
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
        // self.synth.finalize();
        println!("Converting to MP3...");
        convert_wav_to_mp3(WAV_OUTPUT_PATH, &self.save_path).unwrap();
        println!("Done!");
    }
}

fn main() {
    let args = Args::parse();

    if let Some(save_path) = args.save {
        let synth = XSynthRender::new(Default::default(), WAV_OUTPUT_PATH.into());
        let params = synth.get_params();
        let mut player = FilePlayer { synth, save_path };

        player.load_soundfont(params);

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

        println!("Playing triads from {} to {}", key_from, key_to);

        play_triads_from(&mut player, key_from, key_to, args.duration);

        player.synth.finalize();
        println!("Converting to MP3...");
        convert_wav_to_mp3(WAV_OUTPUT_PATH, &player.save_path).unwrap();
        println!("Done!");
    } else {
        let synth = RealtimeSynth::open_with_all_defaults();
        let params = synth.stream_params();
        let sender = synth.get_sender_ref().clone();
        let mut player = Box::new(RealtimePlayer {
            sender,
            _synth: synth,
        });

        player.load_soundfont(params);

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

        println!("Playing triads from {} to {}", key_from, key_to);

        play_triads_from(player.as_mut(), key_from, key_to, args.duration);

        player.finalize();
    };
}

fn note_string_to_key(note_string: &str) -> u8 {
    let note = note_string.trim_end_matches(char::is_numeric);
    let octave = note_string
        .chars()
        .last()
        .unwrap_or('0')
        .to_digit(10)
        .unwrap_or(0) as u8;
    note_to_key(note, octave)
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

fn play_triad(player: &mut dyn Player, key: u8, note_duration: f64) {
    let chord = get_major_chord(key);
    let triad = vec![chord[0], chord[1], chord[2], chord[1], chord[0]];
    for &key in &triad {
        player.play_note(key, note_duration);
    }
    player.wait(note_duration);
    player.play_chord(&chord, note_duration * 2.0);
}

fn play_triads_from(player: &mut dyn Player, key_from: u8, key_to: u8, note_duration: f64) {
    for i in key_from..=(key_to - 7) {
        play_triad(player, i, note_duration);
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
    mp3_buffer.resize(mp3lame_encoder::max_required_buffer_size(pcm_left.len()), 0);
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
