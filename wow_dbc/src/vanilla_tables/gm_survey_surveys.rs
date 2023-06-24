use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GMSurveySurveys {
    pub rows: Vec<GMSurveySurveysRow>,
}

impl DbcTable for GMSurveySurveys {
    type Row = GMSurveySurveysRow;

    const FILENAME: &'static str = "GMSurveySurveys.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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

            // id: primary_key (GMSurveySurveys) uint32
            let id = GMSurveySurveysKey::new(crate::util::read_u32_le(chunk)?);

            // questions: uint32[10]
            let questions = crate::util::read_array_u32::<10>(chunk)?;


            rows.push(GMSurveySurveysRow {
                id,
                questions,
            });
        }

        Ok(GMSurveySurveys { rows, })
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
            // id: primary_key (GMSurveySurveys) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // questions: uint32[10]
            for i in row.questions {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for GMSurveySurveys {
    type PrimaryKey = GMSurveySurveysKey;
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
pub struct GMSurveySurveysKey {
    pub id: u32
}

impl GMSurveySurveysKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for GMSurveySurveysKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for GMSurveySurveysKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for GMSurveySurveysKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GMSurveySurveysRow {
    pub id: GMSurveySurveysKey,
    pub questions: [u32; 10],
}

