use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;
use crate::vanilla_tables::cinematic_sequences::*;
use crate::vanilla_tables::creature_display_info::*;
use crate::vanilla_tables::creature_type::*;
use crate::vanilla_tables::faction_template::*;
use crate::vanilla_tables::sound_entries::*;
use crate::vanilla_tables::spell::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ChrRaces {
    pub rows: Vec<ChrRacesRow>,
}

impl DbcTable for ChrRaces {
    type Row = ChrRacesRow;

    fn filename() -> &'static str { "ChrRaces.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 116 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 116,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 29 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 29,
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

            // id: primary_key (ChrRaces) uint32
            let id = ChrRacesKey::new(crate::util::read_u32_le(chunk)?);

            // flags: Flags
            let flags = Flags::new(crate::util::read_u32_le(chunk)?);

            // faction: foreign_key (FactionTemplate) uint32
            let faction = FactionTemplateKey::new(crate::util::read_u32_le(chunk)?.into());

            // exploration_sound: foreign_key (SoundEntries) uint32
            let exploration_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // male_display: foreign_key (CreatureDisplayInfo) uint32
            let male_display = CreatureDisplayInfoKey::new(crate::util::read_u32_le(chunk)?.into());

            // female_display: foreign_key (CreatureDisplayInfo) uint32
            let female_display = CreatureDisplayInfoKey::new(crate::util::read_u32_le(chunk)?.into());

            // client_prefix: string_ref
            let client_prefix = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // speed_modifier: float
            let speed_modifier = crate::util::read_f32_le(chunk)?;

            // base_lang: BaseLanguage
            let base_lang = BaseLanguage::try_from(crate::util::read_u32_le(chunk)?)?;

            // creature_type: foreign_key (CreatureType) uint32
            let creature_type = CreatureTypeKey::new(crate::util::read_u32_le(chunk)?.into());

            // login_effect: foreign_key (Spell) uint32
            let login_effect = SpellKey::new(crate::util::read_u32_le(chunk)?.into());

            // unknown1: int32
            let unknown1 = crate::util::read_i32_le(chunk)?;

            // res_sickness_spell: foreign_key (Spell) uint32
            let res_sickness_spell = SpellKey::new(crate::util::read_u32_le(chunk)?.into());

            // splash_sound_entry: foreign_key (SoundEntries) uint32
            let splash_sound_entry = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // unknown2: int32
            let unknown2 = crate::util::read_i32_le(chunk)?;

            // client_file_path: string_ref
            let client_file_path = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // cinematic_sequence: foreign_key (CinematicSequences) uint32
            let cinematic_sequence = CinematicSequencesKey::new(crate::util::read_u32_le(chunk)?.into());

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // facial_hair_customisation: string_ref[2]
            let facial_hair_customisation = {
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

            // hair_customisation: string_ref
            let hair_customisation = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(ChrRacesRow {
                id,
                flags,
                faction,
                exploration_sound,
                male_display,
                female_display,
                client_prefix,
                speed_modifier,
                base_lang,
                creature_type,
                login_effect,
                unknown1,
                res_sickness_spell,
                splash_sound_entry,
                unknown2,
                client_file_path,
                cinematic_sequence,
                name,
                facial_hair_customisation,
                hair_customisation,
            });
        }

