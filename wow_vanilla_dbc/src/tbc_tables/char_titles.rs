use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharTitles {
    pub rows: Vec<CharTitlesRow>,
}

impl DbcTable for CharTitles {
    type Row = CharTitlesRow;

    fn filename() -> &'static str { "CharTitles.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 84 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 84,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 21 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 21,
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

            // id: primary_key (CharTitles) int32
            let id = CharTitlesKey::new(crate::util::read_i32_le(chunk)?);

            // condition_id: int32
            let condition_id = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc
            let name_lang = crate::util::read_localized_string(chunk, &string_block)?;

            // name1_lang: string_ref_loc
            let name1_lang = crate::util::read_localized_string(chunk, &string_block)?;

            // mask_id: int32
            let mask_id = crate::util::read_i32_le(chunk)?;


            rows.push(CharTitlesRow {
                id,
                condition_id,
                name_lang,
                name1_lang,
                mask_id,
            });
        }

        Ok(CharTitles { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 21,
            record_size: 84,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (CharTitles) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // condition_id: int32
            b.write_all(&row.condition_id.to_le_bytes())?;

            // name_lang: string_ref_loc
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // name1_lang: string_ref_loc
            b.write_all(&row.name1_lang.string_indices_as_array(&mut string_index))?;

            // mask_id: int32
            b.write_all(&row.mask_id.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for CharTitles {
    type PrimaryKey = CharTitlesKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl CharTitles {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
            row.name1_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
            sum += row.name1_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct CharTitlesKey {
    pub id: i32
}

impl CharTitlesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharTitlesRow {
    pub id: CharTitlesKey,
    pub condition_id: i32,
    pub name_lang: LocalizedString,
    pub name1_lang: LocalizedString,
    pub mask_id: i32,
}

