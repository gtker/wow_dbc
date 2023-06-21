use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::skill_costs_data::SkillCostsDataKey;
use crate::vanilla_tables::skill_line_category::SkillLineCategoryKey;
use crate::vanilla_tables::spell_icon::SpellIconKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillLine {
    pub rows: Vec<SkillLineRow>,
}

impl DbcTable for SkillLine {
    type Row = SkillLineRow;

    fn filename() -> &'static str { "SkillLine.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 88 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 88,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 22 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 22,
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

            // id: primary_key (SkillLine) uint32
            let id = SkillLineKey::new(crate::util::read_u32_le(chunk)?);

            // category: foreign_key (SkillLineCategory) uint32
            let category = SkillLineCategoryKey::new(crate::util::read_u32_le(chunk)?.into());

            // skill_costs: foreign_key (SkillCostsData) uint32
            let skill_costs = SkillCostsDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // display_name: string_ref_loc
            let display_name = crate::util::read_localized_string(chunk, &string_block)?;

            // description: string_ref_loc
            let description = crate::util::read_localized_string(chunk, &string_block)?;

            // spell_icon: foreign_key (SpellIcon) uint32
            let spell_icon = SpellIconKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(SkillLineRow {
                id,
                category,
                skill_costs,
                display_name,
                description,
                spell_icon,
            });
        }

        Ok(SkillLine { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 22,
            record_size: 88,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SkillLine) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // category: foreign_key (SkillLineCategory) uint32
            b.write_all(&(row.category.id as u32).to_le_bytes())?;

            // skill_costs: foreign_key (SkillCostsData) uint32
            b.write_all(&(row.skill_costs.id as u32).to_le_bytes())?;

            // display_name: string_ref_loc
            b.write_all(&row.display_name.string_indices_as_array(&mut string_index))?;

            // description: string_ref_loc
            b.write_all(&row.description.string_indices_as_array(&mut string_index))?;

            // spell_icon: foreign_key (SpellIcon) uint32
            b.write_all(&(row.spell_icon.id as u32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SkillLine {
    type PrimaryKey = SkillLineKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl SkillLine {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.display_name.string_block_as_array(b)?;
            row.description.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.display_name.string_block_size();
            sum += row.description.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SkillLineKey {
    pub id: u32
}

impl SkillLineKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SkillLineKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SkillLineKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SkillLineKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillLineRow {
    pub id: SkillLineKey,
    pub category: SkillLineCategoryKey,
    pub skill_costs: SkillCostsDataKey,
    pub display_name: LocalizedString,
    pub description: LocalizedString,
    pub spell_icon: SpellIconKey,
}

