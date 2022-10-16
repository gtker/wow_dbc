use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;
use crate::vanilla_tables::skill_line::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemSet {
    pub rows: Vec<ItemSetRow>,
}

impl DbcTable for ItemSet {
    type Row = ItemSetRow;

    fn filename() -> &'static str { "ItemSet.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 180 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 180,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 45 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 45,
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

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 45,
            record_size: 180,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ItemSet) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

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

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ItemSet {
    type PrimaryKey = ItemSetKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ItemSet {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ItemSetKey {
    pub id: i32
}

impl ItemSetKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

