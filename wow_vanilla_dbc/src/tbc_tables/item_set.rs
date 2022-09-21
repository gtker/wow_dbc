use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;
use crate::tbc_tables::skill_line::*;

#[derive(Debug, Clone, PartialEq)]
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

            // name_lang: string_ref_loc
            let name_lang = crate::util::read_localized_string(chunk, &string_block)?;

            // item_id: int32[17]
            let item_id = crate::util::read_array_i32::<17>(chunk)?;

            // set_spell_id: int32[8]
            let set_spell_id = crate::util::read_array_i32::<8>(chunk)?;

            // set_threshold: int32[8]
            let set_threshold = crate::util::read_array_i32::<8>(chunk)?;

            // required_skill: foreign_key (SkillLine) int32
            let required_skill = SkillLineKey::new(crate::util::read_i32_le(chunk)?.into());

            // required_skill_rank: int32
            let required_skill_rank = crate::util::read_i32_le(chunk)?;


            rows.push(ItemSetRow {
                id,
                name_lang,
                item_id,
                set_spell_id,
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

            // name_lang: string_ref_loc
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // item_id: int32[17]
            for i in row.item_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // set_spell_id: int32[8]
            for i in row.set_spell_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // set_threshold: int32[8]
            for i in row.set_threshold {
                b.write_all(&i.to_le_bytes())?;
            }


            // required_skill: foreign_key (SkillLine) int32
            b.write_all(&(row.required_skill.id as i32).to_le_bytes())?;

            // required_skill_rank: int32
            b.write_all(&row.required_skill_rank.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ItemSet {
    type PrimaryKey = ItemSetKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ItemSet {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemSetKey {
    pub id: i32
}

impl ItemSetKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemSetRow {
    pub id: ItemSetKey,
    pub name_lang: LocalizedString,
    pub item_id: [i32; 17],
    pub set_spell_id: [i32; 8],
    pub set_threshold: [i32; 8],
    pub required_skill: SkillLineKey,
    pub required_skill_rank: i32,
}

