use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PetPersonality {
    pub rows: Vec<PetPersonalityRow>,
}

impl DbcTable for PetPersonality {
    type Row = PetPersonalityRow;

    const FILENAME: &'static str = "PetPersonality.dbc";
    const FIELD_COUNT: usize = 19;
    const ROW_SIZE: usize = 76;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (PetPersonality) uint32
            let id = PetPersonalityKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // threshold_unhappy: int32
            let threshold_unhappy = crate::util::read_i32_le(chunk)?;

            // threshold_content: int32
            let threshold_content = crate::util::read_i32_le(chunk)?;

            // threshold_happy: int32
            let threshold_happy = crate::util::read_i32_le(chunk)?;

            // damage_unhappy: float
            let damage_unhappy = crate::util::read_f32_le(chunk)?;

            // damage_content: float
            let damage_content = crate::util::read_f32_le(chunk)?;

            // damage_happy: float
            let damage_happy = crate::util::read_f32_le(chunk)?;

            // modifier_unhappy: float
            let modifier_unhappy = crate::util::read_f32_le(chunk)?;

            // modifier_content: float
            let modifier_content = crate::util::read_f32_le(chunk)?;

            // modifier_happy: float
            let modifier_happy = crate::util::read_f32_le(chunk)?;


            rows.push(PetPersonalityRow {
                id,
                name,
                threshold_unhappy,
                threshold_content,
                threshold_happy,
                damage_unhappy,
                damage_content,
                damage_happy,
                modifier_unhappy,
                modifier_content,
                modifier_happy,
            });
        }

        Ok(PetPersonality { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (PetPersonality) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_cache))?;

            // threshold_unhappy: int32
            b.write_all(&row.threshold_unhappy.to_le_bytes())?;

            // threshold_content: int32
            b.write_all(&row.threshold_content.to_le_bytes())?;

            // threshold_happy: int32
            b.write_all(&row.threshold_happy.to_le_bytes())?;

            // damage_unhappy: float
            b.write_all(&row.damage_unhappy.to_le_bytes())?;

            // damage_content: float
            b.write_all(&row.damage_content.to_le_bytes())?;

            // damage_happy: float
            b.write_all(&row.damage_happy.to_le_bytes())?;

            // modifier_unhappy: float
            b.write_all(&row.modifier_unhappy.to_le_bytes())?;

            // modifier_content: float
            b.write_all(&row.modifier_content.to_le_bytes())?;

            // modifier_happy: float
            b.write_all(&row.modifier_happy.to_le_bytes())?;

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

impl Indexable for PetPersonality {
    type PrimaryKey = PetPersonalityKey;
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
pub struct PetPersonalityKey {
    pub id: u32
}

impl PetPersonalityKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for PetPersonalityKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for PetPersonalityKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for PetPersonalityKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for PetPersonalityKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for PetPersonalityKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for PetPersonalityKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for PetPersonalityKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for PetPersonalityKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for PetPersonalityKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for PetPersonalityKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PetPersonalityRow {
    pub id: PetPersonalityKey,
    pub name: LocalizedString,
    pub threshold_unhappy: i32,
    pub threshold_content: i32,
    pub threshold_happy: i32,
    pub damage_unhappy: f32,
    pub damage_content: f32,
    pub damage_happy: f32,
    pub modifier_unhappy: f32,
    pub modifier_content: f32,
    pub modifier_happy: f32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn pet_personality() {
        let mut file = File::open("../vanilla-dbc/PetPersonality.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = PetPersonality::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = PetPersonality::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
