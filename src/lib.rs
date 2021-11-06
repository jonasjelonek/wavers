pub mod util;
pub mod read;
pub mod test;

use std::string::String;
use std::io::{Error as IoError, ErrorKind, Cursor};
use std::time::Duration;

use std::convert::From;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::error::Error;

use util::exts::{Endian, read::*};
use util::math;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum WaveFormat {
    Pcm = 0x0001, 
    MsADPCM = 0x0002, 
    IeeeFloat = 0x0003, 
    Alaw = 0x0006, 
    Mulaw = 0x0007, 
    Aptx = 0x0025, 
    DolbyAC2 = 0x0030, 
    Mpeg1L1L2 = 0x0050,
    Mpeg1L3 = 0x0055, 
    XboxADPCM = 0x0069, 
    
    WaveExt = 0xFFFE,
    None = 0xFFFF,
}

impl WaveFormat {

    pub fn stringify(&self) -> String {
        (match *self {
            WaveFormat::Pcm => "PCM", WaveFormat::MsADPCM => "MS ADPCM", WaveFormat::IeeeFloat => "IEEE FLOAT",
            WaveFormat::Alaw => "ALAW", WaveFormat::Mulaw => "MULAW",
            WaveFormat::Aptx => "APTX",
            WaveFormat::DolbyAC2 => "DOLBY AC2", WaveFormat::Mpeg1L1L2 => "MPEG-1 Layer I, II",
            WaveFormat::Mpeg1L3 => "MPEG-1 Layer III (MP3)", WaveFormat::XboxADPCM => "Xbox ADPCM",
            WaveFormat::WaveExt => "WAVE EXTENSIBLE",
            WaveFormat::None => "NONE",
        }).to_string()
    }

    fn parse(val: u16) -> Option<WaveFormat> {
        match val {
            0x1 => Some(WaveFormat::Pcm), 0x2 => Some(WaveFormat::MsADPCM), 0x3 => Some(WaveFormat::IeeeFloat),
            0x6 => Some(WaveFormat::Alaw), 0x7 => Some(WaveFormat::Mulaw), 0x25 => Some(WaveFormat::Aptx),
            0x30 => Some(WaveFormat::DolbyAC2), 0x50 => Some(WaveFormat::Mpeg1L1L2), 0x55 => Some(WaveFormat::Mpeg1L3),
            0x69 => Some(WaveFormat::XboxADPCM), 0xfffe => Some(WaveFormat::WaveExt),
            _ => None
        }
    }
}


pub trait Sample: Sized + std::fmt::Debug { 

    fn read(cursor: &mut Cursor<Vec<u8>>, format: WaveFormat, bits: u16) -> Result<Self, IoError>;
}

impl Sample for u8 {

    fn read(cursor: &mut Cursor<Vec<u8>>, format: WaveFormat, bits: u16) -> Result<Self, IoError> {
        if bits > 8 {
            return Err(IoError::new(ErrorKind::Unsupported, "{} bits too large for u8"));
        }

        match format {
            WaveFormat::Pcm => Ok( cursor.read_u8()? ),
            WaveFormat::IeeeFloat => return Err(IoError::new(ErrorKind::Unsupported, "u8 is not capable of storing floating-point samples")),
            _ => return Err(IoError::new(ErrorKind::Unsupported, "Unsupported format type")),
        }
    }
}
impl Sample for i16 {

    fn read(cursor: &mut Cursor<Vec<u8>>, format: WaveFormat, bits: u16) -> Result<Self, IoError> {
        if bits > 16 {
            return Err(IoError::new(ErrorKind::Unsupported, "{} bits too large for i16"));
        }

        match format {
            WaveFormat::Pcm => match bits {
                8 => Ok( math::map_u8_to_i16(cursor.read_u8()?)  ),
                16 => Ok( cursor.read_i16(Endian::Little)? ),
                _ => Err( IoError::new(ErrorKind::Unsupported, ""))
            },
            WaveFormat::IeeeFloat => return Err(IoError::new(ErrorKind::Unsupported, "i16 is not capable of storing floating-point samples")),
            _ => return Err(IoError::new(ErrorKind::Unsupported, "Unsupported format type")),
        }
    }
}
impl Sample for i32 {

