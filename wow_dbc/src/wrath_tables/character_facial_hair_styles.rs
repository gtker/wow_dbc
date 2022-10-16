use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::wrath_tables::chr_races::*;

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

        if header.record_size != 32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 8,
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

            // variation_id: int32
            let variation_id = crate::util::read_i32_le(chunk)?;

            // geoset: int32[5]
            let geoset = crate::util::read_array_i32::<5>(chunk)?;


            rows.push(CharacterFacialHairStylesRow {
                race_id,
                sex_id,
                variation_id,
                geoset,
            });
        }

        Ok(CharacterFacialHairStyles { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 8,
            record_size: 32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // race_id: foreign_key (ChrRaces) int32
            b.write_all(&(row.race_id.id as i32).to_le_bytes())?;

            // sex_id: int32
            b.write_all(&row.sex_id.to_le_bytes())?;

            // variation_id: int32
            b.write_all(&row.variation_id.to_le_bytes())?;

            // geoset: int32[5]
            for i in row.geoset {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharacterFacialHairStylesRow {
    pub race_id: ChrRacesKey,
    pub sex_id: i32,
    pub variation_id: i32,
    pub geoset: [i32; 5],
}

