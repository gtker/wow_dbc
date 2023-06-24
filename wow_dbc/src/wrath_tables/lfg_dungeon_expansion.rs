use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LFGDungeonExpansion {
    pub rows: Vec<LFGDungeonExpansionRow>,
}

impl DbcTable for LFGDungeonExpansion {
    type Row = LFGDungeonExpansionRow;

    const FILENAME: &'static str = "LFGDungeonExpansion.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 8,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (LFGDungeonExpansion) int32
            let id = LFGDungeonExpansionKey::new(crate::util::read_i32_le(chunk)?);

            // lfg_id: int32
            let lfg_id = crate::util::read_i32_le(chunk)?;

            // expansion_level: int32
            let expansion_level = crate::util::read_i32_le(chunk)?;

            // random_id: int32
            let random_id = crate::util::read_i32_le(chunk)?;

            // hard_level_min: int32
            let hard_level_min = crate::util::read_i32_le(chunk)?;

            // hard_level_max: int32
            let hard_level_max = crate::util::read_i32_le(chunk)?;

            // target_level_min: int32
            let target_level_min = crate::util::read_i32_le(chunk)?;

            // target_level_max: int32
            let target_level_max = crate::util::read_i32_le(chunk)?;


            rows.push(LFGDungeonExpansionRow {
                id,
                lfg_id,
                expansion_level,
                random_id,
                hard_level_min,
                hard_level_max,
                target_level_min,
                target_level_max,
            });
        }

        Ok(LFGDungeonExpansion { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 8,
            record_size: 32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (LFGDungeonExpansion) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // lfg_id: int32
            b.write_all(&row.lfg_id.to_le_bytes())?;

            // expansion_level: int32
            b.write_all(&row.expansion_level.to_le_bytes())?;

            // random_id: int32
            b.write_all(&row.random_id.to_le_bytes())?;

            // hard_level_min: int32
            b.write_all(&row.hard_level_min.to_le_bytes())?;

            // hard_level_max: int32
            b.write_all(&row.hard_level_max.to_le_bytes())?;

            // target_level_min: int32
            b.write_all(&row.target_level_min.to_le_bytes())?;

            // target_level_max: int32
            b.write_all(&row.target_level_max.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for LFGDungeonExpansion {
    type PrimaryKey = LFGDungeonExpansionKey;
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
pub struct LFGDungeonExpansionKey {
    pub id: i32
}

impl LFGDungeonExpansionKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for LFGDungeonExpansionKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for LFGDungeonExpansionKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for LFGDungeonExpansionKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for LFGDungeonExpansionKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for LFGDungeonExpansionKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LFGDungeonExpansionRow {
    pub id: LFGDungeonExpansionKey,
    pub lfg_id: i32,
    pub expansion_level: i32,
    pub random_id: i32,
    pub hard_level_min: i32,
    pub hard_level_max: i32,
    pub target_level_min: i32,
    pub target_level_max: i32,
}

