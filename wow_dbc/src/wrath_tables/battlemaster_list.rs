use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BattlemasterList {
    pub rows: Vec<BattlemasterListRow>,
}

impl DbcTable for BattlemasterList {
    type Row = BattlemasterListRow;

    const FILENAME: &'static str = "BattlemasterList.dbc";
    const FIELD_COUNT: usize = 32;
    const ROW_SIZE: usize = 128;

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

            // id: primary_key (BattlemasterList) int32
            let id = BattlemasterListKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: int32[8]
            let map_id = crate::util::read_array_i32::<8>(chunk)?;

            // instance_type: int32
            let instance_type = crate::util::read_i32_le(chunk)?;

            // groups_allowed: int32
            let groups_allowed = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // max_group_size: int32
            let max_group_size = crate::util::read_i32_le(chunk)?;

            // holiday_world_state: foreign_key (WorldState) int32
            let holiday_world_state = crate::util::read_i32_le(chunk)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;

            // max_level: int32
            let max_level = crate::util::read_i32_le(chunk)?;


            rows.push(BattlemasterListRow {
                id,
                map_id,
                instance_type,
                groups_allowed,
                name_lang,
                max_group_size,
                holiday_world_state,
                min_level,
                max_level,
            });
        }

        Ok(BattlemasterList { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (BattlemasterList) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: int32[8]
            for i in row.map_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // instance_type: int32
            b.write_all(&row.instance_type.to_le_bytes())?;

            // groups_allowed: int32
            b.write_all(&row.groups_allowed.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // max_group_size: int32
            b.write_all(&row.max_group_size.to_le_bytes())?;

            // holiday_world_state: foreign_key (WorldState) int32
            b.write_all(&row.holiday_world_state.to_le_bytes())?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

            // max_level: int32
            b.write_all(&row.max_level.to_le_bytes())?;

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

impl Indexable for BattlemasterList {
    type PrimaryKey = BattlemasterListKey;
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
pub struct BattlemasterListKey {
    pub id: i32
}

impl BattlemasterListKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for BattlemasterListKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for BattlemasterListKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for BattlemasterListKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for BattlemasterListKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for BattlemasterListKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for BattlemasterListKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for BattlemasterListKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for BattlemasterListKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for BattlemasterListKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for BattlemasterListKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BattlemasterListRow {
    pub id: BattlemasterListKey,
    pub map_id: [i32; 8],
    pub instance_type: i32,
    pub groups_allowed: i32,
    pub name_lang: ExtendedLocalizedString,
    pub max_group_size: i32,
    pub holiday_world_state: i32,
    pub min_level: i32,
    pub max_level: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn battlemaster_list() {
        let mut file = File::open("../wrath-dbc/BattlemasterList.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = BattlemasterList::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = BattlemasterList::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
