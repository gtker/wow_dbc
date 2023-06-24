use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroundEffectDoodad {
    pub rows: Vec<GroundEffectDoodadRow>,
}

impl DbcTable for GroundEffectDoodad {
    type Row = GroundEffectDoodadRow;

    const FILENAME: &'static str = "GroundEffectDoodad.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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

            // id: primary_key (GroundEffectDoodad) uint32
            let id = GroundEffectDoodadKey::new(crate::util::read_u32_le(chunk)?);

            // internal_id: int32
            let internal_id = crate::util::read_i32_le(chunk)?;

            // doodad_path: string_ref
            let doodad_path = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(GroundEffectDoodadRow {
                id,
                internal_id,
                doodad_path,
            });
        }

        Ok(GroundEffectDoodad { rows, })
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
            // id: primary_key (GroundEffectDoodad) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // internal_id: int32
            b.write_all(&row.internal_id.to_le_bytes())?;

            // doodad_path: string_ref
            if !row.doodad_path.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.doodad_path.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for GroundEffectDoodad {
    type PrimaryKey = GroundEffectDoodadKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl GroundEffectDoodad {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.doodad_path.is_empty() { b.write_all(row.doodad_path.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.doodad_path.is_empty() { sum += row.doodad_path.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct GroundEffectDoodadKey {
    pub id: u32
}

impl GroundEffectDoodadKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for GroundEffectDoodadKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for GroundEffectDoodadKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for GroundEffectDoodadKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroundEffectDoodadRow {
    pub id: GroundEffectDoodadKey,
    pub internal_id: i32,
    pub doodad_path: String,
}

