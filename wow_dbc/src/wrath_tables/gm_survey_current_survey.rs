use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::gm_survey_surveys::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GMSurveyCurrentSurvey {
    pub rows: Vec<GMSurveyCurrentSurveyRow>,
}

impl DbcTable for GMSurveyCurrentSurvey {
    type Row = GMSurveyCurrentSurveyRow;

    fn filename() -> &'static str { "GMSurveyCurrentSurvey.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 8,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 2 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 2,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (GMSurveyCurrentSurvey) int32
            let id = GMSurveyCurrentSurveyKey::new(crate::util::read_i32_le(chunk)?);

            // gm_survey_id: foreign_key (GMSurveySurveys) int32
            let gm_survey_id = GMSurveySurveysKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(GMSurveyCurrentSurveyRow {
                id,
                gm_survey_id,
            });
        }

        Ok(GMSurveyCurrentSurvey { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 2,
            record_size: 8,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (GMSurveyCurrentSurvey) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // gm_survey_id: foreign_key (GMSurveySurveys) int32
            b.write_all(&(row.gm_survey_id.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for GMSurveyCurrentSurvey {
    type PrimaryKey = GMSurveyCurrentSurveyKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct GMSurveyCurrentSurveyKey {
    pub id: i32
}

impl GMSurveyCurrentSurveyKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GMSurveyCurrentSurveyRow {
    pub id: GMSurveyCurrentSurveyKey,
    pub gm_survey_id: GMSurveySurveysKey,
}

