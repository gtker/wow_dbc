use crate::DbcTable;
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::chr_races::ChrRacesKey;
use std::io::Write;
use wow_world_base::vanilla::Gender;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharVariations {
    pub rows: Vec<CharVariationsRow>,
}

impl DbcTable for CharVariations {
    type Row = CharVariationsRow;

    const FILENAME: &'static str = "CharVariations.dbc";
    const FIELD_COUNT: usize = 6;
    const ROW_SIZE: usize = 24;

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

            // id: foreign_key (ChrRaces) uint32
            let id = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // gender: Gender
            let gender = crate::util::read_i32_le(chunk)?.try_into()?;

            // unknown_1: int32
            let unknown_1 = crate::util::read_i32_le(chunk)?;

            // mask: int32[2]
            let mask = crate::util::read_array_i32::<2>(chunk)?;

            // unknown_2: int32
            let unknown_2 = crate::util::read_i32_le(chunk)?;


            rows.push(CharVariationsRow {
                id,
                gender,
                unknown_1,
                mask,
                unknown_2,
            });
        }

        Ok(CharVariations { rows, })
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
            // id: foreign_key (ChrRaces) uint32
            b.write_all(&(row.id.id as u32).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i32).to_le_bytes())?;

            // unknown_1: int32
            b.write_all(&row.unknown_1.to_le_bytes())?;

            // mask: int32[2]
            for i in row.mask {
                b.write_all(&i.to_le_bytes())?;
            }


            // unknown_2: int32
            b.write_all(&row.unknown_2.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharVariationsRow {
    pub id: ChrRacesKey,
    pub gender: Gender,
    pub unknown_1: i32,
    pub mask: [i32; 2],
    pub unknown_2: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn char_variations() {
        let mut file = File::open("../vanilla-dbc/CharVariations.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CharVariations::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CharVariations::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
