use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZoneMusic {
    pub rows: Vec<ZoneMusicRow>,
}

impl DbcTable for ZoneMusic {
    type Row = ZoneMusicRow;

    const FILENAME: &'static str = "ZoneMusic.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (ZoneMusic) int32
            let id = ZoneMusicKey::new(crate::util::read_i32_le(chunk)?);

            // set_name: string_ref
            let set_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // silence_interval_min: int32[2]
            let silence_interval_min = crate::util::read_array_i32::<2>(chunk)?;

            // silence_interval_max: int32[2]
            let silence_interval_max = crate::util::read_array_i32::<2>(chunk)?;

            // sounds: int32[2]
            let sounds = crate::util::read_array_i32::<2>(chunk)?;


            rows.push(ZoneMusicRow {
                id,
                set_name,
                silence_interval_min,
                silence_interval_max,
                sounds,
            });
        }

        Ok(ZoneMusic { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 8,
            record_size: 32,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ZoneMusic) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // set_name: string_ref
            if !row.set_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.set_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // silence_interval_min: int32[2]
            for i in row.silence_interval_min {
                b.write_all(&i.to_le_bytes())?;
            }


            // silence_interval_max: int32[2]
            for i in row.silence_interval_max {
                b.write_all(&i.to_le_bytes())?;
            }


            // sounds: int32[2]
            for i in row.sounds {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ZoneMusic {
    type PrimaryKey = ZoneMusicKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl ZoneMusic {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.set_name.is_empty() { b.write_all(row.set_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.set_name.is_empty() { sum += row.set_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ZoneMusicKey {
    pub id: i32
}

impl ZoneMusicKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for ZoneMusicKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for ZoneMusicKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for ZoneMusicKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for ZoneMusicKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for ZoneMusicKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for ZoneMusicKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ZoneMusicKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for ZoneMusicKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ZoneMusicKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ZoneMusicKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZoneMusicRow {
    pub id: ZoneMusicKey,
    pub set_name: String,
    pub silence_interval_min: [i32; 2],
    pub silence_interval_max: [i32; 2],
    pub sounds: [i32; 2],
}

