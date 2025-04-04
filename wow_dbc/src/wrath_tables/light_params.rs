use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::light_skybox::LightSkyboxKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LightParams {
    pub rows: Vec<LightParamsRow>,
}

impl DbcTable for LightParams {
    type Row = LightParamsRow;

    const FILENAME: &'static str = "LightParams.dbc";
    const FIELD_COUNT: usize = 9;
    const ROW_SIZE: usize = 36;

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

            // id: primary_key (LightParams) int32
            let id = LightParamsKey::new(crate::util::read_i32_le(chunk)?);

            // highlight_sky: int32
            let highlight_sky = crate::util::read_i32_le(chunk)?;

            // light_skybox_id: foreign_key (LightSkybox) int32
            let light_skybox_id = LightSkyboxKey::new(crate::util::read_i32_le(chunk)?.into());

            // glow: float
            let glow = crate::util::read_f32_le(chunk)?;

            // water_shallow_alpha: float
            let water_shallow_alpha = crate::util::read_f32_le(chunk)?;

            // water_deep_alpha: float
            let water_deep_alpha = crate::util::read_f32_le(chunk)?;

            // ocean_shallow_alpha: float
            let ocean_shallow_alpha = crate::util::read_f32_le(chunk)?;

            // ocean_deep_alpha: float
            let ocean_deep_alpha = crate::util::read_f32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(LightParamsRow {
                id,
                highlight_sky,
                light_skybox_id,
                glow,
                water_shallow_alpha,
                water_deep_alpha,
                ocean_shallow_alpha,
                ocean_deep_alpha,
                flags,
            });
        }

        Ok(LightParams { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (LightParams) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // highlight_sky: int32
            b.write_all(&row.highlight_sky.to_le_bytes())?;

            // light_skybox_id: foreign_key (LightSkybox) int32
            b.write_all(&(row.light_skybox_id.id as i32).to_le_bytes())?;

            // glow: float
            b.write_all(&row.glow.to_le_bytes())?;

            // water_shallow_alpha: float
            b.write_all(&row.water_shallow_alpha.to_le_bytes())?;

            // water_deep_alpha: float
            b.write_all(&row.water_deep_alpha.to_le_bytes())?;

            // ocean_shallow_alpha: float
            b.write_all(&row.ocean_shallow_alpha.to_le_bytes())?;

            // ocean_deep_alpha: float
            b.write_all(&row.ocean_deep_alpha.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for LightParams {
    type PrimaryKey = LightParamsKey;
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
pub struct LightParamsKey {
    pub id: i32
}

impl LightParamsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for LightParamsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for LightParamsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for LightParamsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for LightParamsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for LightParamsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for LightParamsKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for LightParamsKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for LightParamsKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for LightParamsKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for LightParamsKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LightParamsRow {
    pub id: LightParamsKey,
    pub highlight_sky: i32,
    pub light_skybox_id: LightSkyboxKey,
    pub glow: f32,
    pub water_shallow_alpha: f32,
    pub water_deep_alpha: f32,
    pub ocean_shallow_alpha: f32,
    pub ocean_deep_alpha: f32,
    pub flags: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn light_params() {
        let mut file = File::open("../wrath-dbc/LightParams.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = LightParams::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = LightParams::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
