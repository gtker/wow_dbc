use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ObjectEffectModifier {
    pub rows: Vec<ObjectEffectModifierRow>,
}

impl DbcTable for ObjectEffectModifier {
    type Row = ObjectEffectModifierRow;

    fn filename() -> &'static str { "ObjectEffectModifier.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 8,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (ObjectEffectModifier) int32
            let id = ObjectEffectModifierKey::new(crate::util::read_i32_le(chunk)?);

            // input_type: int32
            let input_type = crate::util::read_i32_le(chunk)?;

            // map_type: int32
            let map_type = crate::util::read_i32_le(chunk)?;

            // output_type: int32
            let output_type = crate::util::read_i32_le(chunk)?;

            // param: float[4]
            let param = crate::util::read_array_f32::<4>(chunk)?;


            rows.push(ObjectEffectModifierRow {
                id,
                input_type,
                map_type,
                output_type,
                param,
            });
        }

        Ok(ObjectEffectModifier { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 8,
            record_size: 32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (ObjectEffectModifier) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // input_type: int32
            b.write_all(&row.input_type.to_le_bytes())?;

            // map_type: int32
            b.write_all(&row.map_type.to_le_bytes())?;

            // output_type: int32
            b.write_all(&row.output_type.to_le_bytes())?;

            // param: float[4]
            for i in row.param {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for ObjectEffectModifier {
    type PrimaryKey = ObjectEffectModifierKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstObjectEffectModifier<const S: usize> {
    pub rows: [ObjectEffectModifierRow; S],
}

impl<const S: usize> ConstObjectEffectModifier<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 32 {
            panic!("invalid record size, expected 32")
        }

        if header.field_count != 8 {
            panic!("invalid field count, expected 8")
        }

        let mut b_offset = 20;
        let mut rows = [
            ObjectEffectModifierRow {
                id: ObjectEffectModifierKey::new(0),
                input_type: 0,
                map_type: 0,
                output_type: 0,
                param: [0.0; 4],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (ObjectEffectModifier) int32
            let id = ObjectEffectModifierKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // input_type: int32
            let input_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // map_type: int32
            let map_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // output_type: int32
            let output_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // param: float[4]
            let param = {
                let mut a = [0.0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = ObjectEffectModifierRow {
                id,
                input_type,
                map_type,
                output_type,
                param,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> ObjectEffectModifier {
        ObjectEffectModifier {
            rows: self.rows.iter().map(|s| ObjectEffectModifierRow {
                id: s.id,
                input_type: s.input_type,
                map_type: s.map_type,
                output_type: s.output_type,
                param: s.param,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ObjectEffectModifierKey {
    pub id: i32
}

impl ObjectEffectModifierKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for ObjectEffectModifierKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for ObjectEffectModifierKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for ObjectEffectModifierKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for ObjectEffectModifierKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ObjectEffectModifierKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ObjectEffectModifierRow {
    pub id: ObjectEffectModifierKey,
    pub input_type: i32,
    pub map_type: i32,
    pub output_type: i32,
    pub param: [f32; 4],
}

