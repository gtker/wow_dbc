use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmotesTextData {
    pub rows: Vec<EmotesTextDataRow>,
}

impl DbcTable for EmotesTextData {
    type Row = EmotesTextDataRow;

    const FILENAME: &'static str = "EmotesTextData.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 72 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 72,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 18 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 18,
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

            // id: primary_key (EmotesTextData) int32
            let id = EmotesTextDataKey::new(crate::util::read_i32_le(chunk)?);

            // text_lang: string_ref_loc (Extended)
            let text_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(EmotesTextDataRow {
                id,
                text_lang,
            });
        }

        Ok(EmotesTextData { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 18,
            record_size: 72,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (EmotesTextData) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // text_lang: string_ref_loc (Extended)
            b.write_all(&row.text_lang.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for EmotesTextData {
    type PrimaryKey = EmotesTextDataKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl EmotesTextData {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.text_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.text_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct EmotesTextDataKey {
    pub id: i32
}

impl EmotesTextDataKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for EmotesTextDataKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for EmotesTextDataKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for EmotesTextDataKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl From<u8> for EmotesTextDataKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for EmotesTextDataKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl TryFrom<u32> for EmotesTextDataKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmotesTextDataRow {
    pub id: EmotesTextDataKey,
    pub text_lang: ExtendedLocalizedString,
}

