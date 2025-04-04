use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::sound_entries::SoundEntriesKey;
use crate::wrath_tables::spell_visual_kit::SpellVisualKitKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellVisual {
    pub rows: Vec<SpellVisualRow>,
}

impl DbcTable for SpellVisual {
    type Row = SpellVisualRow;

    const FILENAME: &'static str = "SpellVisual.dbc";
    const FIELD_COUNT: usize = 32;
    const ROW_SIZE: usize = 128;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
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
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
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
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellVisualKey {
    pub id: i32
}

impl SpellVisualKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

impl TryFrom<u32> for SpellVisualKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SpellVisualKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for SpellVisualKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SpellVisualKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SpellVisualKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell_visual() {
        let mut file = File::open("../wrath-dbc/SpellVisual.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SpellVisual::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellVisual::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
