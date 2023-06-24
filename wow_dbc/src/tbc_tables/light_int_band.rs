use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LightIntBand {
    pub rows: Vec<LightIntBandRow>,
}

impl DbcTable for LightIntBand {
    type Row = LightIntBandRow;

    const FILENAME: &'static str = "LightIntBand.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 136 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 136,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 34 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 34,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (LightIntBand) int32
            let id = LightIntBandKey::new(crate::util::read_i32_le(chunk)?);

            // num: int32
            let num = crate::util::read_i32_le(chunk)?;

            // time: int32[16]
            let time = crate::util::read_array_i32::<16>(chunk)?;

            // data: int32[16]
            let data = crate::util::read_array_i32::<16>(chunk)?;


            rows.push(LightIntBandRow {
                id,
                num,
                time,
                data,
            });
        }

        Ok(LightIntBand { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 34,
            record_size: 136,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (LightIntBand) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // num: int32
            b.write_all(&row.num.to_le_bytes())?;

            // time: int32[16]
            for i in row.time {
                b.write_all(&i.to_le_bytes())?;
            }


            // data: int32[16]
            for i in row.data {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for LightIntBand {
    type PrimaryKey = LightIntBandKey;
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
pub struct LightIntBandKey {
    pub id: i32
}

impl LightIntBandKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for LightIntBandKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for LightIntBandKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for LightIntBandKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl From<u8> for LightIntBandKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for LightIntBandKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl TryFrom<u32> for LightIntBandKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LightIntBandRow {
    pub id: LightIntBandKey,
    pub num: i32,
    pub time: [i32; 16],
    pub data: [i32; 16],
}

