use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::vanilla_tables::gm_survey_surveys::*;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstGMSurveyCurrentSurvey<const S: usize> {
    pub rows: [GMSurveyCurrentSurveyRow; S],
}

impl<const S: usize> ConstGMSurveyCurrentSurvey<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 8 {
            panic!("invalid record size, expected 8")
        }

        if header.field_count != 2 {
            panic!("invalid field count, expected 2")
        }

        let mut b_offset = 20;
        let mut rows = [
            GMSurveyCurrentSurveyRow {
                language: Language::English,
                gm_survey: GMSurveySurveysKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // language: Language
            let language = match Language::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            // gm_survey: foreign_key (GMSurveySurveys) uint32
            let gm_survey = GMSurveySurveysKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = GMSurveyCurrentSurveyRow {
                language,
                gm_survey,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> GMSurveyCurrentSurvey {
        GMSurveyCurrentSurvey {
            rows: self.rows.iter().map(|s| GMSurveyCurrentSurveyRow {
                language: s.language,
                gm_survey: s.gm_survey,
            }).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
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

impl Language {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            0 => Self::English,
            1 => Self::Korean,
            2 => Self::French,
            3 => Self::German,
            4 => Self::Chinese,
            5 => Self::Taiwanese,
            6 => Self::SpanishSpain,
            7 => Self::SpanishLatinAmerica,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for Language {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("Language", value as i64))
    }

}

impl Language {
    pub const fn as_int(&self) -> i32 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GMSurveyCurrentSurveyRow {
    pub language: Language,
    pub gm_survey: GMSurveySurveysKey,
}

