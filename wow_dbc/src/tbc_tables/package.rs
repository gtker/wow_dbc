use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Package {
    pub rows: Vec<PackageRow>,
}

impl DbcTable for Package {
    type Row = PackageRow;

    const FILENAME: &'static str = "Package.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 80 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 80,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 20 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 20,
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

            // id: primary_key (Package) int32
            let id = PackageKey::new(crate::util::read_i32_le(chunk)?);

            // icon: string_ref
            let icon = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // cost: int32
            let cost = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(PackageRow {
                id,
                icon,
                cost,
                name_lang,
            });
        }

        Ok(Package { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 20,
            record_size: 80,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Package) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // icon: string_ref
            if !row.icon.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.icon.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // cost: int32
            b.write_all(&row.cost.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Package {
    type PrimaryKey = PackageKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl Package {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.icon.is_empty() { b.write_all(row.icon.as_bytes())?; b.write_all(&[0])?; };
            row.name_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.icon.is_empty() { sum += row.icon.len() + 1; };
            sum += row.name_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct PackageKey {
    pub id: i32
}

impl PackageKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for PackageKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for PackageKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for PackageKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl From<u8> for PackageKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for PackageKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl TryFrom<u32> for PackageKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageRow {
    pub id: PackageKey,
    pub icon: String,
    pub cost: i32,
    pub name_lang: ExtendedLocalizedString,
}

