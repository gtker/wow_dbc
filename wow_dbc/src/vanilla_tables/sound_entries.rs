use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;
use wow_world_base::vanilla::SoundType;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundEntries {
    pub rows: Vec<SoundEntriesRow>,
}

impl DbcTable for SoundEntries {
    type Row = SoundEntriesRow;

    const FILENAME: &'static str = "SoundEntries.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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

            // id: primary_key (SoundEntries) uint32
            let id = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?);

            // sound_type: SoundType
            let sound_type = crate::util::read_i32_le(chunk)?.try_into()?;

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // files: string_ref[10]
            let files = {
                let mut arr = Vec::with_capacity(10);
                for _ in 0..10 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // frequency: uint32[10]
            let frequency = crate::util::read_array_u32::<10>(chunk)?;

            // directory_base: string_ref
            let directory_base = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // volume: float
            let volume = crate::util::read_f32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // min_distance: float
            let min_distance = crate::util::read_f32_le(chunk)?;

            // distance_cutoff: float
            let distance_cutoff = crate::util::read_f32_le(chunk)?;

            // sound_entries_advanced: int32
            let sound_entries_advanced = crate::util::read_i32_le(chunk)?;


            rows.push(SoundEntriesRow {
                id,
                sound_type,
                name,
                files,
                frequency,
                directory_base,
                volume,
                flags,
                min_distance,
                distance_cutoff,
                sound_entries_advanced,
            });
        }

        Ok(SoundEntries { rows, })
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
            // id: primary_key (SoundEntries) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_type: SoundType
            b.write_all(&(row.sound_type.as_int() as i32).to_le_bytes())?;

            // name: string_ref
            if !row.name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // files: string_ref[10]
            for i in &row.files {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // frequency: uint32[10]
            for i in row.frequency {
                b.write_all(&i.to_le_bytes())?;
            }


            // directory_base: string_ref
            if !row.directory_base.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.directory_base.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // volume: float
            b.write_all(&row.volume.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // min_distance: float
            b.write_all(&row.min_distance.to_le_bytes())?;

            // distance_cutoff: float
            b.write_all(&row.distance_cutoff.to_le_bytes())?;

            // sound_entries_advanced: int32
            b.write_all(&row.sound_entries_advanced.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SoundEntries {
    type PrimaryKey = SoundEntriesKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl SoundEntries {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.name.is_empty() { b.write_all(row.name.as_bytes())?; b.write_all(&[0])?; };
            for s in &row.files {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

            if !row.directory_base.is_empty() { b.write_all(row.directory_base.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.name.is_empty() { sum += row.name.len() + 1; };
            for s in &row.files {
                if !s.is_empty() { sum += s.len() + 1; };
            }

            if !row.directory_base.is_empty() { sum += row.directory_base.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SoundEntriesKey {
    pub id: u32
}

impl SoundEntriesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SoundEntriesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for SoundEntriesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for SoundEntriesKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for SoundEntriesKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SoundEntriesKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for SoundEntriesKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for SoundEntriesKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for SoundEntriesKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SoundEntriesKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SoundEntriesKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundEntriesRow {
    pub id: SoundEntriesKey,
    pub sound_type: SoundType,
    pub name: String,
    pub files: [String; 10],
    pub frequency: [u32; 10],
    pub directory_base: String,
    pub volume: f32,
    pub flags: i32,
    pub min_distance: f32,
    pub distance_cutoff: f32,
    pub sound_entries_advanced: i32,
}

