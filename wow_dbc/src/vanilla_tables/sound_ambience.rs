use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoundAmbience {
    pub rows: Vec<SoundAmbienceRow>,
}

impl DbcTable for SoundAmbience {
    type Row = SoundAmbienceRow;

    fn filename() -> &'static str { "SoundAmbience.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 12 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 12,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 3 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 3,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SoundAmbience) uint32
            let id = SoundAmbienceKey::new(crate::util::read_u32_le(chunk)?);

            // day_sound: foreign_key (SoundEntries) uint32
            let day_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // night_sound: foreign_key (SoundEntries) uint32
            let night_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(SoundAmbienceRow {
                id,
                day_sound,
                night_sound,
            });
        }

        Ok(SoundAmbience { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 3,
            record_size: 12,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SoundAmbience) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // day_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.day_sound.id as u32).to_le_bytes())?;

            // night_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.night_sound.id as u32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SoundAmbience {
    type PrimaryKey = SoundAmbienceKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SoundAmbienceKey {
    pub id: u32
}

impl SoundAmbienceKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SoundAmbienceKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SoundAmbienceKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SoundAmbienceKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoundAmbienceRow {
    pub id: SoundAmbienceKey,
    pub day_sound: SoundEntriesKey,
    pub night_sound: SoundEntriesKey,
}

