use hound;
use hound::SampleFormat;

#[derive(Debug)]
pub(crate) struct AudioConfig {
    channels: u16,
    sample_rate: u32,
    bits_per_sample: u16,
    sample_format: SampleFormat,
}

pub(crate) fn read_wav(file_name: &str) -> AudioConfig {
    let reader = hound::WavReader::open(file_name).expect("Failed to read wav");
    let spec = reader.spec();

    AudioConfig {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        bits_per_sample: spec.bits_per_sample,
        sample_format: spec.sample_format,
    }
}