use crate::header::{HEADER_SIZE, DbcHeader, parse_header};
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::ExtendedLocalizedString;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CreatureFamily {
    pub rows: Vec<CreatureFamilyRow>,
}

impl DbcTable for CreatureFamily {
    type Row = CreatureFamilyRow;

    fn filename() -> &'static str { "CreatureFamily.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 104 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 104,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 26 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 26,
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

            // id: primary_key (CreatureFamily) int32
            let id = CreatureFamilyKey::new(crate::util::read_i32_le(chunk)?);

            // min_scale: float
            let min_scale = crate::util::read_f32_le(chunk)?;

            // min_scale_level: int32
            let min_scale_level = crate::util::read_i32_le(chunk)?;

            // max_scale: float
            let max_scale = crate::util::read_f32_le(chunk)?;

            // max_scale_level: int32
            let max_scale_level = crate::util::read_i32_le(chunk)?;

            // skill_line: int32[2]
            let skill_line = crate::util::read_array_i32::<2>(chunk)?;

            // pet_food_mask: int32
            let pet_food_mask = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // icon_file: string_ref
            let icon_file = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(CreatureFamilyRow {
                id,
                min_scale,
                min_scale_level,
                max_scale,
                max_scale_level,
                skill_line,
                pet_food_mask,
                name_lang,
                icon_file,
            });
        }

        Ok(CreatureFamily { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 26,
            record_size: 104,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (CreatureFamily) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // min_scale: float
            b.write_all(&row.min_scale.to_le_bytes())?;

            // min_scale_level: int32
            b.write_all(&row.min_scale_level.to_le_bytes())?;

            // max_scale: float
            b.write_all(&row.max_scale.to_le_bytes())?;

            // max_scale_level: int32
            b.write_all(&row.max_scale_level.to_le_bytes())?;

            // skill_line: int32[2]
            for i in row.skill_line {
                b.write_all(&i.to_le_bytes())?;
            }


            // pet_food_mask: int32
            b.write_all(&row.pet_food_mask.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // icon_file: string_ref
            if !row.icon_file.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.icon_file.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for CreatureFamily {
    type PrimaryKey = CreatureFamilyKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl CreatureFamily {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
            if !row.icon_file.is_empty() { b.write_all(row.icon_file.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
            if !row.icon_file.is_empty() { sum += row.icon_file.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CreatureFamilyKey {
    pub id: i32
}

impl CreatureFamilyKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for CreatureFamilyKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for CreatureFamilyKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for CreatureFamilyKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for CreatureFamilyKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for CreatureFamilyKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CreatureFamilyRow {
    pub id: CreatureFamilyKey,
    pub min_scale: f32,
    pub min_scale_level: i32,
    pub max_scale: f32,
    pub max_scale_level: i32,
    pub skill_line: [i32; 2],
    pub pet_food_mask: i32,
    pub name_lang: ExtendedLocalizedString,
    pub icon_file: String,
}

