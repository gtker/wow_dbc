use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::item_purchase_group::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemExtendedCost {
    pub rows: Vec<ItemExtendedCostRow>,
}

impl DbcTable for ItemExtendedCost {
    type Row = ItemExtendedCostRow;

    fn filename() -> &'static str { "ItemExtendedCost.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 64 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 64,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 16,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (ItemExtendedCost) int32
            let id = ItemExtendedCostKey::new(crate::util::read_i32_le(chunk)?);

            // honor_points: int32
            let honor_points = crate::util::read_i32_le(chunk)?;

            // arena_points: int32
            let arena_points = crate::util::read_i32_le(chunk)?;

            // arena_bracket: int32
            let arena_bracket = crate::util::read_i32_le(chunk)?;

            // item_id: int32[5]
            let item_id = crate::util::read_array_i32::<5>(chunk)?;

            // item_count: int32[5]
            let item_count = crate::util::read_array_i32::<5>(chunk)?;

            // required_arena_rating: int32
            let required_arena_rating = crate::util::read_i32_le(chunk)?;

            // item_purchase_group: foreign_key (ItemPurchaseGroup) int32
            let item_purchase_group = ItemPurchaseGroupKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(ItemExtendedCostRow {
                id,
                honor_points,
                arena_points,
                arena_bracket,
                item_id,
                item_count,
                required_arena_rating,
                item_purchase_group,
            });
        }

        Ok(ItemExtendedCost { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 16,
            record_size: 64,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (ItemExtendedCost) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // honor_points: int32
            b.write_all(&row.honor_points.to_le_bytes())?;

            // arena_points: int32
            b.write_all(&row.arena_points.to_le_bytes())?;

            // arena_bracket: int32
            b.write_all(&row.arena_bracket.to_le_bytes())?;

            // item_id: int32[5]
            for i in row.item_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // item_count: int32[5]
            for i in row.item_count {
                b.write_all(&i.to_le_bytes())?;
            }


            // required_arena_rating: int32
            b.write_all(&row.required_arena_rating.to_le_bytes())?;

            // item_purchase_group: foreign_key (ItemPurchaseGroup) int32
            b.write_all(&(row.item_purchase_group.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for ItemExtendedCost {
    type PrimaryKey = ItemExtendedCostKey;
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
pub struct ConstItemExtendedCost<const S: usize> {
    pub rows: [ItemExtendedCostRow; S],
}

impl<const S: usize> ConstItemExtendedCost<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 64 {
            panic!("invalid record size, expected 64")
        }

        if header.field_count != 16 {
            panic!("invalid field count, expected 16")
        }

        let mut b_offset = 20;
        let mut rows = [
            ItemExtendedCostRow {
                id: ItemExtendedCostKey::new(0),
                honor_points: 0,
                arena_points: 0,
                arena_bracket: 0,
                item_id: [0; 5],
                item_count: [0; 5],
                required_arena_rating: 0,
                item_purchase_group: ItemPurchaseGroupKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (ItemExtendedCost) int32
            let id = ItemExtendedCostKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // honor_points: int32
            let honor_points = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // arena_points: int32
            let arena_points = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // arena_bracket: int32
            let arena_bracket = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // item_id: int32[5]
            let item_id = {
                let mut a = [0; 5];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // item_count: int32[5]
            let item_count = {
                let mut a = [0; 5];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // required_arena_rating: int32
            let required_arena_rating = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // item_purchase_group: foreign_key (ItemPurchaseGroup) int32
            let item_purchase_group = ItemPurchaseGroupKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = ItemExtendedCostRow {
                id,
                honor_points,
                arena_points,
                arena_bracket,
                item_id,
                item_count,
                required_arena_rating,
                item_purchase_group,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ItemExtendedCostKey {
    pub id: i32
}

impl ItemExtendedCostKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for ItemExtendedCostKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for ItemExtendedCostKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for ItemExtendedCostKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for ItemExtendedCostKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ItemExtendedCostKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemExtendedCostRow {
    pub id: ItemExtendedCostKey,
    pub honor_points: i32,
    pub arena_points: i32,
    pub arena_bracket: i32,
    pub item_id: [i32; 5],
    pub item_count: [i32; 5],
    pub required_arena_rating: i32,
    pub item_purchase_group: ItemPurchaseGroupKey,
}

