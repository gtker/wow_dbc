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
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstMovieVariation<const S: usize> {
    pub rows: [MovieVariationRow; S],
}

impl<const S: usize> ConstMovieVariation<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 12 {
            panic!("invalid record size, expected 12")
        }

        if header.field_count != 3 {
            panic!("invalid field count, expected 3")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            MovieVariationRow {
                id: MovieVariationKey::new(0),
                movie_id: MovieKey::new(0),
                file_data_id: FileDataKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (MovieVariation) int32
            let id = MovieVariationKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // movie_id: foreign_key (Movie) int32
            let movie_id = MovieKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // file_data_id: foreign_key (FileData) int32
            let file_data_id = FileDataKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = MovieVariationRow {
                id,
                movie_id,
                file_data_id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> MovieVariation {
        MovieVariation {
            rows: self.rows.iter().map(|s| MovieVariationRow {
                id: s.id,
                movie_id: s.movie_id,
                file_data_id: s.file_data_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
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

impl From<i8> for MovieVariationKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for MovieVariationKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for MovieVariationKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for MovieVariationKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for MovieVariationKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MovieVariationRow {
    pub id: MovieVariationKey,
    pub movie_id: MovieKey,
    pub file_data_id: FileDataKey,
}

