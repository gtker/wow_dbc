use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::skill_line_category::SkillLineCategoryKey;
use crate::wrath_tables::spell_icon::SpellIconKey;
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

        if header.record_size != 224 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 224,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 56 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 56,
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

            // id: primary_key (SkillLine) int32
            let id = SkillLineKey::new(crate::util::read_i32_le(chunk)?);

            // category_id: foreign_key (SkillLineCategory) int32
            let category_id = SkillLineCategoryKey::new(crate::util::read_i32_le(chunk)?.into());

            // skill_costs_id: int32
            let skill_costs_id = crate::util::read_i32_le(chunk)?;

            // display_name_lang: string_ref_loc (Extended)
            let display_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // spell_icon_id: foreign_key (SpellIcon) int32
            let spell_icon_id = SpellIconKey::new(crate::util::read_i32_le(chunk)?.into());

            // alternate_verb_lang: string_ref_loc (Extended)
            let alternate_verb_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // can_link: int32
            let can_link = crate::util::read_i32_le(chunk)?;


            rows.push(SkillLineRow {
                id,
                category_id,
                skill_costs_id,
                display_name_lang,
                description_lang,
                spell_icon_id,
                alternate_verb_lang,
                can_link,
            });
        }

        Ok(SkillLine { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 56,
            record_size: 224,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SkillLine) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // category_id: foreign_key (SkillLineCategory) int32
            b.write_all(&(row.category_id.id as i32).to_le_bytes())?;

            // skill_costs_id: int32
            b.write_all(&row.skill_costs_id.to_le_bytes())?;

            // display_name_lang: string_ref_loc (Extended)
            b.write_all(&row.display_name_lang.string_indices_as_array(&mut string_index))?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_index))?;

            // spell_icon_id: foreign_key (SpellIcon) int32
            b.write_all(&(row.spell_icon_id.id as i32).to_le_bytes())?;

            // alternate_verb_lang: string_ref_loc (Extended)
            b.write_all(&row.alternate_verb_lang.string_indices_as_array(&mut string_index))?;

            // can_link: int32
            b.write_all(&row.can_link.to_le_bytes())?;

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
            row.display_name_lang.string_block_as_array(b)?;
            row.description_lang.string_block_as_array(b)?;
            row.alternate_verb_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.display_name_lang.string_block_size();
            sum += row.description_lang.string_block_size();
            sum += row.alternate_verb_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SkillLineKey {
    pub id: i32
}

impl SkillLineKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SkillLineKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SkillLineKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SkillLineKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillLineRow {
    pub id: SkillLineKey,
    pub category_id: SkillLineCategoryKey,
    pub skill_costs_id: i32,
    pub display_name_lang: ExtendedLocalizedString,
    pub description_lang: ExtendedLocalizedString,
    pub spell_icon_id: SpellIconKey,
    pub alternate_verb_lang: ExtendedLocalizedString,
    pub can_link: i32,
}

