use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::file_data::*;
use crate::wrath_tables::movie::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MovieVariation {
    pub rows: Vec<MovieVariationRow>,
}

impl DbcTable for MovieVariation {
    type Row = MovieVariationRow;

    fn filename() -> &'static str { "MovieVariation.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 12 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 12,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 3 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 3,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (MovieVariation) int32
            let id = MovieVariationKey::new(crate::util::read_i32_le(chunk)?);

            // movie_id: foreign_key (Movie) int32
            let movie_id = MovieKey::new(crate::util::read_i32_le(chunk)?.into());

            // file_data_id: foreign_key (FileData) int32
            let file_data_id = FileDataKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(MovieVariationRow {
                id,
                movie_id,
                file_data_id,
            });
        }

        Ok(MovieVariation { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 3,
            record_size: 12,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (MovieVariation) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // movie_id: foreign_key (Movie) int32
            b.write_all(&(row.movie_id.id as i32).to_le_bytes())?;

            // file_data_id: foreign_key (FileData) int32
            b.write_all(&(row.file_data_id.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for MovieVariation {
    type PrimaryKey = MovieVariationKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct MovieVariationKey {
    pub id: i32
}

impl MovieVariationKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MovieVariationRow {
    pub id: MovieVariationKey,
    pub movie_id: MovieKey,
    pub file_data_id: FileDataKey,
}

