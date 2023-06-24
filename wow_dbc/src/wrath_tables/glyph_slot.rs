use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlyphSlot {
    pub rows: Vec<GlyphSlotRow>,
}

impl DbcTable for GlyphSlot {
    type Row = GlyphSlotRow;

    const FILENAME: &'static str = "GlyphSlot.dbc";

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

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (GlyphSlot) int32
            let id = GlyphSlotKey::new(crate::util::read_i32_le(chunk)?);

            // ty: int32
            let ty = crate::util::read_i32_le(chunk)?;

            // tooltip: int32
            let tooltip = crate::util::read_i32_le(chunk)?;


            rows.push(GlyphSlotRow {
                id,
                ty,
                tooltip,
            });
        }

        Ok(GlyphSlot { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 3,
            record_size: 12,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (GlyphSlot) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // ty: int32
            b.write_all(&row.ty.to_le_bytes())?;

            // tooltip: int32
            b.write_all(&row.tooltip.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for GlyphSlot {
    type PrimaryKey = GlyphSlotKey;
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
pub struct GlyphSlotKey {
    pub id: i32
}

impl GlyphSlotKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for GlyphSlotKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for GlyphSlotKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for GlyphSlotKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for GlyphSlotKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for GlyphSlotKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for GlyphSlotKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for GlyphSlotKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for GlyphSlotKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for GlyphSlotKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for GlyphSlotKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlyphSlotRow {
    pub id: GlyphSlotKey,
    pub ty: i32,
    pub tooltip: i32,
}

