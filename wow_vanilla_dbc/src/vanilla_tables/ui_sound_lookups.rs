use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq)]
pub struct UISoundLookups {
    pub rows: Vec<UISoundLookupsRow>,
}

impl DbcTable for UISoundLookups {
    type Row = UISoundLookupsRow;

    fn filename() -> &'static str { "UISoundLookups.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 12 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 12,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 3 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 3,
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

            // id: primary_key (UISoundLookups) uint32
            let id = UISoundLookupsKey::new(crate::util::read_u32_le(chunk)?);

            // sound_entry: foreign_key (SoundEntries) uint32
            let sound_entry = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // internal_name: string_ref
            let internal_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(UISoundLookupsRow {
                id,
                sound_entry,
                internal_name,
            });
        }

        Ok(UISoundLookups { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 3,
            record_size: 12,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (UISoundLookups) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_entry: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_entry.id as u32).to_le_bytes())?;

            // internal_name: string_ref
            if !row.internal_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.internal_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for UISoundLookups {
    type PrimaryKey = UISoundLookupsKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl UISoundLookups {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.internal_name.is_empty() { b.write_all(row.internal_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.internal_name.is_empty() { sum += row.internal_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct UISoundLookupsKey {
    pub id: u32
}

impl UISoundLookupsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct UISoundLookupsRow {
    pub id: UISoundLookupsKey,
    pub sound_entry: SoundEntriesKey,
    pub internal_name: String,
}

