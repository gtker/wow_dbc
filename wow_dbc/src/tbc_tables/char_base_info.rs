use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::tbc_tables::chr_classes::*;
use crate::tbc_tables::chr_races::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharBaseInfo {
    pub rows: Vec<CharBaseInfoRow>,
}

impl DbcTable for CharBaseInfo {
    type Row = CharBaseInfoRow;

    fn filename() -> &'static str { "CharBaseInfo.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 2 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 2,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 2 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 2,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // race_id: foreign_key (ChrRaces) int8
            let race_id = ChrRacesKey::new(crate::util::read_i8_le(chunk)?.into());

            // class_id: foreign_key (ChrClasses) int8
            let class_id = ChrClassesKey::new(crate::util::read_i8_le(chunk)?.into());


            rows.push(CharBaseInfoRow {
                race_id,
                class_id,
            });
        }

        Ok(CharBaseInfo { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 2,
            record_size: 2,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // race_id: foreign_key (ChrRaces) int8
            b.write_all(&(row.race_id.id as i8).to_le_bytes())?;

            // class_id: foreign_key (ChrClasses) int8
            b.write_all(&(row.class_id.id as i8).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCharBaseInfo<const S: usize> {
    pub rows: [CharBaseInfoRow; S],
}

impl<const S: usize> ConstCharBaseInfo<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 2 {
            panic!("invalid record size, expected 2")
        }

        if header.field_count != 2 {
            panic!("invalid field count, expected 2")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            CharBaseInfoRow {
                race_id: ChrRacesKey::new(0),
                class_id: ChrClassesKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // race_id: foreign_key (ChrRaces) int8
            let race_id = ChrRacesKey::new(i8::from_le_bytes([b[b_offset + 0]])as i32);
            b_offset += 1;

            // class_id: foreign_key (ChrClasses) int8
            let class_id = ChrClassesKey::new(i8::from_le_bytes([b[b_offset + 0]])as i32);
            b_offset += 1;

            rows[i] = CharBaseInfoRow {
                race_id,
                class_id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CharBaseInfo {
        CharBaseInfo {
            rows: self.rows.iter().map(|s| CharBaseInfoRow {
                race_id: s.race_id,
                class_id: s.class_id,
            }).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharBaseInfoRow {
    pub race_id: ChrRacesKey,
    pub class_id: ChrClassesKey,
}

