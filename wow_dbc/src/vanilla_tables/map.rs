use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::vanilla_tables::area_table::AreaTableKey;
use crate::vanilla_tables::loading_screens::LoadingScreensKey;
use std::io::Write;
pub use wow_world_base::vanilla::InstanceType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Map {
    pub rows: Vec<MapRow>,
}

impl DbcTable for Map {
    type Row = MapRow;

    const FILENAME: &'static str = "Map.dbc";
    const FIELD_COUNT: usize = 42;
    const ROW_SIZE: usize = 168;

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

            // id: primary_key (Map) uint32
            let id = MapKey::new(crate::util::read_u32_le(chunk)?);

            // internal_name: string_ref
            let internal_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // instance_type: InstanceType
            let instance_type = crate::util::read_i32_le(chunk)?.try_into()?;

            // battleground: bool32
            let battleground = crate::util::read_u32_le(chunk)? != 0;

            // map_name: string_ref_loc
            let map_name = crate::util::read_localized_string(chunk, &string_block)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;

            // max_level: int32
            let max_level = crate::util::read_i32_le(chunk)?;

            // max_players: int32
            let max_players = crate::util::read_i32_le(chunk)?;

            // unknown: int32[3]
            let unknown = crate::util::read_array_i32::<3>(chunk)?;

            // area_table: foreign_key (AreaTable) uint32
            let area_table = AreaTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // map_description_horde: string_ref_loc
            let map_description_horde = crate::util::read_localized_string(chunk, &string_block)?;

            // map_description_alliance: string_ref_loc
            let map_description_alliance = crate::util::read_localized_string(chunk, &string_block)?;

            // loading_screen: foreign_key (LoadingScreens) uint32
            let loading_screen = LoadingScreensKey::new(crate::util::read_u32_le(chunk)?.into());

            // raid_offset: int32
            let raid_offset = crate::util::read_i32_le(chunk)?;

            // unknown_2: int32[2]
            let unknown_2 = crate::util::read_array_i32::<2>(chunk)?;


            rows.push(MapRow {
                id,
                internal_name,
                instance_type,
                battleground,
                map_name,
                min_level,
                max_level,
                max_players,
                unknown,
                area_table,
                map_description_horde,
                map_description_alliance,
                loading_screen,
                raid_offset,
                unknown_2,
            });
        }

        Ok(Map { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Map) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // internal_name: string_ref
            b.write_all(&string_cache.add_string(&row.internal_name).to_le_bytes())?;

            // instance_type: InstanceType
            b.write_all(&(row.instance_type.as_int() as i32).to_le_bytes())?;

            // battleground: bool32
            b.write_all(&u32::from(row.battleground).to_le_bytes())?;

            // map_name: string_ref_loc
            b.write_all(&row.map_name.string_indices_as_array(&mut string_cache))?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

            // max_level: int32
            b.write_all(&row.max_level.to_le_bytes())?;

            // max_players: int32
            b.write_all(&row.max_players.to_le_bytes())?;

            // unknown: int32[3]
            for i in row.unknown {
                b.write_all(&i.to_le_bytes())?;
            }


            // area_table: foreign_key (AreaTable) uint32
            b.write_all(&(row.area_table.id as u32).to_le_bytes())?;

            // map_description_horde: string_ref_loc
            b.write_all(&row.map_description_horde.string_indices_as_array(&mut string_cache))?;

            // map_description_alliance: string_ref_loc
            b.write_all(&row.map_description_alliance.string_indices_as_array(&mut string_cache))?;

            // loading_screen: foreign_key (LoadingScreens) uint32
            b.write_all(&(row.loading_screen.id as u32).to_le_bytes())?;

            // raid_offset: int32
            b.write_all(&row.raid_offset.to_le_bytes())?;

            // unknown_2: int32[2]
            for i in row.unknown_2 {
                b.write_all(&i.to_le_bytes())?;
            }


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

impl Indexable for Map {
    type PrimaryKey = MapKey;
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
pub struct MapKey {
    pub id: u32
}

impl MapKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for MapKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for MapKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for MapKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for MapKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for MapKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for MapKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for MapKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for MapKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for MapKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for MapKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapRow {
    pub id: MapKey,
    pub internal_name: String,
    pub instance_type: InstanceType,
    pub battleground: bool,
    pub map_name: LocalizedString,
    pub min_level: i32,
    pub max_level: i32,
    pub max_players: i32,
    pub unknown: [i32; 3],
    pub area_table: AreaTableKey,
    pub map_description_horde: LocalizedString,
    pub map_description_alliance: LocalizedString,
    pub loading_screen: LoadingScreensKey,
    pub raid_offset: i32,
    pub unknown_2: [i32; 2],
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn map() {
        let mut file = File::open("../vanilla-dbc/Map.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Map::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Map::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
