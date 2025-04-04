use crate::DbcTable;
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::chr_classes::ChrClassesKey;
use crate::tbc_tables::chr_races::ChrRacesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharBaseInfo {
    pub rows: Vec<CharBaseInfoRow>,
}

impl DbcTable for CharBaseInfo {
    type Row = CharBaseInfoRow;

    const FILENAME: &'static str = "CharBaseInfo.dbc";
    const FIELD_COUNT: usize = 2;
    const ROW_SIZE: usize = 2;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
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
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharBaseInfoRow {
    pub race_id: ChrRacesKey,
    pub class_id: ChrClassesKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn char_base_info() {
        let mut file = File::open("../tbc-dbc/CharBaseInfo.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CharBaseInfo::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CharBaseInfo::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
