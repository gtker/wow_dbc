use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlyphSlot {
    pub rows: Vec<GlyphSlotRow>,
}

impl DbcTable for GlyphSlot {
    type Row = GlyphSlotRow;

    fn filename() -> &'static str { "GlyphSlot.dbc" }

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
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstGlyphSlot<const S: usize> {
    pub rows: [GlyphSlotRow; S],
}

impl<const S: usize> ConstGlyphSlot<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 12 {
            panic!("invalid record size, expected 12")
        }

        if header.field_count != 3 {
            panic!("invalid field count, expected 3")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            GlyphSlotRow {
                id: GlyphSlotKey::new(0),
                ty: 0,
                tooltip: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (GlyphSlot) int32
            let id = GlyphSlotKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // ty: int32
            let ty = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // tooltip: int32
            let tooltip = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = GlyphSlotRow {
                id,
                ty,
                tooltip,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> GlyphSlot {
        GlyphSlot {
            rows: self.rows.iter().map(|s| GlyphSlotRow {
                id: s.id,
                ty: s.ty,
                tooltip: s.tooltip,
            }).collect(),
        }
    }
    // TODO: Indexable?
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlyphSlotRow {
    pub id: GlyphSlotKey,
    pub ty: i32,
    pub tooltip: i32,
}

