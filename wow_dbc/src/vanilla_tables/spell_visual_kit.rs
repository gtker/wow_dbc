use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::animation_data::*;
use crate::vanilla_tables::camera_shakes::*;
use crate::vanilla_tables::sound_entries::*;
use crate::vanilla_tables::spell_visual_effect_name::*;

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

        if header.record_size != 140 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 140,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 35 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 35,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SpellVisualKit) uint32
            let id = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?);

            // start_anim: foreign_key (AnimationData) uint32
            let start_anim = AnimationDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // anim: foreign_key (AnimationData) uint32
            let anim = AnimationDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // head_effect: foreign_key (SpellVisualEffectName) uint32
            let head_effect = SpellVisualEffectNameKey::new(crate::util::read_u32_le(chunk)?.into());

            // chest_effect: foreign_key (SpellVisualEffectName) uint32
            let chest_effect = SpellVisualEffectNameKey::new(crate::util::read_u32_le(chunk)?.into());

            // base_effect: foreign_key (SpellVisualEffectName) uint32
            let base_effect = SpellVisualEffectNameKey::new(crate::util::read_u32_le(chunk)?.into());

            // left_hand_effect: foreign_key (SpellVisualEffectName) uint32
            let left_hand_effect = SpellVisualEffectNameKey::new(crate::util::read_u32_le(chunk)?.into());

            // right_hand_effect: foreign_key (SpellVisualEffectName) uint32
            let right_hand_effect = SpellVisualEffectNameKey::new(crate::util::read_u32_le(chunk)?.into());

            // breath_effect: foreign_key (SpellVisualEffectName) uint32
            let breath_effect = SpellVisualEffectNameKey::new(crate::util::read_u32_le(chunk)?.into());

            // left_weapon_effect: foreign_key (SpellVisualEffectName) uint32
            let left_weapon_effect = SpellVisualEffectNameKey::new(crate::util::read_u32_le(chunk)?.into());

            // right_weapon_effect: foreign_key (SpellVisualEffectName) uint32
            let right_weapon_effect = SpellVisualEffectNameKey::new(crate::util::read_u32_le(chunk)?.into());

            // special_effects: uint32[3]
            let special_effects = crate::util::read_array_u32::<3>(chunk)?;

            // world_effect: foreign_key (SpellVisualEffectName) uint32
            let world_effect = SpellVisualEffectNameKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound: foreign_key (SoundEntries) uint32
            let sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // shake: foreign_key (CameraShakes) uint32
            let shake = CameraShakesKey::new(crate::util::read_u32_le(chunk)?.into());

            // char_proc: uint32[4]
            let char_proc = crate::util::read_array_u32::<4>(chunk)?;

            // char_param_zero: float[4]
            let char_param_zero = crate::util::read_array_f32::<4>(chunk)?;

            // char_param_one: float[4]
            let char_param_one = crate::util::read_array_f32::<4>(chunk)?;

            // char_param_two: float[4]
            let char_param_two = crate::util::read_array_f32::<4>(chunk)?;

            // unknown1_pad: uint32
            let unknown1_pad = crate::util::read_u32_le(chunk)?;

            // unknown2_pad: uint32
            let unknown2_pad = crate::util::read_u32_le(chunk)?;


            rows.push(SpellVisualKitRow {
                id,
                start_anim,
                anim,
                head_effect,
                chest_effect,
                base_effect,
                left_hand_effect,
                right_hand_effect,
                breath_effect,
                left_weapon_effect,
                right_weapon_effect,
                special_effects,
                world_effect,
                sound,
                shake,
                char_proc,
                char_param_zero,
                char_param_one,
                char_param_two,
                unknown1_pad,
                unknown2_pad,
            });
        }

        Ok(SpellVisualKit { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 35,
            record_size: 140,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SpellVisualKit) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // start_anim: foreign_key (AnimationData) uint32
            b.write_all(&(row.start_anim.id as u32).to_le_bytes())?;

            // anim: foreign_key (AnimationData) uint32
            b.write_all(&(row.anim.id as u32).to_le_bytes())?;

            // head_effect: foreign_key (SpellVisualEffectName) uint32
            b.write_all(&(row.head_effect.id as u32).to_le_bytes())?;

            // chest_effect: foreign_key (SpellVisualEffectName) uint32
            b.write_all(&(row.chest_effect.id as u32).to_le_bytes())?;

            // base_effect: foreign_key (SpellVisualEffectName) uint32
            b.write_all(&(row.base_effect.id as u32).to_le_bytes())?;

            // left_hand_effect: foreign_key (SpellVisualEffectName) uint32
            b.write_all(&(row.left_hand_effect.id as u32).to_le_bytes())?;

            // right_hand_effect: foreign_key (SpellVisualEffectName) uint32
            b.write_all(&(row.right_hand_effect.id as u32).to_le_bytes())?;

            // breath_effect: foreign_key (SpellVisualEffectName) uint32
            b.write_all(&(row.breath_effect.id as u32).to_le_bytes())?;

            // left_weapon_effect: foreign_key (SpellVisualEffectName) uint32
            b.write_all(&(row.left_weapon_effect.id as u32).to_le_bytes())?;

            // right_weapon_effect: foreign_key (SpellVisualEffectName) uint32
            b.write_all(&(row.right_weapon_effect.id as u32).to_le_bytes())?;

            // special_effects: uint32[3]
            for i in row.special_effects {
                b.write_all(&i.to_le_bytes())?;
            }


            // world_effect: foreign_key (SpellVisualEffectName) uint32
            b.write_all(&(row.world_effect.id as u32).to_le_bytes())?;

            // sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound.id as u32).to_le_bytes())?;

            // shake: foreign_key (CameraShakes) uint32
            b.write_all(&(row.shake.id as u32).to_le_bytes())?;

            // char_proc: uint32[4]
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


            // unknown1_pad: uint32
            b.write_all(&row.unknown1_pad.to_le_bytes())?;

            // unknown2_pad: uint32
            b.write_all(&row.unknown2_pad.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellVisualKit {
    type PrimaryKey = SpellVisualKitKey;
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
pub struct ConstSpellVisualKit<const S: usize> {
    pub rows: [SpellVisualKitRow; S],
}

impl<const S: usize> ConstSpellVisualKit<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 140 {
            panic!("invalid record size, expected 140")
        }

        if header.field_count != 35 {
            panic!("invalid field count, expected 35")
        }

        let mut b_offset = 20;
        let mut rows = [
            SpellVisualKitRow {
                id: SpellVisualKitKey::new(0),
                start_anim: AnimationDataKey::new(0),
                anim: AnimationDataKey::new(0),
                head_effect: SpellVisualEffectNameKey::new(0),
                chest_effect: SpellVisualEffectNameKey::new(0),
                base_effect: SpellVisualEffectNameKey::new(0),
                left_hand_effect: SpellVisualEffectNameKey::new(0),
                right_hand_effect: SpellVisualEffectNameKey::new(0),
                breath_effect: SpellVisualEffectNameKey::new(0),
                left_weapon_effect: SpellVisualEffectNameKey::new(0),
                right_weapon_effect: SpellVisualEffectNameKey::new(0),
                special_effects: [0; 3],
                world_effect: SpellVisualEffectNameKey::new(0),
                sound: SoundEntriesKey::new(0),
                shake: CameraShakesKey::new(0),
                char_proc: [0; 4],
                char_param_zero: [0.0; 4],
                char_param_one: [0.0; 4],
                char_param_two: [0.0; 4],
                unknown1_pad: 0,
                unknown2_pad: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SpellVisualKit) uint32
            let id = SpellVisualKitKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // start_anim: foreign_key (AnimationData) uint32
            let start_anim = AnimationDataKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // anim: foreign_key (AnimationData) uint32
            let anim = AnimationDataKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // head_effect: foreign_key (SpellVisualEffectName) uint32
            let head_effect = SpellVisualEffectNameKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // chest_effect: foreign_key (SpellVisualEffectName) uint32
            let chest_effect = SpellVisualEffectNameKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // base_effect: foreign_key (SpellVisualEffectName) uint32
            let base_effect = SpellVisualEffectNameKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // left_hand_effect: foreign_key (SpellVisualEffectName) uint32
            let left_hand_effect = SpellVisualEffectNameKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // right_hand_effect: foreign_key (SpellVisualEffectName) uint32
            let right_hand_effect = SpellVisualEffectNameKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // breath_effect: foreign_key (SpellVisualEffectName) uint32
            let breath_effect = SpellVisualEffectNameKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // left_weapon_effect: foreign_key (SpellVisualEffectName) uint32
            let left_weapon_effect = SpellVisualEffectNameKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // right_weapon_effect: foreign_key (SpellVisualEffectName) uint32
            let right_weapon_effect = SpellVisualEffectNameKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // special_effects: uint32[3]
            let special_effects = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // world_effect: foreign_key (SpellVisualEffectName) uint32
            let world_effect = SpellVisualEffectNameKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound: foreign_key (SoundEntries) uint32
            let sound = SoundEntriesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // shake: foreign_key (CameraShakes) uint32
            let shake = CameraShakesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // char_proc: uint32[4]
            let char_proc = {
                let mut a = [0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // char_param_zero: float[4]
            let char_param_zero = {
                let mut a = [0.0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // char_param_one: float[4]
            let char_param_one = {
                let mut a = [0.0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // char_param_two: float[4]
            let char_param_two = {
                let mut a = [0.0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // unknown1_pad: uint32
            let unknown1_pad = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // unknown2_pad: uint32
            let unknown2_pad = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = SpellVisualKitRow {
                id,
                start_anim,
                anim,
                head_effect,
                chest_effect,
                base_effect,
                left_hand_effect,
                right_hand_effect,
                breath_effect,
                left_weapon_effect,
                right_weapon_effect,
                special_effects,
                world_effect,
                sound,
                shake,
                char_proc,
                char_param_zero,
                char_param_one,
                char_param_two,
                unknown1_pad,
                unknown2_pad,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellVisualKitKey {
    pub id: u32
}

impl SpellVisualKitKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SpellVisualKitKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SpellVisualKitKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SpellVisualKitKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SpellVisualKitRow {
    pub id: SpellVisualKitKey,
    pub start_anim: AnimationDataKey,
    pub anim: AnimationDataKey,
    pub head_effect: SpellVisualEffectNameKey,
    pub chest_effect: SpellVisualEffectNameKey,
    pub base_effect: SpellVisualEffectNameKey,
    pub left_hand_effect: SpellVisualEffectNameKey,
    pub right_hand_effect: SpellVisualEffectNameKey,
    pub breath_effect: SpellVisualEffectNameKey,
    pub left_weapon_effect: SpellVisualEffectNameKey,
    pub right_weapon_effect: SpellVisualEffectNameKey,
    pub special_effects: [u32; 3],
    pub world_effect: SpellVisualEffectNameKey,
    pub sound: SoundEntriesKey,
    pub shake: CameraShakesKey,
    pub char_proc: [u32; 4],
    pub char_param_zero: [f32; 4],
    pub char_param_one: [f32; 4],
    pub char_param_two: [f32; 4],
    pub unknown1_pad: u32,
    pub unknown2_pad: u32,
}