    fn read(cursor: &mut Cursor<Vec<u8>>, format: WaveFormat, bits: u16) -> Result<Self, IoError> {
        if bits > 32 {
            return Err(IoError::new(ErrorKind::Unsupported, "{} bits too large for i16"));
        }

        match format {
            WaveFormat::Pcm => match bits {
                8 => Ok( math::map_u8_to_i32(cursor.read_u8()?)  ),
                16 => Ok( math::map_i16_to_i32(cursor.read_i16(Endian::Little)?) ),
                24 => Ok( cursor.read_i24(Endian::Little)? ),
                32 => Ok( cursor.read_i32(Endian::Little)? ),
                _ => Err( IoError::new(ErrorKind::Unsupported, ""))
            },
            WaveFormat::IeeeFloat => return Err(IoError::new(ErrorKind::Unsupported, "i32 is not capable of storing floating-point samples")),
            _ => return Err(IoError::new(ErrorKind::Unsupported, "Unsupported format type")),
        }
    }
}
impl Sample for i64 {

    fn read(cursor: &mut Cursor<Vec<u8>>, format: WaveFormat, bits: u16) -> Result<Self, IoError> {
        if bits > 32 {
            return Err(IoError::new(ErrorKind::Unsupported, "{} bits too large for i16"));
        }

        match format {
            WaveFormat::Pcm => match bits {
                8 => Ok( math::map_u8_to_i64(cursor.read_u8()?)  ),
                16 => Ok( math::map_i16_to_i64(cursor.read_i16(Endian::Little)?) ),
                24 => Ok( math::map_i24_to_i64(cursor.read_i24(Endian::Little)?) ),
                32 => Ok( math::map_i32_to_i64(cursor.read_i32(Endian::Little)?) ),
                64 => Ok( cursor.read_i64(Endian::Little)? ),
                _ => Err( IoError::new(ErrorKind::Unsupported, ""))
            },
            WaveFormat::IeeeFloat => return Err(IoError::new(ErrorKind::Unsupported, "i32 is not capable of storing floating-point samples")),
            _ => return Err(IoError::new(ErrorKind::Unsupported, "Unsupported format type")),
        }
    }
}
impl Sample for f32 {

    fn read(cursor: &mut Cursor<Vec<u8>>, format: WaveFormat, bits: u16) -> Result<Self, IoError> {
        if bits > 32 {
            return Err(IoError::new(ErrorKind::Unsupported, "{} bits too large for i16"));
        }

        match format {
            WaveFormat::Pcm => match bits {
                8 => Ok( math::map_u8_to_f32(cursor.read_u8()?)  ),
                16 => Ok( math::map_i16_to_f32(cursor.read_i16(Endian::Little)?) ),
                24 => { 
                    let val = cursor.read_i24(Endian::Little)?;
                    //println!("Reading {} as f32: {}", val, math::map_i24_to_f32(val)); 
                    Ok( math::map_i24_to_f32(val) ) 
                },
                32 => Ok( math::map_i32_to_f32(cursor.read_i32(Endian::Little)?) ),
                /* WARNING: f32 is not capable of representing i64 */
                64 => Ok( math::map_i64_to_f32(cursor.read_i64(Endian::Little)?) ),
                _ => Err( IoError::new(ErrorKind::Unsupported, ""))
            },
            WaveFormat::IeeeFloat => {
                match bits {
                    32 => Ok( cursor.read_f32(Endian::Little)? ),
                    64 => Ok( cursor.read_f64(Endian::Little)? as f32 ),
                    _ => Err(IoError::new(ErrorKind::Unsupported, "Only standard 32-bit and 64-bit floating-point samples are supported."))
                }
            }
            _ => return Err(IoError::new(ErrorKind::Unsupported, "Unsupported format type")),
        }
    }
}
impl Sample for f64 {

