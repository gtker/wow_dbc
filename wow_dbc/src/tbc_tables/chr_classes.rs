use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChrClasses {
    pub rows: Vec<ChrClassesRow>,
}

impl DbcTable for ChrClasses {
    type Row = ChrClassesRow;

    const FILENAME: &'static str = "ChrClasses.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 232 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 232,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 58 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 58,
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

            // id: primary_key (ChrClasses) int32
            let id = ChrClassesKey::new(crate::util::read_i32_le(chunk)?);

            // damage_bonus_stat: int32
            let damage_bonus_stat = crate::util::read_i32_le(chunk)?;

            // display_power: foreign_key (PowerType) int32
            let display_power = crate::util::read_i32_le(chunk)?;

            // pet_name_token: string_ref
            let pet_name_token = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // name_female_lang: string_ref_loc (Extended)
            let name_female_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // name_male_lang: string_ref_loc (Extended)
            let name_male_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // filename: string_ref
            let filename = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // spell_class_set: int32
            let spell_class_set = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(ChrClassesRow {
                id,
                damage_bonus_stat,
                display_power,
                pet_name_token,
                name_lang,
                name_female_lang,
                name_male_lang,
                filename,
                spell_class_set,
                flags,
            });
        }

        Ok(ChrClasses { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 58,
            record_size: 232,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ChrClasses) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // damage_bonus_stat: int32
            b.write_all(&row.damage_bonus_stat.to_le_bytes())?;

            // display_power: foreign_key (PowerType) int32
            b.write_all(&row.display_power.to_le_bytes())?;

            // pet_name_token: string_ref
            if !row.pet_name_token.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.pet_name_token.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // name_female_lang: string_ref_loc (Extended)
            b.write_all(&row.name_female_lang.string_indices_as_array(&mut string_index))?;

            // name_male_lang: string_ref_loc (Extended)
            b.write_all(&row.name_male_lang.string_indices_as_array(&mut string_index))?;

            // filename: string_ref
            if !row.filename.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.filename.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // spell_class_set: int32
            b.write_all(&row.spell_class_set.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ChrClasses {
    type PrimaryKey = ChrClassesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ChrClasses {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.pet_name_token.is_empty() { b.write_all(row.pet_name_token.as_bytes())?; b.write_all(&[0])?; };
            row.name_lang.string_block_as_array(b)?;
            row.name_female_lang.string_block_as_array(b)?;
            row.name_male_lang.string_block_as_array(b)?;
            if !row.filename.is_empty() { b.write_all(row.filename.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.pet_name_token.is_empty() { sum += row.pet_name_token.len() + 1; };
            sum += row.name_lang.string_block_size();
            sum += row.name_female_lang.string_block_size();
            sum += row.name_male_lang.string_block_size();
            if !row.filename.is_empty() { sum += row.filename.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ChrClassesKey {
    pub id: i32
}

impl ChrClassesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for ChrClassesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for ChrClassesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for ChrClassesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for ChrClassesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ChrClassesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChrClassesRow {
    pub id: ChrClassesKey,
    pub damage_bonus_stat: i32,
    pub display_power: i32,
    pub pet_name_token: String,
    pub name_lang: ExtendedLocalizedString,
    pub name_female_lang: ExtendedLocalizedString,
    pub name_male_lang: ExtendedLocalizedString,
    pub filename: String,
    pub spell_class_set: i32,
    pub flags: i32,
}

