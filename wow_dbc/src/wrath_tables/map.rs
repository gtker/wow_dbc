use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};
use crate::wrath_tables::area_table::*;
use crate::wrath_tables::loading_screens::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Map {
    pub rows: Vec<MapRow>,
}

impl DbcTable for Map {
    type Row = MapRow;

    fn filename() -> &'static str { "Map.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 264 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 264,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 66 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 66,
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

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 66,
            record_size: 264,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Map) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // directory: string_ref
            if !row.directory.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.directory.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // instance_type: int32
            b.write_all(&row.instance_type.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // p_v_p: int32
            b.write_all(&row.p_v_p.to_le_bytes())?;

            // map_name_lang: string_ref_loc (Extended)
            b.write_all(&row.map_name_lang.string_indices_as_array(&mut string_index))?;

            // area_table_id: foreign_key (AreaTable) int32
            b.write_all(&(row.area_table_id.id as i32).to_le_bytes())?;

            // map_description0_lang: string_ref_loc (Extended)
            b.write_all(&row.map_description0_lang.string_indices_as_array(&mut string_index))?;

            // map_description1_lang: string_ref_loc (Extended)
            b.write_all(&row.map_description1_lang.string_indices_as_array(&mut string_index))?;

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

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Map {
    type PrimaryKey = MapKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl Map {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.directory.is_empty() { b.write_all(row.directory.as_bytes())?; b.write_all(&[0])?; };
            row.map_name_lang.string_block_as_array(b)?;
            row.map_description0_lang.string_block_as_array(b)?;
            row.map_description1_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.directory.is_empty() { sum += row.directory.len() + 1; };
            sum += row.map_name_lang.string_block_size();
            sum += row.map_description0_lang.string_block_size();
            sum += row.map_description1_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstMap<const S: usize> {
    pub rows: [ConstMapRow; S],
}

impl<const S: usize> ConstMap<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 264 {
            panic!("invalid record size, expected 264")
        }

        if header.field_count != 66 {
            panic!("invalid field count, expected 66")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstMapRow {
                id: MapKey::new(0),
                directory: "",
                instance_type: 0,
                flags: 0,
                p_v_p: 0,
                map_name_lang: crate::ConstExtendedLocalizedString::empty(),
                area_table_id: AreaTableKey::new(0),
                map_description0_lang: crate::ConstExtendedLocalizedString::empty(),
                map_description1_lang: crate::ConstExtendedLocalizedString::empty(),
                loading_screen_id: LoadingScreensKey::new(0),
                minimap_icon_scale: 0.0,
                corpse_map_id: MapKey::new(0),
                corpse: [0.0; 2],
                time_of_day_override: 0,
                expansion_id: 0,
                raid_offset: 0,
                max_players: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Map) int32
            let id = MapKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // directory: string_ref
            let directory = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // instance_type: int32
            let instance_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // p_v_p: int32
            let p_v_p = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // map_name_lang: string_ref_loc (Extended)
            let map_name_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // area_table_id: foreign_key (AreaTable) int32
            let area_table_id = AreaTableKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // map_description0_lang: string_ref_loc (Extended)
            let map_description0_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // map_description1_lang: string_ref_loc (Extended)
            let map_description1_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // loading_screen_id: foreign_key (LoadingScreens) int32
            let loading_screen_id = LoadingScreensKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // minimap_icon_scale: float
            let minimap_icon_scale = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // corpse_map_id: foreign_key (Map) int32
            let corpse_map_id = MapKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // corpse: float[2]
            let corpse = {
                let mut a = [0.0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // time_of_day_override: int32
            let time_of_day_override = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // expansion_id: int32
            let expansion_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // raid_offset: int32
            let raid_offset = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // max_players: int32
            let max_players = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstMapRow {
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
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> Map {
        Map {
            rows: self.rows.iter().map(|s| MapRow {
                id: s.id,
                directory: s.directory.to_string(),
                instance_type: s.instance_type,
                flags: s.flags,
                p_v_p: s.p_v_p,
                map_name_lang: s.map_name_lang.to_string(),
                area_table_id: s.area_table_id,
                map_description0_lang: s.map_description0_lang.to_string(),
                map_description1_lang: s.map_description1_lang.to_string(),
                loading_screen_id: s.loading_screen_id,
                minimap_icon_scale: s.minimap_icon_scale,
                corpse_map_id: s.corpse_map_id,
                corpse: s.corpse,
                time_of_day_override: s.time_of_day_override,
                expansion_id: s.expansion_id,
                raid_offset: s.raid_offset,
                max_players: s.max_players,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct MapKey {
    pub id: i32
}

impl MapKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstMapRow {
    pub id: MapKey,
    pub directory: &'static str,
    pub instance_type: i32,
    pub flags: i32,
    pub p_v_p: i32,
    pub map_name_lang: ConstExtendedLocalizedString,
    pub area_table_id: AreaTableKey,
    pub map_description0_lang: ConstExtendedLocalizedString,
    pub map_description1_lang: ConstExtendedLocalizedString,
    pub loading_screen_id: LoadingScreensKey,
    pub minimap_icon_scale: f32,
    pub corpse_map_id: MapKey,
    pub corpse: [f32; 2],
    pub time_of_day_override: i32,
    pub expansion_id: i32,
    pub raid_offset: i32,
    pub max_players: i32,
}

