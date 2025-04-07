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
pub struct WorldMapArea {
    pub rows: Vec<WorldMapAreaRow>,
}

impl DbcTable for WorldMapArea {
    type Row = WorldMapAreaRow;

    const FILENAME: &'static str = "WorldMapArea.dbc";
    const FIELD_COUNT: usize = 11;
    const ROW_SIZE: usize = 44;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (WorldMapArea) int32
            let id = WorldMapAreaKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // area_id: foreign_key (AreaTable) int32
            let area_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // area_name: string_ref
            let area_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // loc_left: float
            let loc_left = crate::util::read_f32_le(chunk)?;

            // loc_right: float
            let loc_right = crate::util::read_f32_le(chunk)?;

            // loc_top: float
            let loc_top = crate::util::read_f32_le(chunk)?;

            // loc_bottom: float
            let loc_bottom = crate::util::read_f32_le(chunk)?;

            // display_map_id: foreign_key (Map) int32
            let display_map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // default_dungeon_floor: int32
            let default_dungeon_floor = crate::util::read_i32_le(chunk)?;

            // parent_world_map_id: foreign_key (WorldMapArea) int32
            let parent_world_map_id = WorldMapAreaKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(WorldMapAreaRow {
                id,
                map_id,
                area_id,
                area_name,
                loc_left,
                loc_right,
                loc_top,
                loc_bottom,
                display_map_id,
                default_dungeon_floor,
                parent_world_map_id,
            });
        }

        Ok(WorldMapArea { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (WorldMapArea) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // area_id: foreign_key (AreaTable) int32
            b.write_all(&(row.area_id.id as i32).to_le_bytes())?;

            // area_name: string_ref
            b.write_all(&string_cache.add_string(&row.area_name).to_le_bytes())?;

            // loc_left: float
            b.write_all(&row.loc_left.to_le_bytes())?;

            // loc_right: float
            b.write_all(&row.loc_right.to_le_bytes())?;

            // loc_top: float
            b.write_all(&row.loc_top.to_le_bytes())?;

            // loc_bottom: float
            b.write_all(&row.loc_bottom.to_le_bytes())?;

            // display_map_id: foreign_key (Map) int32
            b.write_all(&(row.display_map_id.id as i32).to_le_bytes())?;

            // default_dungeon_floor: int32
            b.write_all(&row.default_dungeon_floor.to_le_bytes())?;

            // parent_world_map_id: foreign_key (WorldMapArea) int32
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

impl Indexable for WorldMapArea {
    type PrimaryKey = WorldMapAreaKey;
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
pub struct WorldMapAreaKey {
    pub id: i32
}

impl WorldMapAreaKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for WorldMapAreaKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for WorldMapAreaKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for WorldMapAreaKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for WorldMapAreaKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for WorldMapAreaKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for WorldMapAreaKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for WorldMapAreaKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for WorldMapAreaKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for WorldMapAreaKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for WorldMapAreaKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldMapAreaRow {
    pub id: WorldMapAreaKey,
    pub map_id: MapKey,
    pub area_id: AreaTableKey,
    pub area_name: String,
    pub loc_left: f32,
    pub loc_right: f32,
    pub loc_top: f32,
    pub loc_bottom: f32,
    pub display_map_id: MapKey,
    pub default_dungeon_floor: i32,
    pub parent_world_map_id: WorldMapAreaKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn world_map_area() {
        let mut file = File::open("../wrath-dbc/WorldMapArea.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WorldMapArea::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldMapArea::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
