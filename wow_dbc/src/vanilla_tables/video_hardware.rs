use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoHardware {
    pub rows: Vec<VideoHardwareRow>,
}

impl DbcTable for VideoHardware {
    type Row = VideoHardwareRow;

    const FILENAME: &'static str = "VideoHardware.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 88 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 88,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 22 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 22,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (VideoHardware) uint32
            let id = VideoHardwareKey::new(crate::util::read_u32_le(chunk)?);

            // unknown: uint32[21]
            let unknown = crate::util::read_array_u32::<21>(chunk)?;


            rows.push(VideoHardwareRow {
                id,
                unknown,
            });
        }

        Ok(VideoHardware { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 22,
            record_size: 88,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (VideoHardware) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // unknown: uint32[21]
            for i in row.unknown {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for VideoHardware {
    type PrimaryKey = VideoHardwareKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct VideoHardwareKey {
    pub id: u32
}

impl VideoHardwareKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for VideoHardwareKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for VideoHardwareKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for VideoHardwareKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for VideoHardwareKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for VideoHardwareKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for VideoHardwareKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for VideoHardwareKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for VideoHardwareKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for VideoHardwareKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for VideoHardwareKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoHardwareRow {
    pub id: VideoHardwareKey,
    pub unknown: [u32; 21],
}

