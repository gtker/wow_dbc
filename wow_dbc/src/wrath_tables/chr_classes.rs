use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};
use crate::wrath_tables::cinematic_sequences::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChrClasses {
    pub rows: Vec<ChrClassesRow>,
}

impl DbcTable for ChrClasses {
    type Row = ChrClassesRow;

    fn filename() -> &'static str { "ChrClasses.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 240 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 240,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 60 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 60,
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

            // cinematic_sequence_id: foreign_key (CinematicSequences) int32
            let cinematic_sequence_id = CinematicSequencesKey::new(crate::util::read_i32_le(chunk)?.into());

            // required_expansion: int32
            let required_expansion = crate::util::read_i32_le(chunk)?;


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
                cinematic_sequence_id,
                required_expansion,
            });
        }

        Ok(ChrClasses { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 60,
            record_size: 240,
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

            // cinematic_sequence_id: foreign_key (CinematicSequences) int32
            b.write_all(&(row.cinematic_sequence_id.id as i32).to_le_bytes())?;

            // required_expansion: int32
            b.write_all(&row.required_expansion.to_le_bytes())?;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstChrClasses<const S: usize> {
    pub rows: [ConstChrClassesRow; S],
}

impl<const S: usize> ConstChrClasses<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 240 {
            panic!("invalid record size, expected 240")
        }

        if header.field_count != 60 {
            panic!("invalid field count, expected 60")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstChrClassesRow {
                id: ChrClassesKey::new(0),
                damage_bonus_stat: 0,
                display_power: 0,
                pet_name_token: "",
                name_lang: crate::ConstExtendedLocalizedString::empty(),
                name_female_lang: crate::ConstExtendedLocalizedString::empty(),
                name_male_lang: crate::ConstExtendedLocalizedString::empty(),
                filename: "",
                spell_class_set: 0,
                flags: 0,
                cinematic_sequence_id: CinematicSequencesKey::new(0),
                required_expansion: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (ChrClasses) int32
            let id = ChrClassesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // damage_bonus_stat: int32
            let damage_bonus_stat = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // display_power: foreign_key (PowerType) int32
            let display_power = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // pet_name_token: string_ref
            let pet_name_token = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // name_lang: string_ref_loc (Extended)
            let name_lang = ConstExtendedLocalizedString::new(
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

            // name_female_lang: string_ref_loc (Extended)
            let name_female_lang = ConstExtendedLocalizedString::new(
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

            // name_male_lang: string_ref_loc (Extended)
            let name_male_lang = ConstExtendedLocalizedString::new(
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

            // filename: string_ref
            let filename = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // spell_class_set: int32
            let spell_class_set = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // cinematic_sequence_id: foreign_key (CinematicSequences) int32
            let cinematic_sequence_id = CinematicSequencesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // required_expansion: int32
            let required_expansion = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstChrClassesRow {
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
                cinematic_sequence_id,
                required_expansion,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> ChrClasses {
        ChrClasses {
            rows: self.rows.iter().map(|s| ChrClassesRow {
                id: s.id,
                damage_bonus_stat: s.damage_bonus_stat,
                display_power: s.display_power,
                pet_name_token: s.pet_name_token.to_string(),
                name_lang: s.name_lang.to_string(),
                name_female_lang: s.name_female_lang.to_string(),
                name_male_lang: s.name_male_lang.to_string(),
                filename: s.filename.to_string(),
                spell_class_set: s.spell_class_set,
                flags: s.flags,
                cinematic_sequence_id: s.cinematic_sequence_id,
                required_expansion: s.required_expansion,
            }).collect(),
        }
    }
    // TODO: Indexable?
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
    pub cinematic_sequence_id: CinematicSequencesKey,
    pub required_expansion: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstChrClassesRow {
    pub id: ChrClassesKey,
    pub damage_bonus_stat: i32,
    pub display_power: i32,
    pub pet_name_token: &'static str,
    pub name_lang: ConstExtendedLocalizedString,
    pub name_female_lang: ConstExtendedLocalizedString,
    pub name_male_lang: ConstExtendedLocalizedString,
    pub filename: &'static str,
    pub spell_class_set: i32,
    pub flags: i32,
    pub cinematic_sequence_id: CinematicSequencesKey,
    pub required_expansion: i32,
}

