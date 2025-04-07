use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LFGDungeonExpansion {
    pub rows: Vec<LFGDungeonExpansionRow>,
}

impl DbcTable for LFGDungeonExpansion {
    type Row = LFGDungeonExpansionRow;

    const FILENAME: &'static str = "LFGDungeonExpansion.dbc";
    const FIELD_COUNT: usize = 8;
    const ROW_SIZE: usize = 32;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
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

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

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

        assert_eq!(b.len(), self.rows.len() * Self::ROW_SIZE);
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: string_cache.size(),
        };

        w.write_all(&header.write_header())?;
        w.write_all(&b)?;
        w.write_all(string_cache.buffer())?;
        Ok(())
    }

}

impl Indexable for LFGDungeonExpansion {
    type PrimaryKey = LFGDungeonExpansionKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LFGDungeonExpansionKey {
    pub id: i32
}

impl LFGDungeonExpansionKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

impl TryFrom<u32> for LFGDungeonExpansionKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for LFGDungeonExpansionKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for LFGDungeonExpansionKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for LFGDungeonExpansionKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for LFGDungeonExpansionKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn lfg_dungeon_expansion() {
        let mut file = File::open("../wrath-dbc/LFGDungeonExpansion.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = LFGDungeonExpansion::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = LFGDungeonExpansion::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
