use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::map::MapKey;
use crate::util::StringCache;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Light {
    pub rows: Vec<LightRow>,
}

impl DbcTable for Light {
    type Row = LightRow;

    const FILENAME: &'static str = "Light.dbc";
    const FIELD_COUNT: usize = 12;
    const ROW_SIZE: usize = 48;

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

            // id: primary_key (Light) int32
            let id = LightKey::new(crate::util::read_i32_le(chunk)?);

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // game_coords: float[3]
            let game_coords = crate::util::read_array_f32::<3>(chunk)?;

            // game_falloff_start: float
            let game_falloff_start = crate::util::read_f32_le(chunk)?;

            // game_falloff_end: float
            let game_falloff_end = crate::util::read_f32_le(chunk)?;

            // light_params_id: int32[5]
            let light_params_id = crate::util::read_array_i32::<5>(chunk)?;


            rows.push(LightRow {
                id,
                continent_id,
                game_coords,
                game_falloff_start,
                game_falloff_end,
                light_params_id,
            });
        }

        Ok(Light { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Light) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // continent_id: foreign_key (Map) int32
            b.write_all(&(row.continent_id.id as i32).to_le_bytes())?;

            // game_coords: float[3]
            for i in row.game_coords {
                b.write_all(&i.to_le_bytes())?;
            }


            // game_falloff_start: float
            b.write_all(&row.game_falloff_start.to_le_bytes())?;

            // game_falloff_end: float
            b.write_all(&row.game_falloff_end.to_le_bytes())?;

            // light_params_id: int32[5]
            for i in row.light_params_id {
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

impl Indexable for Light {
    type PrimaryKey = LightKey;
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
pub struct LightKey {
    pub id: i32
}

impl LightKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for LightKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for LightKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for LightKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for LightKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for LightKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for LightKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for LightKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for LightKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for LightKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for LightKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LightRow {
    pub id: LightKey,
    pub continent_id: MapKey,
    pub game_coords: [f32; 3],
    pub game_falloff_start: f32,
    pub game_falloff_end: f32,
    pub light_params_id: [i32; 5],
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn light() {
        let mut file = File::open("../tbc-dbc/Light.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Light::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Light::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
