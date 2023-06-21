use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::map::*;
use crate::wrath_tables::sound_entries_advanced::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundEmitters {
    pub rows: Vec<SoundEmittersRow>,
}

impl DbcTable for SoundEmitters {
    type Row = SoundEmittersRow;

    fn filename() -> &'static str { "SoundEmitters.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 40 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 40,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 10 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 10,
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

            // id: primary_key (SoundEmitters) int32
            let id = SoundEmittersKey::new(crate::util::read_i32_le(chunk)?);

            // position: float[3]
            let position = crate::util::read_array_f32::<3>(chunk)?;

            // direction: float[3]
            let direction = crate::util::read_array_f32::<3>(chunk)?;

            // sound_entry_advanced_id: foreign_key (SoundEntriesAdvanced) int32
            let sound_entry_advanced_id = SoundEntriesAdvancedKey::new(crate::util::read_i32_le(chunk)?.into());

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(SoundEmittersRow {
                id,
                position,
                direction,
                sound_entry_advanced_id,
                map_id,
                name,
            });
        }

        Ok(SoundEmitters { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 10,
            record_size: 40,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SoundEmitters) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // position: float[3]
            for i in row.position {
                b.write_all(&i.to_le_bytes())?;
            }


            // direction: float[3]
            for i in row.direction {
                b.write_all(&i.to_le_bytes())?;
            }


            // sound_entry_advanced_id: foreign_key (SoundEntriesAdvanced) int32
            b.write_all(&(row.sound_entry_advanced_id.id as i32).to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // name: string_ref
            if !row.name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SoundEmitters {
    type PrimaryKey = SoundEmittersKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl SoundEmitters {
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
pub struct SoundEmittersKey {
    pub id: i32
}

impl SoundEmittersKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SoundEmittersKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SoundEmittersKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SoundEmittersKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SoundEmittersKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SoundEmittersKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundEmittersRow {
    pub id: SoundEmittersKey,
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub sound_entry_advanced_id: SoundEntriesAdvancedKey,
    pub map_id: MapKey,
    pub name: String,
}

