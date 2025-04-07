use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::sound_entries::SoundEntriesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZoneMusic {
    pub rows: Vec<ZoneMusicRow>,
}

impl DbcTable for ZoneMusic {
    type Row = ZoneMusicRow;

    const FILENAME: &'static str = "ZoneMusic.dbc";
    const FIELD_COUNT: usize = 8;
    const ROW_SIZE: usize = 32;

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

            // id: primary_key (ZoneMusic) uint32
            let id = ZoneMusicKey::new(crate::util::read_u32_le(chunk)?);

            // set_name: string_ref
            let set_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // silence_interval_min_day: int32
            let silence_interval_min_day = crate::util::read_i32_le(chunk)?;

            // silence_interval_min_night: int32
            let silence_interval_min_night = crate::util::read_i32_le(chunk)?;

            // silence_interval_max_day: int32
            let silence_interval_max_day = crate::util::read_i32_le(chunk)?;

            // silence_interval_max_night: int32
            let silence_interval_max_night = crate::util::read_i32_le(chunk)?;

            // day_sound: foreign_key (SoundEntries) uint32
            let day_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // night_sound: foreign_key (SoundEntries) uint32
            let night_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(ZoneMusicRow {
                id,
                set_name,
                silence_interval_min_day,
                silence_interval_min_night,
                silence_interval_max_day,
                silence_interval_max_night,
                day_sound,
                night_sound,
            });
        }

        Ok(ZoneMusic { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (ZoneMusic) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // set_name: string_ref
            b.write_all(&string_cache.add_string(&row.set_name).to_le_bytes())?;

            // silence_interval_min_day: int32
            b.write_all(&row.silence_interval_min_day.to_le_bytes())?;

            // silence_interval_min_night: int32
            b.write_all(&row.silence_interval_min_night.to_le_bytes())?;

            // silence_interval_max_day: int32
            b.write_all(&row.silence_interval_max_day.to_le_bytes())?;

            // silence_interval_max_night: int32
            b.write_all(&row.silence_interval_max_night.to_le_bytes())?;

            // day_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.day_sound.id as u32).to_le_bytes())?;

            // night_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.night_sound.id as u32).to_le_bytes())?;

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

impl Indexable for ZoneMusic {
    type PrimaryKey = ZoneMusicKey;
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
pub struct ZoneMusicKey {
    pub id: u32
}

impl ZoneMusicKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for ZoneMusicKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for ZoneMusicKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for ZoneMusicKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for ZoneMusicKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ZoneMusicKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for ZoneMusicKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for ZoneMusicKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for ZoneMusicKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ZoneMusicKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ZoneMusicKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZoneMusicRow {
    pub id: ZoneMusicKey,
    pub set_name: String,
    pub silence_interval_min_day: i32,
    pub silence_interval_min_night: i32,
    pub silence_interval_max_day: i32,
    pub silence_interval_max_night: i32,
    pub day_sound: SoundEntriesKey,
    pub night_sound: SoundEntriesKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn zone_music() {
        let mut file = File::open("../vanilla-dbc/ZoneMusic.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ZoneMusic::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ZoneMusic::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
