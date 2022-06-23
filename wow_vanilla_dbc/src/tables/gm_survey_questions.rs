use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;

#[derive(Debug, Clone, PartialEq)]
pub struct GMSurveyQuestions {
    pub rows: Vec<GMSurveyQuestionsRow>,
}

impl DbcTable for GMSurveyQuestions {
    type Row = GMSurveyQuestionsRow;

    fn filename() -> &'static str { "GMSurveyQuestions.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 40 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 40,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 10 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 40,
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

            // id: primary_key (GMSurveyQuestions) uint32
            let id = GMSurveyQuestionsKey::new(crate::util::read_u32_le(chunk)?);

            // question: string_ref_loc
            let question = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(GMSurveyQuestionsRow {
                id,
                question,
            });
        }

        Ok(GMSurveyQuestions { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 10,
            record_size: 40,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (GMSurveyQuestions) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // question: string_ref_loc
            b.write_all(&row.question.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for GMSurveyQuestions {
    type PrimaryKey = GMSurveyQuestionsKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl GMSurveyQuestions {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.question.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.question.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct GMSurveyQuestionsKey {
    pub id: u32
}

impl GMSurveyQuestionsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct GMSurveyQuestionsRow {
    pub id: GMSurveyQuestionsKey,
    pub question: LocalizedString,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gm_survey_questions() {
        let contents = include_bytes!("../../../dbc/GMSurveyQuestions.dbc");
        let actual = GMSurveyQuestions::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = GMSurveyQuestions::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
