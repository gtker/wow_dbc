use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

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
        let header = header::parse_header(&header)?;

        if header.record_size != 32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 8,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (HelmetGeosetVisData) int32
            let id = HelmetGeosetVisDataKey::new(crate::util::read_i32_le(chunk)?);

            // hide_geoset: int32[7]
            let hide_geoset = crate::util::read_array_i32::<7>(chunk)?;


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
            field_count: 8,
            record_size: 32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (HelmetGeosetVisData) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // hide_geoset: int32[7]
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
    pub id: i32
}

impl HelmetGeosetVisDataKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for HelmetGeosetVisDataKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for HelmetGeosetVisDataKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for HelmetGeosetVisDataKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HelmetGeosetVisDataRow {
    pub id: HelmetGeosetVisDataKey,
    pub hide_geoset: [i32; 7],
}

