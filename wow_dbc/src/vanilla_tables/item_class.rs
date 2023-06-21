use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
                    expected: 12,
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
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstItemClass<const S: usize> {
    pub rows: [ConstItemClassRow; S],
}

impl<const S: usize> ConstItemClass<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 48 {
            panic!("invalid record size, expected 48")
        }

        if header.field_count != 12 {
            panic!("invalid field count, expected 12")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstItemClassRow {
                id: ItemClassKey::new(0),
                subclass_map: 0,
                item_class: Class::Item,
                class_name: crate::ConstLocalizedString::empty(),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (ItemClass) uint32
            let id = ItemClassKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // subclass_map: uint32
            let subclass_map = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // item_class: Class
            let item_class = match Class::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            // class_name: string_ref_loc
            let class_name = ConstLocalizedString::new(
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

            rows[i] = ConstItemClassRow {
                id,
                subclass_map,
                item_class,
                class_name,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> ItemClass {
        ItemClass {
            rows: self.rows.iter().map(|s| ItemClassRow {
                id: s.id,
                subclass_map: s.subclass_map,
                item_class: s.item_class,
                class_name: s.class_name.to_string(),
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ItemClassKey {
    pub id: u32
}

impl ItemClassKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for ItemClassKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ItemClassKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for ItemClassKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Class {
    Item,
    Weapon,
}

impl Class {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            0 => Self::Item,
            1 => Self::Weapon,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for Class {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("Class", value as i64))
    }

}

impl Class {
    pub const fn as_int(&self) -> i32 {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemClassRow {
    pub id: ItemClassKey,
    pub subclass_map: u32,
    pub item_class: Class,
    pub class_name: LocalizedString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstItemClassRow {
    pub id: ItemClassKey,
    pub subclass_map: u32,
    pub item_class: Class,
    pub class_name: ConstLocalizedString,
}

