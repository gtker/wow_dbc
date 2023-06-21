use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellRuneCost {
    pub rows: Vec<SpellRuneCostRow>,
}

impl DbcTable for SpellRuneCost {
    type Row = SpellRuneCostRow;

    fn filename() -> &'static str { "SpellRuneCost.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 20 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 20,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 5 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 5,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SpellRuneCost) int32
            let id = SpellRuneCostKey::new(crate::util::read_i32_le(chunk)?);

            // blood: int32
            let blood = crate::util::read_i32_le(chunk)?;

            // unholy: int32
            let unholy = crate::util::read_i32_le(chunk)?;

            // frost: int32
            let frost = crate::util::read_i32_le(chunk)?;

            // runic_power: int32
            let runic_power = crate::util::read_i32_le(chunk)?;


            rows.push(SpellRuneCostRow {
                id,
                blood,
                unholy,
                frost,
                runic_power,
            });
        }

        Ok(SpellRuneCost { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 5,
            record_size: 20,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SpellRuneCost) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // blood: int32
            b.write_all(&row.blood.to_le_bytes())?;

            // unholy: int32
            b.write_all(&row.unholy.to_le_bytes())?;

            // frost: int32
            b.write_all(&row.frost.to_le_bytes())?;

            // runic_power: int32
            b.write_all(&row.runic_power.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellRuneCost {
    type PrimaryKey = SpellRuneCostKey;
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
pub struct ConstSpellRuneCost<const S: usize> {
    pub rows: [SpellRuneCostRow; S],
}

impl<const S: usize> ConstSpellRuneCost<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 20 {
            panic!("invalid record size, expected 20")
        }

        if header.field_count != 5 {
            panic!("invalid field count, expected 5")
        }

        let mut b_offset = 20;
        let mut rows = [
            SpellRuneCostRow {
                id: SpellRuneCostKey::new(0),
                blood: 0,
                unholy: 0,
                frost: 0,
                runic_power: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SpellRuneCost) int32
            let id = SpellRuneCostKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // blood: int32
            let blood = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // unholy: int32
            let unholy = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // frost: int32
            let frost = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // runic_power: int32
            let runic_power = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = SpellRuneCostRow {
                id,
                blood,
                unholy,
                frost,
                runic_power,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellRuneCostKey {
    pub id: i32
}

impl SpellRuneCostKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SpellRuneCostKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SpellRuneCostKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SpellRuneCostKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SpellRuneCostKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SpellRuneCostKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellRuneCostRow {
    pub id: SpellRuneCostKey,
    pub blood: i32,
    pub unholy: i32,
    pub frost: i32,
    pub runic_power: i32,
}

