use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;
use crate::tables::area_table::*;
use crate::tables::loading_screens::*;

#[derive(Debug, Clone, PartialEq)]
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

        if header.record_size != 168 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 168,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 42 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 168,
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
            let instance_type = InstanceType::try_from(crate::util::read_i32_le(chunk)?)?;

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

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 42,
            record_size: 168,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Map) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // internal_name: string_ref
            if !row.internal_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.internal_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // instance_type: InstanceType
            b.write_all(&(row.instance_type.as_int() as i32).to_le_bytes())?;

            // battleground: bool32
            b.write_all(&u32::from(row.battleground).to_le_bytes())?;

            // map_name: string_ref_loc
            b.write_all(&row.map_name.string_indices_as_array(&mut string_index))?;

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
            b.write_all(&row.map_description_horde.string_indices_as_array(&mut string_index))?;

            // map_description_alliance: string_ref_loc
            b.write_all(&row.map_description_alliance.string_indices_as_array(&mut string_index))?;

            // loading_screen: foreign_key (LoadingScreens) uint32
            b.write_all(&(row.loading_screen.id as u32).to_le_bytes())?;

            // raid_offset: int32
            b.write_all(&row.raid_offset.to_le_bytes())?;

            // unknown_2: int32[2]
            for i in row.unknown_2 {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Map {
    type PrimaryKey = MapKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl Map {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.internal_name.is_empty() { b.write_all(row.internal_name.as_bytes())?; b.write_all(&[0])?; };
            row.map_name.string_block_as_array(b)?;
            row.map_description_horde.string_block_as_array(b)?;
            row.map_description_alliance.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.internal_name.is_empty() { sum += row.internal_name.len() + 1; };
            sum += row.map_name.string_block_size();
            sum += row.map_description_horde.string_block_size();
            sum += row.map_description_alliance.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct MapKey {
    pub id: u32
}

impl MapKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum InstanceType {
    Normal,
    Group,
    Raid,
    Battleground,
}

impl TryFrom<i32> for InstanceType {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Normal,
            1 => Self::Group,
            2 => Self::Raid,
            3 => Self::Battleground,
            val => return Err(crate::InvalidEnumError::new("InstanceType", val as i64)),
        })
    }

}

impl InstanceType {
    const fn as_int(&self) -> i32 {
        match self {
            Self::Normal => 0,
            Self::Group => 1,
            Self::Raid => 2,
            Self::Battleground => 3,
        }

    }

}

impl Default for InstanceType {
    fn default() -> Self {
        Self::Normal
    }

}

#[derive(Debug, Clone, PartialEq)]
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

    #[test]
    fn map() {
        let contents = include_bytes!("../../../dbc/Map.dbc");
        let actual = Map::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Map::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
