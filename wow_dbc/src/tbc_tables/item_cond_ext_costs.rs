use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::item_extended_cost::ItemExtendedCostKey;
use crate::util::StringCache;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemCondExtCosts {
    pub rows: Vec<ItemCondExtCostsRow>,
}

impl DbcTable for ItemCondExtCosts {
    type Row = ItemCondExtCostsRow;

    const FILENAME: &'static str = "ItemCondExtCosts.dbc";
    const FIELD_COUNT: usize = 4;
    const ROW_SIZE: usize = 16;

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

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

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

        assert_eq!(b.len(), self.rows.len() * Self::ROW_SIZE);
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: string_cache.size(),
        };

        w.write_all(&header.write_header())?;
        w.write_all(&b)?;
        w.write_all(string_cache.buffer())?;
        Ok(())
    }

}

impl Indexable for ItemCondExtCosts {
    type PrimaryKey = ItemCondExtCostsKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemCondExtCostsKey {
    pub id: i32
}

impl ItemCondExtCostsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for ItemCondExtCostsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for ItemCondExtCostsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for ItemCondExtCostsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for ItemCondExtCostsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for ItemCondExtCostsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for ItemCondExtCostsKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ItemCondExtCostsKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for ItemCondExtCostsKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ItemCondExtCostsKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ItemCondExtCostsKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemCondExtCostsRow {
    pub id: ItemCondExtCostsKey,
    pub cond_extended_cost: i32,
    pub item_extended_cost_entry: ItemExtendedCostKey,
    pub arena_season: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn item_cond_ext_costs() {
        let mut file = File::open("../tbc-dbc/ItemCondExtCosts.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ItemCondExtCosts::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ItemCondExtCosts::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
