use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;

#[derive(Debug, Clone, PartialEq)]
pub struct ItemClass {
    pub rows: Vec<ItemClassRow>,
}

impl DbcTable for ItemClass {
    type Row = ItemClassRow;

    fn filename() -> &'static str { "ItemClass.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 48 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 48,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 12 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 48,
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

            // id: primary_key (ItemClass) uint32
            let id = ItemClassKey::new(crate::util::read_u32_le(chunk)?);

            // subclass_map: uint32
            let subclass_map = crate::util::read_u32_le(chunk)?;

            // item_class: Class
            let item_class = Class::try_from(crate::util::read_i32_le(chunk)?)?;

            // class_name: string_ref_loc
            let class_name = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(ItemClassRow {
                id,
                subclass_map,
                item_class,
                class_name,
            });
        }

        Ok(ItemClass { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 12,
            record_size: 48,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ItemClass) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // subclass_map: uint32
            b.write_all(&row.subclass_map.to_le_bytes())?;

            // item_class: Class
            b.write_all(&(row.item_class.as_int() as i32).to_le_bytes())?;

            // class_name: string_ref_loc
            b.write_all(&row.class_name.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ItemClass {
    type PrimaryKey = ItemClassKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ItemClass {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.class_name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.class_name.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemClassKey {
    pub id: u32
}

impl ItemClassKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Class {
    Item,
    Weapon,
}

impl TryFrom<i32> for Class {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Item,
            1 => Self::Weapon,
            val => return Err(crate::InvalidEnumError::new("Class", val as i64)),
        })
    }

}

impl Class {
    const fn as_int(&self) -> i32 {
        match self {
            Self::Item => 0,
            Self::Weapon => 1,
        }

    }

}

impl Default for Class {
    fn default() -> Self {
        Self::Item
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemClassRow {
    pub id: ItemClassKey,
    pub subclass_map: u32,
    pub item_class: Class,
    pub class_name: LocalizedString,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn item_class() {
        let contents = include_bytes!("../../../dbc/ItemClass.dbc");
        let actual = ItemClass::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ItemClass::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
