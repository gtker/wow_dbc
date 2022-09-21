use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::vanilla_tables::chr_classes::*;
use crate::vanilla_tables::chr_races::*;

#[derive(Debug, Clone, PartialEq, Eq)]
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

            // race: foreign_key (ChrRaces) uint8
            let race = ChrRacesKey::new(crate::util::read_u8_le(chunk)?.into());

            // class: foreign_key (ChrClasses) uint8
            let class = ChrClassesKey::new(crate::util::read_u8_le(chunk)?.into());


            rows.push(CharBaseInfoRow {
                race,
                class,
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
            // race: foreign_key (ChrRaces) uint8
            b.write_all(&(row.race.id as u8).to_le_bytes())?;

            // class: foreign_key (ChrClasses) uint8
            b.write_all(&(row.class.id as u8).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharBaseInfoRow {
    pub race: ChrRacesKey,
    pub class: ChrClassesKey,
}

