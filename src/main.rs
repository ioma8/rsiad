use std::{sync::Arc, time::Duration};

use xsynth_core::{
    channel::{ChannelAudioEvent, ChannelConfigEvent, ChannelEvent},
    soundfont::{SampleSoundfont, SoundfontBase},
};
use xsynth_realtime::{RealtimeEventSender, RealtimeSynth, SynthEvent};

const SF_PATH: &str = "Yamaha_C3_Grand_Piano.sf2";

fn main() {
    let synth = RealtimeSynth::open_with_all_defaults();

    let mut sender = synth.get_sender_ref().clone();

    let params = synth.stream_params();

    println!("Loading Soundfont");
    let soundfonts: Vec<Arc<dyn SoundfontBase>> = vec![Arc::new(
        SampleSoundfont::new(SF_PATH, params, Default::default()).unwrap(),
    )];
    println!("Loaded");

    sender.send_event(SynthEvent::AllChannels(ChannelEvent::Config(
        ChannelConfigEvent::SetSoundfonts(soundfonts),
    )));

    play_triads_from(&mut sender, note_to_key("G", 2), note_to_key("G", 3));
}

fn play_note(sender: &mut RealtimeEventSender, key: u8, duration: f64) {
    sender.send_event(SynthEvent::Channel(
        0,
        ChannelEvent::Audio(ChannelAudioEvent::NoteOn { key, vel: 127 }),
    ));
    spin_sleep::sleep(Duration::from_secs_f64(duration));
    sender.send_event(SynthEvent::Channel(
        0,
        ChannelEvent::Audio(ChannelAudioEvent::NoteOff { key }),
    ));
}

fn play_chord(sender: &mut RealtimeEventSender, keys: &[u8], duration: f64) {
    for &key in keys {
        sender.send_event(SynthEvent::Channel(
            0,
            ChannelEvent::Audio(ChannelAudioEvent::NoteOn { key, vel: 127 }),
        ));
    }
    spin_sleep::sleep(Duration::from_secs_f64(duration));
    for &key in keys {
        sender.send_event(SynthEvent::Channel(
            0,
            ChannelEvent::Audio(ChannelAudioEvent::NoteOff { key }),
        ));
    }
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

fn play_triad(sender: &mut RealtimeEventSender, key: u8) {
    let chord = get_major_chord(key);
    let triad = vec![chord[0], chord[1], chord[2], chord[1], chord[0]];
    for &key in &triad {
        play_note(sender, key, 0.7);
    }
    spin_sleep::sleep(Duration::from_secs_f64(0.7));
    play_chord(sender, &chord, 1.4);
}

fn play_triads_from(sender: &mut RealtimeEventSender, key_from: u8, key_to: u8) {
    for i in key_from..=key_to {
        play_triad(sender, i);
    }
}
