use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::sound_entries_advanced::*;

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

        if header.record_size != 120 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 120,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 30 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 30,
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

            // sound_entries_advanced_id: foreign_key (SoundEntriesAdvanced) int32
            let sound_entries_advanced_id = SoundEntriesAdvancedKey::new(crate::util::read_i32_le(chunk)?.into());


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
                sound_entries_advanced_id,
            });
        }

        Ok(SoundEntries { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 30,
            record_size: 120,
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

            // sound_entries_advanced_id: foreign_key (SoundEntriesAdvanced) int32
            b.write_all(&(row.sound_entries_advanced_id.id as i32).to_le_bytes())?;

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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstSoundEntries<const S: usize> {
    pub rows: [ConstSoundEntriesRow; S],
}

impl<const S: usize> ConstSoundEntries<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 120 {
            panic!("invalid record size, expected 120")
        }

        if header.field_count != 30 {
            panic!("invalid field count, expected 30")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstSoundEntriesRow {
                id: SoundEntriesKey::new(0),
                sound_type: 0,
                name: "",
                file: [""; 10],
                freq: [0; 10],
                directory_base: "",
                volume_float: 0.0,
                flags: 0,
                min_distance: 0.0,
                distance_cutoff: 0.0,
                e_a_x_def: 0,
                sound_entries_advanced_id: SoundEntriesAdvancedKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SoundEntries) int32
            let id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_type: int32
            let sound_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // name: string_ref
            let name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // file: string_ref[10]
            let file = {
                let mut a = [""; 10];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::get_string_from_block(b_offset, b, string_block);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // freq: int32[10]
            let freq = {
                let mut a = [0; 10];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // directory_base: string_ref
            let directory_base = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // volume_float: float
            let volume_float = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // min_distance: float
            let min_distance = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // distance_cutoff: float
            let distance_cutoff = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // e_a_x_def: int32
            let e_a_x_def = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // sound_entries_advanced_id: foreign_key (SoundEntriesAdvanced) int32
            let sound_entries_advanced_id = SoundEntriesAdvancedKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = ConstSoundEntriesRow {
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
                sound_entries_advanced_id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SoundEntries {
        SoundEntries {
            rows: self.rows.iter().map(|s| SoundEntriesRow {
                id: s.id,
                sound_type: s.sound_type,
                name: s.name.to_string(),
                file: s.file.map(|a| a.to_string()),
                freq: s.freq,
                directory_base: s.directory_base.to_string(),
                volume_float: s.volume_float,
                flags: s.flags,
                min_distance: s.min_distance,
                distance_cutoff: s.distance_cutoff,
                e_a_x_def: s.e_a_x_def,
                sound_entries_advanced_id: s.sound_entries_advanced_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
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
    pub sound_entries_advanced_id: SoundEntriesAdvancedKey,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstSoundEntriesRow {
    pub id: SoundEntriesKey,
    pub sound_type: i32,
    pub name: &'static str,
    pub file: [&'static str; 10],
    pub freq: [i32; 10],
    pub directory_base: &'static str,
    pub volume_float: f32,
    pub flags: i32,
    pub min_distance: f32,
    pub distance_cutoff: f32,
    pub e_a_x_def: i32,
    pub sound_entries_advanced_id: SoundEntriesAdvancedKey,
}

