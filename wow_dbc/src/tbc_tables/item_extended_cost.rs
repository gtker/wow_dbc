use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq)]
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

        if header.record_size != 56 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 56,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 14 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 14,
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

            // item_id: int32[5]
            let item_id = crate::util::read_array_i32::<5>(chunk)?;

            // item_count: int32[5]
            let item_count = crate::util::read_array_i32::<5>(chunk)?;

            // required_arena_rating: int32
            let required_arena_rating = crate::util::read_i32_le(chunk)?;


            rows.push(ItemExtendedCostRow {
                id,
                honor_points,
                arena_points,
                item_id,
                item_count,
                required_arena_rating,
            });
        }

        Ok(ItemExtendedCost { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 14,
            record_size: 56,
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

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for ItemExtendedCost {
    type PrimaryKey = ItemExtendedCostKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemExtendedCostKey {
    pub id: i32
}

impl ItemExtendedCostKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemExtendedCostRow {
    pub id: ItemExtendedCostKey,
    pub honor_points: i32,
    pub arena_points: i32,
    pub item_id: [i32; 5],
    pub item_count: [i32; 5],
    pub required_arena_rating: i32,
}
