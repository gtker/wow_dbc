use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cfg_Categories {
    pub rows: Vec<Cfg_CategoriesRow>,
}

impl DbcTable for Cfg_Categories {
    type Row = Cfg_CategoriesRow;

    fn filename() -> &'static str { "Cfg_Categories.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 84 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 84,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 21 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 21,
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

            // id: primary_key (Cfg_Categories) int32
            let id = Cfg_CategoriesKey::new(crate::util::read_i32_le(chunk)?);

            // locale_mask: int32
            let locale_mask = crate::util::read_i32_le(chunk)?;

            // create_charset_mask: int32
            let create_charset_mask = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(Cfg_CategoriesRow {
                id,
                locale_mask,
                create_charset_mask,
                flags,
                name_lang,
            });
        }

        Ok(Cfg_Categories { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 21,
            record_size: 84,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Cfg_Categories) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // locale_mask: int32
            b.write_all(&row.locale_mask.to_le_bytes())?;

            // create_charset_mask: int32
            b.write_all(&row.create_charset_mask.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Cfg_Categories {
    type PrimaryKey = Cfg_CategoriesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl Cfg_Categories {
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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCfg_Categories<const S: usize> {
    pub rows: [ConstCfg_CategoriesRow; S],
}

impl<const S: usize> ConstCfg_Categories<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 84 {
            panic!("invalid record size, expected 84")
        }

        if header.field_count != 21 {
            panic!("invalid field count, expected 21")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstCfg_CategoriesRow {
                id: Cfg_CategoriesKey::new(0),
                locale_mask: 0,
                create_charset_mask: 0,
                flags: 0,
                name_lang: crate::ConstExtendedLocalizedString::empty(),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Cfg_Categories) int32
            let id = Cfg_CategoriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // locale_mask: int32
            let locale_mask = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // create_charset_mask: int32
            let create_charset_mask = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
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

            rows[i] = ConstCfg_CategoriesRow {
                id,
                locale_mask,
                create_charset_mask,
                flags,
                name_lang,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> Cfg_Categories {
        Cfg_Categories {
            rows: self.rows.iter().map(|s| Cfg_CategoriesRow {
                id: s.id,
                locale_mask: s.locale_mask,
                create_charset_mask: s.create_charset_mask,
                flags: s.flags,
                name_lang: s.name_lang.to_string(),
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct Cfg_CategoriesKey {
    pub id: i32
}

impl Cfg_CategoriesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for Cfg_CategoriesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for Cfg_CategoriesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for Cfg_CategoriesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for Cfg_CategoriesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for Cfg_CategoriesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cfg_CategoriesRow {
    pub id: Cfg_CategoriesKey,
    pub locale_mask: i32,
    pub create_charset_mask: i32,
    pub flags: i32,
    pub name_lang: ExtendedLocalizedString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCfg_CategoriesRow {
    pub id: Cfg_CategoriesKey,
    pub locale_mask: i32,
    pub create_charset_mask: i32,
    pub flags: i32,
    pub name_lang: ConstExtendedLocalizedString,
}

