use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerrainTypeSounds {
    pub rows: Vec<TerrainTypeSoundsRow>,
}

impl DbcTable for TerrainTypeSounds {
    type Row = TerrainTypeSoundsRow;

    fn filename() -> &'static str { "TerrainTypeSounds.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 4 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 4,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 1 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 1,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (TerrainTypeSounds) uint32
            let id = TerrainTypeSoundsKey::new(crate::util::read_u32_le(chunk)?);


            rows.push(TerrainTypeSoundsRow {
                id,
            });
        }

        Ok(TerrainTypeSounds { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 1,
            record_size: 4,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (TerrainTypeSounds) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for TerrainTypeSounds {
    type PrimaryKey = TerrainTypeSoundsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstTerrainTypeSounds<const S: usize> {
    pub rows: [TerrainTypeSoundsRow; S],
}

impl<const S: usize> ConstTerrainTypeSounds<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 4 {
            panic!("invalid record size, expected 4")
        }

        if header.field_count != 1 {
            panic!("invalid field count, expected 1")
        }

        let mut b_offset = 20;
        let mut rows = [
            TerrainTypeSoundsRow {
                id: TerrainTypeSoundsKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (TerrainTypeSounds) uint32
            let id = TerrainTypeSoundsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = TerrainTypeSoundsRow {
                id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> TerrainTypeSounds {
        TerrainTypeSounds {
            rows: self.rows.iter().map(|s| TerrainTypeSoundsRow {
                id: s.id,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct TerrainTypeSoundsKey {
    pub id: u32
}

impl TerrainTypeSoundsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for TerrainTypeSoundsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for TerrainTypeSoundsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for TerrainTypeSoundsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerrainTypeSoundsRow {
    pub id: TerrainTypeSoundsKey,
}

