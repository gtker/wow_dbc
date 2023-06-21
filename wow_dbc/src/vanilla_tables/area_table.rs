use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::faction_group::FactionGroupKey;
use crate::vanilla_tables::light::LightKey;
use crate::vanilla_tables::liquid_type::LiquidTypeKey;
use crate::vanilla_tables::map::MapKey;
use crate::vanilla_tables::sound_ambience::SoundAmbienceKey;
use crate::vanilla_tables::sound_provider_preferences::SoundProviderPreferencesKey;
use crate::vanilla_tables::zone_intro_music_table::ZoneIntroMusicTableKey;
use crate::vanilla_tables::zone_music::ZoneMusicKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AreaTable {
    pub rows: Vec<AreaTableRow>,
}

impl DbcTable for AreaTable {
    type Row = AreaTableRow;

    fn filename() -> &'static str { "AreaTable.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 100 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 100,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 25 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 25,
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
            let flags = AreaFlags::new(crate::util::read_i32_le(chunk)?);

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

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 25,
            record_size: 100,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
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
            b.write_all(&row.area_name.string_indices_as_array(&mut string_index))?;

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

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for AreaTable {
    type PrimaryKey = AreaTableKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl AreaTable {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.area_name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.area_name.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
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

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct AreaFlags {
    value: i32,
}

impl AreaFlags {
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    pub const fn as_int(&self) -> i32 {
        self.value
    }

    pub const fn area_flag_snow(&self) -> bool {
        (self.value & 1) != 0
    }

    pub const fn area_flag_unk(&self) -> bool {
        (self.value & 2) != 0
    }

    pub const fn area_flag_development(&self) -> bool {
        (self.value & 4) != 0
    }

    pub const fn area_flag_unk2(&self) -> bool {
        (self.value & 8) != 0
    }

    pub const fn area_flag_unk3(&self) -> bool {
        (self.value & 16) != 0
    }

    pub const fn area_flag_city_slave(&self) -> bool {
        (self.value & 32) != 0
    }

    pub const fn area_flag_city_allow_duels(&self) -> bool {
        (self.value & 64) != 0
    }

    pub const fn area_flag_unk4(&self) -> bool {
        (self.value & 128) != 0
    }

    pub const fn area_flag_city(&self) -> bool {
        (self.value & 256) != 0
    }

    pub const fn area_flag_test(&self) -> bool {
        (self.value & 512) != 0
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

