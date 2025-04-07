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
pub struct UnitBlood {
    pub rows: Vec<UnitBloodRow>,
}

impl DbcTable for UnitBlood {
    type Row = UnitBloodRow;

    const FILENAME: &'static str = "UnitBlood.dbc";
    const FIELD_COUNT: usize = 10;
    const ROW_SIZE: usize = 40;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (UnitBlood) int32
            let id = UnitBloodKey::new(crate::util::read_i32_le(chunk)?);

            // combat_blood_spurt_front: int32[2]
            let combat_blood_spurt_front = crate::util::read_array_i32::<2>(chunk)?;

            // combat_blood_spurt_back: int32[2]
            let combat_blood_spurt_back = crate::util::read_array_i32::<2>(chunk)?;

            // ground_blood: string_ref[5]
            let ground_blood = {
                let mut arr = Vec::with_capacity(5);
                for _ in 0..5 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };


            rows.push(UnitBloodRow {
                id,
                combat_blood_spurt_front,
                combat_blood_spurt_back,
                ground_blood,
            });
        }

        Ok(UnitBlood { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (UnitBlood) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // combat_blood_spurt_front: int32[2]
            for i in row.combat_blood_spurt_front {
                b.write_all(&i.to_le_bytes())?;
            }


            // combat_blood_spurt_back: int32[2]
            for i in row.combat_blood_spurt_back {
                b.write_all(&i.to_le_bytes())?;
            }


            // ground_blood: string_ref[5]
            for i in &row.ground_blood {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
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

impl Indexable for UnitBlood {
    type PrimaryKey = UnitBloodKey;
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
pub struct UnitBloodKey {
    pub id: i32
}

impl UnitBloodKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for UnitBloodKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for UnitBloodKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for UnitBloodKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for UnitBloodKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for UnitBloodKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for UnitBloodKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for UnitBloodKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for UnitBloodKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for UnitBloodKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for UnitBloodKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnitBloodRow {
    pub id: UnitBloodKey,
    pub combat_blood_spurt_front: [i32; 2],
    pub combat_blood_spurt_back: [i32; 2],
    pub ground_blood: [String; 5],
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn unit_blood() {
        let mut file = File::open("../tbc-dbc/UnitBlood.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = UnitBlood::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = UnitBlood::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
