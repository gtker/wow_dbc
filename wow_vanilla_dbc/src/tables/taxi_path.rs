use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tables::taxi_nodes::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TaxiPath {
    pub rows: Vec<TaxiPathRow>,
}

impl DbcTable for TaxiPath {
    type Row = TaxiPathRow;

    fn filename() -> &'static str { "TaxiPath.dbc" }

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
                    expected: 16,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (TaxiPath) uint32
            let id = TaxiPathKey::new(crate::util::read_u32_le(chunk)?);

            // source_taxi_node: foreign_key (TaxiNodes) uint32
            let source_taxi_node = TaxiNodesKey::new(crate::util::read_u32_le(chunk)?.into());

            // destination_taxi_node: foreign_key (TaxiNodes) uint32
            let destination_taxi_node = TaxiNodesKey::new(crate::util::read_u32_le(chunk)?.into());

            // cost: int32
            let cost = crate::util::read_i32_le(chunk)?;


            rows.push(TaxiPathRow {
                id,
                source_taxi_node,
                destination_taxi_node,
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
            // id: primary_key (TaxiPath) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // source_taxi_node: foreign_key (TaxiNodes) uint32
            b.write_all(&(row.source_taxi_node.id as u32).to_le_bytes())?;

            // destination_taxi_node: foreign_key (TaxiNodes) uint32
            b.write_all(&(row.destination_taxi_node.id as u32).to_le_bytes())?;

            // cost: int32
            b.write_all(&row.cost.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for TaxiPath {
    type PrimaryKey = TaxiPathKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct TaxiPathKey {
    pub id: u32
}

impl TaxiPathKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct TaxiPathRow {
    pub id: TaxiPathKey,
    pub source_taxi_node: TaxiNodesKey,
    pub destination_taxi_node: TaxiNodesKey,
    pub cost: i32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn taxi_path() {
        let contents = include_bytes!("../../../dbc/TaxiPath.dbc");
        let actual = TaxiPath::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = TaxiPath::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}