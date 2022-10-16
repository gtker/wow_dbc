use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundEntries {
    pub rows: Vec<SoundEntriesRow>,
}

impl DbcTable for SoundEntries {
    type Row = SoundEntriesRow;

    fn filename() -> &'static str { "SoundEntries.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

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
            // id: primary_key (SoundEntries) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_type: int32
            b.write_all(&row.sound_type.to_le_bytes())?;

            // name: string_ref
            if !row.name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // file: string_ref[10]
            for i in &row.file {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // freq: int32[10]
            for i in row.freq {
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

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SoundEntries {
    type PrimaryKey = SoundEntriesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl SoundEntries {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.name.is_empty() { b.write_all(row.name.as_bytes())?; b.write_all(&[0])?; };
            for s in &row.file {
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
            for s in &row.file {
                if !s.is_empty() { sum += s.len() + 1; };
            }

            if !row.directory_base.is_empty() { sum += row.directory_base.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SoundEntriesKey {
    pub id: i32
}

impl SoundEntriesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i32> for SoundEntriesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

