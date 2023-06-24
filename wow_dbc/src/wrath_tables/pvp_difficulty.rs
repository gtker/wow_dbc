use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::map::MapKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PvpDifficulty {
    pub rows: Vec<PvpDifficultyRow>,
}

impl DbcTable for PvpDifficulty {
    type Row = PvpDifficultyRow;

    const FILENAME: &'static str = "PvpDifficulty.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 24 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 24,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 6 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 6,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (PvpDifficulty) int32
            let id = PvpDifficultyKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // range_index: int32
            let range_index = crate::util::read_i32_le(chunk)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;

            // max_level: int32
            let max_level = crate::util::read_i32_le(chunk)?;

            // difficulty: int32
            let difficulty = crate::util::read_i32_le(chunk)?;


            rows.push(PvpDifficultyRow {
                id,
                map_id,
                range_index,
                min_level,
                max_level,
                difficulty,
            });
        }

        Ok(PvpDifficulty { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 6,
            record_size: 24,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (PvpDifficulty) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // range_index: int32
            b.write_all(&row.range_index.to_le_bytes())?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

            // max_level: int32
            b.write_all(&row.max_level.to_le_bytes())?;

            // difficulty: int32
            b.write_all(&row.difficulty.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for PvpDifficulty {
    type PrimaryKey = PvpDifficultyKey;
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
pub struct PvpDifficultyKey {
    pub id: i32
}

impl PvpDifficultyKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for PvpDifficultyKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for PvpDifficultyKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for PvpDifficultyKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl From<u8> for PvpDifficultyKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for PvpDifficultyKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl TryFrom<u32> for PvpDifficultyKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PvpDifficultyRow {
    pub id: PvpDifficultyKey,
    pub map_id: MapKey,
    pub range_index: i32,
    pub min_level: i32,
    pub max_level: i32,
    pub difficulty: i32,
}

