use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::chr_races::ChrRacesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharHairTextures {
    pub rows: Vec<CharHairTexturesRow>,
}

impl DbcTable for CharHairTextures {
    type Row = CharHairTexturesRow;

    const FILENAME: &'static str = "CharHairTextures.dbc";
    const FIELD_COUNT: usize = 8;
    const ROW_SIZE: usize = 32;

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

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

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

impl Indexable for CharHairTextures {
    type PrimaryKey = CharHairTexturesKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharHairTexturesKey {
    pub id: i32
}

impl CharHairTexturesKey {
    pub const fn new(id: i32) -> Self {
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

impl TryFrom<u32> for CharHairTexturesKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for CharHairTexturesKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for CharHairTexturesKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for CharHairTexturesKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for CharHairTexturesKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn char_hair_textures() {
        let mut file = File::open("../wrath-dbc/CharHairTextures.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CharHairTextures::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CharHairTextures::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
