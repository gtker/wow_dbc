use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lock {
    pub rows: Vec<LockRow>,
}

impl DbcTable for Lock {
    type Row = LockRow;

    const FILENAME: &'static str = "Lock.dbc";
    const FIELD_COUNT: usize = 33;
    const ROW_SIZE: usize = 132;

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

            // id: primary_key (Lock) int32
            let id = LockKey::new(crate::util::read_i32_le(chunk)?);

            // ty: int32[8]
            let ty = crate::util::read_array_i32::<8>(chunk)?;

            // index: int32[8]
            let index = crate::util::read_array_i32::<8>(chunk)?;

            // skill: int32[8]
            let skill = crate::util::read_array_i32::<8>(chunk)?;

            // action: int32[8]
            let action = crate::util::read_array_i32::<8>(chunk)?;


            rows.push(LockRow {
                id,
                ty,
                index,
                skill,
                action,
            });
        }

        Ok(Lock { rows, })
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
            // id: primary_key (Lock) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // ty: int32[8]
            for i in row.ty {
                b.write_all(&i.to_le_bytes())?;
            }


            // index: int32[8]
            for i in row.index {
                b.write_all(&i.to_le_bytes())?;
            }


            // skill: int32[8]
            for i in row.skill {
                b.write_all(&i.to_le_bytes())?;
            }


            // action: int32[8]
            for i in row.action {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for Lock {
    type PrimaryKey = LockKey;
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
pub struct LockKey {
    pub id: i32
}

impl LockKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for LockKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for LockKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for LockKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for LockKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for LockKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for LockKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for LockKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for LockKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for LockKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for LockKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LockRow {
    pub id: LockKey,
    pub ty: [i32; 8],
    pub index: [i32; 8],
    pub skill: [i32; 8],
    pub action: [i32; 8],
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn lock() {
        let mut file = File::open("../wrath-dbc/Lock.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Lock::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Lock::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
