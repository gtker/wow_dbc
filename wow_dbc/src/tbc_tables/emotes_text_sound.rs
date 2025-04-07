use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::chr_races::ChrRacesKey;
use crate::tbc_tables::emotes_text::EmotesTextKey;
use crate::tbc_tables::sound_entries::SoundEntriesKey;
use crate::util::StringCache;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmotesTextSound {
    pub rows: Vec<EmotesTextSoundRow>,
}

impl DbcTable for EmotesTextSound {
    type Row = EmotesTextSoundRow;

    const FILENAME: &'static str = "EmotesTextSound.dbc";
    const FIELD_COUNT: usize = 5;
    const ROW_SIZE: usize = 20;

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

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

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

impl Indexable for EmotesTextSound {
    type PrimaryKey = EmotesTextSoundKey;
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
pub struct EmotesTextSoundKey {
    pub id: i32
}

impl EmotesTextSoundKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for EmotesTextSoundKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for EmotesTextSoundKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for EmotesTextSoundKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for EmotesTextSoundKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for EmotesTextSoundKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for EmotesTextSoundKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for EmotesTextSoundKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for EmotesTextSoundKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for EmotesTextSoundKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for EmotesTextSoundKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmotesTextSoundRow {
    pub id: EmotesTextSoundKey,
    pub emotes_text_id: EmotesTextKey,
    pub race_id: ChrRacesKey,
    pub sex_id: i32,
    pub sound_id: SoundEntriesKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn emotes_text_sound() {
        let mut file = File::open("../tbc-dbc/EmotesTextSound.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = EmotesTextSound::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = EmotesTextSound::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
