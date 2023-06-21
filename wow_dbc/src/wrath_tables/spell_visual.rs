use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::sound_entries::*;
use crate::wrath_tables::spell_visual_kit::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellVisual {
    pub rows: Vec<SpellVisualRow>,
}

impl DbcTable for SpellVisual {
    type Row = SpellVisualRow;

    fn filename() -> &'static str { "SpellVisual.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 128 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 128,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 32,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SpellVisual) int32
            let id = SpellVisualKey::new(crate::util::read_i32_le(chunk)?);

            // precast_kit: int32
            let precast_kit = crate::util::read_i32_le(chunk)?;

            // cast_kit: int32
            let cast_kit = crate::util::read_i32_le(chunk)?;

            // impact_kit: int32
            let impact_kit = crate::util::read_i32_le(chunk)?;

            // state_kit: int32
            let state_kit = crate::util::read_i32_le(chunk)?;

            // state_done_kit: int32
            let state_done_kit = crate::util::read_i32_le(chunk)?;

            // channel_kit: int32
            let channel_kit = crate::util::read_i32_le(chunk)?;

            // has_missile: int32
            let has_missile = crate::util::read_i32_le(chunk)?;

            // missile_model: int32
            let missile_model = crate::util::read_i32_le(chunk)?;

            // missile_path_type: int32
            let missile_path_type = crate::util::read_i32_le(chunk)?;

            // missile_destination_attachment: int32
            let missile_destination_attachment = crate::util::read_i32_le(chunk)?;

            // missile_sound: int32
            let missile_sound = crate::util::read_i32_le(chunk)?;

            // anim_event_sound_id: foreign_key (SoundEntries) int32
            let anim_event_sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // caster_impact_kit: int32
            let caster_impact_kit = crate::util::read_i32_le(chunk)?;

            // target_impact_kit: int32
            let target_impact_kit = crate::util::read_i32_le(chunk)?;

            // missile_attachment: int32
            let missile_attachment = crate::util::read_i32_le(chunk)?;

            // missile_follow_ground_height: int32
            let missile_follow_ground_height = crate::util::read_i32_le(chunk)?;

            // missile_follow_ground_drop_speed: int32
            let missile_follow_ground_drop_speed = crate::util::read_i32_le(chunk)?;

            // missile_follow_ground_approach: int32
            let missile_follow_ground_approach = crate::util::read_i32_le(chunk)?;

            // missile_follow_ground_flags: int32
            let missile_follow_ground_flags = crate::util::read_i32_le(chunk)?;

            // missile_motion: int32
            let missile_motion = crate::util::read_i32_le(chunk)?;

            // missile_targeting_kit: foreign_key (SpellVisualKit) int32
            let missile_targeting_kit = SpellVisualKitKey::new(crate::util::read_i32_le(chunk)?.into());

            // instant_area_kit: int32
            let instant_area_kit = crate::util::read_i32_le(chunk)?;

            // impact_area_kit: int32
            let impact_area_kit = crate::util::read_i32_le(chunk)?;

            // persistent_area_kit: int32
            let persistent_area_kit = crate::util::read_i32_le(chunk)?;

            // missile_cast_offset: float[3]
            let missile_cast_offset = crate::util::read_array_f32::<3>(chunk)?;

            // missile_impact_offset: float[3]
            let missile_impact_offset = crate::util::read_array_f32::<3>(chunk)?;


