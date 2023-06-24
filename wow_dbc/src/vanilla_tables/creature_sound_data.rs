use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::sound_entries::SoundEntriesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreatureSoundData {
    pub rows: Vec<CreatureSoundDataRow>,
}

impl DbcTable for CreatureSoundData {
    type Row = CreatureSoundDataRow;

    const FILENAME: &'static str = "CreatureSoundData.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 120 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 120,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 30 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 30,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (CreatureSoundData) uint32
            let id = CreatureSoundDataKey::new(crate::util::read_u32_le(chunk)?);

            // sound_exertion: foreign_key (SoundEntries) uint32
            let sound_exertion = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_exertion_critical: foreign_key (SoundEntries) uint32
            let sound_exertion_critical = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_injury: foreign_key (SoundEntries) uint32
            let sound_injury = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_injury_critical: foreign_key (SoundEntries) uint32
            let sound_injury_critical = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_injury_crushing_blow: foreign_key (SoundEntries) uint32
            let sound_injury_crushing_blow = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_death: foreign_key (SoundEntries) uint32
            let sound_death = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_stun: foreign_key (SoundEntries) uint32
            let sound_stun = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_stand: foreign_key (SoundEntries) uint32
            let sound_stand = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_footstep: foreign_key (SoundEntries) uint32
            let sound_footstep = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_aggro: foreign_key (SoundEntries) uint32
            let sound_aggro = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_wing_flap: foreign_key (SoundEntries) uint32
            let sound_wing_flap = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_wing_glide: foreign_key (SoundEntries) uint32
            let sound_wing_glide = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_alert: foreign_key (SoundEntries) uint32
            let sound_alert = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_fidget: foreign_key (SoundEntries) uint32
            let sound_fidget = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // custom_attack: uint32
            let custom_attack = crate::util::read_u32_le(chunk)?;

            // npc_sound: foreign_key (SoundEntries) uint32
            let npc_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // loop_sound: foreign_key (SoundEntries) uint32
            let loop_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // creature_impact_type: int32
            let creature_impact_type = crate::util::read_i32_le(chunk)?;

            // sound_jump_start: foreign_key (SoundEntries) uint32
            let sound_jump_start = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_jump_end: foreign_key (SoundEntries) uint32
            let sound_jump_end = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_pet_attack: foreign_key (SoundEntries) uint32
            let sound_pet_attack = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_pet_order: foreign_key (SoundEntries) uint32
            let sound_pet_order = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_pet_dismiss: foreign_key (SoundEntries) uint32
            let sound_pet_dismiss = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // fidget_delay_seconds_min: int32
            let fidget_delay_seconds_min = crate::util::read_i32_le(chunk)?;

            // fidget_delay_seconds_max: int32
            let fidget_delay_seconds_max = crate::util::read_i32_le(chunk)?;

