use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::wrath_tables::file_data::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MovieFileData {
    pub rows: Vec<MovieFileDataRow>,
}

impl DbcTable for MovieFileData {
    type Row = MovieFileDataRow;

    fn filename() -> &'static str { "MovieFileData.dbc" }

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

            // file_data_id: foreign_key (FileData) int32
            let file_data_id = FileDataKey::new(crate::util::read_i32_le(chunk)?.into());

            // resolution: int32
            let resolution = crate::util::read_i32_le(chunk)?;


            rows.push(MovieFileDataRow {
                file_data_id,
                resolution,
            });
        }

        Ok(MovieFileData { rows, })
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
            // file_data_id: foreign_key (FileData) int32
            b.write_all(&(row.file_data_id.id as i32).to_le_bytes())?;

            // resolution: int32
            b.write_all(&row.resolution.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstMovieFileData<const S: usize> {
    pub rows: [MovieFileDataRow; S],
}

impl<const S: usize> ConstMovieFileData<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 8 {
            panic!("invalid record size, expected 8")
        }

        if header.field_count != 2 {
            panic!("invalid field count, expected 2")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            MovieFileDataRow {
                file_data_id: FileDataKey::new(0),
                resolution: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // file_data_id: foreign_key (FileData) int32
            let file_data_id = FileDataKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // resolution: int32
            let resolution = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = MovieFileDataRow {
                file_data_id,
                resolution,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> MovieFileData {
        MovieFileData {
            rows: self.rows.iter().map(|s| MovieFileDataRow {
                file_data_id: s.file_data_id,
                resolution: s.resolution,
            }).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MovieFileDataRow {
    pub file_data_id: FileDataKey,
    pub resolution: i32,
}