            rows.push(SpellVisualRow {
                id,
                precast_kit,
                cast_kit,
                impact_kit,
                state_kit,
                state_done_kit,
                channel_kit,
                has_missile,
                missile_model,
                missile_path_type,
                missile_destination_attachment,
                missile_sound,
                anim_event_sound_id,
                flags,
                caster_impact_kit,
                target_impact_kit,
                missile_attachment,
                missile_follow_ground_height,
                missile_follow_ground_drop_speed,
                missile_follow_ground_approach,
                missile_follow_ground_flags,
                missile_motion,
                missile_targeting_kit,
                instant_area_kit,
                impact_area_kit,
                persistent_area_kit,
                missile_cast_offset,
                missile_impact_offset,
            });
        }

        Ok(SpellVisual { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 32,
            record_size: 128,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SpellVisual) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // precast_kit: int32
            b.write_all(&row.precast_kit.to_le_bytes())?;

            // cast_kit: int32
            b.write_all(&row.cast_kit.to_le_bytes())?;

            // impact_kit: int32
            b.write_all(&row.impact_kit.to_le_bytes())?;

            // state_kit: int32
            b.write_all(&row.state_kit.to_le_bytes())?;

            // state_done_kit: int32
            b.write_all(&row.state_done_kit.to_le_bytes())?;

            // channel_kit: int32
            b.write_all(&row.channel_kit.to_le_bytes())?;

            // has_missile: int32
            b.write_all(&row.has_missile.to_le_bytes())?;

            // missile_model: int32
            b.write_all(&row.missile_model.to_le_bytes())?;

            // missile_path_type: int32
            b.write_all(&row.missile_path_type.to_le_bytes())?;

            // missile_destination_attachment: int32
            b.write_all(&row.missile_destination_attachment.to_le_bytes())?;

            // missile_sound: int32
            b.write_all(&row.missile_sound.to_le_bytes())?;

            // anim_event_sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.anim_event_sound_id.id as i32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // caster_impact_kit: int32
            b.write_all(&row.caster_impact_kit.to_le_bytes())?;

            // target_impact_kit: int32
            b.write_all(&row.target_impact_kit.to_le_bytes())?;

            // missile_attachment: int32
            b.write_all(&row.missile_attachment.to_le_bytes())?;

            // missile_follow_ground_height: int32
            b.write_all(&row.missile_follow_ground_height.to_le_bytes())?;

            // missile_follow_ground_drop_speed: int32
            b.write_all(&row.missile_follow_ground_drop_speed.to_le_bytes())?;

            // missile_follow_ground_approach: int32
            b.write_all(&row.missile_follow_ground_approach.to_le_bytes())?;

            // missile_follow_ground_flags: int32
            b.write_all(&row.missile_follow_ground_flags.to_le_bytes())?;

            // missile_motion: int32
            b.write_all(&row.missile_motion.to_le_bytes())?;

            // missile_targeting_kit: foreign_key (SpellVisualKit) int32
            b.write_all(&(row.missile_targeting_kit.id as i32).to_le_bytes())?;

            // instant_area_kit: int32
            b.write_all(&row.instant_area_kit.to_le_bytes())?;

            // impact_area_kit: int32
            b.write_all(&row.impact_area_kit.to_le_bytes())?;

            // persistent_area_kit: int32
            b.write_all(&row.persistent_area_kit.to_le_bytes())?;

            // missile_cast_offset: float[3]
            for i in row.missile_cast_offset {
                b.write_all(&i.to_le_bytes())?;
            }


            // missile_impact_offset: float[3]
            for i in row.missile_impact_offset {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellVisual {
    type PrimaryKey = SpellVisualKey;
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
pub struct ConstSpellVisual<const S: usize> {
    pub rows: [SpellVisualRow; S],
}

impl<const S: usize> ConstSpellVisual<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 128 {
            panic!("invalid record size, expected 128")
        }

        if header.field_count != 32 {
            panic!("invalid field count, expected 32")
        }

        let mut b_offset = 20;
        let mut rows = [
            SpellVisualRow {
                id: SpellVisualKey::new(0),
                precast_kit: 0,
                cast_kit: 0,
                impact_kit: 0,
                state_kit: 0,
                state_done_kit: 0,
                channel_kit: 0,
                has_missile: 0,
                missile_model: 0,
                missile_path_type: 0,
                missile_destination_attachment: 0,
                missile_sound: 0,
                anim_event_sound_id: SoundEntriesKey::new(0),
                flags: 0,
                caster_impact_kit: 0,
                target_impact_kit: 0,
                missile_attachment: 0,
                missile_follow_ground_height: 0,
                missile_follow_ground_drop_speed: 0,
                missile_follow_ground_approach: 0,
                missile_follow_ground_flags: 0,
                missile_motion: 0,
                missile_targeting_kit: SpellVisualKitKey::new(0),
                instant_area_kit: 0,
                impact_area_kit: 0,
                persistent_area_kit: 0,
                missile_cast_offset: [0.0; 3],
                missile_impact_offset: [0.0; 3],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SpellVisual) int32
            let id = SpellVisualKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // precast_kit: int32
            let precast_kit = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // cast_kit: int32
            let cast_kit = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // impact_kit: int32
            let impact_kit = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // state_kit: int32
            let state_kit = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // state_done_kit: int32
            let state_done_kit = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // channel_kit: int32
            let channel_kit = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // has_missile: int32
            let has_missile = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_model: int32
            let missile_model = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_path_type: int32
            let missile_path_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_destination_attachment: int32
            let missile_destination_attachment = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_sound: int32
            let missile_sound = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // anim_event_sound_id: foreign_key (SoundEntries) int32
            let anim_event_sound_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // caster_impact_kit: int32
            let caster_impact_kit = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // target_impact_kit: int32
            let target_impact_kit = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_attachment: int32
            let missile_attachment = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_follow_ground_height: int32
            let missile_follow_ground_height = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_follow_ground_drop_speed: int32
            let missile_follow_ground_drop_speed = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_follow_ground_approach: int32
            let missile_follow_ground_approach = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_follow_ground_flags: int32
            let missile_follow_ground_flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_motion: int32
            let missile_motion = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_targeting_kit: foreign_key (SpellVisualKit) int32
            let missile_targeting_kit = SpellVisualKitKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // instant_area_kit: int32
            let instant_area_kit = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // impact_area_kit: int32
            let impact_area_kit = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // persistent_area_kit: int32
            let persistent_area_kit = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_cast_offset: float[3]
            let missile_cast_offset = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // missile_impact_offset: float[3]
            let missile_impact_offset = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = SpellVisualRow {
                id,
                precast_kit,
                cast_kit,
                impact_kit,
                state_kit,
                state_done_kit,
                channel_kit,
                has_missile,
                missile_model,
                missile_path_type,
                missile_destination_attachment,
                missile_sound,
                anim_event_sound_id,
                flags,
                caster_impact_kit,
                target_impact_kit,
                missile_attachment,
                missile_follow_ground_height,
                missile_follow_ground_drop_speed,
                missile_follow_ground_approach,
                missile_follow_ground_flags,
                missile_motion,
                missile_targeting_kit,
                instant_area_kit,
                impact_area_kit,
                persistent_area_kit,
                missile_cast_offset,
                missile_impact_offset,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SpellVisual {
        SpellVisual {
            rows: self.rows.iter().map(|s| SpellVisualRow {
                id: s.id,
                precast_kit: s.precast_kit,
                cast_kit: s.cast_kit,
                impact_kit: s.impact_kit,
                state_kit: s.state_kit,
                state_done_kit: s.state_done_kit,
                channel_kit: s.channel_kit,
                has_missile: s.has_missile,
                missile_model: s.missile_model,
                missile_path_type: s.missile_path_type,
                missile_destination_attachment: s.missile_destination_attachment,
                missile_sound: s.missile_sound,
                anim_event_sound_id: s.anim_event_sound_id,
                flags: s.flags,
                caster_impact_kit: s.caster_impact_kit,
                target_impact_kit: s.target_impact_kit,
                missile_attachment: s.missile_attachment,
                missile_follow_ground_height: s.missile_follow_ground_height,
                missile_follow_ground_drop_speed: s.missile_follow_ground_drop_speed,
                missile_follow_ground_approach: s.missile_follow_ground_approach,
                missile_follow_ground_flags: s.missile_follow_ground_flags,
                missile_motion: s.missile_motion,
                missile_targeting_kit: s.missile_targeting_kit,
                instant_area_kit: s.instant_area_kit,
                impact_area_kit: s.impact_area_kit,
                persistent_area_kit: s.persistent_area_kit,
                missile_cast_offset: s.missile_cast_offset,
                missile_impact_offset: s.missile_impact_offset,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellVisualKey {
    pub id: i32
}

impl SpellVisualKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SpellVisualKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SpellVisualKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SpellVisualKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SpellVisualKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SpellVisualKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SpellVisualRow {
    pub id: SpellVisualKey,
    pub precast_kit: i32,
    pub cast_kit: i32,
    pub impact_kit: i32,
    pub state_kit: i32,
    pub state_done_kit: i32,
    pub channel_kit: i32,
    pub has_missile: i32,
    pub missile_model: i32,
    pub missile_path_type: i32,
    pub missile_destination_attachment: i32,
    pub missile_sound: i32,
    pub anim_event_sound_id: SoundEntriesKey,
    pub flags: i32,
    pub caster_impact_kit: i32,
    pub target_impact_kit: i32,
    pub missile_attachment: i32,
    pub missile_follow_ground_height: i32,
    pub missile_follow_ground_drop_speed: i32,
    pub missile_follow_ground_approach: i32,
    pub missile_follow_ground_flags: i32,
    pub missile_motion: i32,
    pub missile_targeting_kit: SpellVisualKitKey,
    pub instant_area_kit: i32,
    pub impact_area_kit: i32,
    pub persistent_area_kit: i32,
    pub missile_cast_offset: [f32; 3],
    pub missile_impact_offset: [f32; 3],
}

