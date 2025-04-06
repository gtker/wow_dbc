use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerDisplay {
    pub rows: Vec<PowerDisplayRow>,
}

impl DbcTable for PowerDisplay {
    type Row = PowerDisplayRow;

    const FILENAME: &'static str = "PowerDisplay.dbc";
    const FIELD_COUNT: usize = 6;
    const ROW_SIZE: usize = 15;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
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
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
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
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerDisplayKey {
    pub id: i32
}

impl PowerDisplayKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

impl TryFrom<u32> for PowerDisplayKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for PowerDisplayKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for PowerDisplayKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for PowerDisplayKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for PowerDisplayKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerDisplayRow {
    pub id: PowerDisplayKey,
    pub actual_type: i32,
    pub global_string_base_tag: String,
    pub red: i8,
    pub green: i8,
    pub blue: i8,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn power_display() {
        let mut file = File::open("../wrath-dbc/PowerDisplay.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = PowerDisplay::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = PowerDisplay::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
