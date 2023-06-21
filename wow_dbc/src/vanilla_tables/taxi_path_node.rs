use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::map::*;
use crate::vanilla_tables::taxi_path::*;

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

        if header.record_size != 36 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 36,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 9 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 9,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (TaxiPathNode) uint32
            let id = TaxiPathNodeKey::new(crate::util::read_u32_le(chunk)?);

            // taxi_path: foreign_key (TaxiPath) uint32
            let taxi_path = TaxiPathKey::new(crate::util::read_u32_le(chunk)?.into());

            // node_index: int32
            let node_index = crate::util::read_i32_le(chunk)?;

            // map: foreign_key (Map) uint32
            let map = MapKey::new(crate::util::read_u32_le(chunk)?.into());

            // location_x: float
            let location_x = crate::util::read_f32_le(chunk)?;

            // location_y: float
            let location_y = crate::util::read_f32_le(chunk)?;

            // location_z: float
            let location_z = crate::util::read_f32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // delay: int32
            let delay = crate::util::read_i32_le(chunk)?;


            rows.push(TaxiPathNodeRow {
                id,
                taxi_path,
                node_index,
                map,
                location_x,
                location_y,
                location_z,
                flags,
                delay,
            });
        }

        Ok(TaxiPathNode { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 9,
            record_size: 36,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (TaxiPathNode) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // taxi_path: foreign_key (TaxiPath) uint32
            b.write_all(&(row.taxi_path.id as u32).to_le_bytes())?;

            // node_index: int32
            b.write_all(&row.node_index.to_le_bytes())?;

            // map: foreign_key (Map) uint32
            b.write_all(&(row.map.id as u32).to_le_bytes())?;

            // location_x: float
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: float
            b.write_all(&row.location_y.to_le_bytes())?;

            // location_z: float
            b.write_all(&row.location_z.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // delay: int32
            b.write_all(&row.delay.to_le_bytes())?;

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
        if header.record_size != 36 {
            panic!("invalid record size, expected 36")
        }

        if header.field_count != 9 {
            panic!("invalid field count, expected 9")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            TaxiPathNodeRow {
                id: TaxiPathNodeKey::new(0),
                taxi_path: TaxiPathKey::new(0),
                node_index: 0,
                map: MapKey::new(0),
                location_x: 0.0,
                location_y: 0.0,
                location_z: 0.0,
                flags: 0,
                delay: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (TaxiPathNode) uint32
            let id = TaxiPathNodeKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // taxi_path: foreign_key (TaxiPath) uint32
            let taxi_path = TaxiPathKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // node_index: int32
            let node_index = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // map: foreign_key (Map) uint32
            let map = MapKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // location_x: float
            let location_x = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // location_y: float
            let location_y = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // location_z: float
            let location_z = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // delay: int32
            let delay = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = TaxiPathNodeRow {
                id,
                taxi_path,
                node_index,
                map,
                location_x,
                location_y,
                location_z,
                flags,
                delay,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> TaxiPathNode {
        TaxiPathNode {
            rows: self.rows.iter().map(|s| TaxiPathNodeRow {
                id: s.id,
                taxi_path: s.taxi_path,
                node_index: s.node_index,
                map: s.map,
                location_x: s.location_x,
                location_y: s.location_y,
                location_z: s.location_z,
                flags: s.flags,
                delay: s.delay,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct TaxiPathNodeKey {
    pub id: u32
}

impl TaxiPathNodeKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
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

impl From<u32> for TaxiPathNodeKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TaxiPathNodeRow {
    pub id: TaxiPathNodeKey,
    pub taxi_path: TaxiPathKey,
    pub node_index: i32,
    pub map: MapKey,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub flags: i32,
    pub delay: i32,
}

