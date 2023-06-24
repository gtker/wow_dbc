use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lock {
    pub rows: Vec<LockRow>,
}

impl DbcTable for Lock {
    type Row = LockRow;

    const FILENAME: &'static str = "Lock.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 132 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 132,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 33 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 33,
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
            field_count: 33,
            record_size: 132,
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
pub struct LockKey {
    pub id: i32
}

impl LockKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

impl TryFrom<u32> for LockKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LockRow {
    pub id: LockKey,
    pub ty: [i32; 8],
    pub index: [i32; 8],
    pub skill: [i32; 8],
    pub action: [i32; 8],
}

