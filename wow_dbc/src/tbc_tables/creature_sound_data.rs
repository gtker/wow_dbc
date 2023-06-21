use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::footstep_terrain_lookup::*;
use crate::tbc_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CreatureSoundData {
    pub rows: Vec<CreatureSoundDataRow>,
}

impl DbcTable for CreatureSoundData {
    type Row = CreatureSoundDataRow;

    fn filename() -> &'static str { "CreatureSoundData.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 148 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 148,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 37 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 37,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (CreatureSoundData) int32
            let id = CreatureSoundDataKey::new(crate::util::read_i32_le(chunk)?);

            // sound_exertion_id: foreign_key (SoundEntries) int32
            let sound_exertion_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_exertion_critical_id: foreign_key (SoundEntries) int32
            let sound_exertion_critical_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_injury_id: foreign_key (SoundEntries) int32
            let sound_injury_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_injury_critical_id: foreign_key (SoundEntries) int32
            let sound_injury_critical_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_injury_crushing_blow_id: int32
            let sound_injury_crushing_blow_id = crate::util::read_i32_le(chunk)?;

            // sound_death_id: foreign_key (SoundEntries) int32
            let sound_death_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_stun_id: foreign_key (SoundEntries) int32
            let sound_stun_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_stand_id: foreign_key (SoundEntries) int32
            let sound_stand_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_footstep_id: foreign_key (FootstepTerrainLookup) int32
            let sound_footstep_id = FootstepTerrainLookupKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_aggro_id: foreign_key (SoundEntries) int32
            let sound_aggro_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_wing_flap_id: foreign_key (SoundEntries) int32
            let sound_wing_flap_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_wing_glide_id: foreign_key (SoundEntries) int32
            let sound_wing_glide_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_alert_id: foreign_key (SoundEntries) int32
            let sound_alert_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_fidget: int32[5]
            let sound_fidget = crate::util::read_array_i32::<5>(chunk)?;

            // custom_attack: int32[4]
            let custom_attack = crate::util::read_array_i32::<4>(chunk)?;

            // n_p_c_sound_id: int32
            let n_p_c_sound_id = crate::util::read_i32_le(chunk)?;

            // loop_sound_id: foreign_key (SoundEntries) int32
            let loop_sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // creature_impact_type: int32
            let creature_impact_type = crate::util::read_i32_le(chunk)?;

            // sound_jump_start_id: foreign_key (SoundEntries) int32
            let sound_jump_start_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_jump_end_id: foreign_key (SoundEntries) int32
            let sound_jump_end_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_pet_attack_id: foreign_key (SoundEntries) int32
            let sound_pet_attack_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_pet_order_id: foreign_key (SoundEntries) int32
            let sound_pet_order_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_pet_dismiss_id: foreign_key (SoundEntries) int32
            let sound_pet_dismiss_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // fidget_delay_seconds_min: float
            let fidget_delay_seconds_min = crate::util::read_f32_le(chunk)?;

            // fidget_delay_seconds_max: float
            let fidget_delay_seconds_max = crate::util::read_f32_le(chunk)?;

