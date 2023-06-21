use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellRadius {
    pub rows: Vec<SpellRadiusRow>,
}

impl DbcTable for SpellRadius {
    type Row = SpellRadiusRow;

    fn filename() -> &'static str { "SpellRadius.dbc" }

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

            // id: primary_key (SpellRadius) uint32
            let id = SpellRadiusKey::new(crate::util::read_u32_le(chunk)?);

            // radius: float
            let radius = crate::util::read_f32_le(chunk)?;

            // radius_per_level: float
            let radius_per_level = crate::util::read_f32_le(chunk)?;

            // radius_max: float
            let radius_max = crate::util::read_f32_le(chunk)?;


            rows.push(SpellRadiusRow {
                id,
                radius,
                radius_per_level,
                radius_max,
            });
        }

        Ok(SpellRadius { rows, })
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
            // id: primary_key (SpellRadius) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // radius: float
            b.write_all(&row.radius.to_le_bytes())?;

            // radius_per_level: float
            b.write_all(&row.radius_per_level.to_le_bytes())?;

            // radius_max: float
            b.write_all(&row.radius_max.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellRadius {
    type PrimaryKey = SpellRadiusKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstSpellRadius<const S: usize> {
    pub rows: [SpellRadiusRow; S],
}

impl<const S: usize> ConstSpellRadius<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 16 {
            panic!("invalid record size, expected 16")
        }

        if header.field_count != 4 {
            panic!("invalid field count, expected 4")
        }

        let mut b_offset = 20;
        let mut rows = [
            SpellRadiusRow {
                id: SpellRadiusKey::new(0),
                radius: 0.0,
                radius_per_level: 0.0,
                radius_max: 0.0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SpellRadius) uint32
            let id = SpellRadiusKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // radius: float
            let radius = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // radius_per_level: float
            let radius_per_level = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // radius_max: float
            let radius_max = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = SpellRadiusRow {
                id,
                radius,
                radius_per_level,
                radius_max,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellRadiusKey {
    pub id: u32
}

impl SpellRadiusKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SpellRadiusKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SpellRadiusKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SpellRadiusKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SpellRadiusRow {
    pub id: SpellRadiusKey,
    pub radius: f32,
    pub radius_per_level: f32,
    pub radius_max: f32,
}

