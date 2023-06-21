use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::object_effect_group::*;
use crate::wrath_tables::object_effect_package::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectEffectPackageElem {
    pub rows: Vec<ObjectEffectPackageElemRow>,
}

impl DbcTable for ObjectEffectPackageElem {
    type Row = ObjectEffectPackageElemRow;

    fn filename() -> &'static str { "ObjectEffectPackageElem.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 16,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 4 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 4,
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
            field_count: 4,
            record_size: 16,
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
pub struct ConstObjectEffectPackageElem<const S: usize> {
    pub rows: [ObjectEffectPackageElemRow; S],
}

impl<const S: usize> ConstObjectEffectPackageElem<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 16 {
            panic!("invalid record size, expected 16")
        }

        if header.field_count != 4 {
            panic!("invalid field count, expected 4")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ObjectEffectPackageElemRow {
                id: ObjectEffectPackageElemKey::new(0),
                object_effect_package_id: ObjectEffectPackageKey::new(0),
                object_effect_group_id: ObjectEffectGroupKey::new(0),
                state_type: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (ObjectEffectPackageElem) int32
            let id = ObjectEffectPackageElemKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // object_effect_package_id: foreign_key (ObjectEffectPackage) int32
            let object_effect_package_id = ObjectEffectPackageKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // object_effect_group_id: foreign_key (ObjectEffectGroup) int32
            let object_effect_group_id = ObjectEffectGroupKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // state_type: int32
            let state_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ObjectEffectPackageElemRow {
                id,
                object_effect_package_id,
                object_effect_group_id,
                state_type,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> ObjectEffectPackageElem {
        ObjectEffectPackageElem {
            rows: self.rows.iter().map(|s| ObjectEffectPackageElemRow {
                id: s.id,
                object_effect_package_id: s.object_effect_package_id,
                object_effect_group_id: s.object_effect_group_id,
                state_type: s.state_type,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ObjectEffectPackageElemKey {
    pub id: i32
}

impl ObjectEffectPackageElemKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectEffectPackageElemRow {
    pub id: ObjectEffectPackageElemKey,
    pub object_effect_package_id: ObjectEffectPackageKey,
    pub object_effect_group_id: ObjectEffectGroupKey,
    pub state_type: i32,
}

