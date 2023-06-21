use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::chr_races::*;

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

            // id: primary_key (CharHairTextures) int32
            let id = CharHairTexturesKey::new(crate::util::read_i32_le(chunk)?);

            // field_0_5_3_3368_001_race: foreign_key (ChrRaces) int32
            let field_0_5_3_3368_001_race = ChrRacesKey::new(crate::util::read_i32_le(chunk)?.into());

            // field_0_5_3_3368_002_gender: int32
            let field_0_5_3_3368_002_gender = crate::util::read_i32_le(chunk)?;

            // field_0_5_3_3368_003: int32
            let field_0_5_3_3368_003 = crate::util::read_i32_le(chunk)?;

            // field_0_5_3_3368_004_mayberacemask: int32
            let field_0_5_3_3368_004_mayberacemask = crate::util::read_i32_le(chunk)?;

            // field_0_5_3_3368_005_the_x_in_hair_xy_blp: int32
            let field_0_5_3_3368_005_the_x_in_hair_xy_blp = crate::util::read_i32_le(chunk)?;

            // field_0_5_3_3368_006: int32
            let field_0_5_3_3368_006 = crate::util::read_i32_le(chunk)?;

            // field_0_5_3_3368_007: int32
            let field_0_5_3_3368_007 = crate::util::read_i32_le(chunk)?;


            rows.push(CharHairTexturesRow {
                id,
                field_0_5_3_3368_001_race,
                field_0_5_3_3368_002_gender,
                field_0_5_3_3368_003,
                field_0_5_3_3368_004_mayberacemask,
                field_0_5_3_3368_005_the_x_in_hair_xy_blp,
                field_0_5_3_3368_006,
                field_0_5_3_3368_007,
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
            // id: primary_key (CharHairTextures) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // field_0_5_3_3368_001_race: foreign_key (ChrRaces) int32
            b.write_all(&(row.field_0_5_3_3368_001_race.id as i32).to_le_bytes())?;

            // field_0_5_3_3368_002_gender: int32
            b.write_all(&row.field_0_5_3_3368_002_gender.to_le_bytes())?;

            // field_0_5_3_3368_003: int32
            b.write_all(&row.field_0_5_3_3368_003.to_le_bytes())?;

            // field_0_5_3_3368_004_mayberacemask: int32
            b.write_all(&row.field_0_5_3_3368_004_mayberacemask.to_le_bytes())?;

            // field_0_5_3_3368_005_the_x_in_hair_xy_blp: int32
            b.write_all(&row.field_0_5_3_3368_005_the_x_in_hair_xy_blp.to_le_bytes())?;

            // field_0_5_3_3368_006: int32
            b.write_all(&row.field_0_5_3_3368_006.to_le_bytes())?;

            // field_0_5_3_3368_007: int32
            b.write_all(&row.field_0_5_3_3368_007.to_le_bytes())?;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCharHairTextures<const S: usize> {
    pub rows: [CharHairTexturesRow; S],
}

impl<const S: usize> ConstCharHairTextures<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 32 {
            panic!("invalid record size, expected 32")
        }

        if header.field_count != 8 {
            panic!("invalid field count, expected 8")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            CharHairTexturesRow {
                id: CharHairTexturesKey::new(0),
                field_0_5_3_3368_001_race: ChrRacesKey::new(0),
                field_0_5_3_3368_002_gender: 0,
                field_0_5_3_3368_003: 0,
                field_0_5_3_3368_004_mayberacemask: 0,
                field_0_5_3_3368_005_the_x_in_hair_xy_blp: 0,
                field_0_5_3_3368_006: 0,
                field_0_5_3_3368_007: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (CharHairTextures) int32
            let id = CharHairTexturesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // field_0_5_3_3368_001_race: foreign_key (ChrRaces) int32
            let field_0_5_3_3368_001_race = ChrRacesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // field_0_5_3_3368_002_gender: int32
            let field_0_5_3_3368_002_gender = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // field_0_5_3_3368_003: int32
            let field_0_5_3_3368_003 = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // field_0_5_3_3368_004_mayberacemask: int32
            let field_0_5_3_3368_004_mayberacemask = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // field_0_5_3_3368_005_the_x_in_hair_xy_blp: int32
            let field_0_5_3_3368_005_the_x_in_hair_xy_blp = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // field_0_5_3_3368_006: int32
            let field_0_5_3_3368_006 = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // field_0_5_3_3368_007: int32
            let field_0_5_3_3368_007 = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = CharHairTexturesRow {
                id,
                field_0_5_3_3368_001_race,
                field_0_5_3_3368_002_gender,
                field_0_5_3_3368_003,
                field_0_5_3_3368_004_mayberacemask,
                field_0_5_3_3368_005_the_x_in_hair_xy_blp,
                field_0_5_3_3368_006,
                field_0_5_3_3368_007,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CharHairTextures {
        CharHairTextures {
            rows: self.rows.iter().map(|s| CharHairTexturesRow {
                id: s.id,
                field_0_5_3_3368_001_race: s.field_0_5_3_3368_001_race,
                field_0_5_3_3368_002_gender: s.field_0_5_3_3368_002_gender,
                field_0_5_3_3368_003: s.field_0_5_3_3368_003,
                field_0_5_3_3368_004_mayberacemask: s.field_0_5_3_3368_004_mayberacemask,
                field_0_5_3_3368_005_the_x_in_hair_xy_blp: s.field_0_5_3_3368_005_the_x_in_hair_xy_blp,
                field_0_5_3_3368_006: s.field_0_5_3_3368_006,
                field_0_5_3_3368_007: s.field_0_5_3_3368_007,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CharHairTexturesKey {
    pub id: i32
}

impl CharHairTexturesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for CharHairTexturesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for CharHairTexturesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for CharHairTexturesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharHairTexturesRow {
    pub id: CharHairTexturesKey,
    pub field_0_5_3_3368_001_race: ChrRacesKey,
    pub field_0_5_3_3368_002_gender: i32,
    pub field_0_5_3_3368_003: i32,
    pub field_0_5_3_3368_004_mayberacemask: i32,
    pub field_0_5_3_3368_005_the_x_in_hair_xy_blp: i32,
    pub field_0_5_3_3368_006: i32,
    pub field_0_5_3_3368_007: i32,
}

