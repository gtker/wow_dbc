use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::animation_data::AnimationDataKey;
use crate::vanilla_tables::camera_shakes::CameraShakesKey;
use crate::vanilla_tables::sound_entries::SoundEntriesKey;
use crate::vanilla_tables::spell_visual_effect_name::SpellVisualEffectNameKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellVisualKit {
    pub rows: Vec<SpellVisualKitRow>,
}

impl DbcTable for SpellVisualKit {
    type Row = SpellVisualKitRow;

    const FILENAME: &'static str = "SpellVisualKit.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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

impl TryFrom<i8> for SpellVisualKitKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for SpellVisualKitKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for SpellVisualKitKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
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

