use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillTiers {
    pub rows: Vec<SkillTiersRow>,
}

impl DbcTable for SkillTiers {
    type Row = SkillTiersRow;

    fn filename() -> &'static str { "SkillTiers.dbc" }

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

            // id: primary_key (SkillTiers) int32
            let id = SkillTiersKey::new(crate::util::read_i32_le(chunk)?);

            // cost: int32[16]
            let cost = crate::util::read_array_i32::<16>(chunk)?;

            // value: int32[16]
            let value = crate::util::read_array_i32::<16>(chunk)?;


            rows.push(SkillTiersRow {
                id,
                cost,
                value,
            });
        }

        Ok(SkillTiers { rows, })
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
            // id: primary_key (SkillTiers) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // cost: int32[16]
            for i in row.cost {
                b.write_all(&i.to_le_bytes())?;
            }


            // value: int32[16]
            for i in row.value {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SkillTiers {
    type PrimaryKey = SkillTiersKey;
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
pub struct ConstSkillTiers<const S: usize> {
    pub rows: [SkillTiersRow; S],
}

impl<const S: usize> ConstSkillTiers<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 132 {
            panic!("invalid record size, expected 132")
        }

        if header.field_count != 33 {
            panic!("invalid field count, expected 33")
        }

        let mut b_offset = 20;
        let mut rows = [
            SkillTiersRow {
                id: SkillTiersKey::new(0),
                cost: [0; 16],
                value: [0; 16],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SkillTiers) int32
            let id = SkillTiersKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // cost: int32[16]
            let cost = {
                let mut a = [0; 16];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // value: int32[16]
            let value = {
                let mut a = [0; 16];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = SkillTiersRow {
                id,
                cost,
                value,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SkillTiers {
        SkillTiers {
            rows: self.rows.iter().map(|s| SkillTiersRow {
                id: s.id,
                cost: s.cost,
                value: s.value,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SkillTiersKey {
    pub id: i32
}

impl SkillTiersKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SkillTiersKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SkillTiersKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SkillTiersKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SkillTiersKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SkillTiersKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillTiersRow {
    pub id: SkillTiersKey,
    pub cost: [i32; 16],
    pub value: [i32; 16],
}

