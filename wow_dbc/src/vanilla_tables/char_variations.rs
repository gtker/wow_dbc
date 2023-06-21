use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Gender;
use crate::vanilla_tables::chr_races::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharVariations {
    pub rows: Vec<CharVariationsRow>,
}

impl DbcTable for CharVariations {
    type Row = CharVariationsRow;

    fn filename() -> &'static str { "CharVariations.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 24 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 24,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 6 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 6,
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
            let gender = Gender::try_from(crate::util::read_i32_le(chunk)?)?;

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
            field_count: 6,
            record_size: 24,
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
pub struct ConstCharVariations<const S: usize> {
    pub rows: [CharVariationsRow; S],
}

impl<const S: usize> ConstCharVariations<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 24 {
            panic!("invalid record size, expected 24")
        }

        if header.field_count != 6 {
            panic!("invalid field count, expected 6")
        }

        let mut b_offset = 20;
        let mut rows = [
            CharVariationsRow {
                id: ChrRacesKey::new(0),
                gender: Gender::Male,
                unknown_1: 0,
                mask: [0; 2],
                unknown_2: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: foreign_key (ChrRaces) uint32
            let id = ChrRacesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // gender: Gender
            let gender = match Gender::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]) as i8) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            // unknown_1: int32
            let unknown_1 = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mask: int32[2]
            let mask = {
                let mut a = [0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // unknown_2: int32
            let unknown_2 = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = CharVariationsRow {
                id,
                gender,
                unknown_1,
                mask,
                unknown_2,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CharVariations {
        CharVariations {
            rows: self.rows.iter().map(|s| CharVariationsRow {
                id: s.id,
                gender: s.gender,
                unknown_1: s.unknown_1,
                mask: s.mask,
                unknown_2: s.unknown_2,
            }).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharVariationsRow {
    pub id: ChrRacesKey,
    pub gender: Gender,
    pub unknown_1: i32,
    pub mask: [i32; 2],
    pub unknown_2: i32,
}

