use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::vanilla_tables::skill_line::SkillLineKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemSet {
    pub rows: Vec<ItemSetRow>,
}

impl DbcTable for ItemSet {
    type Row = ItemSetRow;

    const FILENAME: &'static str = "ItemSet.dbc";
    const FIELD_COUNT: usize = 45;
    const ROW_SIZE: usize = 180;

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

            // id: primary_key (ItemSet) int32
            let id = ItemSetKey::new(crate::util::read_i32_le(chunk)?);

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // items: uint32[10]
            let items = crate::util::read_array_u32::<10>(chunk)?;

            // bank_item: uint32[7]
            let bank_item = crate::util::read_array_u32::<7>(chunk)?;

            // set_spell: uint32[8]
            let set_spell = crate::util::read_array_u32::<8>(chunk)?;

            // set_threshold: uint32[8]
            let set_threshold = crate::util::read_array_u32::<8>(chunk)?;

            // required_skill: foreign_key (SkillLine) uint32
            let required_skill = SkillLineKey::new(crate::util::read_u32_le(chunk)?.into());

            // required_skill_rank: uint32
            let required_skill_rank = crate::util::read_u32_le(chunk)?;


            rows.push(ItemSetRow {
                id,
                name,
                items,
                bank_item,
                set_spell,
                set_threshold,
                required_skill,
                required_skill_rank,
            });
        }

        Ok(ItemSet { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (ItemSet) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_cache))?;

            // items: uint32[10]
            for i in row.items {
                b.write_all(&i.to_le_bytes())?;
            }


            // bank_item: uint32[7]
            for i in row.bank_item {
                b.write_all(&i.to_le_bytes())?;
            }


            // set_spell: uint32[8]
            for i in row.set_spell {
                b.write_all(&i.to_le_bytes())?;
            }


            // set_threshold: uint32[8]
            for i in row.set_threshold {
                b.write_all(&i.to_le_bytes())?;
            }


            // required_skill: foreign_key (SkillLine) uint32
            b.write_all(&(row.required_skill.id as u32).to_le_bytes())?;

            // required_skill_rank: uint32
            b.write_all(&row.required_skill_rank.to_le_bytes())?;

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

impl Indexable for ItemSet {
    type PrimaryKey = ItemSetKey;
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
pub struct ItemSetKey {
    pub id: i32
}

impl ItemSetKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for ItemSetKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for ItemSetKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for ItemSetKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for ItemSetKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for ItemSetKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for ItemSetKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ItemSetKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for ItemSetKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ItemSetKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ItemSetKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemSetRow {
    pub id: ItemSetKey,
    pub name: LocalizedString,
    pub items: [u32; 10],
    pub bank_item: [u32; 7],
    pub set_spell: [u32; 8],
    pub set_threshold: [u32; 8],
    pub required_skill: SkillLineKey,
    pub required_skill_rank: u32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn item_set() {
        let mut file = File::open("../vanilla-dbc/ItemSet.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ItemSet::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ItemSet::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
