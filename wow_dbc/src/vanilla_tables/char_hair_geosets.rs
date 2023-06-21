use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::Gender;
use crate::vanilla_tables::chr_races::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharHairGeosets {
    pub rows: Vec<CharHairGeosetsRow>,
}

impl DbcTable for CharHairGeosets {
    type Row = CharHairGeosetsRow;

    fn filename() -> &'static str { "CharHairGeosets.dbc" }

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

            // id: primary_key (CharHairGeosets) uint32
            let id = CharHairGeosetsKey::new(crate::util::read_u32_le(chunk)?);

            // race: foreign_key (ChrRaces) uint32
            let race = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // gender: Gender
            let gender = Gender::try_from(crate::util::read_i32_le(chunk)?)?;

            // variation: uint32
            let variation = crate::util::read_u32_le(chunk)?;

            // geoset: int32
            let geoset = crate::util::read_i32_le(chunk)?;

            // show_scalp: Scalp
            let show_scalp = Scalp::try_from(crate::util::read_i32_le(chunk)?)?;


            rows.push(CharHairGeosetsRow {
                id,
                race,
                gender,
                variation,
                geoset,
                show_scalp,
            });
        }

        Ok(CharHairGeosets { rows, })
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
            // id: primary_key (CharHairGeosets) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // race: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race.id as u32).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i32).to_le_bytes())?;

            // variation: uint32
            b.write_all(&row.variation.to_le_bytes())?;

            // geoset: int32
            b.write_all(&row.geoset.to_le_bytes())?;

            // show_scalp: Scalp
            b.write_all(&(row.show_scalp.as_int() as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for CharHairGeosets {
    type PrimaryKey = CharHairGeosetsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCharHairGeosets<const S: usize> {
    pub rows: [CharHairGeosetsRow; S],
}

impl<const S: usize> ConstCharHairGeosets<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 24 {
            panic!("invalid record size, expected 24")
        }

        if header.field_count != 6 {
            panic!("invalid field count, expected 6")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            CharHairGeosetsRow {
                id: CharHairGeosetsKey::new(0),
                race: ChrRacesKey::new(0),
                gender: Gender::Male,
                variation: 0,
                geoset: 0,
                show_scalp: Scalp::Hair,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (CharHairGeosets) uint32
            let id = CharHairGeosetsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // race: foreign_key (ChrRaces) uint32
            let race = ChrRacesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // gender: Gender
            let gender = match Gender::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]) as i8) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            // variation: uint32
            let variation = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // geoset: int32
            let geoset = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // show_scalp: Scalp
            let show_scalp = match Scalp::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            rows[i] = CharHairGeosetsRow {
                id,
                race,
                gender,
                variation,
                geoset,
                show_scalp,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CharHairGeosets {
        CharHairGeosets {
            rows: self.rows.iter().map(|s| CharHairGeosetsRow {
                id: s.id,
                race: s.race,
                gender: s.gender,
                variation: s.variation,
                geoset: s.geoset,
                show_scalp: s.show_scalp,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CharHairGeosetsKey {
    pub id: u32
}

impl CharHairGeosetsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for CharHairGeosetsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for CharHairGeosetsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for CharHairGeosetsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Scalp {
    Hair,
    Bald,
}

impl Scalp {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            0 => Self::Hair,
            1 => Self::Bald,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for Scalp {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("Scalp", value as i64))
    }

}

impl Scalp {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::Hair => 0,
            Self::Bald => 1,
        }

    }

}

impl Default for Scalp {
    fn default() -> Self {
        Self::Hair
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharHairGeosetsRow {
    pub id: CharHairGeosetsKey,
    pub race: ChrRacesKey,
    pub gender: Gender,
    pub variation: u32,
    pub geoset: i32,
    pub show_scalp: Scalp,
}

