use crate::header::{HEADER_SIZE, DbcHeader, parse_header};
use crate::DbcTable;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PaperDollItemFrame {
    pub rows: Vec<PaperDollItemFrameRow>,
}

impl DbcTable for PaperDollItemFrame {
    type Row = PaperDollItemFrameRow;

    fn filename() -> &'static str { "PaperDollItemFrame.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // item_button_name: string_ref
            let item_button_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // slot_icon: string_ref
            let slot_icon = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // slot_number: int32
            let slot_number = crate::util::read_i32_le(chunk)?;


            rows.push(PaperDollItemFrameRow {
                item_button_name,
                slot_icon,
                slot_number,
            });
        }

        Ok(PaperDollItemFrame { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 3,
            record_size: 12,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // item_button_name: string_ref
            if !row.item_button_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.item_button_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // slot_icon: string_ref
            if !row.slot_icon.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.slot_icon.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // slot_number: int32
            b.write_all(&row.slot_number.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl PaperDollItemFrame {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.item_button_name.is_empty() { b.write_all(row.item_button_name.as_bytes())?; b.write_all(&[0])?; };
            if !row.slot_icon.is_empty() { b.write_all(row.slot_icon.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.item_button_name.is_empty() { sum += row.item_button_name.len() + 1; };
            if !row.slot_icon.is_empty() { sum += row.slot_icon.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PaperDollItemFrameRow {
    pub item_button_name: String,
    pub slot_icon: String,
    pub slot_number: i32,
}

