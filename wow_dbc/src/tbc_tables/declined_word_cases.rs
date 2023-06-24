use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::declined_word::DeclinedWordKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclinedWordCases {
    pub rows: Vec<DeclinedWordCasesRow>,
}

impl DbcTable for DeclinedWordCases {
    type Row = DeclinedWordCasesRow;

    const FILENAME: &'static str = "DeclinedWordCases.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 16,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 4 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 4,
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

            // id: primary_key (DeclinedWordCases) int32
            let id = DeclinedWordCasesKey::new(crate::util::read_i32_le(chunk)?);

            // declined_word_id: foreign_key (DeclinedWord) int32
            let declined_word_id = DeclinedWordKey::new(crate::util::read_i32_le(chunk)?.into());

            // case_index: int32
            let case_index = crate::util::read_i32_le(chunk)?;

            // declined_word: string_ref
            let declined_word = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(DeclinedWordCasesRow {
                id,
                declined_word_id,
                case_index,
                declined_word,
            });
        }

        Ok(DeclinedWordCases { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 4,
            record_size: 16,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (DeclinedWordCases) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // declined_word_id: foreign_key (DeclinedWord) int32
            b.write_all(&(row.declined_word_id.id as i32).to_le_bytes())?;

            // case_index: int32
            b.write_all(&row.case_index.to_le_bytes())?;

            // declined_word: string_ref
            if !row.declined_word.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.declined_word.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for DeclinedWordCases {
    type PrimaryKey = DeclinedWordCasesKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl DeclinedWordCases {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.declined_word.is_empty() { b.write_all(row.declined_word.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.declined_word.is_empty() { sum += row.declined_word.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct DeclinedWordCasesKey {
    pub id: i32
}

impl DeclinedWordCasesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for DeclinedWordCasesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for DeclinedWordCasesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for DeclinedWordCasesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for DeclinedWordCasesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for DeclinedWordCasesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for DeclinedWordCasesKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for DeclinedWordCasesKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for DeclinedWordCasesKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for DeclinedWordCasesKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for DeclinedWordCasesKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclinedWordCasesRow {
    pub id: DeclinedWordCasesKey,
    pub declined_word_id: DeclinedWordKey,
    pub case_index: i32,
    pub declined_word: String,
}

