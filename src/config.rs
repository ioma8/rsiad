use xsynth_core::{
    channel::ChannelInitOptions,
    channel_group::{ChannelGroupConfig, ParallelismOptions, SynthFormat, ThreadCount},
    soundfont::{EnvelopeCurveType, EnvelopeOptions, Interpolator, SoundfontInitOptions},
    AudioStreamParams, ChannelCount,
};

#[derive(Clone, Debug, PartialEq)]
pub struct XSynthRenderConfig {
    pub group_options: ChannelGroupConfig,

    pub sf_options: SoundfontInitOptions,

    pub use_limiter: bool,
}

impl Default for XSynthRenderConfig {
    fn default() -> Self {
        Self {
            group_options: ChannelGroupConfig {
                channel_init_options: ChannelInitOptions {
                    fade_out_killing: true,
                },
                format: SynthFormat::Midi,
                audio_params: AudioStreamParams::new(44100, ChannelCount::Stereo),
                parallelism: ParallelismOptions {
                    channel: ThreadCount::Auto,
                    key: ThreadCount::Auto,
                },
            },
            sf_options: SoundfontInitOptions {
                bank: None,
                preset: None,
                vol_envelope_options: EnvelopeOptions {
                    attack_curve: EnvelopeCurveType::Exponential,
                    decay_curve: EnvelopeCurveType::Linear,
                    release_curve: EnvelopeCurveType::Linear,
                },
                use_effects: true,
                interpolator: Interpolator::Linear,
            },
            use_limiter: false,
        }
    }
}