            // birth_sound_id: foreign_key (SoundEntries) int32
            let birth_sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // spell_cast_directed_sound_id: foreign_key (SoundEntries) int32
            let spell_cast_directed_sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // submerge_sound_id: foreign_key (SoundEntries) int32
            let submerge_sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // submerged_sound_id: foreign_key (SoundEntries) int32
            let submerged_sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(CreatureSoundDataRow {
                id,
                sound_exertion_id,
                sound_exertion_critical_id,
                sound_injury_id,
                sound_injury_critical_id,
                sound_injury_crushing_blow_id,
                sound_death_id,
                sound_stun_id,
                sound_stand_id,
                sound_footstep_id,
                sound_aggro_id,
                sound_wing_flap_id,
                sound_wing_glide_id,
                sound_alert_id,
                sound_fidget,
                custom_attack,
                n_p_c_sound_id,
                loop_sound_id,
                creature_impact_type,
                sound_jump_start_id,
                sound_jump_end_id,
                sound_pet_attack_id,
                sound_pet_order_id,
                sound_pet_dismiss_id,
                fidget_delay_seconds_min,
                fidget_delay_seconds_max,
                birth_sound_id,
                spell_cast_directed_sound_id,
                submerge_sound_id,
                submerged_sound_id,
            });
        }

        Ok(CreatureSoundData { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 37,
            record_size: 148,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (CreatureSoundData) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_exertion_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_exertion_id.id as i32).to_le_bytes())?;

            // sound_exertion_critical_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_exertion_critical_id.id as i32).to_le_bytes())?;

            // sound_injury_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_injury_id.id as i32).to_le_bytes())?;

            // sound_injury_critical_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_injury_critical_id.id as i32).to_le_bytes())?;

            // sound_injury_crushing_blow_id: int32
            b.write_all(&row.sound_injury_crushing_blow_id.to_le_bytes())?;

            // sound_death_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_death_id.id as i32).to_le_bytes())?;

            // sound_stun_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_stun_id.id as i32).to_le_bytes())?;

            // sound_stand_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_stand_id.id as i32).to_le_bytes())?;

            // sound_footstep_id: foreign_key (FootstepTerrainLookup) int32
            b.write_all(&(row.sound_footstep_id.id as i32).to_le_bytes())?;

            // sound_aggro_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_aggro_id.id as i32).to_le_bytes())?;

            // sound_wing_flap_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_wing_flap_id.id as i32).to_le_bytes())?;

            // sound_wing_glide_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_wing_glide_id.id as i32).to_le_bytes())?;

            // sound_alert_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_alert_id.id as i32).to_le_bytes())?;

            // sound_fidget: int32[5]
            for i in row.sound_fidget {
                b.write_all(&i.to_le_bytes())?;
            }


            // custom_attack: int32[4]
            for i in row.custom_attack {
                b.write_all(&i.to_le_bytes())?;
            }


            // n_p_c_sound_id: int32
            b.write_all(&row.n_p_c_sound_id.to_le_bytes())?;

            // loop_sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.loop_sound_id.id as i32).to_le_bytes())?;

            // creature_impact_type: int32
            b.write_all(&row.creature_impact_type.to_le_bytes())?;

            // sound_jump_start_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_jump_start_id.id as i32).to_le_bytes())?;

            // sound_jump_end_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_jump_end_id.id as i32).to_le_bytes())?;

            // sound_pet_attack_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_pet_attack_id.id as i32).to_le_bytes())?;

            // sound_pet_order_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_pet_order_id.id as i32).to_le_bytes())?;

            // sound_pet_dismiss_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_pet_dismiss_id.id as i32).to_le_bytes())?;

            // fidget_delay_seconds_min: float
            b.write_all(&row.fidget_delay_seconds_min.to_le_bytes())?;

            // fidget_delay_seconds_max: float
            b.write_all(&row.fidget_delay_seconds_max.to_le_bytes())?;

            // birth_sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.birth_sound_id.id as i32).to_le_bytes())?;

            // spell_cast_directed_sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.spell_cast_directed_sound_id.id as i32).to_le_bytes())?;

            // submerge_sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.submerge_sound_id.id as i32).to_le_bytes())?;

            // submerged_sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.submerged_sound_id.id as i32).to_le_bytes())?;

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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstCreatureSoundData<const S: usize> {
    pub rows: [CreatureSoundDataRow; S],
}

