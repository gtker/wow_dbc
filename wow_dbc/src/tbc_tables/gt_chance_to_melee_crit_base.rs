use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct gtChanceToMeleeCritBase {
    pub rows: Vec<gtChanceToMeleeCritBaseRow>,
}

impl DbcTable for gtChanceToMeleeCritBase {
    type Row = gtChanceToMeleeCritBaseRow;

    fn filename() -> &'static str { "gtChanceToMeleeCritBase.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 4 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 4,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 1 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 1,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // data: float
            let data = crate::util::read_f32_le(chunk)?;


            rows.push(gtChanceToMeleeCritBaseRow {
                data,
            });
        }

        Ok(gtChanceToMeleeCritBase { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 1,
            record_size: 4,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // data: float
            b.write_all(&row.data.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct gtChanceToMeleeCritBaseRow {
    pub data: f32,
}

