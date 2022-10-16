use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::taxi_path::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LoadingScreenTaxiSplines {
    pub rows: Vec<LoadingScreenTaxiSplinesRow>,
}

impl DbcTable for LoadingScreenTaxiSplines {
    type Row = LoadingScreenTaxiSplinesRow;

    fn filename() -> &'static str { "LoadingScreenTaxiSplines.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 76 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 76,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 19 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 19,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (LoadingScreenTaxiSplines) int32
            let id = LoadingScreenTaxiSplinesKey::new(crate::util::read_i32_le(chunk)?);

            // path_id: foreign_key (TaxiPath) int32
            let path_id = TaxiPathKey::new(crate::util::read_i32_le(chunk)?.into());

            // locx: float[8]
            let locx = crate::util::read_array_f32::<8>(chunk)?;

            // locy: float[8]
            let locy = crate::util::read_array_f32::<8>(chunk)?;

            // leg_index: int32
            let leg_index = crate::util::read_i32_le(chunk)?;


            rows.push(LoadingScreenTaxiSplinesRow {
                id,
                path_id,
                locx,
                locy,
                leg_index,
            });
        }

        Ok(LoadingScreenTaxiSplines { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 19,
            record_size: 76,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (LoadingScreenTaxiSplines) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // path_id: foreign_key (TaxiPath) int32
            b.write_all(&(row.path_id.id as i32).to_le_bytes())?;

            // locx: float[8]
            for i in row.locx {
                b.write_all(&i.to_le_bytes())?;
            }


            // locy: float[8]
            for i in row.locy {
                b.write_all(&i.to_le_bytes())?;
            }


            // leg_index: int32
            b.write_all(&row.leg_index.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for LoadingScreenTaxiSplines {
    type PrimaryKey = LoadingScreenTaxiSplinesKey;
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
pub struct LoadingScreenTaxiSplinesKey {
    pub id: i32
}

impl LoadingScreenTaxiSplinesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for LoadingScreenTaxiSplinesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for LoadingScreenTaxiSplinesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for LoadingScreenTaxiSplinesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for LoadingScreenTaxiSplinesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for LoadingScreenTaxiSplinesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LoadingScreenTaxiSplinesRow {
    pub id: LoadingScreenTaxiSplinesKey,
    pub path_id: TaxiPathKey,
    pub locx: [f32; 8],
    pub locy: [f32; 8],
    pub leg_index: i32,
}

