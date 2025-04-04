use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellRange {
    pub rows: Vec<SpellRangeRow>,
}

impl DbcTable for SpellRange {
    type Row = SpellRangeRow;

    const FILENAME: &'static str = "SpellRange.dbc";
    const FIELD_COUNT: usize = 40;
    const ROW_SIZE: usize = 160;

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

            // id: primary_key (SpellRange) int32
            let id = SpellRangeKey::new(crate::util::read_i32_le(chunk)?);

            // range_min: float[2]
            let range_min = crate::util::read_array_f32::<2>(chunk)?;

            // range_max: float[2]
            let range_max = crate::util::read_array_f32::<2>(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // display_name_lang: string_ref_loc (Extended)
            let display_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // display_name_short_lang: string_ref_loc (Extended)
            let display_name_short_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(SpellRangeRow {
                id,
                range_min,
                range_max,
                flags,
                display_name_lang,
                display_name_short_lang,
            });
        }

        Ok(SpellRange { rows, })
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
            // id: primary_key (SpellRange) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // range_min: float[2]
            for i in row.range_min {
                b.write_all(&i.to_le_bytes())?;
            }


            // range_max: float[2]
            for i in row.range_max {
                b.write_all(&i.to_le_bytes())?;
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // display_name_lang: string_ref_loc (Extended)
            b.write_all(&row.display_name_lang.string_indices_as_array(&mut string_index))?;

            // display_name_short_lang: string_ref_loc (Extended)
            b.write_all(&row.display_name_short_lang.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SpellRange {
    type PrimaryKey = SpellRangeKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl SpellRange {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.display_name_lang.string_block_as_array(b)?;
            row.display_name_short_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.display_name_lang.string_block_size();
            sum += row.display_name_short_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellRangeKey {
    pub id: i32
}

impl SpellRangeKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for SpellRangeKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for SpellRangeKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for SpellRangeKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for SpellRangeKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for SpellRangeKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for SpellRangeKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SpellRangeKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for SpellRangeKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SpellRangeKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SpellRangeKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellRangeRow {
    pub id: SpellRangeKey,
    pub range_min: [f32; 2],
    pub range_max: [f32; 2],
    pub flags: i32,
    pub display_name_lang: ExtendedLocalizedString,
    pub display_name_short_lang: ExtendedLocalizedString,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell_range() {
        let mut file = File::open("../wrath-dbc/SpellRange.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SpellRange::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellRange::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
