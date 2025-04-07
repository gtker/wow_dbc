use crate::DbcTable;
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::chr_classes::ChrClassesKey;
use crate::vanilla_tables::chr_races::ChrRacesKey;
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

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // race: foreign_key (ChrRaces) uint8
            b.write_all(&(row.race.id as u8).to_le_bytes())?;

            // class: foreign_key (ChrClasses) uint8
            b.write_all(&(row.class.id as u8).to_le_bytes())?;

        }

        assert_eq!(b.len(), self.rows.len() * Self::ROW_SIZE);
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: string_cache.size(),
        };

        w.write_all(&header.write_header())?;
        w.write_all(&b)?;
        w.write_all(string_cache.buffer())?;
        Ok(())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharBaseInfoRow {
    pub race: ChrRacesKey,
    pub class: ChrClassesKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn char_base_info() {
        let mut file = File::open("../vanilla-dbc/CharBaseInfo.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CharBaseInfo::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CharBaseInfo::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
