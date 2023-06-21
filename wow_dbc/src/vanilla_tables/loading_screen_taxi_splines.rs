use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::taxi_path::*;

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
pub struct ConstLoadingScreenTaxiSplines<const S: usize> {
    pub rows: [LoadingScreenTaxiSplinesRow; S],
}

impl<const S: usize> ConstLoadingScreenTaxiSplines<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 76 {
            panic!("invalid record size, expected 76")
        }

        if header.field_count != 19 {
            panic!("invalid field count, expected 19")
        }

        let mut b_offset = 20;
        let mut rows = [
            LoadingScreenTaxiSplinesRow {
                id: LoadingScreenTaxiSplinesKey::new(0),
                taxi_path: TaxiPathKey::new(0),
                location_x: [0.0; 8],
                location_y: [0.0; 8],
                leg_index: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (LoadingScreenTaxiSplines) uint32
            let id = LoadingScreenTaxiSplinesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // taxi_path: foreign_key (TaxiPath) uint32
            let taxi_path = TaxiPathKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // location_x: float[8]
            let location_x = {
                let mut a = [0.0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // location_y: float[8]
            let location_y = {
                let mut a = [0.0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // leg_index: int32
            let leg_index = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = LoadingScreenTaxiSplinesRow {
                id,
                taxi_path,
                location_x,
                location_y,
                leg_index,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct LoadingScreenTaxiSplinesKey {
    pub id: u32
}

impl LoadingScreenTaxiSplinesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
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

impl From<u32> for LoadingScreenTaxiSplinesKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LoadingScreenTaxiSplinesRow {
    pub id: LoadingScreenTaxiSplinesKey,
    pub taxi_path: TaxiPathKey,
    pub location_x: [f32; 8],
    pub location_y: [f32; 8],
    pub leg_index: i32,
}

