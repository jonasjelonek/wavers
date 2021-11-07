# wavers
A small library for working with WAVE files written in Rust

This is my first project in Rust. Any criticism is appreciated!

## Features

`wavers` supports parsing WAVE files, Wave64 is currently not supported.   
Only uncompressed sample data is supported.

- [x] uncompressed PCM 
- [x] IEEE Float
- [ ] compressed audio data
- [ ] passthrough of compressed audio data
- [x] Bits per sample:
  - [x] 8-bit unsigned integer
  - [x] 16, 24, 32, 64-bit signed integer
  - [x] 32, 64-bit floating-point
- [x] Parsing LIST chunk for metadata
- [x] ID3 chunk recognition
- [ ] ID3 parsing
- [x] Dynamic sample parsing (see below for explanation)

**What does dynamic sample parsing mean?**

Sample data is hold as byte array internally. The developer that uses this library can more or less decide in which data type the samples should be delivered.
```rust
let samples = match wave_file.samples::<f32>() {
  Ok(data) => data,
  Err(e) => panic!("An error occured while retrieving sample data: {}", e)
};
```
Sample data can be retrieved in all data types that implement the internal `Sample` trait. By default, this trait is implemented for `u8`, `i16`, `i32`, `i64`, `f32` and `f64`. The dynamic parsing also prevents from using unsuitable data types, e.g. an error is returned when you try to read samples as `i16` from a WAVE file with `32 bps`.   
Additionally, sample data is mapped to the dynamic ranges of the data types. For example, when you read samples as f32 from a WAVE file with `PCM` and `16 bps`, the samples will be mapped from `-32768 - 32767` to `-1.0 - 1.0`. 

## Usage

The library will soon be released in `crates.io`. Then you can just add it as a dependency. In the meantime, source can be downloaded and added as a local dependency.

```rust
  let path = PathBuf::from("examples/piano_16000.wav");
  let file = match File::open(&path) {
      Ok(x) => x,
      Err(e) => panic!("Could not open file '{}': {}", path.display(), e),
  };

  let wave_file = WaveReader::new(file).decode().unwrap();
  let samples = wave_file.samples::<f32>().unwrap();
```
Several information and metadata can be retrieved from `wave_file` then. Sample data can be passed to a playback library (e.g. [rodio](https://github.com/RustAudio/rodio))

For a usable example see `test.rs`.

## Contribution

As this is my first bigger project in Rust, the code may be far from perfect. Please open issues or make pull requests for improvements, bugs, bug fixes etc.   
**Any criticism is highly appreciated!**
