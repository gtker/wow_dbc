use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::sound_ambience::*;
use crate::wrath_tables::sound_provider_preferences::*;
use crate::wrath_tables::zone_music::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldChunkSounds {
    pub rows: Vec<WorldChunkSoundsRow>,
}

impl DbcTable for WorldChunkSounds {
    type Row = WorldChunkSoundsRow;

    fn filename() -> &'static str { "WorldChunkSounds.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 36 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 36,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 9 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 9,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (WorldChunkSounds) int32
            let id = WorldChunkSoundsKey::new(crate::util::read_i32_le(chunk)?);

            // chunk_x: int32
            let chunk_x = crate::util::read_i32_le(chunk)?;

            // chunk_y: int32
            let chunk_y = crate::util::read_i32_le(chunk)?;

            // subchunk_x: int32
            let subchunk_x = crate::util::read_i32_le(chunk)?;

            // subchunk_y: int32
            let subchunk_y = crate::util::read_i32_le(chunk)?;

            // zone_intro_music_id: int32
            let zone_intro_music_id = crate::util::read_i32_le(chunk)?;

            // zone_music_id: foreign_key (ZoneMusic) int32
            let zone_music_id = ZoneMusicKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_ambience_id: foreign_key (SoundAmbience) int32
            let sound_ambience_id = SoundAmbienceKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_provider_preferences_id: foreign_key (SoundProviderPreferences) int32
            let sound_provider_preferences_id = SoundProviderPreferencesKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(WorldChunkSoundsRow {
                id,
                chunk_x,
                chunk_y,
                subchunk_x,
                subchunk_y,
                zone_intro_music_id,
                zone_music_id,
                sound_ambience_id,
                sound_provider_preferences_id,
            });
        }

        Ok(WorldChunkSounds { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 9,
            record_size: 36,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (WorldChunkSounds) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // chunk_x: int32
            b.write_all(&row.chunk_x.to_le_bytes())?;

            // chunk_y: int32
            b.write_all(&row.chunk_y.to_le_bytes())?;

            // subchunk_x: int32
            b.write_all(&row.subchunk_x.to_le_bytes())?;

            // subchunk_y: int32
            b.write_all(&row.subchunk_y.to_le_bytes())?;

            // zone_intro_music_id: int32
            b.write_all(&row.zone_intro_music_id.to_le_bytes())?;

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

impl Indexable for WorldChunkSounds {
    type PrimaryKey = WorldChunkSoundsKey;
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
pub struct WorldChunkSoundsKey {
    pub id: i32
}

impl WorldChunkSoundsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for WorldChunkSoundsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for WorldChunkSoundsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for WorldChunkSoundsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for WorldChunkSoundsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for WorldChunkSoundsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldChunkSoundsRow {
    pub id: WorldChunkSoundsKey,
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub subchunk_x: i32,
    pub subchunk_y: i32,
    pub zone_intro_music_id: i32,
    pub zone_music_id: ZoneMusicKey,
    pub sound_ambience_id: SoundAmbienceKey,
    pub sound_provider_preferences_id: SoundProviderPreferencesKey,
}

