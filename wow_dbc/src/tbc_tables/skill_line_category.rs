use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillLineCategory {
    pub rows: Vec<SkillLineCategoryRow>,
}

impl DbcTable for SkillLineCategory {
    type Row = SkillLineCategoryRow;

    fn filename() -> &'static str { "SkillLineCategory.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 76 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 76,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 19 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 19,
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

            // id: primary_key (SkillLineCategory) int32
            let id = SkillLineCategoryKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // sort_index: int32
            let sort_index = crate::util::read_i32_le(chunk)?;


            rows.push(SkillLineCategoryRow {
                id,
                name_lang,
                sort_index,
            });
        }

        Ok(SkillLineCategory { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 19,
            record_size: 76,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SkillLineCategory) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // sort_index: int32
            b.write_all(&row.sort_index.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SkillLineCategory {
    type PrimaryKey = SkillLineCategoryKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl SkillLineCategory {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstSkillLineCategory<const S: usize> {
    pub rows: [ConstSkillLineCategoryRow; S],
}

impl<const S: usize> ConstSkillLineCategory<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 76 {
            panic!("invalid record size, expected 76")
        }

        if header.field_count != 19 {
            panic!("invalid field count, expected 19")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstSkillLineCategoryRow {
                id: SkillLineCategoryKey::new(0),
                name_lang: crate::ConstExtendedLocalizedString::empty(),
                sort_index: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SkillLineCategory) int32
            let id = SkillLineCategoryKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
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

            // sort_index: int32
            let sort_index = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstSkillLineCategoryRow {
                id,
                name_lang,
                sort_index,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SkillLineCategory {
        SkillLineCategory {
            rows: self.rows.iter().map(|s| SkillLineCategoryRow {
                id: s.id,
                name_lang: s.name_lang.to_string(),
                sort_index: s.sort_index,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SkillLineCategoryKey {
    pub id: i32
}

impl SkillLineCategoryKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SkillLineCategoryKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SkillLineCategoryKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SkillLineCategoryKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SkillLineCategoryKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SkillLineCategoryKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillLineCategoryRow {
    pub id: SkillLineCategoryKey,
    pub name_lang: ExtendedLocalizedString,
    pub sort_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstSkillLineCategoryRow {
    pub id: SkillLineCategoryKey,
    pub name_lang: ConstExtendedLocalizedString,
    pub sort_index: i32,
}

