use crate::header::{HEADER_SIZE, DbcHeader, parse_header};
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::ExtendedLocalizedString;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Achievement_Category {
    pub rows: Vec<Achievement_CategoryRow>,
}

impl DbcTable for Achievement_Category {
    type Row = Achievement_CategoryRow;

    fn filename() -> &'static str { "Achievement_Category.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 80 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 80,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 20 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 20,
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

            // id: primary_key (Achievement_Category) int32
            let id = Achievement_CategoryKey::new(crate::util::read_i32_le(chunk)?);

            // parent: foreign_key (Achievement_Category) int32
            let parent = Achievement_CategoryKey::new(crate::util::read_i32_le(chunk)?.into());

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // ui_order: int32
            let ui_order = crate::util::read_i32_le(chunk)?;


            rows.push(Achievement_CategoryRow {
                id,
                parent,
                name_lang,
                ui_order,
            });
        }

        Ok(Achievement_Category { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 20,
            record_size: 80,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Achievement_Category) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // parent: foreign_key (Achievement_Category) int32
            b.write_all(&(row.parent.id as i32).to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // ui_order: int32
            b.write_all(&row.ui_order.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Achievement_Category {
    type PrimaryKey = Achievement_CategoryKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl Achievement_Category {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct Achievement_CategoryKey {
    pub id: i32
}

impl Achievement_CategoryKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for Achievement_CategoryKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for Achievement_CategoryKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for Achievement_CategoryKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for Achievement_CategoryKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for Achievement_CategoryKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Achievement_CategoryRow {
    pub id: Achievement_CategoryKey,
    pub parent: Achievement_CategoryKey,
    pub name_lang: ExtendedLocalizedString,
    pub ui_order: i32,
}

