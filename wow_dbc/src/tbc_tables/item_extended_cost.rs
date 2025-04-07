use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemExtendedCost {
    pub rows: Vec<ItemExtendedCostRow>,
}

impl DbcTable for ItemExtendedCost {
    type Row = ItemExtendedCostRow;

    const FILENAME: &'static str = "ItemExtendedCost.dbc";
    const FIELD_COUNT: usize = 14;
    const ROW_SIZE: usize = 56;

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

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

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

impl Indexable for ItemExtendedCost {
    type PrimaryKey = ItemExtendedCostKey;
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
pub struct ItemExtendedCostKey {
    pub id: i32
}

impl ItemExtendedCostKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

impl TryFrom<u32> for ItemExtendedCostKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ItemExtendedCostKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for ItemExtendedCostKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ItemExtendedCostKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ItemExtendedCostKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemExtendedCostRow {
    pub id: ItemExtendedCostKey,
    pub honor_points: i32,
    pub arena_points: i32,
    pub item_id: [i32; 5],
    pub item_count: [i32; 5],
    pub required_arena_rating: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn item_extended_cost() {
        let mut file = File::open("../tbc-dbc/ItemExtendedCost.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ItemExtendedCost::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ItemExtendedCost::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
