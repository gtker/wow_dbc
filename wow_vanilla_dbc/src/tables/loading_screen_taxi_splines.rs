use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tables::taxi_path::*;

#[derive(Debug, Clone, PartialEq)]
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
                    expected: 76,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (LoadingScreenTaxiSplines) uint32
            let id = LoadingScreenTaxiSplinesKey::new(crate::util::read_u32_le(chunk)?);

            // taxi_path: foreign_key (TaxiPath) uint32
            let taxi_path = TaxiPathKey::new(crate::util::read_u32_le(chunk)?.into());

            // location_x: float[8]
            let location_x = crate::util::read_array_f32::<8>(chunk)?;

            // location_y: float[8]
            let location_y = crate::util::read_array_f32::<8>(chunk)?;

            // leg_index: int32
            let leg_index = crate::util::read_i32_le(chunk)?;


            rows.push(LoadingScreenTaxiSplinesRow {
                id,
                taxi_path,
                location_x,
                location_y,
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
            // id: primary_key (LoadingScreenTaxiSplines) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // taxi_path: foreign_key (TaxiPath) uint32
            b.write_all(&(row.taxi_path.id as u32).to_le_bytes())?;

            // location_x: float[8]
            for i in row.location_x {
                b.write_all(&i.to_le_bytes())?;
            }


            // location_y: float[8]
            for i in row.location_y {
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
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct LoadingScreenTaxiSplinesKey {
    pub id: u32
}

impl LoadingScreenTaxiSplinesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadingScreenTaxiSplinesRow {
    pub id: LoadingScreenTaxiSplinesKey,
    pub taxi_path: TaxiPathKey,
    pub location_x: [f32; 8],
    pub location_y: [f32; 8],
    pub leg_index: i32,
}

