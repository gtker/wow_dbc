use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharTitles {
    pub rows: Vec<CharTitlesRow>,
}

impl DbcTable for CharTitles {
    type Row = CharTitlesRow;

    fn filename() -> &'static str { "CharTitles.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 148 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 148,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 37 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 37,
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

            // id: primary_key (CharTitles) int32
            let id = CharTitlesKey::new(crate::util::read_i32_le(chunk)?);

            // condition_id: int32
            let condition_id = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // name1_lang: string_ref_loc (Extended)
            let name1_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // mask_id: int32
            let mask_id = crate::util::read_i32_le(chunk)?;


            rows.push(CharTitlesRow {
                id,
                condition_id,
                name_lang,
                name1_lang,
                mask_id,
            });
        }

        Ok(CharTitles { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 37,
            record_size: 148,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (CharTitles) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // condition_id: int32
            b.write_all(&row.condition_id.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // name1_lang: string_ref_loc (Extended)
            b.write_all(&row.name1_lang.string_indices_as_array(&mut string_index))?;

            // mask_id: int32
            b.write_all(&row.mask_id.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for CharTitles {
    type PrimaryKey = CharTitlesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl CharTitles {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
            row.name1_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
            sum += row.name1_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCharTitles<const S: usize> {
    pub rows: [ConstCharTitlesRow; S],
}

impl<const S: usize> ConstCharTitles<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 148 {
            panic!("invalid record size, expected 148")
        }

        if header.field_count != 37 {
            panic!("invalid field count, expected 37")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstCharTitlesRow {
                id: CharTitlesKey::new(0),
                condition_id: 0,
                name_lang: crate::ConstExtendedLocalizedString::empty(),
                name1_lang: crate::ConstExtendedLocalizedString::empty(),
                mask_id: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (CharTitles) int32
            let id = CharTitlesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // condition_id: int32
            let condition_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
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

            // name1_lang: string_ref_loc (Extended)
            let name1_lang = ConstExtendedLocalizedString::new(
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

            // mask_id: int32
            let mask_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstCharTitlesRow {
                id,
                condition_id,
                name_lang,
                name1_lang,
                mask_id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CharTitles {
        CharTitles {
            rows: self.rows.iter().map(|s| CharTitlesRow {
                id: s.id,
                condition_id: s.condition_id,
                name_lang: s.name_lang.to_string(),
                name1_lang: s.name1_lang.to_string(),
                mask_id: s.mask_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CharTitlesKey {
    pub id: i32
}

impl CharTitlesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for CharTitlesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for CharTitlesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for CharTitlesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for CharTitlesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for CharTitlesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharTitlesRow {
    pub id: CharTitlesKey,
    pub condition_id: i32,
    pub name_lang: ExtendedLocalizedString,
    pub name1_lang: ExtendedLocalizedString,
    pub mask_id: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCharTitlesRow {
    pub id: CharTitlesKey,
    pub condition_id: i32,
    pub name_lang: ConstExtendedLocalizedString,
    pub name1_lang: ConstExtendedLocalizedString,
    pub mask_id: i32,
}

