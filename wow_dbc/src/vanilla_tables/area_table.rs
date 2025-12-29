use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::vanilla_tables::faction_group::FactionGroupKey;
use crate::vanilla_tables::light::LightKey;
use crate::vanilla_tables::liquid_type::LiquidTypeKey;
use crate::vanilla_tables::map::MapKey;
use crate::vanilla_tables::sound_ambience::SoundAmbienceKey;
use crate::vanilla_tables::sound_provider_preferences::SoundProviderPreferencesKey;
use crate::vanilla_tables::zone_intro_music_table::ZoneIntroMusicTableKey;
use crate::vanilla_tables::zone_music::ZoneMusicKey;
use std::io::Write;
pub use wow_world_base::vanilla::AreaFlags;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaTable {
    pub rows: Vec<AreaTableRow>,
}

impl DbcTable for AreaTable {
    type Row = AreaTableRow;

    const FILENAME: &'static str = "AreaTable.dbc";
    const FIELD_COUNT: usize = 25;
    const ROW_SIZE: usize = 100;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (AreaTable) uint32
            let id = AreaTableKey::new(crate::util::read_u32_le(chunk)?);

            // map: foreign_key (Map) uint32
            let map = MapKey::new(crate::util::read_u32_le(chunk)?.into());

            // parent_area_table: foreign_key (AreaTable) uint32
            let parent_area_table = AreaTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // area_bit: int32
            let area_bit = crate::util::read_i32_le(chunk)?;

            // flags: AreaFlags
            let flags = AreaFlags::new(crate::util::read_i32_le(chunk)? as _);

            // sound_preferences: foreign_key (SoundProviderPreferences) uint32
            let sound_preferences = SoundProviderPreferencesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_preferences_underwater: foreign_key (SoundProviderPreferences) uint32
            let sound_preferences_underwater = SoundProviderPreferencesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_ambience: foreign_key (SoundAmbience) uint32
            let sound_ambience = SoundAmbienceKey::new(crate::util::read_u32_le(chunk)?.into());

            // zone_music: foreign_key (ZoneMusic) uint32
            let zone_music = ZoneMusicKey::new(crate::util::read_u32_le(chunk)?.into());

            // zone_music_intro: foreign_key (ZoneIntroMusicTable) uint32
            let zone_music_intro = ZoneIntroMusicTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // exploration_level: int32
            let exploration_level = crate::util::read_i32_le(chunk)?;

            // area_name: string_ref_loc
            let area_name = crate::util::read_localized_string(chunk, &string_block)?;

            // faction_group: foreign_key (FactionGroup) uint32
            let faction_group = FactionGroupKey::new(crate::util::read_u32_le(chunk)?.into());

            // liquid_type: foreign_key (LiquidType) uint32
            let liquid_type = LiquidTypeKey::new(crate::util::read_u32_le(chunk)?.into());

            // min_elevation: int32
            let min_elevation = crate::util::read_i32_le(chunk)?;

            // ambient_multiplier: float
            let ambient_multiplier = crate::util::read_f32_le(chunk)?;

            // light: foreign_key (Light) uint32
            let light = LightKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(AreaTableRow {
                id,
                map,
                parent_area_table,
                area_bit,
                flags,
                sound_preferences,
                sound_preferences_underwater,
                sound_ambience,
                zone_music,
                zone_music_intro,
                exploration_level,
                area_name,
                faction_group,
                liquid_type,
                min_elevation,
                ambient_multiplier,
                light,
            });
        }

        Ok(AreaTable { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (AreaTable) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map: foreign_key (Map) uint32
            b.write_all(&(row.map.id as u32).to_le_bytes())?;

            // parent_area_table: foreign_key (AreaTable) uint32
            b.write_all(&(row.parent_area_table.id as u32).to_le_bytes())?;

            // area_bit: int32
            b.write_all(&row.area_bit.to_le_bytes())?;

            // flags: AreaFlags
            b.write_all(&(row.flags.as_int() as i32).to_le_bytes())?;

            // sound_preferences: foreign_key (SoundProviderPreferences) uint32
            b.write_all(&(row.sound_preferences.id as u32).to_le_bytes())?;

            // sound_preferences_underwater: foreign_key (SoundProviderPreferences) uint32
            b.write_all(&(row.sound_preferences_underwater.id as u32).to_le_bytes())?;

            // sound_ambience: foreign_key (SoundAmbience) uint32
            b.write_all(&(row.sound_ambience.id as u32).to_le_bytes())?;

            // zone_music: foreign_key (ZoneMusic) uint32
            b.write_all(&(row.zone_music.id as u32).to_le_bytes())?;

            // zone_music_intro: foreign_key (ZoneIntroMusicTable) uint32
            b.write_all(&(row.zone_music_intro.id as u32).to_le_bytes())?;

            // exploration_level: int32
            b.write_all(&row.exploration_level.to_le_bytes())?;

            // area_name: string_ref_loc
            b.write_all(&row.area_name.string_indices_as_array(&mut string_cache))?;

            // faction_group: foreign_key (FactionGroup) uint32
            b.write_all(&(row.faction_group.id as u32).to_le_bytes())?;

            // liquid_type: foreign_key (LiquidType) uint32
            b.write_all(&(row.liquid_type.id as u32).to_le_bytes())?;

            // min_elevation: int32
            b.write_all(&row.min_elevation.to_le_bytes())?;

            // ambient_multiplier: float
            b.write_all(&row.ambient_multiplier.to_le_bytes())?;

            // light: foreign_key (Light) uint32
            b.write_all(&(row.light.id as u32).to_le_bytes())?;

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

impl Indexable for AreaTable {
    type PrimaryKey = AreaTableKey;
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
pub struct AreaTableKey {
    pub id: u32
}

impl AreaTableKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for AreaTableKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for AreaTableKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for AreaTableKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for AreaTableKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for AreaTableKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for AreaTableKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for AreaTableKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for AreaTableKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for AreaTableKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for AreaTableKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaTableRow {
    pub id: AreaTableKey,
    pub map: MapKey,
    pub parent_area_table: AreaTableKey,
    pub area_bit: i32,
    pub flags: AreaFlags,
    pub sound_preferences: SoundProviderPreferencesKey,
    pub sound_preferences_underwater: SoundProviderPreferencesKey,
    pub sound_ambience: SoundAmbienceKey,
    pub zone_music: ZoneMusicKey,
    pub zone_music_intro: ZoneIntroMusicTableKey,
    pub exploration_level: i32,
    pub area_name: LocalizedString,
    pub faction_group: FactionGroupKey,
    pub liquid_type: LiquidTypeKey,
    pub min_elevation: i32,
    pub ambient_multiplier: f32,
    pub light: LightKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn area_table() {
        let mut file = File::open("../vanilla-dbc/AreaTable.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = AreaTable::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = AreaTable::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
