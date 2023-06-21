use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};
use crate::vanilla_tables::area_table::*;
use crate::vanilla_tables::map::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldStateUI {
    pub rows: Vec<WorldStateUIRow>,
}

impl DbcTable for WorldStateUI {
    type Row = WorldStateUIRow;

    fn filename() -> &'static str { "WorldStateUI.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 156 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 156,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 39 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 39,
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
            field_count: 39,
            record_size: 156,
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
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstWorldStateUI<const S: usize> {
    pub rows: [ConstWorldStateUIRow; S],
}

impl<const S: usize> ConstWorldStateUI<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 156 {
            panic!("invalid record size, expected 156")
        }

        if header.field_count != 39 {
            panic!("invalid field count, expected 39")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstWorldStateUIRow {
                id: WorldStateUIKey::new(0),
                map: MapKey::new(0),
                area_table: AreaTableKey::new(0),
                icon: "",
                state_variable: crate::ConstLocalizedString::empty(),
                tooltip: crate::ConstLocalizedString::empty(),
                state: 0,
                world_state: 0,
                ty: 0,
                dynamic_icon: "",
                dynamic_tooltip: crate::ConstLocalizedString::empty(),
                extended_ui: "",
                unknown: [0; 3],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (WorldStateUI) uint32
            let id = WorldStateUIKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // map: foreign_key (Map) uint32
            let map = MapKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // area_table: foreign_key (AreaTable) uint32
            let area_table = AreaTableKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // icon: string_ref
            let icon = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // state_variable: string_ref_loc
            let state_variable = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            // tooltip: string_ref_loc
            let tooltip = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            // state: int32
            let state = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // world_state: uint32
            let world_state = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // ty: int32
            let ty = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // dynamic_icon: string_ref
            let dynamic_icon = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // dynamic_tooltip: string_ref_loc
            let dynamic_tooltip = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            // extended_ui: string_ref
            let extended_ui = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // unknown: uint32[3]
            let unknown = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = ConstWorldStateUIRow {
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
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> WorldStateUI {
        WorldStateUI {
            rows: self.rows.iter().map(|s| WorldStateUIRow {
                id: s.id,
                map: s.map,
                area_table: s.area_table,
                icon: s.icon.to_string(),
                state_variable: s.state_variable.to_string(),
                tooltip: s.tooltip.to_string(),
                state: s.state,
                world_state: s.world_state,
                ty: s.ty,
                dynamic_icon: s.dynamic_icon.to_string(),
                dynamic_tooltip: s.dynamic_tooltip.to_string(),
                extended_ui: s.extended_ui.to_string(),
                unknown: s.unknown,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstWorldStateUIRow {
    pub id: WorldStateUIKey,
    pub map: MapKey,
    pub area_table: AreaTableKey,
    pub icon: &'static str,
    pub state_variable: ConstLocalizedString,
    pub tooltip: ConstLocalizedString,
    pub state: i32,
    pub world_state: u32,
    pub ty: i32,
    pub dynamic_icon: &'static str,
    pub dynamic_tooltip: ConstLocalizedString,
    pub extended_ui: &'static str,
    pub unknown: [u32; 3],
}

