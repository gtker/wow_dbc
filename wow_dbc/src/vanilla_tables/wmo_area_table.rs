use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::area_table::AreaTableKey;
use crate::vanilla_tables::sound_ambience::SoundAmbienceKey;
use crate::vanilla_tables::sound_provider_preferences::SoundProviderPreferencesKey;
use crate::vanilla_tables::zone_intro_music_table::ZoneIntroMusicTableKey;
use crate::vanilla_tables::zone_music::ZoneMusicKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WMOAreaTable {
    pub rows: Vec<WMOAreaTableRow>,
}

impl DbcTable for WMOAreaTable {
    type Row = WMOAreaTableRow;

    const FILENAME: &'static str = "WMOAreaTable.dbc";
    const FIELD_COUNT: usize = 20;
    const ROW_SIZE: usize = 80;

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

            // id: primary_key (WMOAreaTable) uint32
            let id = WMOAreaTableKey::new(crate::util::read_u32_le(chunk)?);

            // wmo_id: uint32
            let wmo_id = crate::util::read_u32_le(chunk)?;

            // name_set_id: uint32
            let name_set_id = crate::util::read_u32_le(chunk)?;

            // wmo_group_id: int32
            let wmo_group_id = crate::util::read_i32_le(chunk)?;

            // sound_provider_preferences: foreign_key (SoundProviderPreferences) uint32
            let sound_provider_preferences = SoundProviderPreferencesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_provider_preferences_underwater: foreign_key (SoundProviderPreferences) uint32
            let sound_provider_preferences_underwater = SoundProviderPreferencesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_ambience: foreign_key (SoundAmbience) uint32
            let sound_ambience = SoundAmbienceKey::new(crate::util::read_u32_le(chunk)?.into());

            // zone_music: foreign_key (ZoneMusic) uint32
            let zone_music = ZoneMusicKey::new(crate::util::read_u32_le(chunk)?.into());

            // zone_intro_music: foreign_key (ZoneIntroMusicTable) uint32
            let zone_intro_music = ZoneIntroMusicTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // flags: uint32
            let flags = crate::util::read_u32_le(chunk)?;

            // area_table: foreign_key (AreaTable) uint32
            let area_table = AreaTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(WMOAreaTableRow {
                id,
                wmo_id,
                name_set_id,
                wmo_group_id,
                sound_provider_preferences,
                sound_provider_preferences_underwater,
                sound_ambience,
                zone_music,
                zone_intro_music,
                flags,
                area_table,
                name,
            });
        }

        Ok(WMOAreaTable { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (WMOAreaTable) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // wmo_id: uint32
            b.write_all(&row.wmo_id.to_le_bytes())?;

            // name_set_id: uint32
            b.write_all(&row.name_set_id.to_le_bytes())?;

            // wmo_group_id: int32
            b.write_all(&row.wmo_group_id.to_le_bytes())?;

            // sound_provider_preferences: foreign_key (SoundProviderPreferences) uint32
            b.write_all(&(row.sound_provider_preferences.id as u32).to_le_bytes())?;

            // sound_provider_preferences_underwater: foreign_key (SoundProviderPreferences) uint32
            b.write_all(&(row.sound_provider_preferences_underwater.id as u32).to_le_bytes())?;

            // sound_ambience: foreign_key (SoundAmbience) uint32
            b.write_all(&(row.sound_ambience.id as u32).to_le_bytes())?;

            // zone_music: foreign_key (ZoneMusic) uint32
            b.write_all(&(row.zone_music.id as u32).to_le_bytes())?;

            // zone_intro_music: foreign_key (ZoneIntroMusicTable) uint32
            b.write_all(&(row.zone_intro_music.id as u32).to_le_bytes())?;

            // flags: uint32
            b.write_all(&row.flags.to_le_bytes())?;

            // area_table: foreign_key (AreaTable) uint32
            b.write_all(&(row.area_table.id as u32).to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for WMOAreaTable {
    type PrimaryKey = WMOAreaTableKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl WMOAreaTable {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WMOAreaTableKey {
    pub id: u32
}

impl WMOAreaTableKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for WMOAreaTableKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for WMOAreaTableKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for WMOAreaTableKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for WMOAreaTableKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for WMOAreaTableKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for WMOAreaTableKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for WMOAreaTableKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for WMOAreaTableKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for WMOAreaTableKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for WMOAreaTableKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WMOAreaTableRow {
    pub id: WMOAreaTableKey,
    pub wmo_id: u32,
    pub name_set_id: u32,
    pub wmo_group_id: i32,
    pub sound_provider_preferences: SoundProviderPreferencesKey,
    pub sound_provider_preferences_underwater: SoundProviderPreferencesKey,
    pub sound_ambience: SoundAmbienceKey,
    pub zone_music: ZoneMusicKey,
    pub zone_intro_music: ZoneIntroMusicTableKey,
    pub flags: u32,
    pub area_table: AreaTableKey,
    pub name: LocalizedString,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn wmo_area_table() {
        let mut file = File::open("../vanilla-dbc/WMOAreaTable.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = WMOAreaTable::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = WMOAreaTable::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
