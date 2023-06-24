use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::animation_data::AnimationDataKey;
use crate::vanilla_tables::sound_entries::SoundEntriesKey;
use std::io::Write;
use wow_world_base::vanilla::{
    EmoteFlags, EmoteSpecProc,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Emotes {
    pub rows: Vec<EmotesRow>,
}

impl DbcTable for Emotes {
    type Row = EmotesRow;

    const FILENAME: &'static str = "Emotes.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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
                    expected: 7,
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

            // id: primary_key (Emotes) uint32
            let id = EmotesKey::new(crate::util::read_u32_le(chunk)?);

            // emote_slash_command: string_ref
            let emote_slash_command = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // animation_data: foreign_key (AnimationData) uint32
            let animation_data = AnimationDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // flags: EmoteFlags
            let flags = EmoteFlags::new(crate::util::read_i32_le(chunk)? as _);

            // spec_proc: EmoteSpecProc
            let spec_proc = crate::util::read_i32_le(chunk)?.try_into()?;

            // emote_spec_proc_param: int32
            let emote_spec_proc_param = crate::util::read_i32_le(chunk)?;

            // event_sound_entry: foreign_key (SoundEntries) uint32
            let event_sound_entry = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(EmotesRow {
                id,
                emote_slash_command,
                animation_data,
                flags,
                spec_proc,
                emote_spec_proc_param,
                event_sound_entry,
            });
        }

        Ok(Emotes { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 7,
            record_size: 28,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Emotes) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // emote_slash_command: string_ref
            if !row.emote_slash_command.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.emote_slash_command.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // animation_data: foreign_key (AnimationData) uint32
            b.write_all(&(row.animation_data.id as u32).to_le_bytes())?;

            // flags: EmoteFlags
            b.write_all(&(row.flags.as_int() as i32).to_le_bytes())?;

            // spec_proc: EmoteSpecProc
            b.write_all(&(row.spec_proc.as_int() as i32).to_le_bytes())?;

            // emote_spec_proc_param: int32
            b.write_all(&row.emote_spec_proc_param.to_le_bytes())?;

            // event_sound_entry: foreign_key (SoundEntries) uint32
            b.write_all(&(row.event_sound_entry.id as u32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Emotes {
    type PrimaryKey = EmotesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl Emotes {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.emote_slash_command.is_empty() { b.write_all(row.emote_slash_command.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.emote_slash_command.is_empty() { sum += row.emote_slash_command.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct EmotesKey {
    pub id: u32
}

impl EmotesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for EmotesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for EmotesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for EmotesKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmotesRow {
    pub id: EmotesKey,
    pub emote_slash_command: String,
    pub animation_data: AnimationDataKey,
    pub flags: EmoteFlags,
    pub spec_proc: EmoteSpecProc,
    pub emote_spec_proc_param: i32,
    pub event_sound_entry: SoundEntriesKey,
}

