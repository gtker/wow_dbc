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
pub struct DurabilityCosts {
    pub rows: Vec<DurabilityCostsRow>,
}

impl DbcTable for DurabilityCosts {
    type Row = DurabilityCostsRow;

    const FILENAME: &'static str = "DurabilityCosts.dbc";
    const FIELD_COUNT: usize = 30;
    const ROW_SIZE: usize = 120;

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

            // id: primary_key (DurabilityCosts) uint32
            let id = DurabilityCostsKey::new(crate::util::read_u32_le(chunk)?);

            // weapon_subclass_cost: int32[21]
            let weapon_subclass_cost = crate::util::read_array_i32::<21>(chunk)?;

            // armour_subclass_cost: int32[8]
            let armour_subclass_cost = crate::util::read_array_i32::<8>(chunk)?;


            rows.push(DurabilityCostsRow {
                id,
                weapon_subclass_cost,
                armour_subclass_cost,
            });
        }

        Ok(DurabilityCosts { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (DurabilityCosts) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // weapon_subclass_cost: int32[21]
            for i in row.weapon_subclass_cost {
                b.write_all(&i.to_le_bytes())?;
            }


            // armour_subclass_cost: int32[8]
            for i in row.armour_subclass_cost {
                b.write_all(&i.to_le_bytes())?;
            }


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

impl Indexable for DurabilityCosts {
    type PrimaryKey = DurabilityCostsKey;
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
pub struct DurabilityCostsKey {
    pub id: u32
}

impl DurabilityCostsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for DurabilityCostsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for DurabilityCostsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for DurabilityCostsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for DurabilityCostsKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for DurabilityCostsKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for DurabilityCostsKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for DurabilityCostsKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for DurabilityCostsKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for DurabilityCostsKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for DurabilityCostsKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DurabilityCostsRow {
    pub id: DurabilityCostsKey,
    pub weapon_subclass_cost: [i32; 21],
    pub armour_subclass_cost: [i32; 8],
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn durability_costs() {
        let mut file = File::open("../vanilla-dbc/DurabilityCosts.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = DurabilityCosts::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = DurabilityCosts::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
