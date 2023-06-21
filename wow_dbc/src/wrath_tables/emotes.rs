use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::animation_data::*;
use crate::wrath_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Emotes {
    pub rows: Vec<EmotesRow>,
}

impl DbcTable for Emotes {
    type Row = EmotesRow;

    fn filename() -> &'static str { "Emotes.dbc" }

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

            // id: primary_key (Emotes) int32
            let id = EmotesKey::new(crate::util::read_i32_le(chunk)?);

            // emote_slash_command: string_ref
            let emote_slash_command = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // anim_id: foreign_key (AnimationData) int32
            let anim_id = AnimationDataKey::new(crate::util::read_i32_le(chunk)?.into());

            // emote_flags: int32
            let emote_flags = crate::util::read_i32_le(chunk)?;

            // emote_spec_proc: int32
            let emote_spec_proc = crate::util::read_i32_le(chunk)?;

            // emote_spec_proc_param: int32
            let emote_spec_proc_param = crate::util::read_i32_le(chunk)?;

            // event_sound_id: foreign_key (SoundEntries) int32
            let event_sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(EmotesRow {
                id,
                emote_slash_command,
                anim_id,
                emote_flags,
                emote_spec_proc,
                emote_spec_proc_param,
                event_sound_id,
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
            // id: primary_key (Emotes) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // emote_slash_command: string_ref
            if !row.emote_slash_command.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.emote_slash_command.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // anim_id: foreign_key (AnimationData) int32
            b.write_all(&(row.anim_id.id as i32).to_le_bytes())?;

            // emote_flags: int32
            b.write_all(&row.emote_flags.to_le_bytes())?;

            // emote_spec_proc: int32
            b.write_all(&row.emote_spec_proc.to_le_bytes())?;

            // emote_spec_proc_param: int32
            b.write_all(&row.emote_spec_proc_param.to_le_bytes())?;

            // event_sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.event_sound_id.id as i32).to_le_bytes())?;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstEmotes<const S: usize> {
    pub rows: [ConstEmotesRow; S],
}

impl<const S: usize> ConstEmotes<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 28 {
            panic!("invalid record size, expected 28")
        }

        if header.field_count != 7 {
            panic!("invalid field count, expected 7")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstEmotesRow {
                id: EmotesKey::new(0),
                emote_slash_command: "",
                anim_id: AnimationDataKey::new(0),
                emote_flags: 0,
                emote_spec_proc: 0,
                emote_spec_proc_param: 0,
                event_sound_id: SoundEntriesKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Emotes) int32
            let id = EmotesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // emote_slash_command: string_ref
            let emote_slash_command = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // anim_id: foreign_key (AnimationData) int32
            let anim_id = AnimationDataKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // emote_flags: int32
            let emote_flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // emote_spec_proc: int32
            let emote_spec_proc = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // emote_spec_proc_param: int32
            let emote_spec_proc_param = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // event_sound_id: foreign_key (SoundEntries) int32
            let event_sound_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = ConstEmotesRow {
                id,
                emote_slash_command,
                anim_id,
                emote_flags,
                emote_spec_proc,
                emote_spec_proc_param,
                event_sound_id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> Emotes {
        Emotes {
            rows: self.rows.iter().map(|s| EmotesRow {
                id: s.id,
                emote_slash_command: s.emote_slash_command.to_string(),
                anim_id: s.anim_id,
                emote_flags: s.emote_flags,
                emote_spec_proc: s.emote_spec_proc,
                emote_spec_proc_param: s.emote_spec_proc_param,
                event_sound_id: s.event_sound_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct EmotesKey {
    pub id: i32
}

impl EmotesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for EmotesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for EmotesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for EmotesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmotesRow {
    pub id: EmotesKey,
    pub emote_slash_command: String,
    pub anim_id: AnimationDataKey,
    pub emote_flags: i32,
    pub emote_spec_proc: i32,
    pub emote_spec_proc_param: i32,
    pub event_sound_id: SoundEntriesKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstEmotesRow {
    pub id: EmotesKey,
    pub emote_slash_command: &'static str,
    pub anim_id: AnimationDataKey,
    pub emote_flags: i32,
    pub emote_spec_proc: i32,
    pub emote_spec_proc_param: i32,
    pub event_sound_id: SoundEntriesKey,
}

