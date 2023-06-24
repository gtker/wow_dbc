use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::taxi_nodes::TaxiNodesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaxiPath {
    pub rows: Vec<TaxiPathRow>,
}

impl DbcTable for TaxiPath {
    type Row = TaxiPathRow;

    const FILENAME: &'static str = "TaxiPath.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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

            // id: primary_key (TaxiPath) int32
            let id = TaxiPathKey::new(crate::util::read_i32_le(chunk)?);

            // from_taxi_node: foreign_key (TaxiNodes) int32
            let from_taxi_node = TaxiNodesKey::new(crate::util::read_i32_le(chunk)?.into());

            // to_taxi_node: foreign_key (TaxiNodes) int32
            let to_taxi_node = TaxiNodesKey::new(crate::util::read_i32_le(chunk)?.into());

            // cost: int32
            let cost = crate::util::read_i32_le(chunk)?;


            rows.push(TaxiPathRow {
                id,
                from_taxi_node,
                to_taxi_node,
                cost,
            });
        }

        Ok(TaxiPath { rows, })
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
            // id: primary_key (TaxiPath) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // from_taxi_node: foreign_key (TaxiNodes) int32
            b.write_all(&(row.from_taxi_node.id as i32).to_le_bytes())?;

            // to_taxi_node: foreign_key (TaxiNodes) int32
            b.write_all(&(row.to_taxi_node.id as i32).to_le_bytes())?;

            // cost: int32
            b.write_all(&row.cost.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for TaxiPath {
    type PrimaryKey = TaxiPathKey;
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
pub struct TaxiPathKey {
    pub id: i32
}

impl TaxiPathKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for TaxiPathKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for TaxiPathKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for TaxiPathKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl From<u8> for TaxiPathKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for TaxiPathKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl TryFrom<u32> for TaxiPathKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaxiPathRow {
    pub id: TaxiPathKey,
    pub from_taxi_node: TaxiNodesKey,
    pub to_taxi_node: TaxiNodesKey,
    pub cost: i32,
}

