use crate::DbcTable;
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::area_table::AreaTableKey;
use crate::wrath_tables::sound_ambience::SoundAmbienceKey;
use crate::wrath_tables::sound_provider_preferences::SoundProviderPreferencesKey;
use crate::wrath_tables::wmo_area_table::WMOAreaTableKey;
use crate::wrath_tables::zone_intro_music_table::ZoneIntroMusicTableKey;
use crate::wrath_tables::zone_music::ZoneMusicKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldStateZoneSounds {
    pub rows: Vec<WorldStateZoneSoundsRow>,
}

impl DbcTable for WorldStateZoneSounds {
    type Row = WorldStateZoneSoundsRow;

    const FILENAME: &'static str = "WorldStateZoneSounds.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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

            // world_state_id: foreign_key (WorldState) int32
            let world_state_id = crate::util::read_i32_le(chunk)?;

            // world_state_value: int32
            let world_state_value = crate::util::read_i32_le(chunk)?;

            // area_id: foreign_key (AreaTable) int32
            let area_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // w_m_o_area_id: foreign_key (WMOAreaTable) int32
            let w_m_o_area_id = WMOAreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // zone_intro_music_id: foreign_key (ZoneIntroMusicTable) int32
            let zone_intro_music_id = ZoneIntroMusicTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // zone_music_id: foreign_key (ZoneMusic) int32
            let zone_music_id = ZoneMusicKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_ambience_id: foreign_key (SoundAmbience) int32
            let sound_ambience_id = SoundAmbienceKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_provider_preferences_id: foreign_key (SoundProviderPreferences) int32
            let sound_provider_preferences_id = SoundProviderPreferencesKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(WorldStateZoneSoundsRow {
                world_state_id,
                world_state_value,
                area_id,
                w_m_o_area_id,
                zone_intro_music_id,
                zone_music_id,
                sound_ambience_id,
                sound_provider_preferences_id,
            });
        }

        Ok(WorldStateZoneSounds { rows, })
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
            // world_state_id: foreign_key (WorldState) int32
            b.write_all(&row.world_state_id.to_le_bytes())?;

            // world_state_value: int32
            b.write_all(&row.world_state_value.to_le_bytes())?;

            // area_id: foreign_key (AreaTable) int32
            b.write_all(&(row.area_id.id as i32).to_le_bytes())?;

            // w_m_o_area_id: foreign_key (WMOAreaTable) int32
            b.write_all(&(row.w_m_o_area_id.id as i32).to_le_bytes())?;

            // zone_intro_music_id: foreign_key (ZoneIntroMusicTable) int32
            b.write_all(&(row.zone_intro_music_id.id as i32).to_le_bytes())?;

            // zone_music_id: foreign_key (ZoneMusic) int32
            b.write_all(&(row.zone_music_id.id as i32).to_le_bytes())?;

            // sound_ambience_id: foreign_key (SoundAmbience) int32
            b.write_all(&(row.sound_ambience_id.id as i32).to_le_bytes())?;

            // sound_provider_preferences_id: foreign_key (SoundProviderPreferences) int32
            b.write_all(&(row.sound_provider_preferences_id.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldStateZoneSoundsRow {
    pub world_state_id: i32,
    pub world_state_value: i32,
    pub area_id: AreaTableKey,
    pub w_m_o_area_id: WMOAreaTableKey,
    pub zone_intro_music_id: ZoneIntroMusicTableKey,
    pub zone_music_id: ZoneMusicKey,
    pub sound_ambience_id: SoundAmbienceKey,
    pub sound_provider_preferences_id: SoundProviderPreferencesKey,
}

