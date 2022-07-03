use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tables::chr_races::*;
use crate::tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VocalUISounds {
    pub rows: Vec<VocalUISoundsRow>,
}

impl DbcTable for VocalUISounds {
    type Row = VocalUISoundsRow;

    fn filename() -> &'static str { "VocalUISounds.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 28 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 28,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 7 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 28,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (VocalUISounds) uint32
            let id = VocalUISoundsKey::new(crate::util::read_u32_le(chunk)?);

            // vocal_ui_enum: int32
            let vocal_ui_enum = crate::util::read_i32_le(chunk)?;

            // race: foreign_key (ChrRaces) uint32
            let race = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // normal_male_sound: foreign_key (SoundEntries) uint32
            let normal_male_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // normal_female_sound: foreign_key (SoundEntries) uint32
            let normal_female_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // pissed_male_sound: foreign_key (SoundEntries) uint32
            let pissed_male_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // pissed_female_sound: foreign_key (SoundEntries) uint32
            let pissed_female_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(VocalUISoundsRow {
                id,
                vocal_ui_enum,
                race,
                normal_male_sound,
                normal_female_sound,
                pissed_male_sound,
                pissed_female_sound,
            });
        }

        Ok(VocalUISounds { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 7,
            record_size: 28,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (VocalUISounds) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // vocal_ui_enum: int32
            b.write_all(&row.vocal_ui_enum.to_le_bytes())?;

            // race: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race.id as u32).to_le_bytes())?;

            // normal_male_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.normal_male_sound.id as u32).to_le_bytes())?;

            // normal_female_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.normal_female_sound.id as u32).to_le_bytes())?;

            // pissed_male_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.pissed_male_sound.id as u32).to_le_bytes())?;

            // pissed_female_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.pissed_female_sound.id as u32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for VocalUISounds {
    type PrimaryKey = VocalUISoundsKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct VocalUISoundsKey {
    pub id: u32
}

impl VocalUISoundsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct VocalUISoundsRow {
    pub id: VocalUISoundsKey,
    pub vocal_ui_enum: i32,
    pub race: ChrRacesKey,
    pub normal_male_sound: SoundEntriesKey,
    pub normal_female_sound: SoundEntriesKey,
    pub pissed_male_sound: SoundEntriesKey,
    pub pissed_female_sound: SoundEntriesKey,
}

