use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::area_table::AreaTableKey;
use crate::wrath_tables::map::MapKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldStateUI {
    pub rows: Vec<WorldStateUIRow>,
}

impl DbcTable for WorldStateUI {
    type Row = WorldStateUIRow;

    const FILENAME: &'static str = "WorldStateUI.dbc";
    const FIELD_COUNT: usize = 63;
    const ROW_SIZE: usize = 252;

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

            // id: primary_key (WorldStateUI) int32
            let id = WorldStateUIKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // area_id: foreign_key (AreaTable) int32
            let area_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // phase_shift: int32
            let phase_shift = crate::util::read_i32_le(chunk)?;

            // icon: string_ref
            let icon = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // string_lang: string_ref_loc (Extended)
            let string_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // tooltip_lang: string_ref_loc (Extended)
            let tooltip_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

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
                phase_shift,
                icon,
                string_lang,
                tooltip_lang,
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

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (WorldStateUI) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // area_id: foreign_key (AreaTable) int32
            b.write_all(&(row.area_id.id as i32).to_le_bytes())?;

            // phase_shift: int32
            b.write_all(&row.phase_shift.to_le_bytes())?;

            // icon: string_ref
            b.write_all(&string_cache.add_string(&row.icon).to_le_bytes())?;

            // string_lang: string_ref_loc (Extended)
            b.write_all(&row.string_lang.string_indices_as_array(&mut string_cache))?;

            // tooltip_lang: string_ref_loc (Extended)
            b.write_all(&row.tooltip_lang.string_indices_as_array(&mut string_cache))?;

            // state_variable: int32
            b.write_all(&row.state_variable.to_le_bytes())?;

            // ty: int32
            b.write_all(&row.ty.to_le_bytes())?;

            // dynamic_icon: string_ref
            b.write_all(&string_cache.add_string(&row.dynamic_icon).to_le_bytes())?;

            // dynamic_tooltip_lang: string_ref_loc (Extended)
            b.write_all(&row.dynamic_tooltip_lang.string_indices_as_array(&mut string_cache))?;

            // extended_u_i: string_ref
            b.write_all(&string_cache.add_string(&row.extended_u_i).to_le_bytes())?;

            // extended_u_i_state_variable: int32[3]
            for i in row.extended_u_i_state_variable {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldStateUIKey {
    pub id: i32
}

impl WorldStateUIKey {
    pub const fn new(id: i32) -> Self {
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

impl TryFrom<u32> for WorldStateUIKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for WorldStateUIKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for WorldStateUIKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for WorldStateUIKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for WorldStateUIKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldStateUIRow {
    pub id: WorldStateUIKey,
    pub map_id: MapKey,
    pub area_id: AreaTableKey,
    pub phase_shift: i32,
    pub icon: String,
    pub string_lang: ExtendedLocalizedString,
    pub tooltip_lang: ExtendedLocalizedString,
    pub state_variable: i32,
    pub ty: i32,
    pub dynamic_icon: String,
    pub dynamic_tooltip_lang: ExtendedLocalizedString,
    pub extended_u_i: String,
    pub extended_u_i_state_variable: [i32; 3],
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn world_state_ui() {
        let mut file = File::open("../wrath-dbc/WorldStateUI.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WorldStateUI::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldStateUI::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
