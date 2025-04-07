use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundEntries {
    pub rows: Vec<SoundEntriesRow>,
}

impl DbcTable for SoundEntries {
    type Row = SoundEntriesRow;

    const FILENAME: &'static str = "SoundEntries.dbc";
    const FIELD_COUNT: usize = 29;
    const ROW_SIZE: usize = 116;

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

            // id: primary_key (SoundEntries) int32
            let id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?);

            // sound_type: int32
            let sound_type = crate::util::read_i32_le(chunk)?;

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // file: string_ref[10]
            let file = {
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

            // freq: int32[10]
            let freq = crate::util::read_array_i32::<10>(chunk)?;

            // directory_base: string_ref
            let directory_base = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // volume_float: float
            let volume_float = crate::util::read_f32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // min_distance: float
            let min_distance = crate::util::read_f32_le(chunk)?;

            // distance_cutoff: float
            let distance_cutoff = crate::util::read_f32_le(chunk)?;

            // e_a_x_def: int32
            let e_a_x_def = crate::util::read_i32_le(chunk)?;


            rows.push(SoundEntriesRow {
                id,
                sound_type,
                name,
                file,
                freq,
                directory_base,
                volume_float,
                flags,
                min_distance,
                distance_cutoff,
                e_a_x_def,
            });
        }

        Ok(SoundEntries { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SoundEntries) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_type: int32
            b.write_all(&row.sound_type.to_le_bytes())?;

            // name: string_ref
            b.write_all(&string_cache.add_string(&row.name).to_le_bytes())?;

            // file: string_ref[10]
            for i in &row.file {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // freq: int32[10]
            for i in row.freq {
                b.write_all(&i.to_le_bytes())?;
            }


            // directory_base: string_ref
            b.write_all(&string_cache.add_string(&row.directory_base).to_le_bytes())?;

            // volume_float: float
            b.write_all(&row.volume_float.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // min_distance: float
            b.write_all(&row.min_distance.to_le_bytes())?;

            // distance_cutoff: float
            b.write_all(&row.distance_cutoff.to_le_bytes())?;

            // e_a_x_def: int32
            b.write_all(&row.e_a_x_def.to_le_bytes())?;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundEntriesKey {
    pub id: i32
}

impl SoundEntriesKey {
    pub const fn new(id: i32) -> Self {
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

impl From<i8> for SoundEntriesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for SoundEntriesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for SoundEntriesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for SoundEntriesKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SoundEntriesKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for SoundEntriesKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SoundEntriesKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SoundEntriesKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundEntriesRow {
    pub id: SoundEntriesKey,
    pub sound_type: i32,
    pub name: String,
    pub file: [String; 10],
    pub freq: [i32; 10],
    pub directory_base: String,
    pub volume_float: f32,
    pub flags: i32,
    pub min_distance: f32,
    pub distance_cutoff: f32,
    pub e_a_x_def: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn sound_entries() {
        let mut file = File::open("../tbc-dbc/SoundEntries.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SoundEntries::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SoundEntries::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
