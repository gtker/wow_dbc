use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::chr_races::*;
use crate::tbc_tables::emotes_text::*;
use crate::tbc_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmotesTextSound {
    pub rows: Vec<EmotesTextSoundRow>,
}

impl DbcTable for EmotesTextSound {
    type Row = EmotesTextSoundRow;

    fn filename() -> &'static str { "EmotesTextSound.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 20 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 20,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 5 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 5,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (EmotesTextSound) int32
            let id = EmotesTextSoundKey::new(crate::util::read_i32_le(chunk)?);

            // emotes_text_id: foreign_key (EmotesText) int32
            let emotes_text_id = EmotesTextKey::new(crate::util::read_i32_le(chunk)?.into());

            // race_id: foreign_key (ChrRaces) int32
            let race_id = ChrRacesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sex_id: int32
            let sex_id = crate::util::read_i32_le(chunk)?;

            // sound_id: foreign_key (SoundEntries) int32
            let sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(EmotesTextSoundRow {
                id,
                emotes_text_id,
                race_id,
                sex_id,
                sound_id,
            });
        }

        Ok(EmotesTextSound { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 5,
            record_size: 20,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (EmotesTextSound) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // emotes_text_id: foreign_key (EmotesText) int32
            b.write_all(&(row.emotes_text_id.id as i32).to_le_bytes())?;

            // race_id: foreign_key (ChrRaces) int32
            b.write_all(&(row.race_id.id as i32).to_le_bytes())?;

            // sex_id: int32
            b.write_all(&row.sex_id.to_le_bytes())?;

            // sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_id.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for EmotesTextSound {
    type PrimaryKey = EmotesTextSoundKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct EmotesTextSoundKey {
    pub id: i32
}

impl EmotesTextSoundKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmotesTextSoundRow {
    pub id: EmotesTextSoundKey,
    pub emotes_text_id: EmotesTextKey,
    pub race_id: ChrRacesKey,
    pub sex_id: i32,
    pub sound_id: SoundEntriesKey,
}

