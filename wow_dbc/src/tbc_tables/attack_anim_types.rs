use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttackAnimTypes {
    pub rows: Vec<AttackAnimTypesRow>,
}

impl DbcTable for AttackAnimTypes {
    type Row = AttackAnimTypesRow;

    fn filename() -> &'static str { "AttackAnimTypes.dbc" }

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // anim_id: int32
            let anim_id = crate::util::read_i32_le(chunk)?;

            // anim_name: string_ref
            let anim_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(AttackAnimTypesRow {
                anim_id,
                anim_name,
            });
        }

        Ok(AttackAnimTypes { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 2,
            record_size: 8,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // anim_id: int32
            b.write_all(&row.anim_id.to_le_bytes())?;

            // anim_name: string_ref
            if !row.anim_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.anim_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl AttackAnimTypes {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.anim_name.is_empty() { b.write_all(row.anim_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.anim_name.is_empty() { sum += row.anim_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstAttackAnimTypes<const S: usize> {
    pub rows: [ConstAttackAnimTypesRow; S],
}

impl<const S: usize> ConstAttackAnimTypes<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 8 {
            panic!("invalid record size, expected 8")
        }

        if header.field_count != 2 {
            panic!("invalid field count, expected 2")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstAttackAnimTypesRow {
                anim_id: 0,
                anim_name: "",
            }
        ; S];

        let mut i = 0;
        while i < S {
            // anim_id: int32
            let anim_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // anim_name: string_ref
            let anim_name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            rows[i] = ConstAttackAnimTypesRow {
                anim_id,
                anim_name,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> AttackAnimTypes {
        AttackAnimTypes {
            rows: self.rows.iter().map(|s| AttackAnimTypesRow {
                anim_id: s.anim_id,
                anim_name: s.anim_name.to_string(),
            }).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttackAnimTypesRow {
    pub anim_id: i32,
    pub anim_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstAttackAnimTypesRow {
    pub anim_id: i32,
    pub anim_name: &'static str,
}

