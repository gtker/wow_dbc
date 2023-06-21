use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::wrath_tables::chr_races::*;

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

            // race_id: foreign_key (ChrRaces) int32
            let race_id = ChrRacesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sex_id: int32
            let sex_id = crate::util::read_i32_le(chunk)?;

            // texture_hold_layer: int32[4]
            let texture_hold_layer = crate::util::read_array_i32::<4>(chunk)?;


            rows.push(CharVariationsRow {
                race_id,
                sex_id,
                texture_hold_layer,
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
            // race_id: foreign_key (ChrRaces) int32
            b.write_all(&(row.race_id.id as i32).to_le_bytes())?;

            // sex_id: int32
            b.write_all(&row.sex_id.to_le_bytes())?;

            // texture_hold_layer: int32[4]
            for i in row.texture_hold_layer {
                b.write_all(&i.to_le_bytes())?;
            }


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
                race_id: ChrRacesKey::new(0),
                sex_id: 0,
                texture_hold_layer: [0; 4],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // race_id: foreign_key (ChrRaces) int32
            let race_id = ChrRacesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sex_id: int32
            let sex_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // texture_hold_layer: int32[4]
            let texture_hold_layer = {
                let mut a = [0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = CharVariationsRow {
                race_id,
                sex_id,
                texture_hold_layer,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CharVariations {
        CharVariations {
            rows: self.rows.iter().map(|s| CharVariationsRow {
                race_id: s.race_id,
                sex_id: s.sex_id,
                texture_hold_layer: s.texture_hold_layer,
            }).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharVariationsRow {
    pub race_id: ChrRacesKey,
    pub sex_id: i32,
    pub texture_hold_layer: [i32; 4],
}

