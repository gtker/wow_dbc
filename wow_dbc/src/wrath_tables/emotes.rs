use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::animation_data::AnimationDataKey;
use crate::wrath_tables::sound_entries::SoundEntriesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Emotes {
    pub rows: Vec<EmotesRow>,
}

impl DbcTable for Emotes {
    type Row = EmotesRow;

    const FILENAME: &'static str = "Emotes.dbc";
    const FIELD_COUNT: usize = 7;
    const ROW_SIZE: usize = 28;

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

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Emotes) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // emote_slash_command: string_ref
            b.write_all(&string_cache.add_string(&row.emote_slash_command).to_le_bytes())?;

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

impl Indexable for Emotes {
    type PrimaryKey = EmotesKey;
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
pub struct EmotesKey {
    pub id: i32
}

impl EmotesKey {
    pub const fn new(id: i32) -> Self {
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

impl TryFrom<u32> for EmotesKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for EmotesKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for EmotesKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for EmotesKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for EmotesKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmotesRow {
    pub id: EmotesKey,
    pub emote_slash_command: String,
    pub anim_id: AnimationDataKey,
    pub emote_flags: i32,
    pub emote_spec_proc: i32,
    pub emote_spec_proc_param: i32,
    pub event_sound_id: SoundEntriesKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn emotes() {
        let mut file = File::open("../wrath-dbc/Emotes.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Emotes::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Emotes::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
