use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Gender;
use crate::vanilla_tables::chr_races::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharacterFacialHairStyles {
    pub rows: Vec<CharacterFacialHairStylesRow>,
}

impl DbcTable for CharacterFacialHairStyles {
    type Row = CharacterFacialHairStylesRow;

    fn filename() -> &'static str { "CharacterFacialHairStyles.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 36 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 36,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 9 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 9,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // race: foreign_key (ChrRaces) uint32
            let race = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // gender: Gender
            let gender = Gender::try_from(crate::util::read_i32_le(chunk)?)?;

            // variation_id: uint32
            let variation_id = crate::util::read_u32_le(chunk)?;

            // geoset: int32[6]
            let geoset = crate::util::read_array_i32::<6>(chunk)?;


            rows.push(CharacterFacialHairStylesRow {
                race,
                gender,
                variation_id,
                geoset,
            });
        }

        Ok(CharacterFacialHairStyles { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 9,
            record_size: 36,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // race: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race.id as u32).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i32).to_le_bytes())?;

            // variation_id: uint32
            b.write_all(&row.variation_id.to_le_bytes())?;

            // geoset: int32[6]
            for i in row.geoset {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCharacterFacialHairStyles<const S: usize> {
    pub rows: [CharacterFacialHairStylesRow; S],
}

impl<const S: usize> ConstCharacterFacialHairStyles<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 36 {
            panic!("invalid record size, expected 36")
        }

        if header.field_count != 9 {
            panic!("invalid field count, expected 9")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            CharacterFacialHairStylesRow {
                race: ChrRacesKey::new(0),
                gender: Gender::Male,
                variation_id: 0,
                geoset: [0; 6],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // race: foreign_key (ChrRaces) uint32
            let race = ChrRacesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // gender: Gender
            let gender = match Gender::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]) as i8) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            // variation_id: uint32
            let variation_id = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // geoset: int32[6]
            let geoset = {
                let mut a = [0; 6];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = CharacterFacialHairStylesRow {
                race,
                gender,
                variation_id,
                geoset,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CharacterFacialHairStyles {
        CharacterFacialHairStyles {
            rows: self.rows.iter().map(|s| CharacterFacialHairStylesRow {
                race: s.race,
                gender: s.gender,
                variation_id: s.variation_id,
                geoset: s.geoset,
            }).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharacterFacialHairStylesRow {
    pub race: ChrRacesKey,
    pub gender: Gender,
    pub variation_id: u32,
    pub geoset: [i32; 6],
}

