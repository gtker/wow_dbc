use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::object_effect_group::ObjectEffectGroupKey;
use crate::wrath_tables::object_effect_package::ObjectEffectPackageKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectEffectPackageElem {
    pub rows: Vec<ObjectEffectPackageElemRow>,
}

impl DbcTable for ObjectEffectPackageElem {
    type Row = ObjectEffectPackageElemRow;

    const FILENAME: &'static str = "ObjectEffectPackageElem.dbc";
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

            // id: primary_key (ObjectEffectPackageElem) int32
            let id = ObjectEffectPackageElemKey::new(crate::util::read_i32_le(chunk)?);

            // object_effect_package_id: foreign_key (ObjectEffectPackage) int32
            let object_effect_package_id = ObjectEffectPackageKey::new(crate::util::read_i32_le(chunk)?.into());

            // object_effect_group_id: foreign_key (ObjectEffectGroup) int32
            let object_effect_group_id = ObjectEffectGroupKey::new(crate::util::read_i32_le(chunk)?.into());

            // state_type: int32
            let state_type = crate::util::read_i32_le(chunk)?;


            rows.push(ObjectEffectPackageElemRow {
                id,
                object_effect_package_id,
                object_effect_group_id,
                state_type,
            });
        }

        Ok(ObjectEffectPackageElem { rows, })
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
            // id: primary_key (ObjectEffectPackageElem) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // object_effect_package_id: foreign_key (ObjectEffectPackage) int32
            b.write_all(&(row.object_effect_package_id.id as i32).to_le_bytes())?;

            // object_effect_group_id: foreign_key (ObjectEffectGroup) int32
            b.write_all(&(row.object_effect_group_id.id as i32).to_le_bytes())?;

            // state_type: int32
            b.write_all(&row.state_type.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for ObjectEffectPackageElem {
    type PrimaryKey = ObjectEffectPackageElemKey;
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
pub struct ObjectEffectPackageElemKey {
    pub id: i32
}

impl ObjectEffectPackageElemKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for ObjectEffectPackageElemKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for ObjectEffectPackageElemKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for ObjectEffectPackageElemKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for ObjectEffectPackageElemKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for ObjectEffectPackageElemKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for ObjectEffectPackageElemKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ObjectEffectPackageElemKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for ObjectEffectPackageElemKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ObjectEffectPackageElemKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ObjectEffectPackageElemKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectEffectPackageElemRow {
    pub id: ObjectEffectPackageElemKey,
    pub object_effect_package_id: ObjectEffectPackageKey,
    pub object_effect_group_id: ObjectEffectGroupKey,
    pub state_type: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn object_effect_package_elem() {
        let mut file = File::open("../wrath-dbc/ObjectEffectPackageElem.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ObjectEffectPackageElem::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ObjectEffectPackageElem::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
