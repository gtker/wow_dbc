use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lock {
    pub rows: Vec<LockRow>,
}

impl DbcTable for Lock {
    type Row = LockRow;

    fn filename() -> &'static str { "Lock.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

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

            // id: primary_key (Lock) uint32
            let id = LockKey::new(crate::util::read_u32_le(chunk)?);

            // ty: Type[8]
            let ty = {
                let mut arr = [Type::default(); 8];
                for i in arr.iter_mut() {
                    *i = Type::try_from(crate::util::read_i32_le(chunk)?)?;
                }

                arr
            };

            // property: uint32[8]
            let property = crate::util::read_array_u32::<8>(chunk)?;

            // required_skill: int32[8]
            let required_skill = crate::util::read_array_i32::<8>(chunk)?;

            // action: int32[8]
            let action = crate::util::read_array_i32::<8>(chunk)?;


            rows.push(LockRow {
                id,
                ty,
                property,
                required_skill,
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
            // id: primary_key (Lock) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // ty: Type[8]
            for i in row.ty {
                b.write_all(&(i.as_int() as i32).to_le_bytes())?;
            }


            // property: uint32[8]
            for i in row.property {
                b.write_all(&i.to_le_bytes())?;
            }


            // required_skill: int32[8]
            for i in row.required_skill {
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
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct LockKey {
    pub id: u32
}

impl LockKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Type {
    None,
    ItemRequired,
    LocktypeReference,
}

impl TryFrom<i32> for Type {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::None,
            1 => Self::ItemRequired,
            2 => Self::LocktypeReference,
            val => return Err(crate::InvalidEnumError::new("Type", val as i64)),
        })
    }

}

impl Type {
    const fn as_int(&self) -> i32 {
        match self {
            Self::None => 0,
            Self::ItemRequired => 1,
            Self::LocktypeReference => 2,
        }

    }

}

impl Default for Type {
    fn default() -> Self {
        Self::None
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockRow {
    pub id: LockKey,
    pub ty: [Type; 8],
    pub property: [u32; 8],
    pub required_skill: [i32; 8],
    pub action: [i32; 8],
}

