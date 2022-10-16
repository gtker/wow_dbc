use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::animation_data::*;
use crate::tbc_tables::sound_entries::*;
use crate::tbc_tables::spell_effect_camera_shakes::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellVisualKit {
    pub rows: Vec<SpellVisualKitRow>,
}

impl DbcTable for SpellVisualKit {
    type Row = SpellVisualKitRow;

    fn filename() -> &'static str { "SpellVisualKit.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 152 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 152,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 38 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 38,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SpellVisualKit) int32
            let id = SpellVisualKitKey::new(crate::util::read_i32_le(chunk)?);

            // kit_type: int32
            let kit_type = crate::util::read_i32_le(chunk)?;

            // anim_id: foreign_key (AnimationData) int32
            let anim_id = AnimationDataKey::new(crate::util::read_i32_le(chunk)?.into());

            // head_effect: int32
            let head_effect = crate::util::read_i32_le(chunk)?;

            // chest_effect: int32
            let chest_effect = crate::util::read_i32_le(chunk)?;

            // base_effect: int32
            let base_effect = crate::util::read_i32_le(chunk)?;

            // left_hand_effect: int32
            let left_hand_effect = crate::util::read_i32_le(chunk)?;

            // right_hand_effect: int32
            let right_hand_effect = crate::util::read_i32_le(chunk)?;

            // breath_effect: int32
            let breath_effect = crate::util::read_i32_le(chunk)?;

            // left_weapon_effect: int32
            let left_weapon_effect = crate::util::read_i32_le(chunk)?;

            // right_weapon_effect: int32
            let right_weapon_effect = crate::util::read_i32_le(chunk)?;

            // special_effect: int32[3]
            let special_effect = crate::util::read_array_i32::<3>(chunk)?;

            // world_effect: int32
            let world_effect = crate::util::read_i32_le(chunk)?;

            // sound_id: foreign_key (SoundEntries) int32
            let sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // shake_id: foreign_key (SpellEffectCameraShakes) int32
            let shake_id = SpellEffectCameraShakesKey::new(crate::util::read_i32_le(chunk)?.into());

            // char_proc: int32[4]
            let char_proc = crate::util::read_array_i32::<4>(chunk)?;

            // char_param_zero: float[4]
            let char_param_zero = crate::util::read_array_f32::<4>(chunk)?;

            // char_param_one: float[4]
            let char_param_one = crate::util::read_array_f32::<4>(chunk)?;

            // char_param_two: float[4]
            let char_param_two = crate::util::read_array_f32::<4>(chunk)?;

            // char_param_three: float[4]
            let char_param_three = crate::util::read_array_f32::<4>(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(SpellVisualKitRow {
                id,
                kit_type,
                anim_id,
                head_effect,
                chest_effect,
                base_effect,
                left_hand_effect,
                right_hand_effect,
                breath_effect,
                left_weapon_effect,
                right_weapon_effect,
                special_effect,
                world_effect,
                sound_id,
                shake_id,
                char_proc,
                char_param_zero,
                char_param_one,
                char_param_two,
                char_param_three,
                flags,
            });
        }

        Ok(SpellVisualKit { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 38,
            record_size: 152,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SpellVisualKit) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // kit_type: int32
            b.write_all(&row.kit_type.to_le_bytes())?;

            // anim_id: foreign_key (AnimationData) int32
            b.write_all(&(row.anim_id.id as i32).to_le_bytes())?;

            // head_effect: int32
            b.write_all(&row.head_effect.to_le_bytes())?;

            // chest_effect: int32
            b.write_all(&row.chest_effect.to_le_bytes())?;

            // base_effect: int32
            b.write_all(&row.base_effect.to_le_bytes())?;

            // left_hand_effect: int32
            b.write_all(&row.left_hand_effect.to_le_bytes())?;

            // right_hand_effect: int32
            b.write_all(&row.right_hand_effect.to_le_bytes())?;

            // breath_effect: int32
            b.write_all(&row.breath_effect.to_le_bytes())?;

            // left_weapon_effect: int32
            b.write_all(&row.left_weapon_effect.to_le_bytes())?;

            // right_weapon_effect: int32
            b.write_all(&row.right_weapon_effect.to_le_bytes())?;

            // special_effect: int32[3]
            for i in row.special_effect {
                b.write_all(&i.to_le_bytes())?;
            }


            // world_effect: int32
            b.write_all(&row.world_effect.to_le_bytes())?;

            // sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_id.id as i32).to_le_bytes())?;

            // shake_id: foreign_key (SpellEffectCameraShakes) int32
            b.write_all(&(row.shake_id.id as i32).to_le_bytes())?;

            // char_proc: int32[4]
            for i in row.char_proc {
                b.write_all(&i.to_le_bytes())?;
            }


            // char_param_zero: float[4]
            for i in row.char_param_zero {
                b.write_all(&i.to_le_bytes())?;
            }


            // char_param_one: float[4]
            for i in row.char_param_one {
                b.write_all(&i.to_le_bytes())?;
            }


            // char_param_two: float[4]
            for i in row.char_param_two {
                b.write_all(&i.to_le_bytes())?;
            }


            // char_param_three: float[4]
            for i in row.char_param_three {
                b.write_all(&i.to_le_bytes())?;
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellVisualKit {
    type PrimaryKey = SpellVisualKitKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellVisualKitKey {
    pub id: i32
}

impl SpellVisualKitKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellVisualKitRow {
    pub id: SpellVisualKitKey,
    pub kit_type: i32,
    pub anim_id: AnimationDataKey,
    pub head_effect: i32,
    pub chest_effect: i32,
    pub base_effect: i32,
    pub left_hand_effect: i32,
    pub right_hand_effect: i32,
    pub breath_effect: i32,
    pub left_weapon_effect: i32,
    pub right_weapon_effect: i32,
    pub special_effect: [i32; 3],
    pub world_effect: i32,
    pub sound_id: SoundEntriesKey,
    pub shake_id: SpellEffectCameraShakesKey,
    pub char_proc: [i32; 4],
    pub char_param_zero: [f32; 4],
    pub char_param_one: [f32; 4],
    pub char_param_two: [f32; 4],
    pub char_param_three: [f32; 4],
    pub flags: i32,
}

