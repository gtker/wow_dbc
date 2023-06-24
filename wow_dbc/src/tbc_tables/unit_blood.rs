use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitBlood {
    pub rows: Vec<UnitBloodRow>,
}

impl DbcTable for UnitBlood {
    type Row = UnitBloodRow;

    const FILENAME: &'static str = "UnitBlood.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 40 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 40,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 10 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 10,
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

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 10,
            record_size: 40,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
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
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for UnitBlood {
    type PrimaryKey = UnitBloodKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl UnitBlood {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            for s in &row.ground_blood {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            for s in &row.ground_blood {
                if !s.is_empty() { sum += s.len() + 1; };
            }

        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
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
pub struct UnitBloodRow {
    pub id: UnitBloodKey,
    pub combat_blood_spurt_front: [i32; 2],
    pub combat_blood_spurt_back: [i32; 2],
    pub ground_blood: [String; 5],
}

