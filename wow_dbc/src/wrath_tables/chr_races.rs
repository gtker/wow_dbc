use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::cinematic_sequences::CinematicSequencesKey;
use crate::wrath_tables::creature_display_info::CreatureDisplayInfoKey;
use crate::wrath_tables::creature_type::CreatureTypeKey;
use crate::wrath_tables::faction_template::FactionTemplateKey;
use crate::wrath_tables::languages::LanguagesKey;
use crate::wrath_tables::sound_entries::SoundEntriesKey;
use crate::wrath_tables::spell::SpellKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChrRaces {
    pub rows: Vec<ChrRacesRow>,
}

impl DbcTable for ChrRaces {
    type Row = ChrRacesRow;

    const FILENAME: &'static str = "ChrRaces.dbc";
    const FIELD_COUNT: usize = 69;
    const ROW_SIZE: usize = 276;

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

            // id: primary_key (ChrRaces) int32
            let id = ChrRacesKey::new(crate::util::read_i32_le(chunk)?);

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // faction_id: foreign_key (FactionTemplate) int32
            let faction_id = FactionTemplateKey::new(crate::util::read_i32_le(chunk)?.into());

            // exploration_sound_id: foreign_key (SoundEntries) int32
            let exploration_sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // male_display_id: foreign_key (CreatureDisplayInfo) int32
            let male_display_id = CreatureDisplayInfoKey::new(crate::util::read_i32_le(chunk)?.into());

            // female_display_id: foreign_key (CreatureDisplayInfo) int32
            let female_display_id = CreatureDisplayInfoKey::new(crate::util::read_i32_le(chunk)?.into());

            // client_prefix: string_ref
            let client_prefix = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // base_language: foreign_key (Languages) int32
            let base_language = LanguagesKey::new(crate::util::read_i32_le(chunk)?.into());

            // creature_type: foreign_key (CreatureType) int32
            let creature_type = CreatureTypeKey::new(crate::util::read_i32_le(chunk)?.into());

            // res_sickness_spell_id: foreign_key (Spell) int32
            let res_sickness_spell_id = SpellKey::new(crate::util::read_i32_le(chunk)?.into());

            // splash_sound_id: foreign_key (SoundEntries) int32
            let splash_sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // client_file_string: string_ref
            let client_file_string = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // cinematic_sequence_id: foreign_key (CinematicSequences) int32
            let cinematic_sequence_id = CinematicSequencesKey::new(crate::util::read_i32_le(chunk)?.into());

            // alliance: int32
            let alliance = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // name_female_lang: string_ref_loc (Extended)
            let name_female_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // name_male_lang: string_ref_loc (Extended)
            let name_male_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // facial_hair_customization: string_ref[2]
            let facial_hair_customization = {
                let mut arr = Vec::with_capacity(2);
                for _ in 0..2 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // hair_customization: string_ref
            let hair_customization = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // required_expansion: int32
            let required_expansion = crate::util::read_i32_le(chunk)?;


            rows.push(ChrRacesRow {
                id,
                flags,
                faction_id,
                exploration_sound_id,
                male_display_id,
                female_display_id,
                client_prefix,
                base_language,
                creature_type,
                res_sickness_spell_id,
                splash_sound_id,
                client_file_string,
                cinematic_sequence_id,
                alliance,
                name_lang,
                name_female_lang,
                name_male_lang,
                facial_hair_customization,
                hair_customization,
                required_expansion,
            });
        }

        Ok(ChrRaces { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (ChrRaces) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // faction_id: foreign_key (FactionTemplate) int32
            b.write_all(&(row.faction_id.id as i32).to_le_bytes())?;

            // exploration_sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.exploration_sound_id.id as i32).to_le_bytes())?;

            // male_display_id: foreign_key (CreatureDisplayInfo) int32
            b.write_all(&(row.male_display_id.id as i32).to_le_bytes())?;

            // female_display_id: foreign_key (CreatureDisplayInfo) int32
            b.write_all(&(row.female_display_id.id as i32).to_le_bytes())?;

            // client_prefix: string_ref
            b.write_all(&string_cache.add_string(&row.client_prefix).to_le_bytes())?;

            // base_language: foreign_key (Languages) int32
            b.write_all(&(row.base_language.id as i32).to_le_bytes())?;

            // creature_type: foreign_key (CreatureType) int32
            b.write_all(&(row.creature_type.id as i32).to_le_bytes())?;

            // res_sickness_spell_id: foreign_key (Spell) int32
            b.write_all(&(row.res_sickness_spell_id.id as i32).to_le_bytes())?;

            // splash_sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.splash_sound_id.id as i32).to_le_bytes())?;

            // client_file_string: string_ref
            b.write_all(&string_cache.add_string(&row.client_file_string).to_le_bytes())?;

            // cinematic_sequence_id: foreign_key (CinematicSequences) int32
            b.write_all(&(row.cinematic_sequence_id.id as i32).to_le_bytes())?;

            // alliance: int32
            b.write_all(&row.alliance.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // name_female_lang: string_ref_loc (Extended)
            b.write_all(&row.name_female_lang.string_indices_as_array(&mut string_cache))?;

            // name_male_lang: string_ref_loc (Extended)
            b.write_all(&row.name_male_lang.string_indices_as_array(&mut string_cache))?;

            // facial_hair_customization: string_ref[2]
            for i in &row.facial_hair_customization {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // hair_customization: string_ref
            b.write_all(&string_cache.add_string(&row.hair_customization).to_le_bytes())?;

            // required_expansion: int32
            b.write_all(&row.required_expansion.to_le_bytes())?;

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

impl Indexable for ChrRaces {
    type PrimaryKey = ChrRacesKey;
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
pub struct ChrRacesKey {
    pub id: i32
}

impl ChrRacesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for ChrRacesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for ChrRacesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for ChrRacesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for ChrRacesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for ChrRacesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for ChrRacesKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ChrRacesKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for ChrRacesKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ChrRacesKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ChrRacesKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChrRacesRow {
    pub id: ChrRacesKey,
    pub flags: i32,
    pub faction_id: FactionTemplateKey,
    pub exploration_sound_id: SoundEntriesKey,
    pub male_display_id: CreatureDisplayInfoKey,
    pub female_display_id: CreatureDisplayInfoKey,
    pub client_prefix: String,
    pub base_language: LanguagesKey,
    pub creature_type: CreatureTypeKey,
    pub res_sickness_spell_id: SpellKey,
    pub splash_sound_id: SoundEntriesKey,
    pub client_file_string: String,
    pub cinematic_sequence_id: CinematicSequencesKey,
    pub alliance: i32,
    pub name_lang: ExtendedLocalizedString,
    pub name_female_lang: ExtendedLocalizedString,
    pub name_male_lang: ExtendedLocalizedString,
    pub facial_hair_customization: [String; 2],
    pub hair_customization: String,
    pub required_expansion: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn chr_races() {
        let mut file = File::open("../wrath-dbc/ChrRaces.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ChrRaces::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ChrRaces::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
