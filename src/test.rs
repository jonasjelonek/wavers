use std::fs::File;
use std::path::PathBuf;

use crate::read::WaveReader;
use rodio;

#[test]
fn read_wav_file() -> () {
    let path = PathBuf::from("examples/piano_16000.wav");
    let file = match File::open(&path) {
        Ok(x) => x,
        Err(e) => panic!("Could not open file '{}': {}", path.display(), e),
    };

    let wave_file = WaveReader::new(file).decode().unwrap();
    let samples = wave_file.samples::<f32>().unwrap();

    println!("Format: {}", wave_file.format.stringify());
    println!("Bits per sample: {}", wave_file.bits_per_sample);
    println!("Sample rate: {}", wave_file.sample_rate);
    let minutes = wave_file.duration().as_secs() / 60;
    let seconds = wave_file.duration().as_secs() % 60;
    println!("Duration: {:02}:{:02}", minutes, seconds);

    match wave_file.metadata.name {
        Some(x) => match wave_file.metadata.artist {
            Some(y) => println!("Playing '{}' by '{}'", x, y),
            None => println!("Playing '{}'", x),
        },
        None => {}
    };

    let sample_buffer = rodio::buffer::SamplesBuffer::new(wave_file.channels, wave_file.sample_rate, samples);
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink: rodio::Sink = rodio::Sink::try_new(&stream_handle).unwrap();

    sink.set_volume(0.8);

    println!("Playing now...");
    sink.append(sample_buffer);

    sink.sleep_until_end();
}