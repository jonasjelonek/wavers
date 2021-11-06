use crate::{WaveFile, WaveFormat};
use crate::util::exts::{Endian, read::*};
use crate::WaveDecodeError;

use std::io::{Cursor, ErrorKind};
use std::convert::From;

const RIFF_MAGIC: u32 = 0x46464952;
const WAVE_MAGIC: u32 = 0x45564157;
const CHUNK_FMT_: u32 = 0x20746d66;
const CHUNK_FACT: u32 = 0x74636166;
const CHUNK_DATA: u32 = 0x61746164;
const CHUNK_ID3_: u32 = 0x20336469;                 /* THIS IS NOT INCLUDED IN WAVE SPECIFICATION!! */
const CHUNK_ID3_ALT: u32 = 0x20334449;
const CHUNK_LIST: u32 = 0x5453494c;                 /* List chunk id */
const CHUNK_LIST_SUB_INFO: u32 = 0x4f464e49;        /* List chunk type INFO */

const LIST_INFO_IART: u32 = 0x54524149;	            /* The artist of the original subject of the file */
const LIST_INFO_ICMT: u32 = 0x544d4349;	            /* General comments about the file or its subject */
const LIST_INFO_ICOP: u32 = 0x504f4349;	            /* Copyright information about the file (e.g., "Copyright Some Company 2011") */
const LIST_INFO_ICRD: u32 = 0x44524349;             /* The date the subject of the file was created (creation date) */
const LIST_INFO_IGNR: u32 = 0x524e4749;	            /* The genre of the subject */
const LIST_INFO_IKEY: u32 = 0x59454b49;	            /* A list of keywords for the file or its subject */
const LIST_INFO_INAM: u32 = 0x4d414e49;	            /* Title of the subject of the file (name) */
const LIST_INFO_IPRD: u32 = 0x44525049;	            /* Name of the title the subject was originally intended for */
const LIST_INFO_ISBJ: u32 = 0x4a425349;	            /* Description of the contents of the file (subject) */
const LIST_INFO_ISFT: u32 = 0x54465349;	            /* Name of the software package used to create the file */

pub struct WaveReader<R> 
where R: SizedDataRead {
    source: R,
}