            // birth_sound: foreign_key (SoundEntries) uint32
            let birth_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // spell_cast_directed_sound: foreign_key (SoundEntries) uint32
            let spell_cast_directed_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // submerge_sound: foreign_key (SoundEntries) uint32
            let submerge_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // submerged_sound: foreign_key (SoundEntries) uint32
            let submerged_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(CreatureSoundDataRow {
                id,
                sound_exertion,
                sound_exertion_critical,
                sound_injury,
                sound_injury_critical,
                sound_injury_crushing_blow,
                sound_death,
                sound_stun,
                sound_stand,
                sound_footstep,
                sound_aggro,
                sound_wing_flap,
                sound_wing_glide,
                sound_alert,
                sound_fidget,
                custom_attack,
                npc_sound,
                loop_sound,
                creature_impact_type,
                sound_jump_start,
                sound_jump_end,
                sound_pet_attack,
                sound_pet_order,
                sound_pet_dismiss,
                fidget_delay_seconds_min,
                fidget_delay_seconds_max,
                birth_sound,
                spell_cast_directed_sound,
                submerge_sound,
                submerged_sound,
            });
        }

        Ok(CreatureSoundData { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 30,
            record_size: 120,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (CreatureSoundData) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_exertion: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_exertion.id as u32).to_le_bytes())?;

            // sound_exertion_critical: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_exertion_critical.id as u32).to_le_bytes())?;

            // sound_injury: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_injury.id as u32).to_le_bytes())?;

            // sound_injury_critical: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_injury_critical.id as u32).to_le_bytes())?;

            // sound_injury_crushing_blow: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_injury_crushing_blow.id as u32).to_le_bytes())?;

            // sound_death: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_death.id as u32).to_le_bytes())?;

            // sound_stun: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_stun.id as u32).to_le_bytes())?;

            // sound_stand: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_stand.id as u32).to_le_bytes())?;

            // sound_footstep: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_footstep.id as u32).to_le_bytes())?;

            // sound_aggro: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_aggro.id as u32).to_le_bytes())?;

            // sound_wing_flap: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_wing_flap.id as u32).to_le_bytes())?;

            // sound_wing_glide: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_wing_glide.id as u32).to_le_bytes())?;

            // sound_alert: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_alert.id as u32).to_le_bytes())?;

            // sound_fidget: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_fidget.id as u32).to_le_bytes())?;

            // custom_attack: uint32
            b.write_all(&row.custom_attack.to_le_bytes())?;

            // npc_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.npc_sound.id as u32).to_le_bytes())?;

            // loop_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.loop_sound.id as u32).to_le_bytes())?;

            // creature_impact_type: int32
            b.write_all(&row.creature_impact_type.to_le_bytes())?;

            // sound_jump_start: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_jump_start.id as u32).to_le_bytes())?;

            // sound_jump_end: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_jump_end.id as u32).to_le_bytes())?;

            // sound_pet_attack: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_pet_attack.id as u32).to_le_bytes())?;

            // sound_pet_order: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_pet_order.id as u32).to_le_bytes())?;

            // sound_pet_dismiss: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_pet_dismiss.id as u32).to_le_bytes())?;

            // fidget_delay_seconds_min: int32
            b.write_all(&row.fidget_delay_seconds_min.to_le_bytes())?;

            // fidget_delay_seconds_max: int32
            b.write_all(&row.fidget_delay_seconds_max.to_le_bytes())?;

            // birth_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.birth_sound.id as u32).to_le_bytes())?;

            // spell_cast_directed_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.spell_cast_directed_sound.id as u32).to_le_bytes())?;

            // submerge_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.submerge_sound.id as u32).to_le_bytes())?;

            // submerged_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.submerged_sound.id as u32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for CreatureSoundData {
    type PrimaryKey = CreatureSoundDataKey;
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
pub struct CreatureSoundDataKey {
    pub id: u32
}

impl CreatureSoundDataKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for CreatureSoundDataKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for CreatureSoundDataKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for CreatureSoundDataKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<i8> for CreatureSoundDataKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for CreatureSoundDataKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for CreatureSoundDataKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreatureSoundDataRow {
    pub id: CreatureSoundDataKey,
    pub sound_exertion: SoundEntriesKey,
    pub sound_exertion_critical: SoundEntriesKey,
    pub sound_injury: SoundEntriesKey,
    pub sound_injury_critical: SoundEntriesKey,
    pub sound_injury_crushing_blow: SoundEntriesKey,
    pub sound_death: SoundEntriesKey,
    pub sound_stun: SoundEntriesKey,
    pub sound_stand: SoundEntriesKey,
    pub sound_footstep: SoundEntriesKey,
    pub sound_aggro: SoundEntriesKey,
    pub sound_wing_flap: SoundEntriesKey,
    pub sound_wing_glide: SoundEntriesKey,
    pub sound_alert: SoundEntriesKey,
    pub sound_fidget: SoundEntriesKey,
    pub custom_attack: u32,
    pub npc_sound: SoundEntriesKey,
    pub loop_sound: SoundEntriesKey,
    pub creature_impact_type: i32,
    pub sound_jump_start: SoundEntriesKey,
    pub sound_jump_end: SoundEntriesKey,
    pub sound_pet_attack: SoundEntriesKey,
    pub sound_pet_order: SoundEntriesKey,
    pub sound_pet_dismiss: SoundEntriesKey,
    pub fidget_delay_seconds_min: i32,
    pub fidget_delay_seconds_max: i32,
    pub birth_sound: SoundEntriesKey,
    pub spell_cast_directed_sound: SoundEntriesKey,
    pub submerge_sound: SoundEntriesKey,
    pub submerged_sound: SoundEntriesKey,
}

