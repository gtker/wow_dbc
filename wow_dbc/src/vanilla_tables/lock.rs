use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
pub struct ConstLock<const S: usize> {
    pub rows: [LockRow; S],
}

impl<const S: usize> ConstLock<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 132 {
            panic!("invalid record size, expected 132")
        }

        if header.field_count != 33 {
            panic!("invalid field count, expected 33")
        }

        let mut b_offset = 20;
        let mut rows = [
            LockRow {
                id: LockKey::new(0),
                ty: [Type::None; 8],
                property: [0; 8],
                required_skill: [0; 8],
                action: [0; 8],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Lock) uint32
            let id = LockKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // ty: Type[8]
            let ty = {
                let mut a = [Type::None; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = match Type::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])) {
                        Some(e) => e,
                        None => panic!(),
                    };
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // property: uint32[8]
            let property = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // required_skill: int32[8]
            let required_skill = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // action: int32[8]
            let action = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = LockRow {
                id,
                ty,
                property,
                required_skill,
                action,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> Lock {
        Lock {
            rows: self.rows.iter().map(|s| LockRow {
                id: s.id,
                ty: s.ty,
                property: s.property,
                required_skill: s.required_skill,
                action: s.action,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct LockKey {
    pub id: u32
}

impl LockKey {
    pub const fn new(id: u32) -> Self {
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

impl From<u32> for LockKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Type {
    None,
    ItemRequired,
    LocktypeReference,
}

impl Type {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::ItemRequired,
            2 => Self::LocktypeReference,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for Type {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("Type", value as i64))
    }

}

impl Type {
    pub const fn as_int(&self) -> i32 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LockRow {
    pub id: LockKey,
    pub ty: [Type; 8],
    pub property: [u32; 8],
    pub required_skill: [i32; 8],
    pub action: [i32; 8],
}

