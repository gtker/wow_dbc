use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DurabilityQuality {
    pub rows: Vec<DurabilityQualityRow>,
}

impl DbcTable for DurabilityQuality {
    type Row = DurabilityQualityRow;

    fn filename() -> &'static str { "DurabilityQuality.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 8,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 2 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 2,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (DurabilityQuality) uint32
            let id = DurabilityQualityKey::new(crate::util::read_u32_le(chunk)?);

            // data: float
            let data = crate::util::read_f32_le(chunk)?;


            rows.push(DurabilityQualityRow {
                id,
                data,
            });
        }

        Ok(DurabilityQuality { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 2,
            record_size: 8,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (DurabilityQuality) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // data: float
            b.write_all(&row.data.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for DurabilityQuality {
    type PrimaryKey = DurabilityQualityKey;
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
pub struct ConstDurabilityQuality<const S: usize> {
    pub rows: [DurabilityQualityRow; S],
}

impl<const S: usize> ConstDurabilityQuality<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 8 {
            panic!("invalid record size, expected 8")
        }

        if header.field_count != 2 {
            panic!("invalid field count, expected 2")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            DurabilityQualityRow {
                id: DurabilityQualityKey::new(0),
                data: 0.0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (DurabilityQuality) uint32
            let id = DurabilityQualityKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // data: float
            let data = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = DurabilityQualityRow {
                id,
                data,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> DurabilityQuality {
        DurabilityQuality {
            rows: self.rows.iter().map(|s| DurabilityQualityRow {
                id: s.id,
                data: s.data,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct DurabilityQualityKey {
    pub id: u32
}

impl DurabilityQualityKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for DurabilityQualityKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for DurabilityQualityKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for DurabilityQualityKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DurabilityQualityRow {
    pub id: DurabilityQualityKey,
    pub data: f32,
}

