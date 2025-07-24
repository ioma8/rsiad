# 🎶 RSIAD - Rust Triad 🎶

A simple command-line tool for singers to warm up their voices, especially for opera. 🎤 It plays musical triads as a pitch reference for vocal exercises.

## 🚀 Getting Started

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install)

### Run

To start your warm-up session, run:

```bash
cargo run
```

## 📥 Downloads

Pre-generated warmup exercises for all vocal ranges can be downloaded as MP3 files from the [GitHub Releases](https://github.com/ioma8/rsiad/releases) section.

## ⌨️ CLI Usage

```
Usage: rsiad [OPTIONS]

Options:
  -s, --save <SAVE>          If set, saves the output to a file as mp3 instead of playing it in realtime
  -d, --duration <DURATION>  Duration of the note in seconds [default: 0.7]
  -f, --from <FROM>          Starting key of the range
  -t, --to <TO>              Ending key of the range
  -r, --range <RANGE>        Tone range of the singer [possible values: bass, baritone, tenor, alto, mezzo-soprano, soprano]
  -h, --help                 Print help
  -V, --version              Print version
```

Enjoy your practice! 🎵