impl<R> WaveReader<R> 
where R: SizedDataRead {

    pub fn new(source: R) -> WaveReader<R> {
        WaveReader::<R> { source: source }
    }

    pub fn decode(&mut self) -> Result<WaveFile, WaveDecodeError> {
        let mut buf: Vec<u8> = Vec::new();
        let bytes_read = match self.source.read_to_end(&mut buf) {
            Ok(len) => len,
            Err(e) => panic!("Could not read input contents: {:?}", e),
        };

        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buf);

        /* Read leading file information to verify it's actually a riff wave file. */
        let file_header = cursor.read_u32(Endian::Little)?;
        let file_size = cursor.read_u32(Endian::Little)? + 8;    /* Adding 8 because the information does not contain file_header and file_size. */
        let file_format = cursor.read_u32(Endian::Little)?;
        if file_header != RIFF_MAGIC || file_format != WAVE_MAGIC {
            panic!("Source has invalid RIFF WAVE header.");
        }

        if (file_size as usize) != bytes_read {
            /* Buffer does not contain same amount of data that is specified in header */
            panic!("Bytestream size ({} B) is not equal to specified file size in riff header ({} B)", file_size, bytes_read);
        }

        let mut wave_file = WaveFile::new();
        wave_file.file_size = file_size;
        self.read_chunks(&mut wave_file, &mut cursor)?;

        Ok(wave_file)
    }

    fn read_chunks(&mut self, wave_file: &mut WaveFile, cursor: &mut Cursor<Vec<u8>>) -> Result<(), WaveDecodeError> {
        let mut has_fmt: bool = false;
        let mut has_fact: bool = false;

        loop {
            if cursor.position() == (wave_file.file_size as u64) {
                break;
            }
            
            match cursor.read_u32(Endian::Little) {
                Ok(CHUNK_FMT_) => {
                    self.read_fmt_chunk(wave_file, cursor)?;
                    has_fmt = true;
                },
                Ok(CHUNK_FACT) => {
                    self.read_fact_chunk(wave_file, cursor)?;
                    has_fact = true;
                },
                Ok(CHUNK_DATA) => self.read_data_chunk(wave_file, cursor)?,
                Ok(CHUNK_LIST) => self.read_list_chunk(wave_file, cursor)? ,
                Ok(CHUNK_ID3_) | Ok(CHUNK_ID3_ALT) => self.read_id3_chunk(wave_file, cursor)? ,
                Ok(x) => { 
                    println!("Skipping unexpected chunk {:x} at {}", x, cursor.position() - 4);
                    let chunk_size = cursor.read_u32(Endian::Little)?;
                    cursor.skip_bytes(chunk_size)?;
                },
                Err(e) => {
                    match e.kind() {
                        ErrorKind::UnexpectedEof => { println!("Unexpected EOF at {} with file size {}", cursor.position(), wave_file.file_size); break; },  // Reached end of file, just leave the loop.
                        _ => panic!("Read error: {}", e), /* Some other error occured. */
                    };
                },
            };
        }

        if !has_fmt {
            panic!("Wave container does not have mandatory fmt chunk.");
        }
        if wave_file.format != WaveFormat::Pcm && !has_fact {
            /* If the format is not PCM the file needs to have a fact chunk (see specification Rev. 3). */
            panic!("Mandatory FACT chunk could not be found in this file.");
        }

        Ok(())
    }

    /**
     * 
     */
    fn read_fmt_chunk(&self, wave_file: &mut WaveFile, cursor: &mut Cursor<Vec<u8>>) -> Result<(), WaveDecodeError> {
        let sect_length = cursor.read_u32(Endian::Little)?;
        match sect_length {
            16 | 18 | 40 => true,
            _ => return Err(WaveDecodeError { message: format!("Unexpected fmt section length: {}", sect_length) }),
        };

        wave_file.format = match WaveFormat::parse( cursor.read_u16(Endian::Little)? ) {
            Some(x) => x,
            None => return Err(WaveDecodeError { message: String::from("Unknown wave type.") }),
        };
        wave_file.channels = cursor.read_u16(Endian::Little)?;
        wave_file.sample_rate = cursor.read_u32(Endian::Little)?;
        wave_file.data_rate = cursor.read_u32(Endian::Little)?;
        wave_file.frame_size = cursor.read_u16(Endian::Little)?;
        wave_file.bits_per_sample = cursor.read_u16(Endian::Little)?;
        
        if sect_length == 18 || sect_length == 40 {
            let extension_size = cursor.read_u16(Endian::Little)?;
            match extension_size {
                22 => {
                    wave_file.valid_bps = Some( cursor.read_u16(Endian::Little)? );
                    wave_file.channel_mask = Some ( cursor.read_u32(Endian::Little)? );
                    wave_file.sub_format = Some ( cursor.read_u128(Endian::Little)? );
                },
                0 => (),
                _ => { // Extension size must be present when dealing with non-PCM format
                    if wave_file.format != WaveFormat::Pcm {
                        return Err(WaveDecodeError { message: String::from("Invalid or no extension size field.") });
                    }
                }
            };
        }

        Ok(())
    }

    /** 
     * 
     */
    fn read_fact_chunk(&self, wave_file: &mut WaveFile, cursor: &mut Cursor<Vec<u8>>) -> Result<(), WaveDecodeError> {
        // Fact chunk length, can be abandoned here cause fact chunk usually has only one field.
        cursor.read_u32(Endian::Little)?; 

        // Wave files that have a format different from PCM need to have a fact chunk,
        // but the included information is rather redundant.
        wave_file.num_of_samples = Some( cursor.read_u32(Endian::Little)? );

        Ok(())
    }

    /**
     * 
     */
    fn read_list_chunk(&self, wave_file: &mut WaveFile, cursor: &mut Cursor<Vec<u8>>) -> Result<(), WaveDecodeError> {
        let chunk_size = cursor.read_u32(Endian::Little)?;
        if cursor.read_u32(Endian::Little)? != CHUNK_LIST_SUB_INFO {
            return Err(WaveDecodeError { message: String::from("Unsupported subchunk in LIST chunk") });
        }

        let mut count: u32 = 4;     // After reading chunk_size another 4 bytes have been read.
        while count < chunk_size  {
            let info_size;
            match cursor.read_u32(Endian::Little) {
                Ok(x) => {
                    info_size = cursor.read_u32(Endian::Little)?;
                    match x {
                        LIST_INFO_IART => wave_file.metadata.artist = Some( cursor.read_string(info_size)? ),
                        LIST_INFO_ICOP => wave_file.metadata.copyright = Some( cursor.read_string(info_size)? ),
                        LIST_INFO_ICRD => wave_file.metadata.date = Some( cursor.read_string(info_size)? ),
                        LIST_INFO_IGNR => wave_file.metadata.genre = Some( cursor.read_string(info_size)? ),
                        LIST_INFO_IKEY => wave_file.metadata.keywords = Some( cursor.read_string(info_size)? ),
                        LIST_INFO_INAM => wave_file.metadata.name = Some( cursor.read_string(info_size)? ),
                        LIST_INFO_IPRD => wave_file.metadata.title = Some( cursor.read_string(info_size)? ),
                        LIST_INFO_ISFT => wave_file.metadata.encoder = Some( cursor.read_string(info_size)? ),
                        LIST_INFO_ICMT => wave_file.metadata.comments = Some( cursor.read_string(info_size)? ),
                        LIST_INFO_ISBJ => wave_file.metadata.description = Some( cursor.read_string(info_size)? ),
                        // All other info types are currently not supported, therefore skip.
                        _ => cursor.skip_bytes(info_size)?,
                    };
                },
                Err(e) => {
                    match e.kind() {
                        ErrorKind::UnexpectedEof => break,
                        _ => return Err(WaveDecodeError { message: format!("Read error: {}", e) }),
                    }
                },
            }

            // Text information needs to be word-aligned (2-byte aligned)
            let dist = cursor.position() % 2;
            if dist != 0 {
                cursor.skip_bytes(dist as u32)?;
            }
            // Increase counter by info id size, size of 'size' field, the actual size of the information and potential padding.
            count += 8 + info_size + (dist as u32);
        }

        Ok(())
    }

    // As ID3 is used in multiple audio formats the actual parsing should be placed in a separate module for reuse in other format decoders
    fn read_id3_chunk(&self, _wave_file: &mut WaveFile, cursor: &mut Cursor<Vec<u8>>) -> Result<(), WaveDecodeError> {
        // Do something with chunk info
        // TODO: pass over to separate ID3 parser
        let size = cursor.read_u32(Endian::Little)?;
        cursor.skip_bytes(size)?;

        Ok(())
    }

    /** 
     * 
     */
    fn read_data_chunk(&self, wave_file: &mut WaveFile, cursor: &mut Cursor<Vec<u8>>) -> Result<(), WaveDecodeError> {
        let size = cursor.read_u32(Endian::Little)?;

        let curr_pos = cursor.position() as usize;
        println!("Data section at 0x{:x}", curr_pos);
        wave_file.sample_data = (cursor.get_ref()[curr_pos..(curr_pos + (size as usize))]).to_vec();
        cursor.skip_bytes(size)?;

        // Size of data chunk can be odd, then a pad byte is at the current position of the cursor. Need to skip this.
        let padding = (cursor.position() % 2) as u32;
        if padding != 0 {
            cursor.skip_bytes(padding)?;
        }

        Ok(())
    }
}