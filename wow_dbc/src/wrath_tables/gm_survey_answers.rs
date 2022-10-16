use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::ExtendedLocalizedString;
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
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct GMSurveyAnswersKey {
    pub id: i32
}

impl GMSurveyAnswersKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GMSurveyAnswersRow {
    pub id: GMSurveyAnswersKey,
    pub sort_index: i32,
    pub g_m_survey_question_id: GMSurveyQuestionsKey,
    pub answer_lang: ExtendedLocalizedString,
}

