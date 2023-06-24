use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::cinematic_sequences::CinematicSequencesKey;
use crate::tbc_tables::creature_display_info::CreatureDisplayInfoKey;
use crate::tbc_tables::creature_type::CreatureTypeKey;
use crate::tbc_tables::faction_template::FactionTemplateKey;
use crate::tbc_tables::languages::LanguagesKey;
use crate::tbc_tables::sound_entries::SoundEntriesKey;
use crate::tbc_tables::spell::SpellKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ChrRaces {
    pub rows: Vec<ChrRacesRow>,
}

impl DbcTable for ChrRaces {
    type Row = ChrRacesRow;

    const FILENAME: &'static str = "ChrRaces.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 276 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 276,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 69 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 69,
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

            // mount_scale: float
            let mount_scale = crate::util::read_f32_le(chunk)?;

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
                mount_scale,
                base_language,
                creature_type,
                res_sickness_spell_id,
                splash_sound_id,
                client_file_string,
                cinematic_sequence_id,
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

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 69,
            record_size: 276,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
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
            if !row.client_prefix.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.client_prefix.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // mount_scale: float
            b.write_all(&row.mount_scale.to_le_bytes())?;

            // base_language: foreign_key (Languages) int32
            b.write_all(&(row.base_language.id as i32).to_le_bytes())?;

            // creature_type: foreign_key (CreatureType) int32
            b.write_all(&(row.creature_type.id as i32).to_le_bytes())?;

            // res_sickness_spell_id: foreign_key (Spell) int32
            b.write_all(&(row.res_sickness_spell_id.id as i32).to_le_bytes())?;

            // splash_sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.splash_sound_id.id as i32).to_le_bytes())?;

            // client_file_string: string_ref
            if !row.client_file_string.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.client_file_string.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // cinematic_sequence_id: foreign_key (CinematicSequences) int32
            b.write_all(&(row.cinematic_sequence_id.id as i32).to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // name_female_lang: string_ref_loc (Extended)
            b.write_all(&row.name_female_lang.string_indices_as_array(&mut string_index))?;

            // name_male_lang: string_ref_loc (Extended)
            b.write_all(&row.name_male_lang.string_indices_as_array(&mut string_index))?;

            // facial_hair_customization: string_ref[2]
            for i in &row.facial_hair_customization {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // hair_customization: string_ref
            if !row.hair_customization.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.hair_customization.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // required_expansion: int32
            b.write_all(&row.required_expansion.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ChrRaces {
    type PrimaryKey = ChrRacesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ChrRaces {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.client_prefix.is_empty() { b.write_all(row.client_prefix.as_bytes())?; b.write_all(&[0])?; };
            if !row.client_file_string.is_empty() { b.write_all(row.client_file_string.as_bytes())?; b.write_all(&[0])?; };
            row.name_lang.string_block_as_array(b)?;
            row.name_female_lang.string_block_as_array(b)?;
            row.name_male_lang.string_block_as_array(b)?;
            for s in &row.facial_hair_customization {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

            if !row.hair_customization.is_empty() { b.write_all(row.hair_customization.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.client_prefix.is_empty() { sum += row.client_prefix.len() + 1; };
            if !row.client_file_string.is_empty() { sum += row.client_file_string.len() + 1; };
            sum += row.name_lang.string_block_size();
            sum += row.name_female_lang.string_block_size();
            sum += row.name_male_lang.string_block_size();
            for s in &row.facial_hair_customization {
                if !s.is_empty() { sum += s.len() + 1; };
            }

            if !row.hair_customization.is_empty() { sum += row.hair_customization.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ChrRacesKey {
    pub id: i32
}

impl ChrRacesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ChrRacesRow {
    pub id: ChrRacesKey,
    pub flags: i32,
    pub faction_id: FactionTemplateKey,
    pub exploration_sound_id: SoundEntriesKey,
    pub male_display_id: CreatureDisplayInfoKey,
    pub female_display_id: CreatureDisplayInfoKey,
    pub client_prefix: String,
    pub mount_scale: f32,
    pub base_language: LanguagesKey,
    pub creature_type: CreatureTypeKey,
    pub res_sickness_spell_id: SpellKey,
    pub splash_sound_id: SoundEntriesKey,
    pub client_file_string: String,
    pub cinematic_sequence_id: CinematicSequencesKey,
    pub name_lang: ExtendedLocalizedString,
    pub name_female_lang: ExtendedLocalizedString,
    pub name_male_lang: ExtendedLocalizedString,
    pub facial_hair_customization: [String; 2],
    pub hair_customization: String,
    pub required_expansion: i32,
}

