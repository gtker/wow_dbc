use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::sound_entries::SoundEntriesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZoneIntroMusicTable {
    pub rows: Vec<ZoneIntroMusicTableRow>,
}

impl DbcTable for ZoneIntroMusicTable {
    type Row = ZoneIntroMusicTableRow;

    const FILENAME: &'static str = "ZoneIntroMusicTable.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (ZoneIntroMusicTable) uint32
            let id = ZoneIntroMusicTableKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // intro_sound: foreign_key (SoundEntries) uint32
            let intro_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // priority_over_ambience: bool32
            let priority_over_ambience = crate::util::read_u32_le(chunk)? != 0;

            // min_delay: int32
            let min_delay = crate::util::read_i32_le(chunk)?;


            rows.push(ZoneIntroMusicTableRow {
                id,
                name,
                intro_sound,
                priority_over_ambience,
                min_delay,
            });
        }

        Ok(ZoneIntroMusicTable { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 5,
            record_size: 20,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ZoneIntroMusicTable) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref
            if !row.name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // intro_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.intro_sound.id as u32).to_le_bytes())?;

            // priority_over_ambience: bool32
            b.write_all(&u32::from(row.priority_over_ambience).to_le_bytes())?;

            // min_delay: int32
            b.write_all(&row.min_delay.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ZoneIntroMusicTable {
    type PrimaryKey = ZoneIntroMusicTableKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl ZoneIntroMusicTable {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.name.is_empty() { b.write_all(row.name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.name.is_empty() { sum += row.name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ZoneIntroMusicTableKey {
    pub id: u32
}

impl ZoneIntroMusicTableKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for ZoneIntroMusicTableKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for ZoneIntroMusicTableKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for ZoneIntroMusicTableKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for ZoneIntroMusicTableKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ZoneIntroMusicTableKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for ZoneIntroMusicTableKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for ZoneIntroMusicTableKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for ZoneIntroMusicTableKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ZoneIntroMusicTableKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ZoneIntroMusicTableKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZoneIntroMusicTableRow {
    pub id: ZoneIntroMusicTableKey,
    pub name: String,
    pub intro_sound: SoundEntriesKey,
    pub priority_over_ambience: bool,
    pub min_delay: i32,
}

