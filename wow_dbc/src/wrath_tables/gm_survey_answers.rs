use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};
use crate::wrath_tables::gm_survey_questions::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GMSurveyAnswers {
    pub rows: Vec<GMSurveyAnswersRow>,
}

impl DbcTable for GMSurveyAnswers {
    type Row = GMSurveyAnswersRow;

    fn filename() -> &'static str { "GMSurveyAnswers.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 80 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 80,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 20 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 20,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (GMSurveyAnswers) int32
            let id = GMSurveyAnswersKey::new(crate::util::read_i32_le(chunk)?);

            // sort_index: int32
            let sort_index = crate::util::read_i32_le(chunk)?;

            // g_m_survey_question_id: foreign_key (GMSurveyQuestions) int32
            let g_m_survey_question_id = GMSurveyQuestionsKey::new(crate::util::read_i32_le(chunk)?.into());

            // answer_lang: string_ref_loc (Extended)
            let answer_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(GMSurveyAnswersRow {
                id,
                sort_index,
                g_m_survey_question_id,
                answer_lang,
            });
        }

        Ok(GMSurveyAnswers { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 20,
            record_size: 80,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (GMSurveyAnswers) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sort_index: int32
            b.write_all(&row.sort_index.to_le_bytes())?;

            // g_m_survey_question_id: foreign_key (GMSurveyQuestions) int32
            b.write_all(&(row.g_m_survey_question_id.id as i32).to_le_bytes())?;

            // answer_lang: string_ref_loc (Extended)
            b.write_all(&row.answer_lang.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for GMSurveyAnswers {
    type PrimaryKey = GMSurveyAnswersKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl GMSurveyAnswers {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.answer_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.answer_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstGMSurveyAnswers<const S: usize> {
    pub rows: [ConstGMSurveyAnswersRow; S],
}

impl<const S: usize> ConstGMSurveyAnswers<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 80 {
            panic!("invalid record size, expected 80")
        }

        if header.field_count != 20 {
            panic!("invalid field count, expected 20")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstGMSurveyAnswersRow {
                id: GMSurveyAnswersKey::new(0),
                sort_index: 0,
                g_m_survey_question_id: GMSurveyQuestionsKey::new(0),
                answer_lang: crate::ConstExtendedLocalizedString::empty(),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (GMSurveyAnswers) int32
            let id = GMSurveyAnswersKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sort_index: int32
            let sort_index = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // g_m_survey_question_id: foreign_key (GMSurveyQuestions) int32
            let g_m_survey_question_id = GMSurveyQuestionsKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // answer_lang: string_ref_loc (Extended)
            let answer_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            rows[i] = ConstGMSurveyAnswersRow {
                id,
                sort_index,
                g_m_survey_question_id,
                answer_lang,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> GMSurveyAnswers {
        GMSurveyAnswers {
            rows: self.rows.iter().map(|s| GMSurveyAnswersRow {
                id: s.id,
                sort_index: s.sort_index,
                g_m_survey_question_id: s.g_m_survey_question_id,
                answer_lang: s.answer_lang.to_string(),
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct GMSurveyAnswersKey {
    pub id: i32
}

impl GMSurveyAnswersKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for GMSurveyAnswersKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for GMSurveyAnswersKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for GMSurveyAnswersKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for GMSurveyAnswersKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for GMSurveyAnswersKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GMSurveyAnswersRow {
    pub id: GMSurveyAnswersKey,
    pub sort_index: i32,
    pub g_m_survey_question_id: GMSurveyQuestionsKey,
    pub answer_lang: ExtendedLocalizedString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstGMSurveyAnswersRow {
    pub id: GMSurveyAnswersKey,
    pub sort_index: i32,
    pub g_m_survey_question_id: GMSurveyQuestionsKey,
    pub answer_lang: ConstExtendedLocalizedString,
}

