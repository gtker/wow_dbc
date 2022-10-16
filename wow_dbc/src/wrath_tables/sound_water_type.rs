use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoundWaterType {
    pub rows: Vec<SoundWaterTypeRow>,
}

impl DbcTable for SoundWaterType {
    type Row = SoundWaterTypeRow;

    fn filename() -> &'static str { "SoundWaterType.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 16,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 4 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 4,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SoundWaterType) int32
            let id = SoundWaterTypeKey::new(crate::util::read_i32_le(chunk)?);

            // sound_type: int32
            let sound_type = crate::util::read_i32_le(chunk)?;

            // sound_subtype: int32
            let sound_subtype = crate::util::read_i32_le(chunk)?;

            // sound_id: foreign_key (SoundEntries) int32
            let sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(SoundWaterTypeRow {
                id,
                sound_type,
                sound_subtype,
                sound_id,
            });
        }

        Ok(SoundWaterType { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 4,
            record_size: 16,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SoundWaterType) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_type: int32
            b.write_all(&row.sound_type.to_le_bytes())?;

            // sound_subtype: int32
            b.write_all(&row.sound_subtype.to_le_bytes())?;

            // sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_id.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SoundWaterType {
    type PrimaryKey = SoundWaterTypeKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SoundWaterTypeKey {
    pub id: i32
}

impl SoundWaterTypeKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SoundWaterTypeKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SoundWaterTypeKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SoundWaterTypeKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SoundWaterTypeKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SoundWaterTypeKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoundWaterTypeRow {
    pub id: SoundWaterTypeKey,
    pub sound_type: i32,
    pub sound_subtype: i32,
    pub sound_id: SoundEntriesKey,
}

