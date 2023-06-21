use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::material::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item {
    pub rows: Vec<ItemRow>,
}

impl DbcTable for Item {
    type Row = ItemRow;

    fn filename() -> &'static str { "Item.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 8,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (Item) int32
            let id = ItemKey::new(crate::util::read_i32_le(chunk)?);

            // class_id: int32
            let class_id = crate::util::read_i32_le(chunk)?;

            // subclass_id: int32
            let subclass_id = crate::util::read_i32_le(chunk)?;

            // sound_override_subclass_id: int32
            let sound_override_subclass_id = crate::util::read_i32_le(chunk)?;

            // material: foreign_key (Material) int32
            let material = MaterialKey::new(crate::util::read_i32_le(chunk)?.into());

            // display_info_id: int32
            let display_info_id = crate::util::read_i32_le(chunk)?;

            // inventory_type: int32
            let inventory_type = crate::util::read_i32_le(chunk)?;

            // sheathe_type: int32
            let sheathe_type = crate::util::read_i32_le(chunk)?;


            rows.push(ItemRow {
                id,
                class_id,
                subclass_id,
                sound_override_subclass_id,
                material,
                display_info_id,
                inventory_type,
                sheathe_type,
            });
        }

        Ok(Item { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 8,
            record_size: 32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (Item) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // class_id: int32
            b.write_all(&row.class_id.to_le_bytes())?;

            // subclass_id: int32
            b.write_all(&row.subclass_id.to_le_bytes())?;

            // sound_override_subclass_id: int32
            b.write_all(&row.sound_override_subclass_id.to_le_bytes())?;

            // material: foreign_key (Material) int32
            b.write_all(&(row.material.id as i32).to_le_bytes())?;

            // display_info_id: int32
            b.write_all(&row.display_info_id.to_le_bytes())?;

            // inventory_type: int32
            b.write_all(&row.inventory_type.to_le_bytes())?;

            // sheathe_type: int32
            b.write_all(&row.sheathe_type.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for Item {
    type PrimaryKey = ItemKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstItem<const S: usize> {
    pub rows: [ItemRow; S],
}

impl<const S: usize> ConstItem<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 32 {
            panic!("invalid record size, expected 32")
        }

        if header.field_count != 8 {
            panic!("invalid field count, expected 8")
        }

        let mut b_offset = 20;
        let mut rows = [
            ItemRow {
                id: ItemKey::new(0),
                class_id: 0,
                subclass_id: 0,
                sound_override_subclass_id: 0,
                material: MaterialKey::new(0),
                display_info_id: 0,
                inventory_type: 0,
                sheathe_type: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Item) int32
            let id = ItemKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // class_id: int32
            let class_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // subclass_id: int32
            let subclass_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // sound_override_subclass_id: int32
            let sound_override_subclass_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // material: foreign_key (Material) int32
            let material = MaterialKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // display_info_id: int32
            let display_info_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // inventory_type: int32
            let inventory_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // sheathe_type: int32
            let sheathe_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ItemRow {
                id,
                class_id,
                subclass_id,
                sound_override_subclass_id,
                material,
                display_info_id,
                inventory_type,
                sheathe_type,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ItemKey {
    pub id: i32
}

impl ItemKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for ItemKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for ItemKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for ItemKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for ItemKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ItemKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemRow {
    pub id: ItemKey,
    pub class_id: i32,
    pub subclass_id: i32,
    pub sound_override_subclass_id: i32,
    pub material: MaterialKey,
    pub display_info_id: i32,
    pub inventory_type: i32,
    pub sheathe_type: i32,
}

