use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read};
use std::str;

macro_rules! read_string {
    ($r:ident, $len:expr) => {{
        let mut data = [0u8; $len];
        $r.read_exact(&mut data)?;
        data
    }};
}

pub(crate) use read_string;

pub trait CRead {
    fn read_char(&mut self) -> io::Result<i8>;
    fn read_uchar(&mut self) -> io::Result<u8>;
    fn read_short(&mut self) -> io::Result<i16>;
    fn read_ushort(&mut self) -> io::Result<u16>;
    fn read_long(&mut self) -> io::Result<i32>;
    fn read_ulong(&mut self) -> io::Result<u32>;
    fn read_float(&mut self) -> io::Result<f32>;
    fn read_vector3_float(&mut self) -> io::Result<(f32, f32, f32)>;
}

impl<T> CRead for T
where
    T: Read,
{
    fn read_char(&mut self) -> io::Result<i8> {
        self.read_i8()
    }
    fn read_uchar(&mut self) -> io::Result<u8> {
        self.read_u8()
    }
    fn read_short(&mut self) -> io::Result<i16> {
        self.read_i16::<LittleEndian>()
    }
    fn read_ushort(&mut self) -> io::Result<u16> {
        self.read_u16::<LittleEndian>()
    }
    fn read_long(&mut self) -> io::Result<i32> {
        self.read_i32::<LittleEndian>()
    }
    fn read_ulong(&mut self) -> io::Result<u32> {
        self.read_u32::<LittleEndian>()
    }
    fn read_float(&mut self) -> io::Result<f32> {
        self.read_f32::<LittleEndian>()
    }
    fn read_vector3_float(&mut self) -> io::Result<(f32, f32, f32)> {
        Ok((
            self.read_f32::<LittleEndian>()?,
            self.read_f32::<LittleEndian>()?,
            self.read_f32::<LittleEndian>()?,
        ))
    }
}

pub fn from_cstring(data: &[u8]) -> Result<String, str::Utf8Error> {
    let end = data.iter().position(|&v| v == 0).unwrap_or(data.len());
    let data = str::from_utf8(&data[..end])?;
    Ok(data.to_owned())
}
