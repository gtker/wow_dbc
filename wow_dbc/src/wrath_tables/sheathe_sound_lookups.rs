use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::material::MaterialKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SheatheSoundLookups {
    pub rows: Vec<SheatheSoundLookupsRow>,
}

impl DbcTable for SheatheSoundLookups {
    type Row = SheatheSoundLookupsRow;

    const FILENAME: &'static str = "SheatheSoundLookups.dbc";
    const FIELD_COUNT: usize = 7;
    const ROW_SIZE: usize = 28;

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

            // id: primary_key (SheatheSoundLookups) int32
            let id = SheatheSoundLookupsKey::new(crate::util::read_i32_le(chunk)?);

            // class_id: int32
            let class_id = crate::util::read_i32_le(chunk)?;

            // subclass_id: int32
            let subclass_id = crate::util::read_i32_le(chunk)?;

            // material: foreign_key (Material) int32
            let material = MaterialKey::new(crate::util::read_i32_le(chunk)?.into());

            // check_material: int32
            let check_material = crate::util::read_i32_le(chunk)?;

            // sheathe_sound: int32
            let sheathe_sound = crate::util::read_i32_le(chunk)?;

            // unsheathe_sound: int32
            let unsheathe_sound = crate::util::read_i32_le(chunk)?;


            rows.push(SheatheSoundLookupsRow {
                id,
                class_id,
                subclass_id,
                material,
                check_material,
                sheathe_sound,
                unsheathe_sound,
            });
        }

        Ok(SheatheSoundLookups { rows, })
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
            // id: primary_key (SheatheSoundLookups) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // class_id: int32
            b.write_all(&row.class_id.to_le_bytes())?;

            // subclass_id: int32
            b.write_all(&row.subclass_id.to_le_bytes())?;

            // material: foreign_key (Material) int32
            b.write_all(&(row.material.id as i32).to_le_bytes())?;

            // check_material: int32
            b.write_all(&row.check_material.to_le_bytes())?;

            // sheathe_sound: int32
            b.write_all(&row.sheathe_sound.to_le_bytes())?;

            // unsheathe_sound: int32
            b.write_all(&row.unsheathe_sound.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SheatheSoundLookups {
    type PrimaryKey = SheatheSoundLookupsKey;
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
pub struct SheatheSoundLookupsKey {
    pub id: i32
}

impl SheatheSoundLookupsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for SheatheSoundLookupsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for SheatheSoundLookupsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for SheatheSoundLookupsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for SheatheSoundLookupsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for SheatheSoundLookupsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for SheatheSoundLookupsKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SheatheSoundLookupsKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for SheatheSoundLookupsKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SheatheSoundLookupsKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SheatheSoundLookupsKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SheatheSoundLookupsRow {
    pub id: SheatheSoundLookupsKey,
    pub class_id: i32,
    pub subclass_id: i32,
    pub material: MaterialKey,
    pub check_material: i32,
    pub sheathe_sound: i32,
    pub unsheathe_sound: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn sheathe_sound_lookups() {
        let mut file = File::open("../wrath-dbc/SheatheSoundLookups.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SheatheSoundLookups::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SheatheSoundLookups::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
