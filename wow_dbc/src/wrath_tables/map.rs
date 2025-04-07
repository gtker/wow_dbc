use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::area_table::AreaTableKey;
use crate::wrath_tables::loading_screens::LoadingScreensKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Map {
    pub rows: Vec<MapRow>,
}

impl DbcTable for Map {
    type Row = MapRow;

    const FILENAME: &'static str = "Map.dbc";
    const FIELD_COUNT: usize = 66;
    const ROW_SIZE: usize = 264;

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

            // id: primary_key (Map) int32
            let id = MapKey::new(crate::util::read_i32_le(chunk)?);

            // directory: string_ref
            let directory = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // instance_type: int32
            let instance_type = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // p_v_p: int32
            let p_v_p = crate::util::read_i32_le(chunk)?;

            // map_name_lang: string_ref_loc (Extended)
            let map_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // area_table_id: foreign_key (AreaTable) int32
            let area_table_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // map_description0_lang: string_ref_loc (Extended)
            let map_description0_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // map_description1_lang: string_ref_loc (Extended)
            let map_description1_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // loading_screen_id: foreign_key (LoadingScreens) int32
            let loading_screen_id = LoadingScreensKey::new(crate::util::read_i32_le(chunk)?.into());

            // minimap_icon_scale: float
            let minimap_icon_scale = crate::util::read_f32_le(chunk)?;

            // corpse_map_id: foreign_key (Map) int32
            let corpse_map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // corpse: float[2]
            let corpse = crate::util::read_array_f32::<2>(chunk)?;

            // time_of_day_override: int32
            let time_of_day_override = crate::util::read_i32_le(chunk)?;

            // expansion_id: int32
            let expansion_id = crate::util::read_i32_le(chunk)?;

            // raid_offset: int32
            let raid_offset = crate::util::read_i32_le(chunk)?;

            // max_players: int32
            let max_players = crate::util::read_i32_le(chunk)?;


            rows.push(MapRow {
                id,
                directory,
                instance_type,
                flags,
                p_v_p,
                map_name_lang,
                area_table_id,
                map_description0_lang,
                map_description1_lang,
                loading_screen_id,
                minimap_icon_scale,
                corpse_map_id,
                corpse,
                time_of_day_override,
                expansion_id,
                raid_offset,
                max_players,
            });
        }

        Ok(Map { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Map) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // directory: string_ref
            b.write_all(&string_cache.add_string(&row.directory).to_le_bytes())?;

            // instance_type: int32
            b.write_all(&row.instance_type.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // p_v_p: int32
            b.write_all(&row.p_v_p.to_le_bytes())?;

            // map_name_lang: string_ref_loc (Extended)
            b.write_all(&row.map_name_lang.string_indices_as_array(&mut string_cache))?;

            // area_table_id: foreign_key (AreaTable) int32
            b.write_all(&(row.area_table_id.id as i32).to_le_bytes())?;

            // map_description0_lang: string_ref_loc (Extended)
            b.write_all(&row.map_description0_lang.string_indices_as_array(&mut string_cache))?;

            // map_description1_lang: string_ref_loc (Extended)
            b.write_all(&row.map_description1_lang.string_indices_as_array(&mut string_cache))?;

            // loading_screen_id: foreign_key (LoadingScreens) int32
            b.write_all(&(row.loading_screen_id.id as i32).to_le_bytes())?;

            // minimap_icon_scale: float
            b.write_all(&row.minimap_icon_scale.to_le_bytes())?;

            // corpse_map_id: foreign_key (Map) int32
            b.write_all(&(row.corpse_map_id.id as i32).to_le_bytes())?;

            // corpse: float[2]
            for i in row.corpse {
                b.write_all(&i.to_le_bytes())?;
            }


            // time_of_day_override: int32
            b.write_all(&row.time_of_day_override.to_le_bytes())?;

            // expansion_id: int32
            b.write_all(&row.expansion_id.to_le_bytes())?;

            // raid_offset: int32
            b.write_all(&row.raid_offset.to_le_bytes())?;

            // max_players: int32
            b.write_all(&row.max_players.to_le_bytes())?;

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
    pub id: i32
}

impl MapKey {
    pub const fn new(id: i32) -> Self {
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

impl From<i8> for MapKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for MapKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for MapKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for MapKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for MapKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for MapKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for MapKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for MapKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapRow {
    pub id: MapKey,
    pub directory: String,
    pub instance_type: i32,
    pub flags: i32,
    pub p_v_p: i32,
    pub map_name_lang: ExtendedLocalizedString,
    pub area_table_id: AreaTableKey,
    pub map_description0_lang: ExtendedLocalizedString,
    pub map_description1_lang: ExtendedLocalizedString,
    pub loading_screen_id: LoadingScreensKey,
    pub minimap_icon_scale: f32,
    pub corpse_map_id: MapKey,
    pub corpse: [f32; 2],
    pub time_of_day_override: i32,
    pub expansion_id: i32,
    pub raid_offset: i32,
    pub max_players: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn map() {
        let mut file = File::open("../wrath-dbc/Map.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Map::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Map::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
