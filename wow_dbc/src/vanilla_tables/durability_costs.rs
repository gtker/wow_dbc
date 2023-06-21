use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DurabilityCosts {
    pub rows: Vec<DurabilityCostsRow>,
}

impl DbcTable for DurabilityCosts {
    type Row = DurabilityCostsRow;

    fn filename() -> &'static str { "DurabilityCosts.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 120 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 120,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 30 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 30,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (DurabilityCosts) uint32
            let id = DurabilityCostsKey::new(crate::util::read_u32_le(chunk)?);

            // weapon_subclass_cost: int32[21]
            let weapon_subclass_cost = crate::util::read_array_i32::<21>(chunk)?;

            // armour_subclass_cost: int32[8]
            let armour_subclass_cost = crate::util::read_array_i32::<8>(chunk)?;


            rows.push(DurabilityCostsRow {
                id,
                weapon_subclass_cost,
                armour_subclass_cost,
            });
        }

        Ok(DurabilityCosts { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 30,
            record_size: 120,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (DurabilityCosts) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // weapon_subclass_cost: int32[21]
            for i in row.weapon_subclass_cost {
                b.write_all(&i.to_le_bytes())?;
            }


            // armour_subclass_cost: int32[8]
            for i in row.armour_subclass_cost {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for DurabilityCosts {
    type PrimaryKey = DurabilityCostsKey;
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
pub struct ConstDurabilityCosts<const S: usize> {
    pub rows: [DurabilityCostsRow; S],
}

impl<const S: usize> ConstDurabilityCosts<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 120 {
            panic!("invalid record size, expected 120")
        }

        if header.field_count != 30 {
            panic!("invalid field count, expected 30")
        }

        let mut b_offset = 20;
        let mut rows = [
            DurabilityCostsRow {
                id: DurabilityCostsKey::new(0),
                weapon_subclass_cost: [0; 21],
                armour_subclass_cost: [0; 8],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (DurabilityCosts) uint32
            let id = DurabilityCostsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // weapon_subclass_cost: int32[21]
            let weapon_subclass_cost = {
                let mut a = [0; 21];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // armour_subclass_cost: int32[8]
            let armour_subclass_cost = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = DurabilityCostsRow {
                id,
                weapon_subclass_cost,
                armour_subclass_cost,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct DurabilityCostsKey {
    pub id: u32
}

impl DurabilityCostsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for DurabilityCostsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for DurabilityCostsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for DurabilityCostsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DurabilityCostsRow {
    pub id: DurabilityCostsKey,
    pub weapon_subclass_cost: [i32; 21],
    pub armour_subclass_cost: [i32; 8],
}

