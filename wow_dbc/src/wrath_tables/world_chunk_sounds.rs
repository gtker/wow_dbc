use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::sound_ambience::SoundAmbienceKey;
use crate::wrath_tables::sound_provider_preferences::SoundProviderPreferencesKey;
use crate::wrath_tables::zone_music::ZoneMusicKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldChunkSounds {
    pub rows: Vec<WorldChunkSoundsRow>,
}

impl DbcTable for WorldChunkSounds {
    type Row = WorldChunkSoundsRow;

    const FILENAME: &'static str = "WorldChunkSounds.dbc";
    const FIELD_COUNT: usize = 9;
    const ROW_SIZE: usize = 36;

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

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

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

        assert_eq!(b.len(), self.rows.len() * Self::ROW_SIZE);
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: string_cache.size(),
        };

        w.write_all(&header.write_header())?;
        w.write_all(&b)?;
        w.write_all(string_cache.buffer())?;
        Ok(())
    }

}

impl Indexable for WorldChunkSounds {
    type PrimaryKey = WorldChunkSoundsKey;
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
pub struct WorldChunkSoundsKey {
    pub id: i32
}

impl WorldChunkSoundsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

impl TryFrom<u32> for WorldChunkSoundsKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for WorldChunkSoundsKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for WorldChunkSoundsKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for WorldChunkSoundsKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for WorldChunkSoundsKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn world_chunk_sounds() {
        let mut file = File::open("../wrath-dbc/WorldChunkSounds.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WorldChunkSounds::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WorldChunkSounds::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
