use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::map::MapKey;
use crate::wrath_tables::taxi_path::TaxiPathKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaxiPathNode {
    pub rows: Vec<TaxiPathNodeRow>,
}

impl DbcTable for TaxiPathNode {
    type Row = TaxiPathNodeRow;

    const FILENAME: &'static str = "TaxiPathNode.dbc";
    const FIELD_COUNT: usize = 11;
    const ROW_SIZE: usize = 44;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
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
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
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
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaxiPathNodeKey {
    pub id: i32
}

impl TaxiPathNodeKey {
    pub const fn new(id: i32) -> Self {
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

impl TryFrom<u32> for TaxiPathNodeKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for TaxiPathNodeKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for TaxiPathNodeKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for TaxiPathNodeKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for TaxiPathNodeKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn taxi_path_node() {
        let mut file = File::open("../wrath-dbc/TaxiPathNode.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = TaxiPathNode::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = TaxiPathNode::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
