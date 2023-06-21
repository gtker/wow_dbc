use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::object_effect_group::*;
use crate::wrath_tables::object_effect_modifier::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ObjectEffect {
    pub rows: Vec<ObjectEffectRow>,
}

impl DbcTable for ObjectEffect {
    type Row = ObjectEffectRow;

    fn filename() -> &'static str { "ObjectEffect.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 48 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 48,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 12 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 12,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (ObjectEffect) int32
            let id = ObjectEffectKey::new(crate::util::read_i32_le(chunk)?);

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // object_effect_group_id: foreign_key (ObjectEffectGroup) int32
            let object_effect_group_id = ObjectEffectGroupKey::new(crate::util::read_i32_le(chunk)?.into());

            // trigger_type: int32
            let trigger_type = crate::util::read_i32_le(chunk)?;

            // event_type: int32
            let event_type = crate::util::read_i32_le(chunk)?;

            // effect_rec_type: int32
            let effect_rec_type = crate::util::read_i32_le(chunk)?;

            // effect_rec_id: foreign_key (SoundKit) int32
            let effect_rec_id = crate::util::read_i32_le(chunk)?;

            // attachment: int32
            let attachment = crate::util::read_i32_le(chunk)?;

            // offset: float[3]
            let offset = crate::util::read_array_f32::<3>(chunk)?;

            // object_effect_modifier_id: foreign_key (ObjectEffectModifier) int32
            let object_effect_modifier_id = ObjectEffectModifierKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(ObjectEffectRow {
                id,
                name,
                object_effect_group_id,
                trigger_type,
                event_type,
                effect_rec_type,
                effect_rec_id,
                attachment,
                offset,
                object_effect_modifier_id,
            });
        }

        Ok(ObjectEffect { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 12,
            record_size: 48,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ObjectEffect) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref
            if !row.name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // object_effect_group_id: foreign_key (ObjectEffectGroup) int32
            b.write_all(&(row.object_effect_group_id.id as i32).to_le_bytes())?;

            // trigger_type: int32
            b.write_all(&row.trigger_type.to_le_bytes())?;

            // event_type: int32
            b.write_all(&row.event_type.to_le_bytes())?;

            // effect_rec_type: int32
            b.write_all(&row.effect_rec_type.to_le_bytes())?;

            // effect_rec_id: foreign_key (SoundKit) int32
            b.write_all(&row.effect_rec_id.to_le_bytes())?;

            // attachment: int32
            b.write_all(&row.attachment.to_le_bytes())?;

            // offset: float[3]
            for i in row.offset {
                b.write_all(&i.to_le_bytes())?;
            }


            // object_effect_modifier_id: foreign_key (ObjectEffectModifier) int32
            b.write_all(&(row.object_effect_modifier_id.id as i32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ObjectEffect {
    type PrimaryKey = ObjectEffectKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ObjectEffect {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.name.is_empty() { b.write_all(row.name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.name.is_empty() { sum += row.name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstObjectEffect<const S: usize> {
    pub rows: [ConstObjectEffectRow; S],
}

impl<const S: usize> ConstObjectEffect<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 48 {
            panic!("invalid record size, expected 48")
        }

        if header.field_count != 12 {
            panic!("invalid field count, expected 12")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstObjectEffectRow {
                id: ObjectEffectKey::new(0),
                name: "",
                object_effect_group_id: ObjectEffectGroupKey::new(0),
                trigger_type: 0,
                event_type: 0,
                effect_rec_type: 0,
                effect_rec_id: 0,
                attachment: 0,
                offset: [0.0; 3],
                object_effect_modifier_id: ObjectEffectModifierKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (ObjectEffect) int32
            let id = ObjectEffectKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // name: string_ref
            let name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // object_effect_group_id: foreign_key (ObjectEffectGroup) int32
            let object_effect_group_id = ObjectEffectGroupKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // trigger_type: int32
            let trigger_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // event_type: int32
            let event_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // effect_rec_type: int32
            let effect_rec_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // effect_rec_id: foreign_key (SoundKit) int32
            let effect_rec_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // attachment: int32
            let attachment = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // offset: float[3]
            let offset = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // object_effect_modifier_id: foreign_key (ObjectEffectModifier) int32
            let object_effect_modifier_id = ObjectEffectModifierKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = ConstObjectEffectRow {
                id,
                name,
                object_effect_group_id,
                trigger_type,
                event_type,
                effect_rec_type,
                effect_rec_id,
                attachment,
                offset,
                object_effect_modifier_id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> ObjectEffect {
        ObjectEffect {
            rows: self.rows.iter().map(|s| ObjectEffectRow {
                id: s.id,
                name: s.name.to_string(),
                object_effect_group_id: s.object_effect_group_id,
                trigger_type: s.trigger_type,
                event_type: s.event_type,
                effect_rec_type: s.effect_rec_type,
                effect_rec_id: s.effect_rec_id,
                attachment: s.attachment,
                offset: s.offset,
                object_effect_modifier_id: s.object_effect_modifier_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ObjectEffectKey {
    pub id: i32
}

impl ObjectEffectKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for ObjectEffectKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for ObjectEffectKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for ObjectEffectKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for ObjectEffectKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ObjectEffectKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ObjectEffectRow {
    pub id: ObjectEffectKey,
    pub name: String,
    pub object_effect_group_id: ObjectEffectGroupKey,
    pub trigger_type: i32,
    pub event_type: i32,
    pub effect_rec_type: i32,
    pub effect_rec_id: i32,
    pub attachment: i32,
    pub offset: [f32; 3],
    pub object_effect_modifier_id: ObjectEffectModifierKey,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstObjectEffectRow {
    pub id: ObjectEffectKey,
    pub name: &'static str,
    pub object_effect_group_id: ObjectEffectGroupKey,
    pub trigger_type: i32,
    pub event_type: i32,
    pub effect_rec_type: i32,
    pub effect_rec_id: i32,
    pub attachment: i32,
    pub offset: [f32; 3],
    pub object_effect_modifier_id: ObjectEffectModifierKey,
}

