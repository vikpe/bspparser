use crate::Entry;
use binrw::{BinRead, BinResult};
use std::io::{Read, Seek, SeekFrom};

pub trait FromReader {
    type OutputType;
    fn element_count(size: u32) -> u32;
    fn from_reader<R: Read + Seek>(reader: &mut R) -> BinResult<Self::OutputType>;
}

impl<T: BinRead + for<'a> BinRead<Args<'a> = ()>> FromReader for T {
    type OutputType = T;

    fn element_count(size: u32) -> u32 {
        size / (size_of::<T>() as u32)
    }

    fn from_reader<R: Read + Seek>(reader: &mut R) -> BinResult<Self::OutputType> {
        T::read_le(reader)
    }
}

pub fn read_vec<T: FromReader>(
    reader: &mut (impl Read + Seek),
    entry: &Entry,
) -> BinResult<Vec<T::OutputType>> {
    reader.seek(SeekFrom::Start(entry.offset as u64))?;
    let count = T::element_count(entry.size);
    let mut elements = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let element = T::from_reader(reader)?;
        elements.push(element);
    }
    Ok(elements)
}
