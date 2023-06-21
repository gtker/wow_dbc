use crate::header::{HEADER_SIZE, DbcHeader, parse_header};
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
        let header = parse_header(&header)?;

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

            // id: primary_key (DurabilityCosts) int32
            let id = DurabilityCostsKey::new(crate::util::read_i32_le(chunk)?);

            // weapon_sub_class_cost: int32[21]
            let weapon_sub_class_cost = crate::util::read_array_i32::<21>(chunk)?;

            // armor_sub_class_cost: int32[8]
            let armor_sub_class_cost = crate::util::read_array_i32::<8>(chunk)?;


            rows.push(DurabilityCostsRow {
                id,
                weapon_sub_class_cost,
                armor_sub_class_cost,
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
            // id: primary_key (DurabilityCosts) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // weapon_sub_class_cost: int32[21]
            for i in row.weapon_sub_class_cost {
                b.write_all(&i.to_le_bytes())?;
            }


            // armor_sub_class_cost: int32[8]
            for i in row.armor_sub_class_cost {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct DurabilityCostsKey {
    pub id: i32
}

impl DurabilityCostsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for DurabilityCostsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for DurabilityCostsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for DurabilityCostsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DurabilityCostsRow {
    pub id: DurabilityCostsKey,
    pub weapon_sub_class_cost: [i32; 21],
    pub armor_sub_class_cost: [i32; 8],
}

