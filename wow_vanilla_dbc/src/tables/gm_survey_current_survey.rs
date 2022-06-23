use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::tables::gm_survey_surveys::*;

#[derive(Debug, Clone, PartialEq)]
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

            // language: Language
            let language = Language::try_from(crate::util::read_i32_le(chunk)?)?;

            // gm_survey: foreign_key (GMSurveySurveys) uint32
            let gm_survey = GMSurveySurveysKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(GMSurveyCurrentSurveyRow {
                language,
                gm_survey,
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
            // language: Language
            b.write_all(&(row.language.as_int() as i32).to_le_bytes())?;

            // gm_survey: foreign_key (GMSurveySurveys) uint32
            b.write_all(&(row.gm_survey.id as u32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Language {
    English,
    Korean,
    French,
    German,
    Chinese,
    Taiwanese,
    SpanishSpain,
    SpanishLatinAmerica,
}

impl TryFrom<i32> for Language {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::English,
            1 => Self::Korean,
            2 => Self::French,
            3 => Self::German,
            4 => Self::Chinese,
            5 => Self::Taiwanese,
            6 => Self::SpanishSpain,
            7 => Self::SpanishLatinAmerica,
            val => return Err(crate::InvalidEnumError::new("Language", val as i64)),
        })
    }

}

impl Language {
    const fn as_int(&self) -> i32 {
        match self {
            Self::English => 0,
            Self::Korean => 1,
            Self::French => 2,
            Self::German => 3,
            Self::Chinese => 4,
            Self::Taiwanese => 5,
            Self::SpanishSpain => 6,
            Self::SpanishLatinAmerica => 7,
        }

    }

}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct GMSurveyCurrentSurveyRow {
    pub language: Language,
    pub gm_survey: GMSurveySurveysKey,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gm_survey_current_survey() {
        let contents = include_bytes!("../../../dbc/GMSurveyCurrentSurvey.dbc");
        let actual = GMSurveyCurrentSurvey::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = GMSurveyCurrentSurvey::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