impl<const S: usize> ConstCreatureSoundData<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 148 {
            panic!("invalid record size, expected 148")
        }

        if header.field_count != 37 {
            panic!("invalid field count, expected 37")
        }

        let mut b_offset = 20;
        let mut rows = [
            CreatureSoundDataRow {
                id: CreatureSoundDataKey::new(0),
                sound_exertion_id: SoundEntriesKey::new(0),
                sound_exertion_critical_id: SoundEntriesKey::new(0),
                sound_injury_id: SoundEntriesKey::new(0),
                sound_injury_critical_id: SoundEntriesKey::new(0),
                sound_injury_crushing_blow_id: 0,
                sound_death_id: SoundEntriesKey::new(0),
                sound_stun_id: SoundEntriesKey::new(0),
                sound_stand_id: SoundEntriesKey::new(0),
                sound_footstep_id: FootstepTerrainLookupKey::new(0),
                sound_aggro_id: SoundEntriesKey::new(0),
                sound_wing_flap_id: SoundEntriesKey::new(0),
                sound_wing_glide_id: SoundEntriesKey::new(0),
                sound_alert_id: SoundEntriesKey::new(0),
                sound_fidget: [0; 5],
                custom_attack: [0; 4],
                n_p_c_sound_id: 0,
                loop_sound_id: SoundEntriesKey::new(0),
                creature_impact_type: 0,
                sound_jump_start_id: SoundEntriesKey::new(0),
                sound_jump_end_id: SoundEntriesKey::new(0),
                sound_pet_attack_id: SoundEntriesKey::new(0),
                sound_pet_order_id: SoundEntriesKey::new(0),
                sound_pet_dismiss_id: SoundEntriesKey::new(0),
                fidget_delay_seconds_min: 0.0,
                fidget_delay_seconds_max: 0.0,
                birth_sound_id: SoundEntriesKey::new(0),
                spell_cast_directed_sound_id: SoundEntriesKey::new(0),
                submerge_sound_id: SoundEntriesKey::new(0),
                submerged_sound_id: SoundEntriesKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (CreatureSoundData) int32
            let id = CreatureSoundDataKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_exertion_id: foreign_key (SoundEntries) int32
            let sound_exertion_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_exertion_critical_id: foreign_key (SoundEntries) int32
            let sound_exertion_critical_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_injury_id: foreign_key (SoundEntries) int32
            let sound_injury_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_injury_critical_id: foreign_key (SoundEntries) int32
            let sound_injury_critical_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_injury_crushing_blow_id: int32
            let sound_injury_crushing_blow_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // sound_death_id: foreign_key (SoundEntries) int32
            let sound_death_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_stun_id: foreign_key (SoundEntries) int32
            let sound_stun_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_stand_id: foreign_key (SoundEntries) int32
            let sound_stand_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_footstep_id: foreign_key (FootstepTerrainLookup) int32
            let sound_footstep_id = FootstepTerrainLookupKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_aggro_id: foreign_key (SoundEntries) int32
            let sound_aggro_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_wing_flap_id: foreign_key (SoundEntries) int32
            let sound_wing_flap_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_wing_glide_id: foreign_key (SoundEntries) int32
            let sound_wing_glide_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_alert_id: foreign_key (SoundEntries) int32
            let sound_alert_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_fidget: int32[5]
            let sound_fidget = {
                let mut a = [0; 5];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // custom_attack: int32[4]
            let custom_attack = {
                let mut a = [0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // n_p_c_sound_id: int32
            let n_p_c_sound_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // loop_sound_id: foreign_key (SoundEntries) int32
            let loop_sound_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // creature_impact_type: int32
            let creature_impact_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // sound_jump_start_id: foreign_key (SoundEntries) int32
            let sound_jump_start_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_jump_end_id: foreign_key (SoundEntries) int32
            let sound_jump_end_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_pet_attack_id: foreign_key (SoundEntries) int32
            let sound_pet_attack_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_pet_order_id: foreign_key (SoundEntries) int32
            let sound_pet_order_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_pet_dismiss_id: foreign_key (SoundEntries) int32
            let sound_pet_dismiss_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // fidget_delay_seconds_min: float
            let fidget_delay_seconds_min = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // fidget_delay_seconds_max: float
            let fidget_delay_seconds_max = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // birth_sound_id: foreign_key (SoundEntries) int32
            let birth_sound_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // spell_cast_directed_sound_id: foreign_key (SoundEntries) int32
            let spell_cast_directed_sound_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // submerge_sound_id: foreign_key (SoundEntries) int32
            let submerge_sound_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // submerged_sound_id: foreign_key (SoundEntries) int32
            let submerged_sound_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = CreatureSoundDataRow {
                id,
                sound_exertion_id,
                sound_exertion_critical_id,
                sound_injury_id,
                sound_injury_critical_id,
                sound_injury_crushing_blow_id,
                sound_death_id,
                sound_stun_id,
                sound_stand_id,
                sound_footstep_id,
                sound_aggro_id,
                sound_wing_flap_id,
                sound_wing_glide_id,
                sound_alert_id,
                sound_fidget,
                custom_attack,
                n_p_c_sound_id,
                loop_sound_id,
                creature_impact_type,
                sound_jump_start_id,
                sound_jump_end_id,
                sound_pet_attack_id,
                sound_pet_order_id,
                sound_pet_dismiss_id,
                fidget_delay_seconds_min,
                fidget_delay_seconds_max,
                birth_sound_id,
                spell_cast_directed_sound_id,
                submerge_sound_id,
                submerged_sound_id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CreatureSoundData {
        CreatureSoundData {
            rows: self.rows.iter().map(|s| CreatureSoundDataRow {
                id: s.id,
                sound_exertion_id: s.sound_exertion_id,
                sound_exertion_critical_id: s.sound_exertion_critical_id,
                sound_injury_id: s.sound_injury_id,
                sound_injury_critical_id: s.sound_injury_critical_id,
                sound_injury_crushing_blow_id: s.sound_injury_crushing_blow_id,
                sound_death_id: s.sound_death_id,
                sound_stun_id: s.sound_stun_id,
                sound_stand_id: s.sound_stand_id,
                sound_footstep_id: s.sound_footstep_id,
                sound_aggro_id: s.sound_aggro_id,
                sound_wing_flap_id: s.sound_wing_flap_id,
                sound_wing_glide_id: s.sound_wing_glide_id,
                sound_alert_id: s.sound_alert_id,
                sound_fidget: s.sound_fidget,
                custom_attack: s.custom_attack,
                n_p_c_sound_id: s.n_p_c_sound_id,
                loop_sound_id: s.loop_sound_id,
                creature_impact_type: s.creature_impact_type,
                sound_jump_start_id: s.sound_jump_start_id,
                sound_jump_end_id: s.sound_jump_end_id,
                sound_pet_attack_id: s.sound_pet_attack_id,
                sound_pet_order_id: s.sound_pet_order_id,
                sound_pet_dismiss_id: s.sound_pet_dismiss_id,
                fidget_delay_seconds_min: s.fidget_delay_seconds_min,
                fidget_delay_seconds_max: s.fidget_delay_seconds_max,
                birth_sound_id: s.birth_sound_id,
                spell_cast_directed_sound_id: s.spell_cast_directed_sound_id,
                submerge_sound_id: s.submerge_sound_id,
                submerged_sound_id: s.submerged_sound_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CreatureSoundDataKey {
    pub id: i32
}

impl CreatureSoundDataKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for CreatureSoundDataKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for CreatureSoundDataKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for CreatureSoundDataKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CreatureSoundDataRow {
    pub id: CreatureSoundDataKey,
    pub sound_exertion_id: SoundEntriesKey,
    pub sound_exertion_critical_id: SoundEntriesKey,
    pub sound_injury_id: SoundEntriesKey,
    pub sound_injury_critical_id: SoundEntriesKey,
    pub sound_injury_crushing_blow_id: i32,
    pub sound_death_id: SoundEntriesKey,
    pub sound_stun_id: SoundEntriesKey,
    pub sound_stand_id: SoundEntriesKey,
    pub sound_footstep_id: FootstepTerrainLookupKey,
    pub sound_aggro_id: SoundEntriesKey,
    pub sound_wing_flap_id: SoundEntriesKey,
    pub sound_wing_glide_id: SoundEntriesKey,
    pub sound_alert_id: SoundEntriesKey,
    pub sound_fidget: [i32; 5],
    pub custom_attack: [i32; 4],
    pub n_p_c_sound_id: i32,
    pub loop_sound_id: SoundEntriesKey,
    pub creature_impact_type: i32,
    pub sound_jump_start_id: SoundEntriesKey,
    pub sound_jump_end_id: SoundEntriesKey,
    pub sound_pet_attack_id: SoundEntriesKey,
    pub sound_pet_order_id: SoundEntriesKey,
    pub sound_pet_dismiss_id: SoundEntriesKey,
    pub fidget_delay_seconds_min: f32,
    pub fidget_delay_seconds_max: f32,
    pub birth_sound_id: SoundEntriesKey,
    pub spell_cast_directed_sound_id: SoundEntriesKey,
    pub submerge_sound_id: SoundEntriesKey,
    pub submerged_sound_id: SoundEntriesKey,
}

