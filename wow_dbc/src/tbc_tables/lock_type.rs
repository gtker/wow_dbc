use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::ExtendedLocalizedString;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LockType {
    pub rows: Vec<LockTypeRow>,
}

impl DbcTable for LockType {
    type Row = LockTypeRow;

    fn filename() -> &'static str { "LockType.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 212 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 212,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 53 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 53,
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

            // id: primary_key (LockType) int32
            let id = LockTypeKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // resource_name_lang: string_ref_loc (Extended)
            let resource_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // verb_lang: string_ref_loc (Extended)
            let verb_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // cursor_name: string_ref
            let cursor_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(LockTypeRow {
                id,
                name_lang,
                resource_name_lang,
                verb_lang,
                cursor_name,
            });
        }

        Ok(LockType { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 53,
            record_size: 212,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (LockType) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // resource_name_lang: string_ref_loc (Extended)
            b.write_all(&row.resource_name_lang.string_indices_as_array(&mut string_index))?;

            // verb_lang: string_ref_loc (Extended)
            b.write_all(&row.verb_lang.string_indices_as_array(&mut string_index))?;

            // cursor_name: string_ref
            if !row.cursor_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.cursor_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for LockType {
    type PrimaryKey = LockTypeKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl LockType {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
            row.resource_name_lang.string_block_as_array(b)?;
            row.verb_lang.string_block_as_array(b)?;
            if !row.cursor_name.is_empty() { b.write_all(row.cursor_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
            sum += row.resource_name_lang.string_block_size();
            sum += row.verb_lang.string_block_size();
            if !row.cursor_name.is_empty() { sum += row.cursor_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct LockTypeKey {
    pub id: i32
}

impl LockTypeKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i32> for LockTypeKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LockTypeRow {
    pub id: LockTypeKey,
    pub name_lang: ExtendedLocalizedString,
    pub resource_name_lang: ExtendedLocalizedString,
    pub verb_lang: ExtendedLocalizedString,
    pub cursor_name: String,
}

