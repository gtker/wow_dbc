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
                ty: [0; 8],
                index: [0; 8],
                skill: [0; 8],
                action: [0; 8],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Lock) int32
            let id = LockKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // ty: int32[8]
            let ty = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // index: int32[8]
            let index = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // skill: int32[8]
            let skill = {
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
                index,
                skill,
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
                index: s.index,
                skill: s.skill,
                action: s.action,
            }).collect(),
        }
    }
    // TODO: Indexable?
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LockRow {
    pub id: LockKey,
    pub ty: [i32; 8],
    pub index: [i32; 8],
    pub skill: [i32; 8],
    pub action: [i32; 8],
}

