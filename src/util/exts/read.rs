use std::io::{Read, Cursor, Error};

use crate::util::exts::{Endian, Endian::*};

/**
 * The `ReadExt` trait extends implementing structs etc. by methods to directly read
 * signed or unsigned numerics with a given byteorder.
 */
pub trait SizedDataRead: Read + Sized {

    fn read_u8(&mut self) -> Result<u8, Error>;
    fn read_u16(&mut self, endian: Endian) -> Result<u16, Error>;
    fn read_u24(&mut self, endian: Endian) -> Result<u32, Error>;
    fn read_u32(&mut self, endian: Endian) -> Result<u32, Error>;
    fn read_u64(&mut self, endian: Endian) -> Result<u64, Error>;
    fn read_u128(&mut self, endian: Endian) -> Result<u128, Error>;

    fn read_i16(&mut self, endian: Endian) -> Result<i16, Error>;
    fn read_i24(&mut self, endian: Endian) -> Result<i32, Error>;
    fn read_i32(&mut self, endian: Endian) -> Result<i32, Error>;
    fn read_i64(&mut self, endian: Endian) -> Result<i64, Error>;

    fn read_f32(&mut self, endian: Endian) -> Result<f32, Error>;
    fn read_f64(&mut self, endian: Endian) -> Result<f64, Error>;

    fn read_string(&mut self, len: u32) -> Result<String, Error>;
}

impl<R: Read> SizedDataRead for R {

    fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buf: [u8; 1] = [ 0 ];
        match self.read_exact(&mut buf) {
            Ok(()) => Ok( buf[0] ),
            Err(e) => return Err(e),
        }
    }

    fn read_u16(&mut self, endian: Endian) -> Result<u16, Error> {
        let mut buf: [u8; 2] = [0; 2];
        match self.read_exact(&mut buf) {
            Ok(()) => match endian {
                Little => Ok(u16::from_le_bytes(buf)),
                Big => Ok(u16::from_be_bytes(buf)),
            },
            Err(e) => return Err(e),
        }
    }

    fn read_u24(&mut self, endian: Endian) -> Result<u32, Error> {
        let mut buf: [u8; 3] = [0; 3];
        match self.read_exact(&mut buf) {
            Ok(()) => match endian {
                Little => Ok(u32::from_le_bytes( [ buf[0], buf[1], buf[2], 0 ])),
                Big => Ok(u32::from_be_bytes( [ 0, buf[0], buf[1], buf[2] ])),
            },
            Err(e) => return Err(e),
        }
    }

    fn read_u32(&mut self, endian: Endian) -> Result<u32, Error> {
        let mut buf: [u8; 4] = [0; 4];
        match self.read_exact(&mut buf) {
            Ok(()) => match endian {
                Little => Ok(u32::from_le_bytes(buf)),
                Big => Ok(u32::from_be_bytes(buf)),
            },
            Err(e) => return Err(e),
        }
    }

    fn read_u64(&mut self, endian: Endian) -> Result<u64, Error> {
        let mut buf: [u8; 8] = [0; 8];
        match self.read_exact(&mut buf) {
            Ok(()) => match endian {
                Little => Ok(u64::from_le_bytes(buf)),
                Big => Ok(u64::from_be_bytes(buf)),
            },
            Err(e) => return Err(e),
        }
    }

    fn read_u128(&mut self, endian: Endian) -> Result<u128, Error> {
        let mut buf: [u8; 16] = [0; 16];
        match self.read_exact(&mut buf) {
            Ok(()) => match endian {
                Little => Ok(u128::from_le_bytes(buf)),
                Big => Ok(u128::from_be_bytes(buf)),
            },
            Err(e) => return Err(e),
        }
    }

    fn read_i16(&mut self, endian: Endian) -> Result<i16, Error> {
        let mut buf: [u8; 2] = [0; 2];
        match self.read_exact(&mut buf) {
            Ok(()) => match endian {
                Little => Ok(i16::from_le_bytes(buf)),
                Big => Ok(i16::from_be_bytes(buf)),
            },
            Err(e) => return Err(e),
        }
    }

    fn read_i24(&mut self, endian: Endian) -> Result<i32, Error> {
        let mut buf: [u8; 3] = [0; 3];
        match self.read_exact(&mut buf) {
            Ok(()) => match endian {
                Little => {
                    if (buf[2] & 0x80) >> 7 == 1 {  // get sign bit
                        Ok(i32::from_le_bytes([ buf[0], buf[1], buf[2], 0xff ]))
                    } else {
                        Ok(i32::from_le_bytes( [ buf[0], buf[1], buf[2], 0 ]))
                    }
                },
                Big => {
                    if (buf[0] & 0x80) >> 7 == 1 {  // get sign bit
                        Ok( !(i32::from_be_bytes([ 0xff, buf[0], buf[1], buf[2] ])) )
                    } else {
                        Ok(i32::from_be_bytes( [ 0, buf[0], buf[1], buf[2] ]))
                    }
                },
            },
            Err(e) => return Err(e),
        }
    }

    fn read_i32(&mut self, endian: Endian) -> Result<i32, Error> {
        let mut buf: [u8; 4] = [0; 4];
        match self.read_exact(&mut buf) {
            Ok(()) => match endian {
                Little => Ok(i32::from_le_bytes(buf)),
                Big => Ok(i32::from_be_bytes(buf)),
            },
            Err(e) => return Err(e),
        }
    }

    fn read_i64(&mut self, endian: Endian) -> Result<i64, Error> {
        let mut buf: [u8; 8] = [0; 8];
        match self.read_exact(&mut buf) {
            Ok(()) => match endian {
                Little => Ok(i64::from_le_bytes(buf)),
                Big => Ok(i64::from_be_bytes(buf)),
            },
            Err(e) => return Err(e),
        }
    }

    fn read_f32(&mut self, endian: Endian) -> Result<f32, Error> {
        let mut buf: [u8; 4] = [0; 4];
        match self.read_exact(&mut buf) {
            Ok(()) => match endian {
                Little => Ok(f32::from_le_bytes(buf)),
                Big => Ok(f32::from_be_bytes(buf)),
            },
            Err(e) => return Err(e),
        }
    }

    fn read_f64(&mut self, endian: Endian) -> Result<f64, Error> {
        let mut buf: [u8; 8] = [0; 8];
        match self.read_exact(&mut buf) {
            Ok(()) => match endian {
                Little => Ok(f64::from_le_bytes(buf)),
                Big => Ok(f64::from_be_bytes(buf)),
            },
            Err(e) => return Err(e),
        }
    }

    fn read_string(&mut self, len: u32) -> Result<String, Error> {
        let mut str: String = String::new();
        match self.take(len as u64).read_to_string(&mut str) {
            Ok(_) => Ok(str),
            Err(e) => Err(e),
        }
    }
}

pub trait CursorExt: Read + Sized {

    fn skip_bytes(&mut self, len: u32) -> Result<(), Error>;
}

impl<R> CursorExt for Cursor<R> 
where R: AsRef<[u8]> {

    fn skip_bytes(&mut self, len: u32) -> Result<(), Error> {
        //let stream_len = self.stream_len()?;
        //if stream_len > self.position() + (len as u64) {
        //    self.set_position(stream_len - 1);
        //} else {
            self.set_position(self.position() + (len as u64));
        //}
        Ok(())
    }
}