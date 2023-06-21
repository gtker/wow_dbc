use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::Gender;
use crate::vanilla_tables::chr_races::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharHairTextures {
    pub rows: Vec<CharHairTexturesRow>,
}

impl DbcTable for CharHairTextures {
    type Row = CharHairTexturesRow;

    fn filename() -> &'static str { "CharHairTextures.dbc" }

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

            // id: primary_key (CharHairTextures) uint32
            let id = CharHairTexturesKey::new(crate::util::read_u32_le(chunk)?);

            // race: foreign_key (ChrRaces) uint32
            let race = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // gender: Gender
            let gender = Gender::try_from(crate::util::read_i32_le(chunk)?)?;

            // variation: uint32
            let variation = crate::util::read_u32_le(chunk)?;

            // geoset: int32
            let geoset = crate::util::read_i32_le(chunk)?;

            // show_scalp: uint32
            let show_scalp = crate::util::read_u32_le(chunk)?;

            // unknown_padding: uint32[2]
            let unknown_padding = crate::util::read_array_u32::<2>(chunk)?;


            rows.push(CharHairTexturesRow {
                id,
                race,
                gender,
                variation,
                geoset,
                show_scalp,
                unknown_padding,
            });
        }

        Ok(CharHairTextures { rows, })
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
            // id: primary_key (CharHairTextures) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // race: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race.id as u32).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i32).to_le_bytes())?;

            // variation: uint32
            b.write_all(&row.variation.to_le_bytes())?;

            // geoset: int32
            b.write_all(&row.geoset.to_le_bytes())?;

            // show_scalp: uint32
            b.write_all(&row.show_scalp.to_le_bytes())?;

            // unknown_padding: uint32[2]
            for i in row.unknown_padding {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for CharHairTextures {
    type PrimaryKey = CharHairTexturesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CharHairTexturesKey {
    pub id: u32
}

impl CharHairTexturesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for CharHairTexturesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for CharHairTexturesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for CharHairTexturesKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharHairTexturesRow {
    pub id: CharHairTexturesKey,
    pub race: ChrRacesKey,
    pub gender: Gender,
    pub variation: u32,
    pub geoset: i32,
    pub show_scalp: u32,
    pub unknown_padding: [u32; 2],
}

