use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};
use crate::tbc_tables::area_table::*;
use crate::tbc_tables::faction::*;
use crate::tbc_tables::map::*;

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

        if header.record_size != 252 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 252,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 63 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 63,
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

            // id: primary_key (WorldStateUI) int32
            let id = WorldStateUIKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // area_id: foreign_key (AreaTable) int32
            let area_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // icon: string_ref
            let icon = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // string_lang: string_ref_loc (Extended)
            let string_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // tooltip_lang: string_ref_loc (Extended)
            let tooltip_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // faction_id: foreign_key (Faction) int32
            let faction_id = FactionKey::new(crate::util::read_i32_le(chunk)?.into());

            // state_variable: int32
            let state_variable = crate::util::read_i32_le(chunk)?;

            // ty: int32
            let ty = crate::util::read_i32_le(chunk)?;

            // dynamic_icon: string_ref
            let dynamic_icon = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // dynamic_tooltip_lang: string_ref_loc (Extended)
            let dynamic_tooltip_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // extended_u_i: string_ref
            let extended_u_i = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // extended_u_i_state_variable: int32[3]
            let extended_u_i_state_variable = crate::util::read_array_i32::<3>(chunk)?;


            rows.push(WorldStateUIRow {
                id,
                map_id,
                area_id,
                icon,
                string_lang,
                tooltip_lang,
                faction_id,
                state_variable,
                ty,
                dynamic_icon,
                dynamic_tooltip_lang,
                extended_u_i,
                extended_u_i_state_variable,
            });
        }

        Ok(WorldStateUI { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 63,
            record_size: 252,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (WorldStateUI) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // area_id: foreign_key (AreaTable) int32
            b.write_all(&(row.area_id.id as i32).to_le_bytes())?;

            // icon: string_ref
            if !row.icon.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.icon.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // string_lang: string_ref_loc (Extended)
            b.write_all(&row.string_lang.string_indices_as_array(&mut string_index))?;

            // tooltip_lang: string_ref_loc (Extended)
            b.write_all(&row.tooltip_lang.string_indices_as_array(&mut string_index))?;

            // faction_id: foreign_key (Faction) int32
            b.write_all(&(row.faction_id.id as i32).to_le_bytes())?;

            // state_variable: int32
            b.write_all(&row.state_variable.to_le_bytes())?;

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

            // dynamic_tooltip_lang: string_ref_loc (Extended)
            b.write_all(&row.dynamic_tooltip_lang.string_indices_as_array(&mut string_index))?;

            // extended_u_i: string_ref
            if !row.extended_u_i.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.extended_u_i.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // extended_u_i_state_variable: int32[3]
            for i in row.extended_u_i_state_variable {
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
            row.string_lang.string_block_as_array(b)?;
            row.tooltip_lang.string_block_as_array(b)?;
            if !row.dynamic_icon.is_empty() { b.write_all(row.dynamic_icon.as_bytes())?; b.write_all(&[0])?; };
            row.dynamic_tooltip_lang.string_block_as_array(b)?;
            if !row.extended_u_i.is_empty() { b.write_all(row.extended_u_i.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.icon.is_empty() { sum += row.icon.len() + 1; };
            sum += row.string_lang.string_block_size();
            sum += row.tooltip_lang.string_block_size();
            if !row.dynamic_icon.is_empty() { sum += row.dynamic_icon.len() + 1; };
            sum += row.dynamic_tooltip_lang.string_block_size();
            if !row.extended_u_i.is_empty() { sum += row.extended_u_i.len() + 1; };
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
        if header.record_size != 252 {
            panic!("invalid record size, expected 252")
        }

        if header.field_count != 63 {
            panic!("invalid field count, expected 63")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstWorldStateUIRow {
                id: WorldStateUIKey::new(0),
                map_id: MapKey::new(0),
                area_id: AreaTableKey::new(0),
                icon: "",
                string_lang: crate::ConstExtendedLocalizedString::empty(),
                tooltip_lang: crate::ConstExtendedLocalizedString::empty(),
                faction_id: FactionKey::new(0),
                state_variable: 0,
                ty: 0,
                dynamic_icon: "",
                dynamic_tooltip_lang: crate::ConstExtendedLocalizedString::empty(),
                extended_u_i: "",
                extended_u_i_state_variable: [0; 3],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (WorldStateUI) int32
            let id = WorldStateUIKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // area_id: foreign_key (AreaTable) int32
            let area_id = AreaTableKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // icon: string_ref
            let icon = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // string_lang: string_ref_loc (Extended)
            let string_lang = ConstExtendedLocalizedString::new(
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

            // tooltip_lang: string_ref_loc (Extended)
            let tooltip_lang = ConstExtendedLocalizedString::new(
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

            // faction_id: foreign_key (Faction) int32
            let faction_id = FactionKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // state_variable: int32
            let state_variable = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // ty: int32
            let ty = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // dynamic_icon: string_ref
            let dynamic_icon = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // dynamic_tooltip_lang: string_ref_loc (Extended)
            let dynamic_tooltip_lang = ConstExtendedLocalizedString::new(
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

            // extended_u_i: string_ref
            let extended_u_i = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // extended_u_i_state_variable: int32[3]
            let extended_u_i_state_variable = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = ConstWorldStateUIRow {
                id,
                map_id,
                area_id,
                icon,
                string_lang,
                tooltip_lang,
                faction_id,
                state_variable,
                ty,
                dynamic_icon,
                dynamic_tooltip_lang,
                extended_u_i,
                extended_u_i_state_variable,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> WorldStateUI {
        WorldStateUI {
            rows: self.rows.iter().map(|s| WorldStateUIRow {
                id: s.id,
                map_id: s.map_id,
                area_id: s.area_id,
                icon: s.icon.to_string(),
                string_lang: s.string_lang.to_string(),
                tooltip_lang: s.tooltip_lang.to_string(),
                faction_id: s.faction_id,
                state_variable: s.state_variable,
                ty: s.ty,
                dynamic_icon: s.dynamic_icon.to_string(),
                dynamic_tooltip_lang: s.dynamic_tooltip_lang.to_string(),
                extended_u_i: s.extended_u_i.to_string(),
                extended_u_i_state_variable: s.extended_u_i_state_variable,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct WorldStateUIKey {
    pub id: i32
}

impl WorldStateUIKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for WorldStateUIKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for WorldStateUIKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for WorldStateUIKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldStateUIRow {
    pub id: WorldStateUIKey,
    pub map_id: MapKey,
    pub area_id: AreaTableKey,
    pub icon: String,
    pub string_lang: ExtendedLocalizedString,
    pub tooltip_lang: ExtendedLocalizedString,
    pub faction_id: FactionKey,
    pub state_variable: i32,
    pub ty: i32,
    pub dynamic_icon: String,
    pub dynamic_tooltip_lang: ExtendedLocalizedString,
    pub extended_u_i: String,
    pub extended_u_i_state_variable: [i32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstWorldStateUIRow {
    pub id: WorldStateUIKey,
    pub map_id: MapKey,
    pub area_id: AreaTableKey,
    pub icon: &'static str,
    pub string_lang: ConstExtendedLocalizedString,
    pub tooltip_lang: ConstExtendedLocalizedString,
    pub faction_id: FactionKey,
    pub state_variable: i32,
    pub ty: i32,
    pub dynamic_icon: &'static str,
    pub dynamic_tooltip_lang: ConstExtendedLocalizedString,
    pub extended_u_i: &'static str,
    pub extended_u_i_state_variable: [i32; 3],
}

