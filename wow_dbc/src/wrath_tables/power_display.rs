use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PowerDisplay {
    pub rows: Vec<PowerDisplayRow>,
}

impl DbcTable for PowerDisplay {
    type Row = PowerDisplayRow;

    fn filename() -> &'static str { "PowerDisplay.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 15 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 15,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 6 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 6,
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

            // id: primary_key (PowerDisplay) int32
            let id = PowerDisplayKey::new(crate::util::read_i32_le(chunk)?);

            // actual_type: int32
            let actual_type = crate::util::read_i32_le(chunk)?;

            // global_string_base_tag: string_ref
            let global_string_base_tag = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // red: int8
            let red = crate::util::read_i8_le(chunk)?;

            // green: int8
            let green = crate::util::read_i8_le(chunk)?;

            // blue: int8
            let blue = crate::util::read_i8_le(chunk)?;


            rows.push(PowerDisplayRow {
                id,
                actual_type,
                global_string_base_tag,
                red,
                green,
                blue,
            });
        }

        Ok(PowerDisplay { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 6,
            record_size: 15,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (PowerDisplay) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // actual_type: int32
            b.write_all(&row.actual_type.to_le_bytes())?;

            // global_string_base_tag: string_ref
            if !row.global_string_base_tag.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.global_string_base_tag.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // red: int8
            b.write_all(&row.red.to_le_bytes())?;

            // green: int8
            b.write_all(&row.green.to_le_bytes())?;

            // blue: int8
            b.write_all(&row.blue.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for PowerDisplay {
    type PrimaryKey = PowerDisplayKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl PowerDisplay {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.global_string_base_tag.is_empty() { b.write_all(row.global_string_base_tag.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.global_string_base_tag.is_empty() { sum += row.global_string_base_tag.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstPowerDisplay<const S: usize> {
    pub rows: [ConstPowerDisplayRow; S],
}

impl<const S: usize> ConstPowerDisplay<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 15 {
            panic!("invalid record size, expected 15")
        }

        if header.field_count != 6 {
            panic!("invalid field count, expected 6")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstPowerDisplayRow {
                id: PowerDisplayKey::new(0),
                actual_type: 0,
                global_string_base_tag: "",
                red: 0,
                green: 0,
                blue: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (PowerDisplay) int32
            let id = PowerDisplayKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // actual_type: int32
            let actual_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // global_string_base_tag: string_ref
            let global_string_base_tag = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // red: int8
            let red = i8::from_le_bytes([b[b_offset + 0]]);
            b_offset += 1;

            // green: int8
            let green = i8::from_le_bytes([b[b_offset + 0]]);
            b_offset += 1;

            // blue: int8
            let blue = i8::from_le_bytes([b[b_offset + 0]]);
            b_offset += 1;

            rows[i] = ConstPowerDisplayRow {
                id,
                actual_type,
                global_string_base_tag,
                red,
                green,
                blue,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> PowerDisplay {
        PowerDisplay {
            rows: self.rows.iter().map(|s| PowerDisplayRow {
                id: s.id,
                actual_type: s.actual_type,
                global_string_base_tag: s.global_string_base_tag.to_string(),
                red: s.red,
                green: s.green,
                blue: s.blue,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct PowerDisplayKey {
    pub id: i32
}

impl PowerDisplayKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for PowerDisplayKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for PowerDisplayKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for PowerDisplayKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for PowerDisplayKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for PowerDisplayKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PowerDisplayRow {
    pub id: PowerDisplayKey,
    pub actual_type: i32,
    pub global_string_base_tag: String,
    pub red: i8,
    pub green: i8,
    pub blue: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstPowerDisplayRow {
    pub id: PowerDisplayKey,
    pub actual_type: i32,
    pub global_string_base_tag: &'static str,
    pub red: i8,
    pub green: i8,
    pub blue: i8,
}

