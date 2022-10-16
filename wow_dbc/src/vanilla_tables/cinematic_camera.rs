use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CinematicCamera {
    pub rows: Vec<CinematicCameraRow>,
}

impl DbcTable for CinematicCamera {
    type Row = CinematicCameraRow;

    fn filename() -> &'static str { "CinematicCamera.dbc" }

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

            // id: primary_key (CinematicCamera) uint32
            let id = CinematicCameraKey::new(crate::util::read_u32_le(chunk)?);

            // model: string_ref
            let model = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // sound_entry: foreign_key (SoundEntries) uint32
            let sound_entry = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // location_x: float
            let location_x = crate::util::read_f32_le(chunk)?;

            // location_y: float
            let location_y = crate::util::read_f32_le(chunk)?;

            // location_z: float
            let location_z = crate::util::read_f32_le(chunk)?;

            // rotation: float
            let rotation = crate::util::read_f32_le(chunk)?;


            rows.push(CinematicCameraRow {
                id,
                model,
                sound_entry,
                location_x,
                location_y,
                location_z,
                rotation,
            });
        }

        Ok(CinematicCamera { rows, })
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
            // id: primary_key (CinematicCamera) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model: string_ref
            if !row.model.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.model.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // sound_entry: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_entry.id as u32).to_le_bytes())?;

            // location_x: float
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: float
            b.write_all(&row.location_y.to_le_bytes())?;

            // location_z: float
            b.write_all(&row.location_z.to_le_bytes())?;

            // rotation: float
            b.write_all(&row.rotation.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for CinematicCamera {
    type PrimaryKey = CinematicCameraKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl CinematicCamera {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.model.is_empty() { b.write_all(row.model.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.model.is_empty() { sum += row.model.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CinematicCameraKey {
    pub id: u32
}

impl CinematicCameraKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u32> for CinematicCameraKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CinematicCameraRow {
    pub id: CinematicCameraKey,
    pub model: String,
    pub sound_entry: SoundEntriesKey,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub rotation: f32,
}

