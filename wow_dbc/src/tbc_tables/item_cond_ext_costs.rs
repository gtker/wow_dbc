use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::item_extended_cost::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemCondExtCosts {
    pub rows: Vec<ItemCondExtCostsRow>,
}

impl DbcTable for ItemCondExtCosts {
    type Row = ItemCondExtCostsRow;

    fn filename() -> &'static str { "ItemCondExtCosts.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 16,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 4 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 4,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (ItemCondExtCosts) int32
            let id = ItemCondExtCostsKey::new(crate::util::read_i32_le(chunk)?);

            // cond_extended_cost: int32
            let cond_extended_cost = crate::util::read_i32_le(chunk)?;

            // item_extended_cost_entry: foreign_key (ItemExtendedCost) int32
            let item_extended_cost_entry = ItemExtendedCostKey::new(crate::util::read_i32_le(chunk)?.into());

            // arena_season: int32
            let arena_season = crate::util::read_i32_le(chunk)?;


            rows.push(ItemCondExtCostsRow {
                id,
                cond_extended_cost,
                item_extended_cost_entry,
                arena_season,
            });
        }

        Ok(ItemCondExtCosts { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 4,
            record_size: 16,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (ItemCondExtCosts) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // cond_extended_cost: int32
            b.write_all(&row.cond_extended_cost.to_le_bytes())?;

            // item_extended_cost_entry: foreign_key (ItemExtendedCost) int32
            b.write_all(&(row.item_extended_cost_entry.id as i32).to_le_bytes())?;

            // arena_season: int32
            b.write_all(&row.arena_season.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for ItemCondExtCosts {
    type PrimaryKey = ItemCondExtCostsKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemCondExtCostsKey {
    pub id: i32
}

impl ItemCondExtCostsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemCondExtCostsRow {
    pub id: ItemCondExtCostsKey,
    pub cond_extended_cost: i32,
    pub item_extended_cost_entry: ItemExtendedCostKey,
    pub arena_season: i32,
}
