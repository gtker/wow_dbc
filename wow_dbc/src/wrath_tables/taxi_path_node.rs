use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::map::*;
use crate::wrath_tables::taxi_path::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TaxiPathNode {
    pub rows: Vec<TaxiPathNodeRow>,
}

impl DbcTable for TaxiPathNode {
    type Row = TaxiPathNodeRow;

    fn filename() -> &'static str { "TaxiPathNode.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 44 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 44,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 11 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 11,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (TaxiPathNode) int32
            let id = TaxiPathNodeKey::new(crate::util::read_i32_le(chunk)?);

            // path_id: foreign_key (TaxiPath) int32
            let path_id = TaxiPathKey::new(crate::util::read_i32_le(chunk)?.into());

            // node_index: int32
            let node_index = crate::util::read_i32_le(chunk)?;

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // loc: float[3]
            let loc = crate::util::read_array_f32::<3>(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // delay: int32
            let delay = crate::util::read_i32_le(chunk)?;

            // arrival_event_id: int32
            let arrival_event_id = crate::util::read_i32_le(chunk)?;

            // departure_event_id: int32
            let departure_event_id = crate::util::read_i32_le(chunk)?;


            rows.push(TaxiPathNodeRow {
                id,
                path_id,
                node_index,
                continent_id,
                loc,
                flags,
                delay,
                arrival_event_id,
                departure_event_id,
            });
        }

        Ok(TaxiPathNode { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 11,
            record_size: 44,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (TaxiPathNode) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // path_id: foreign_key (TaxiPath) int32
            b.write_all(&(row.path_id.id as i32).to_le_bytes())?;

            // node_index: int32
            b.write_all(&row.node_index.to_le_bytes())?;

            // continent_id: foreign_key (Map) int32
            b.write_all(&(row.continent_id.id as i32).to_le_bytes())?;

            // loc: float[3]
            for i in row.loc {
                b.write_all(&i.to_le_bytes())?;
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // delay: int32
            b.write_all(&row.delay.to_le_bytes())?;

            // arrival_event_id: int32
            b.write_all(&row.arrival_event_id.to_le_bytes())?;

            // departure_event_id: int32
            b.write_all(&row.departure_event_id.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for TaxiPathNode {
    type PrimaryKey = TaxiPathNodeKey;
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
pub struct ConstTaxiPathNode<const S: usize> {
    pub rows: [TaxiPathNodeRow; S],
}

impl<const S: usize> ConstTaxiPathNode<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 44 {
            panic!("invalid record size, expected 44")
        }

        if header.field_count != 11 {
            panic!("invalid field count, expected 11")
        }

        let mut b_offset = 20;
        let mut rows = [
            TaxiPathNodeRow {
                id: TaxiPathNodeKey::new(0),
                path_id: TaxiPathKey::new(0),
                node_index: 0,
                continent_id: MapKey::new(0),
                loc: [0.0; 3],
                flags: 0,
                delay: 0,
                arrival_event_id: 0,
                departure_event_id: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (TaxiPathNode) int32
            let id = TaxiPathNodeKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // path_id: foreign_key (TaxiPath) int32
            let path_id = TaxiPathKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // node_index: int32
            let node_index = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // loc: float[3]
            let loc = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // delay: int32
            let delay = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // arrival_event_id: int32
            let arrival_event_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // departure_event_id: int32
            let departure_event_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = TaxiPathNodeRow {
                id,
                path_id,
                node_index,
                continent_id,
                loc,
                flags,
                delay,
                arrival_event_id,
                departure_event_id,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct TaxiPathNodeKey {
    pub id: i32
}

impl TaxiPathNodeKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for TaxiPathNodeKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for TaxiPathNodeKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for TaxiPathNodeKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for TaxiPathNodeKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for TaxiPathNodeKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TaxiPathNodeRow {
    pub id: TaxiPathNodeKey,
    pub path_id: TaxiPathKey,
    pub node_index: i32,
    pub continent_id: MapKey,
    pub loc: [f32; 3],
    pub flags: i32,
    pub delay: i32,
    pub arrival_event_id: i32,
    pub departure_event_id: i32,
}

