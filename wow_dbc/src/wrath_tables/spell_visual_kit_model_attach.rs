use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::spell_visual_effect_name::*;
use crate::wrath_tables::spell_visual_kit::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellVisualKitModelAttach {
    pub rows: Vec<SpellVisualKitModelAttachRow>,
}

impl DbcTable for SpellVisualKitModelAttach {
    type Row = SpellVisualKitModelAttachRow;

    fn filename() -> &'static str { "SpellVisualKitModelAttach.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 40 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 40,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 10 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 10,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SpellVisualKitModelAttach) int32
            let id = SpellVisualKitModelAttachKey::new(crate::util::read_i32_le(chunk)?);

            // parent_spell_visual_kit_id: foreign_key (SpellVisualKit) int32
            let parent_spell_visual_kit_id = SpellVisualKitKey::new(crate::util::read_i32_le(chunk)?.into());

            // spell_visual_effect_name_id: foreign_key (SpellVisualEffectName) int32
            let spell_visual_effect_name_id = SpellVisualEffectNameKey::new(crate::util::read_i32_le(chunk)?.into());

            // attachment_id: int32
            let attachment_id = crate::util::read_i32_le(chunk)?;

            // offset: float[3]
            let offset = crate::util::read_array_f32::<3>(chunk)?;

            // yaw: float
            let yaw = crate::util::read_f32_le(chunk)?;

            // pitch: float
            let pitch = crate::util::read_f32_le(chunk)?;

            // roll: float
            let roll = crate::util::read_f32_le(chunk)?;


            rows.push(SpellVisualKitModelAttachRow {
                id,
                parent_spell_visual_kit_id,
                spell_visual_effect_name_id,
                attachment_id,
                offset,
                yaw,
                pitch,
                roll,
            });
        }

        Ok(SpellVisualKitModelAttach { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 10,
            record_size: 40,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SpellVisualKitModelAttach) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // parent_spell_visual_kit_id: foreign_key (SpellVisualKit) int32
            b.write_all(&(row.parent_spell_visual_kit_id.id as i32).to_le_bytes())?;

            // spell_visual_effect_name_id: foreign_key (SpellVisualEffectName) int32
            b.write_all(&(row.spell_visual_effect_name_id.id as i32).to_le_bytes())?;

            // attachment_id: int32
            b.write_all(&row.attachment_id.to_le_bytes())?;

            // offset: float[3]
            for i in row.offset {
                b.write_all(&i.to_le_bytes())?;
            }


            // yaw: float
            b.write_all(&row.yaw.to_le_bytes())?;

            // pitch: float
            b.write_all(&row.pitch.to_le_bytes())?;

            // roll: float
            b.write_all(&row.roll.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellVisualKitModelAttach {
    type PrimaryKey = SpellVisualKitModelAttachKey;
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
pub struct ConstSpellVisualKitModelAttach<const S: usize> {
    pub rows: [SpellVisualKitModelAttachRow; S],
}

impl<const S: usize> ConstSpellVisualKitModelAttach<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 40 {
            panic!("invalid record size, expected 40")
        }

        if header.field_count != 10 {
            panic!("invalid field count, expected 10")
        }

        let mut b_offset = 20;
        let mut rows = [
            SpellVisualKitModelAttachRow {
                id: SpellVisualKitModelAttachKey::new(0),
                parent_spell_visual_kit_id: SpellVisualKitKey::new(0),
                spell_visual_effect_name_id: SpellVisualEffectNameKey::new(0),
                attachment_id: 0,
                offset: [0.0; 3],
                yaw: 0.0,
                pitch: 0.0,
                roll: 0.0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SpellVisualKitModelAttach) int32
            let id = SpellVisualKitModelAttachKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // parent_spell_visual_kit_id: foreign_key (SpellVisualKit) int32
            let parent_spell_visual_kit_id = SpellVisualKitKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // spell_visual_effect_name_id: foreign_key (SpellVisualEffectName) int32
            let spell_visual_effect_name_id = SpellVisualEffectNameKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // attachment_id: int32
            let attachment_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
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

            // yaw: float
            let yaw = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // pitch: float
            let pitch = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // roll: float
            let roll = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = SpellVisualKitModelAttachRow {
                id,
                parent_spell_visual_kit_id,
                spell_visual_effect_name_id,
                attachment_id,
                offset,
                yaw,
                pitch,
                roll,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SpellVisualKitModelAttach {
        SpellVisualKitModelAttach {
            rows: self.rows.iter().map(|s| SpellVisualKitModelAttachRow {
                id: s.id,
                parent_spell_visual_kit_id: s.parent_spell_visual_kit_id,
                spell_visual_effect_name_id: s.spell_visual_effect_name_id,
                attachment_id: s.attachment_id,
                offset: s.offset,
                yaw: s.yaw,
                pitch: s.pitch,
                roll: s.roll,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellVisualKitModelAttachKey {
    pub id: i32
}

impl SpellVisualKitModelAttachKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SpellVisualKitModelAttachKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SpellVisualKitModelAttachKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SpellVisualKitModelAttachKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SpellVisualKitModelAttachKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SpellVisualKitModelAttachKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SpellVisualKitModelAttachRow {
    pub id: SpellVisualKitModelAttachKey,
    pub parent_spell_visual_kit_id: SpellVisualKitKey,
    pub spell_visual_effect_name_id: SpellVisualEffectNameKey,
    pub attachment_id: i32,
    pub offset: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

