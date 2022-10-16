use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::light_skybox::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LightParams {
    pub rows: Vec<LightParamsRow>,
}

impl DbcTable for LightParams {
    type Row = LightParamsRow;

    fn filename() -> &'static str { "LightParams.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 36 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 36,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 9 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 9,
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
            field_count: 9,
            record_size: 36,
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
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct LightParamsKey {
    pub id: i32
}

impl LightParamsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

