use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::area_table::AreaTableKey;
use crate::wrath_tables::map::MapKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DungeonMap {
    pub rows: Vec<DungeonMapRow>,
}

impl DbcTable for DungeonMap {
    type Row = DungeonMapRow;

    const FILENAME: &'static str = "DungeonMap.dbc";
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

            // id: primary_key (DungeonMap) int32
            let id = DungeonMapKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // floor_index: int32
            let floor_index = crate::util::read_i32_le(chunk)?;

            // min_x: float
            let min_x = crate::util::read_f32_le(chunk)?;

            // max_x: float
            let max_x = crate::util::read_f32_le(chunk)?;

            // min_y: float
            let min_y = crate::util::read_f32_le(chunk)?;

            // max_y: float
            let max_y = crate::util::read_f32_le(chunk)?;

            // parent_world_map_id: foreign_key (AreaTable) int32
            let parent_world_map_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(DungeonMapRow {
                id,
                map_id,
                floor_index,
                min_x,
                max_x,
                min_y,
                max_y,
                parent_world_map_id,
            });
        }

        Ok(DungeonMap { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (DungeonMap) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // floor_index: int32
            b.write_all(&row.floor_index.to_le_bytes())?;

            // min_x: float
            b.write_all(&row.min_x.to_le_bytes())?;

            // max_x: float
            b.write_all(&row.max_x.to_le_bytes())?;

            // min_y: float
            b.write_all(&row.min_y.to_le_bytes())?;

            // max_y: float
            b.write_all(&row.max_y.to_le_bytes())?;

            // parent_world_map_id: foreign_key (AreaTable) int32
            b.write_all(&(row.parent_world_map_id.id as i32).to_le_bytes())?;

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

impl Indexable for DungeonMap {
    type PrimaryKey = DungeonMapKey;
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
pub struct DungeonMapKey {
    pub id: i32
}

impl DungeonMapKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for DungeonMapKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for DungeonMapKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for DungeonMapKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for DungeonMapKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for DungeonMapKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for DungeonMapKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for DungeonMapKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for DungeonMapKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for DungeonMapKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for DungeonMapKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DungeonMapRow {
    pub id: DungeonMapKey,
    pub map_id: MapKey,
    pub floor_index: i32,
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub parent_world_map_id: AreaTableKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn dungeon_map() {
        let mut file = File::open("../wrath-dbc/DungeonMap.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = DungeonMap::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = DungeonMap::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
