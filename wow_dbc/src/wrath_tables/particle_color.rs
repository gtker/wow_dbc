use crate::header::{HEADER_SIZE, DbcHeader, parse_header};
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParticleColor {
    pub rows: Vec<ParticleColorRow>,
}

impl DbcTable for ParticleColor {
    type Row = ParticleColorRow;

    fn filename() -> &'static str { "ParticleColor.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 40 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 40,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 10 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 10,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (ParticleColor) int32
            let id = ParticleColorKey::new(crate::util::read_i32_le(chunk)?);

            // start: int32[3]
            let start = crate::util::read_array_i32::<3>(chunk)?;

            // m_id: int32[3]
            let m_id = crate::util::read_array_i32::<3>(chunk)?;

            // end: int32[3]
            let end = crate::util::read_array_i32::<3>(chunk)?;


            rows.push(ParticleColorRow {
                id,
                start,
                m_id,
                end,
            });
        }

        Ok(ParticleColor { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 10,
            record_size: 40,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (ParticleColor) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // start: int32[3]
            for i in row.start {
                b.write_all(&i.to_le_bytes())?;
            }


            // m_id: int32[3]
            for i in row.m_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // end: int32[3]
            for i in row.end {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for ParticleColor {
    type PrimaryKey = ParticleColorKey;
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
pub struct ParticleColorKey {
    pub id: i32
}

impl ParticleColorKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for ParticleColorKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for ParticleColorKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for ParticleColorKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for ParticleColorKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ParticleColorKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParticleColorRow {
    pub id: ParticleColorKey,
    pub start: [i32; 3],
    pub m_id: [i32; 3],
    pub end: [i32; 3],
}

