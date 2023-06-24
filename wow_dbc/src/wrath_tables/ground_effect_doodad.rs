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

            // id: primary_key (GroundEffectDoodad) int32
            let id = GroundEffectDoodadKey::new(crate::util::read_i32_le(chunk)?);

            // doodadpath: string_ref
            let doodadpath = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(GroundEffectDoodadRow {
                id,
                doodadpath,
                flags,
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
            // id: primary_key (GroundEffectDoodad) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // doodadpath: string_ref
            if !row.doodadpath.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.doodadpath.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for GroundEffectDoodad {
    type PrimaryKey = GroundEffectDoodadKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl GroundEffectDoodad {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.doodadpath.is_empty() { b.write_all(row.doodadpath.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.doodadpath.is_empty() { sum += row.doodadpath.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct GroundEffectDoodadKey {
    pub id: i32
}

impl GroundEffectDoodadKey {
    pub const fn new(id: i32) -> Self {
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

impl From<i8> for GroundEffectDoodadKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for GroundEffectDoodadKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for GroundEffectDoodadKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for GroundEffectDoodadKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for GroundEffectDoodadKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for GroundEffectDoodadKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for GroundEffectDoodadKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for GroundEffectDoodadKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroundEffectDoodadRow {
    pub id: GroundEffectDoodadKey,
    pub doodadpath: String,
    pub flags: i32,
}

