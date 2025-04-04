use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeaponImpactSounds {
    pub rows: Vec<WeaponImpactSoundsRow>,
}

impl DbcTable for WeaponImpactSounds {
    type Row = WeaponImpactSoundsRow;

    const FILENAME: &'static str = "WeaponImpactSounds.dbc";
    const FIELD_COUNT: usize = 23;
    const ROW_SIZE: usize = 92;

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

            // id: primary_key (WeaponImpactSounds) int32
            let id = WeaponImpactSoundsKey::new(crate::util::read_i32_le(chunk)?);

            // weapon_sub_class_id: int32
            let weapon_sub_class_id = crate::util::read_i32_le(chunk)?;

            // parry_sound_type: int32
            let parry_sound_type = crate::util::read_i32_le(chunk)?;

            // impact_sound_id: int32[10]
            let impact_sound_id = crate::util::read_array_i32::<10>(chunk)?;

            // crit_impact_sound_id: int32[10]
            let crit_impact_sound_id = crate::util::read_array_i32::<10>(chunk)?;


            rows.push(WeaponImpactSoundsRow {
                id,
                weapon_sub_class_id,
                parry_sound_type,
                impact_sound_id,
                crit_impact_sound_id,
            });
        }

        Ok(WeaponImpactSounds { rows, })
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
            // id: primary_key (WeaponImpactSounds) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // weapon_sub_class_id: int32
            b.write_all(&row.weapon_sub_class_id.to_le_bytes())?;

            // parry_sound_type: int32
            b.write_all(&row.parry_sound_type.to_le_bytes())?;

            // impact_sound_id: int32[10]
            for i in row.impact_sound_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // crit_impact_sound_id: int32[10]
            for i in row.crit_impact_sound_id {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for WeaponImpactSounds {
    type PrimaryKey = WeaponImpactSoundsKey;
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
pub struct WeaponImpactSoundsKey {
    pub id: i32
}

impl WeaponImpactSoundsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for WeaponImpactSoundsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for WeaponImpactSoundsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for WeaponImpactSoundsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for WeaponImpactSoundsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for WeaponImpactSoundsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for WeaponImpactSoundsKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for WeaponImpactSoundsKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for WeaponImpactSoundsKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for WeaponImpactSoundsKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for WeaponImpactSoundsKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeaponImpactSoundsRow {
    pub id: WeaponImpactSoundsKey,
    pub weapon_sub_class_id: i32,
    pub parry_sound_type: i32,
    pub impact_sound_id: [i32; 10],
    pub crit_impact_sound_id: [i32; 10],
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn weapon_impact_sounds() {
        let mut file = File::open("../tbc-dbc/WeaponImpactSounds.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WeaponImpactSounds::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WeaponImpactSounds::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