        Ok(ChrRaces { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 29,
            record_size: 116,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ChrRaces) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: Flags
            b.write_all(&(row.flags.as_int() as u32).to_le_bytes())?;

            // faction: foreign_key (FactionTemplate) uint32
            b.write_all(&(row.faction.id as u32).to_le_bytes())?;

            // exploration_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.exploration_sound.id as u32).to_le_bytes())?;

            // male_display: foreign_key (CreatureDisplayInfo) uint32
            b.write_all(&(row.male_display.id as u32).to_le_bytes())?;

            // female_display: foreign_key (CreatureDisplayInfo) uint32
            b.write_all(&(row.female_display.id as u32).to_le_bytes())?;

            // client_prefix: string_ref
            if !row.client_prefix.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.client_prefix.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // speed_modifier: float
            b.write_all(&row.speed_modifier.to_le_bytes())?;

            // base_lang: BaseLanguage
            b.write_all(&(row.base_lang.as_int() as u32).to_le_bytes())?;

            // creature_type: foreign_key (CreatureType) uint32
            b.write_all(&(row.creature_type.id as u32).to_le_bytes())?;

            // login_effect: foreign_key (Spell) uint32
            b.write_all(&(row.login_effect.id as u32).to_le_bytes())?;

            // unknown1: int32
            b.write_all(&row.unknown1.to_le_bytes())?;

            // res_sickness_spell: foreign_key (Spell) uint32
            b.write_all(&(row.res_sickness_spell.id as u32).to_le_bytes())?;

            // splash_sound_entry: foreign_key (SoundEntries) uint32
            b.write_all(&(row.splash_sound_entry.id as u32).to_le_bytes())?;

            // unknown2: int32
            b.write_all(&row.unknown2.to_le_bytes())?;

            // client_file_path: string_ref
            if !row.client_file_path.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.client_file_path.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // cinematic_sequence: foreign_key (CinematicSequences) uint32
            b.write_all(&(row.cinematic_sequence.id as u32).to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // facial_hair_customisation: string_ref[2]
            for i in &row.facial_hair_customisation {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // hair_customisation: string_ref
            if !row.hair_customisation.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.hair_customisation.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ChrRaces {
    type PrimaryKey = ChrRacesKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ChrRaces {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.client_prefix.is_empty() { b.write_all(row.client_prefix.as_bytes())?; b.write_all(&[0])?; };
            if !row.client_file_path.is_empty() { b.write_all(row.client_file_path.as_bytes())?; b.write_all(&[0])?; };
            row.name.string_block_as_array(b)?;
            for s in &row.facial_hair_customisation {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

            if !row.hair_customisation.is_empty() { b.write_all(row.hair_customisation.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.client_prefix.is_empty() { sum += row.client_prefix.len() + 1; };
            if !row.client_file_path.is_empty() { sum += row.client_file_path.len() + 1; };
            sum += row.name.string_block_size();
            for s in &row.facial_hair_customisation {
                if !s.is_empty() { sum += s.len() + 1; };
            }

            if !row.hair_customisation.is_empty() { sum += row.hair_customisation.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct ChrRacesKey {
    pub id: u32
}

impl ChrRacesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum BaseLanguage {
    Horde,
    Alliance,
}

impl TryFrom<u32> for BaseLanguage {
    type Error = crate::InvalidEnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Horde,
            7 => Self::Alliance,
            val => return Err(crate::InvalidEnumError::new("BaseLanguage", val as i64)),
        })
    }

}

impl BaseLanguage {
    const fn as_int(&self) -> u32 {
        match self {
            Self::Horde => 1,
            Self::Alliance => 7,
        }

    }

}

impl Default for BaseLanguage {
    fn default() -> Self {
        Self::Horde
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct Flags {
    value: u32,
}

impl Flags {
    const fn new(value: u32) -> Self {
        Self { value }
    }

    const fn as_int(&self) -> u32 {
        self.value
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct ChrRacesRow {
    pub id: ChrRacesKey,
    pub flags: Flags,
    pub faction: FactionTemplateKey,
    pub exploration_sound: SoundEntriesKey,
    pub male_display: CreatureDisplayInfoKey,
    pub female_display: CreatureDisplayInfoKey,
    pub client_prefix: String,
    pub speed_modifier: f32,
    pub base_lang: BaseLanguage,
    pub creature_type: CreatureTypeKey,
    pub login_effect: SpellKey,
    pub unknown1: i32,
    pub res_sickness_spell: SpellKey,
    pub splash_sound_entry: SoundEntriesKey,
    pub unknown2: i32,
    pub client_file_path: String,
    pub cinematic_sequence: CinematicSequencesKey,
    pub name: LocalizedString,
    pub facial_hair_customisation: [String; 2],
    pub hair_customisation: String,
}

