use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::sound_entries::SoundEntriesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeaponSwingSounds2 {
    pub rows: Vec<WeaponSwingSounds2Row>,
}

impl DbcTable for WeaponSwingSounds2 {
    type Row = WeaponSwingSounds2Row;

    const FILENAME: &'static str = "WeaponSwingSounds2.dbc";
    const FIELD_COUNT: usize = 4;
    const ROW_SIZE: usize = 16;

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

            // id: primary_key (WeaponSwingSounds2) int32
            let id = WeaponSwingSounds2Key::new(crate::util::read_i32_le(chunk)?);

            // swing_type: int32
            let swing_type = crate::util::read_i32_le(chunk)?;

            // crit: int32
            let crit = crate::util::read_i32_le(chunk)?;

            // sound_id: foreign_key (SoundEntries) int32
            let sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(WeaponSwingSounds2Row {
                id,
                swing_type,
                crit,
                sound_id,
            });
        }

        Ok(WeaponSwingSounds2 { rows, })
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
            // id: primary_key (WeaponSwingSounds2) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // swing_type: int32
            b.write_all(&row.swing_type.to_le_bytes())?;

            // crit: int32
            b.write_all(&row.crit.to_le_bytes())?;

            // sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_id.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for WeaponSwingSounds2 {
    type PrimaryKey = WeaponSwingSounds2Key;
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
pub struct WeaponSwingSounds2Key {
    pub id: i32
}

impl WeaponSwingSounds2Key {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for WeaponSwingSounds2Key {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for WeaponSwingSounds2Key {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for WeaponSwingSounds2Key {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for WeaponSwingSounds2Key {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for WeaponSwingSounds2Key {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for WeaponSwingSounds2Key {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for WeaponSwingSounds2Key {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for WeaponSwingSounds2Key {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for WeaponSwingSounds2Key {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for WeaponSwingSounds2Key {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeaponSwingSounds2Row {
    pub id: WeaponSwingSounds2Key,
    pub swing_type: i32,
    pub crit: i32,
    pub sound_id: SoundEntriesKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn weapon_swing_sounds2() {
        let mut file = File::open("../tbc-dbc/WeaponSwingSounds2.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WeaponSwingSounds2::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WeaponSwingSounds2::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