    fn read(cursor: &mut Cursor<Vec<u8>>, format: WaveFormat, bits: u16) -> Result<Self, IoError> {
        if bits > 32 {
            return Err(IoError::new(ErrorKind::Unsupported, "{} bits too large for i16"));
        }

        match format {
            WaveFormat::Pcm => match bits {
                8 => Ok( math::map_u8_to_f64(cursor.read_u8()?)  ),
                16 => Ok( math::map_i16_to_f64(cursor.read_i16(Endian::Little)?) ),
                24 => Ok( math::map_i24_to_f64(cursor.read_i24(Endian::Little)?) ),
                32 => Ok( math::map_i32_to_f64(cursor.read_i32(Endian::Little)?) ),
                /* WARNING: f64 is not capable of representing i64 */
                64 => Ok( math::map_i64_to_f64(cursor.read_i64(Endian::Little)?) ),
                _ => Err( IoError::new(ErrorKind::Unsupported, ""))
            },
            WaveFormat::IeeeFloat => {
                match bits {
                    32 => Ok( cursor.read_f32(Endian::Little)? as f64 ),
                    64 => Ok( cursor.read_f64(Endian::Little)? ),
                    _ => Err(IoError::new(ErrorKind::Unsupported, "Only standard 32-bit and 64-bit floating-point samples are supported."))
                }
            }
            _ => return Err(IoError::new(ErrorKind::Unsupported, "Unsupported format type")),
        }
    }
}

#[derive(Debug)]
pub struct WaveFile {

    pub format: WaveFormat,
    pub channels: u16,
    pub sample_rate: u32,
    pub data_rate: u32,
    pub bits_per_sample: u16,
    pub num_of_samples: Option<u32>,
    pub valid_bps: Option<u16>,
    pub channel_mask: Option<u32>,
    pub sub_format: Option<u128>,

    file_size: u32,
    frame_size: u16,

    sample_data: Vec<u8>,

    pub metadata: AudioMetadata,
}

impl WaveFile {

    pub fn new() -> WaveFile {
        WaveFile {
            format: WaveFormat::None,
            channels: 0,
            sample_rate: 0,
            data_rate: 0,                  // sample_rate * frame_size
            bits_per_sample: 0,
            num_of_samples: None,
            valid_bps: None,
            channel_mask: None,
            sub_format: None,

            file_size: 0,
            frame_size: 0,

            sample_data: vec![],
            
            metadata: AudioMetadata::new(),
        }
    }

    #[inline(always)]
    pub fn frame_size(&self) -> u16 {
        self.frame_size
    }

    #[inline(always)]
    pub fn num_of_samples(&self) -> u32 {
        match self.num_of_samples {
            Some(x) => x,
            None => ((self.sample_data.len() as u32) / self.frame_size as u32) * (self.channels as u32)
        }
    }

    #[inline(always)]
    pub fn duration(&self) -> Duration {
        Duration::new((( (self.sample_data.len() as u32) / self.frame_size as u32) / self.sample_rate) as u64, 0)
    }

    pub fn samples<S: Sample>(&self) -> Result<Vec<S>, IoError> {
        let sample_data = self.sample_data.clone();
        let sample_data_len = sample_data.len();
        let mut cursor = Cursor::new(sample_data);

        let mut samples: Vec<S> = Vec::new();
        loop {
            if cursor.position() == sample_data_len as u64 {
                break;
            }

            let sample: S = Sample::read(&mut cursor, self.format, self.bits_per_sample)?;
            samples.push(sample);
        }

        Ok(samples)
    }
}

#[derive(Clone, Debug)]
pub struct AudioMetadata {
    pub album: Option<String>,
    pub artist: Option<String>,
    pub copyright: Option<String>,
    pub date: Option<String>,
    pub genre: Option<String>,
    pub keywords: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    pub comments: Option<String>,
    pub description: Option<String>,

    pub encoder: Option<String>,
}

impl AudioMetadata {

    pub fn new() -> AudioMetadata {
        AudioMetadata {
            album: None,
            artist: None,
            copyright: None,
            date: None,
            genre: None,
            keywords: None,
            name: None,
            title: None,
            comments: None,
            description: None,

            encoder: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WaveDecodeError {
    pub message: String
}

impl Display for WaveDecodeError {

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Wave decode error ({})", self.message)
    }
}
impl From<IoError> for WaveDecodeError { 

    fn from(v: IoError) -> WaveDecodeError {
        WaveDecodeError { message: format!("Internal IO error: {}", v) }
    }    
}
impl Error for WaveDecodeError {}