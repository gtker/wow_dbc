use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::{ConstLocalizedString, LocalizedString};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cfg_Categories {
    pub rows: Vec<Cfg_CategoriesRow>,
}

impl DbcTable for Cfg_Categories {
    type Row = Cfg_CategoriesRow;

    fn filename() -> &'static str { "Cfg_Categories.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 44 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 44,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 11 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 11,
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

            // category: Category
            let category = Category::try_from(crate::util::read_i32_le(chunk)?)?;

            // region: Region
            let region = Region::try_from(crate::util::read_i32_le(chunk)?)?;

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(Cfg_CategoriesRow {
                category,
                region,
                name,
            });
        }

        Ok(Cfg_Categories { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 11,
            record_size: 44,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // category: Category
            b.write_all(&(row.category.as_int() as i32).to_le_bytes())?;

            // region: Region
            b.write_all(&(row.region.as_int() as i32).to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Cfg_Categories {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
        }

        sum as u32
    }

}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCfg_Categories<const S: usize> {
    pub rows: [ConstCfg_CategoriesRow; S],
}

impl<const S: usize> ConstCfg_Categories<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 44 {
            panic!("invalid record size, expected 44")
        }

        if header.field_count != 11 {
            panic!("invalid field count, expected 11")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstCfg_CategoriesRow {
                category: Category::One,
                region: Region::UnitedStates,
                name: crate::ConstLocalizedString::empty(),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // category: Category
            let category = match Category::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            // region: Region
            let region = match Region::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            // name: string_ref_loc
            let name = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            rows[i] = ConstCfg_CategoriesRow {
                category,
                region,
                name,
            };
            i += 1;
        }

        Self { rows }
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Category {
    One,
    Two,
    Three,
    Five,
}

impl Category {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            5 => Self::Five,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for Category {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("Category", value as i64))
    }

}

impl Category {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Five => 5,
        }

    }

}

impl Default for Category {
    fn default() -> Self {
        Self::One
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Region {
    UnitedStates,
    Korea,
    Europe,
    Taiwan,
    China,
    TestServer,
    QaServer,
}

impl Region {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            1 => Self::UnitedStates,
            2 => Self::Korea,
            3 => Self::Europe,
            4 => Self::Taiwan,
            5 => Self::China,
            99 => Self::TestServer,
            101 => Self::QaServer,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for Region {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("Region", value as i64))
    }

}

impl Region {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::UnitedStates => 1,
            Self::Korea => 2,
            Self::Europe => 3,
            Self::Taiwan => 4,
            Self::China => 5,
            Self::TestServer => 99,
            Self::QaServer => 101,
        }

    }

}

impl Default for Region {
    fn default() -> Self {
        Self::UnitedStates
    }

}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cfg_CategoriesRow {
    pub category: Category,
    pub region: Region,
    pub name: LocalizedString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCfg_CategoriesRow {
    pub category: Category,
    pub region: Region,
    pub name: ConstLocalizedString,
}

