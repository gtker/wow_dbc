use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::area_table::AreaTableKey;
use crate::vanilla_tables::map::MapKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldStateUI {
    pub rows: Vec<WorldStateUIRow>,
}

impl DbcTable for WorldStateUI {
    type Row = WorldStateUIRow;

    const FILENAME: &'static str = "WorldStateUI.dbc";
    const FIELD_COUNT: usize = 39;
    const ROW_SIZE: usize = 156;

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

            // id: primary_key (WorldStateUI) uint32
            let id = WorldStateUIKey::new(crate::util::read_u32_le(chunk)?);

            // map: foreign_key (Map) uint32
            let map = MapKey::new(crate::util::read_u32_le(chunk)?.into());

            // area_table: foreign_key (AreaTable) uint32
            let area_table = AreaTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // icon: string_ref
            let icon = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // state_variable: string_ref_loc
            let state_variable = crate::util::read_localized_string(chunk, &string_block)?;

            // tooltip: string_ref_loc
            let tooltip = crate::util::read_localized_string(chunk, &string_block)?;

            // state: int32
            let state = crate::util::read_i32_le(chunk)?;

            // world_state: uint32
            let world_state = crate::util::read_u32_le(chunk)?;

            // ty: int32
            let ty = crate::util::read_i32_le(chunk)?;

            // dynamic_icon: string_ref
            let dynamic_icon = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // dynamic_tooltip: string_ref_loc
            let dynamic_tooltip = crate::util::read_localized_string(chunk, &string_block)?;

            // extended_ui: string_ref
            let extended_ui = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // unknown: uint32[3]
            let unknown = crate::util::read_array_u32::<3>(chunk)?;


            rows.push(WorldStateUIRow {
                id,
                map,
                area_table,
                icon,
                state_variable,
                tooltip,
                state,
                world_state,
                ty,
                dynamic_icon,
                dynamic_tooltip,
                extended_ui,
                unknown,
            });
        }

        Ok(WorldStateUI { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (WorldStateUI) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map: foreign_key (Map) uint32
            b.write_all(&(row.map.id as u32).to_le_bytes())?;

            // area_table: foreign_key (AreaTable) uint32
            b.write_all(&(row.area_table.id as u32).to_le_bytes())?;

            // icon: string_ref
            if !row.icon.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.icon.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // state_variable: string_ref_loc
            b.write_all(&row.state_variable.string_indices_as_array(&mut string_index))?;

            // tooltip: string_ref_loc
            b.write_all(&row.tooltip.string_indices_as_array(&mut string_index))?;

            // state: int32
            b.write_all(&row.state.to_le_bytes())?;

            // world_state: uint32
            b.write_all(&row.world_state.to_le_bytes())?;

            // ty: int32
            b.write_all(&row.ty.to_le_bytes())?;

            // dynamic_icon: string_ref
            if !row.dynamic_icon.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.dynamic_icon.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // dynamic_tooltip: string_ref_loc
            b.write_all(&row.dynamic_tooltip.string_indices_as_array(&mut string_index))?;

            // extended_ui: string_ref
            if !row.extended_ui.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.extended_ui.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // unknown: uint32[3]
            for i in row.unknown {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for WorldStateUI {
    type PrimaryKey = WorldStateUIKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl WorldStateUI {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.icon.is_empty() { b.write_all(row.icon.as_bytes())?; b.write_all(&[0])?; };
            row.state_variable.string_block_as_array(b)?;
            row.tooltip.string_block_as_array(b)?;
            if !row.dynamic_icon.is_empty() { b.write_all(row.dynamic_icon.as_bytes())?; b.write_all(&[0])?; };
            row.dynamic_tooltip.string_block_as_array(b)?;
            if !row.extended_ui.is_empty() { b.write_all(row.extended_ui.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.icon.is_empty() { sum += row.icon.len() + 1; };
            sum += row.state_variable.string_block_size();
            sum += row.tooltip.string_block_size();
            if !row.dynamic_icon.is_empty() { sum += row.dynamic_icon.len() + 1; };
            sum += row.dynamic_tooltip.string_block_size();
            if !row.extended_ui.is_empty() { sum += row.extended_ui.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldStateUIKey {
    pub id: u32
}

impl WorldStateUIKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for WorldStateUIKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for WorldStateUIKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for WorldStateUIKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for WorldStateUIKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for WorldStateUIKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for WorldStateUIKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for WorldStateUIKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for WorldStateUIKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for WorldStateUIKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for WorldStateUIKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldStateUIRow {
    pub id: WorldStateUIKey,
    pub map: MapKey,
    pub area_table: AreaTableKey,
    pub icon: String,
    pub state_variable: LocalizedString,
    pub tooltip: LocalizedString,
    pub state: i32,
    pub world_state: u32,
    pub ty: i32,
    pub dynamic_icon: String,
    pub dynamic_tooltip: LocalizedString,
    pub extended_ui: String,
    pub unknown: [u32; 3],
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn world_state_ui() {
        let mut file = File::open("../vanilla-dbc/WorldStateUI.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WorldStateUI::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldStateUI::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
