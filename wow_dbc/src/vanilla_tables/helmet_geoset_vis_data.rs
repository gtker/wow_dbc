use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HelmetGeosetVisData {
    pub rows: Vec<HelmetGeosetVisDataRow>,
}

impl DbcTable for HelmetGeosetVisData {
    type Row = HelmetGeosetVisDataRow;

    fn filename() -> &'static str { "HelmetGeosetVisData.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 24 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 24,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 6 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 6,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (HelmetGeosetVisData) uint32
            let id = HelmetGeosetVisDataKey::new(crate::util::read_u32_le(chunk)?);

            // hide_geoset: int32[5]
            let hide_geoset = crate::util::read_array_i32::<5>(chunk)?;


            rows.push(HelmetGeosetVisDataRow {
                id,
                hide_geoset,
            });
        }

        Ok(HelmetGeosetVisData { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 6,
            record_size: 24,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (HelmetGeosetVisData) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // hide_geoset: int32[5]
            for i in row.hide_geoset {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for HelmetGeosetVisData {
    type PrimaryKey = HelmetGeosetVisDataKey;
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
pub struct HelmetGeosetVisDataKey {
    pub id: u32
}

impl HelmetGeosetVisDataKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for HelmetGeosetVisDataKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for HelmetGeosetVisDataKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for HelmetGeosetVisDataKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HelmetGeosetVisDataRow {
    pub id: HelmetGeosetVisDataKey,
    pub hide_geoset: [i32; 5],
}

