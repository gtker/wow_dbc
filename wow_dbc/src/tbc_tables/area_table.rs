use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};
use crate::tbc_tables::map::*;
use crate::tbc_tables::sound_ambience::*;
use crate::tbc_tables::sound_provider_preferences::*;
use crate::tbc_tables::zone_intro_music_table::*;
use crate::tbc_tables::zone_music::*;

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
        let header = header::parse_header(&header)?;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (AreaTable) int32
            let id = AreaTableKey::new(crate::util::read_i32_le(chunk)?);

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // parent_area_id: foreign_key (AreaTable) int32
            let parent_area_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // area_bit: int32
            let area_bit = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // sound_provider_pref: foreign_key (SoundProviderPreferences) int32
            let sound_provider_pref = SoundProviderPreferencesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_provider_pref_underwater: foreign_key (SoundProviderPreferences) int32
            let sound_provider_pref_underwater = SoundProviderPreferencesKey::new(crate::util::read_i32_le(chunk)?.into());

            // ambience_id: foreign_key (SoundAmbience) int32
            let ambience_id = SoundAmbienceKey::new(crate::util::read_i32_le(chunk)?.into());

            // zone_music: foreign_key (ZoneMusic) int32
            let zone_music = ZoneMusicKey::new(crate::util::read_i32_le(chunk)?.into());

            // intro_sound: foreign_key (ZoneIntroMusicTable) int32
            let intro_sound = ZoneIntroMusicTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // exploration_level: int32
            let exploration_level = crate::util::read_i32_le(chunk)?;

            // area_name_lang: string_ref_loc (Extended)
            let area_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // faction_group_mask: int32
            let faction_group_mask = crate::util::read_i32_le(chunk)?;

            // liquid_type_id: int32[4]
            let liquid_type_id = crate::util::read_array_i32::<4>(chunk)?;

            // min_elevation: float
            let min_elevation = crate::util::read_f32_le(chunk)?;

            // ambient_multiplier: float
            let ambient_multiplier = crate::util::read_f32_le(chunk)?;


            rows.push(AreaTableRow {
                id,
                continent_id,
                parent_area_id,
                area_bit,
                flags,
                sound_provider_pref,
                sound_provider_pref_underwater,
                ambience_id,
                zone_music,
                intro_sound,
                exploration_level,
                area_name_lang,
                faction_group_mask,
                liquid_type_id,
                min_elevation,
                ambient_multiplier,
            });
        }

        Ok(AreaTable { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 35,
            record_size: 140,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (AreaTable) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // continent_id: foreign_key (Map) int32
            b.write_all(&(row.continent_id.id as i32).to_le_bytes())?;

            // parent_area_id: foreign_key (AreaTable) int32
            b.write_all(&(row.parent_area_id.id as i32).to_le_bytes())?;

            // area_bit: int32
            b.write_all(&row.area_bit.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // sound_provider_pref: foreign_key (SoundProviderPreferences) int32
            b.write_all(&(row.sound_provider_pref.id as i32).to_le_bytes())?;

            // sound_provider_pref_underwater: foreign_key (SoundProviderPreferences) int32
            b.write_all(&(row.sound_provider_pref_underwater.id as i32).to_le_bytes())?;

            // ambience_id: foreign_key (SoundAmbience) int32
            b.write_all(&(row.ambience_id.id as i32).to_le_bytes())?;

            // zone_music: foreign_key (ZoneMusic) int32
            b.write_all(&(row.zone_music.id as i32).to_le_bytes())?;

            // intro_sound: foreign_key (ZoneIntroMusicTable) int32
            b.write_all(&(row.intro_sound.id as i32).to_le_bytes())?;

            // exploration_level: int32
            b.write_all(&row.exploration_level.to_le_bytes())?;

            // area_name_lang: string_ref_loc (Extended)
            b.write_all(&row.area_name_lang.string_indices_as_array(&mut string_index))?;

            // faction_group_mask: int32
            b.write_all(&row.faction_group_mask.to_le_bytes())?;

            // liquid_type_id: int32[4]
            for i in row.liquid_type_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // min_elevation: float
            b.write_all(&row.min_elevation.to_le_bytes())?;

            // ambient_multiplier: float
            b.write_all(&row.ambient_multiplier.to_le_bytes())?;

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
            row.area_name_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.area_name_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstAreaTable<const S: usize> {
    pub rows: [ConstAreaTableRow; S],
}

impl<const S: usize> ConstAreaTable<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 140 {
            panic!("invalid record size, expected 140")
        }

        if header.field_count != 35 {
            panic!("invalid field count, expected 35")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstAreaTableRow {
                id: AreaTableKey::new(0),
                continent_id: MapKey::new(0),
                parent_area_id: AreaTableKey::new(0),
                area_bit: 0,
                flags: 0,
                sound_provider_pref: SoundProviderPreferencesKey::new(0),
                sound_provider_pref_underwater: SoundProviderPreferencesKey::new(0),
                ambience_id: SoundAmbienceKey::new(0),
                zone_music: ZoneMusicKey::new(0),
                intro_sound: ZoneIntroMusicTableKey::new(0),
                exploration_level: 0,
                area_name_lang: crate::ConstExtendedLocalizedString::empty(),
                faction_group_mask: 0,
                liquid_type_id: [0; 4],
                min_elevation: 0.0,
                ambient_multiplier: 0.0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (AreaTable) int32
            let id = AreaTableKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // parent_area_id: foreign_key (AreaTable) int32
            let parent_area_id = AreaTableKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // area_bit: int32
            let area_bit = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // sound_provider_pref: foreign_key (SoundProviderPreferences) int32
            let sound_provider_pref = SoundProviderPreferencesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_provider_pref_underwater: foreign_key (SoundProviderPreferences) int32
            let sound_provider_pref_underwater = SoundProviderPreferencesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // ambience_id: foreign_key (SoundAmbience) int32
            let ambience_id = SoundAmbienceKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // zone_music: foreign_key (ZoneMusic) int32
            let zone_music = ZoneMusicKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // intro_sound: foreign_key (ZoneIntroMusicTable) int32
            let intro_sound = ZoneIntroMusicTableKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // exploration_level: int32
            let exploration_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // area_name_lang: string_ref_loc (Extended)
            let area_name_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // faction_group_mask: int32
            let faction_group_mask = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // liquid_type_id: int32[4]
            let liquid_type_id = {
                let mut a = [0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // min_elevation: float
            let min_elevation = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // ambient_multiplier: float
            let ambient_multiplier = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstAreaTableRow {
                id,
                continent_id,
                parent_area_id,
                area_bit,
                flags,
                sound_provider_pref,
                sound_provider_pref_underwater,
                ambience_id,
                zone_music,
                intro_sound,
                exploration_level,
                area_name_lang,
                faction_group_mask,
                liquid_type_id,
                min_elevation,
                ambient_multiplier,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct AreaTableKey {
    pub id: i32
}

impl AreaTableKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for AreaTableKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for AreaTableKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for AreaTableKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AreaTableRow {
    pub id: AreaTableKey,
    pub continent_id: MapKey,
    pub parent_area_id: AreaTableKey,
    pub area_bit: i32,
    pub flags: i32,
    pub sound_provider_pref: SoundProviderPreferencesKey,
    pub sound_provider_pref_underwater: SoundProviderPreferencesKey,
    pub ambience_id: SoundAmbienceKey,
    pub zone_music: ZoneMusicKey,
    pub intro_sound: ZoneIntroMusicTableKey,
    pub exploration_level: i32,
    pub area_name_lang: ExtendedLocalizedString,
    pub faction_group_mask: i32,
    pub liquid_type_id: [i32; 4],
    pub min_elevation: f32,
    pub ambient_multiplier: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstAreaTableRow {
    pub id: AreaTableKey,
    pub continent_id: MapKey,
    pub parent_area_id: AreaTableKey,
    pub area_bit: i32,
    pub flags: i32,
    pub sound_provider_pref: SoundProviderPreferencesKey,
    pub sound_provider_pref_underwater: SoundProviderPreferencesKey,
    pub ambience_id: SoundAmbienceKey,
    pub zone_music: ZoneMusicKey,
    pub intro_sound: ZoneIntroMusicTableKey,
    pub exploration_level: i32,
    pub area_name_lang: ConstExtendedLocalizedString,
    pub faction_group_mask: i32,
    pub liquid_type_id: [i32; 4],
    pub min_elevation: f32,
    pub ambient_multiplier: f32,
}

