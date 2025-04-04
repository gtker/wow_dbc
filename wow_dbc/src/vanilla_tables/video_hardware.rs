use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VideoHardware {
    pub rows: Vec<VideoHardwareRow>,
}

impl DbcTable for VideoHardware {
    type Row = VideoHardwareRow;

    const FILENAME: &'static str = "VideoHardware.dbc";
    const FIELD_COUNT: usize = 22;
    const ROW_SIZE: usize = 88;

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

            // id: primary_key (VideoHardware) uint32
            let id = VideoHardwareKey::new(crate::util::read_u32_le(chunk)?);

            // vendor_id: uint32
            let vendor_id = crate::util::read_u32_le(chunk)?;

            // device_id: uint32
            let device_id = crate::util::read_u32_le(chunk)?;

            // farclip_idx: uint32
            let farclip_idx = crate::util::read_u32_le(chunk)?;

            // terrain_l_o_d_dist_idx: uint32
            let terrain_l_o_d_dist_idx = crate::util::read_u32_le(chunk)?;

            // terrain_shadow_l_o_d: uint32
            let terrain_shadow_l_o_d = crate::util::read_u32_le(chunk)?;

            // detail_doodad_density_idx: uint32
            let detail_doodad_density_idx = crate::util::read_u32_le(chunk)?;

            // detail_doodad_alpha: uint32
            let detail_doodad_alpha = crate::util::read_u32_le(chunk)?;

            // animating_doodad_idx: uint32
            let animating_doodad_idx = crate::util::read_u32_le(chunk)?;

            // trilinear: uint32
            let trilinear = crate::util::read_u32_le(chunk)?;

            // num_lights: uint32
            let num_lights = crate::util::read_u32_le(chunk)?;

            // specularity: uint32
            let specularity = crate::util::read_u32_le(chunk)?;

            // water_l_o_d_idx: uint32
            let water_l_o_d_idx = crate::util::read_u32_le(chunk)?;

            // particle_density_idx: uint32
            let particle_density_idx = crate::util::read_u32_le(chunk)?;

            // unit_draw_dist_idx: uint32
            let unit_draw_dist_idx = crate::util::read_u32_le(chunk)?;

            // small_cull_dist_idx: uint32
            let small_cull_dist_idx = crate::util::read_u32_le(chunk)?;

            // resolution_idx: uint32
            let resolution_idx = crate::util::read_u32_le(chunk)?;

            // base_mip_level: uint32
            let base_mip_level = crate::util::read_u32_le(chunk)?;

            // ogl_overrides: string_ref
            let ogl_overrides = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // d3d_overrides: string_ref
            let d3d_overrides = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // fix_lag: uint32
            let fix_lag = crate::util::read_u32_le(chunk)?;

            // multisample: uint32
            let multisample = crate::util::read_u32_le(chunk)?;


            rows.push(VideoHardwareRow {
                id,
                vendor_id,
                device_id,
                farclip_idx,
                terrain_l_o_d_dist_idx,
                terrain_shadow_l_o_d,
                detail_doodad_density_idx,
                detail_doodad_alpha,
                animating_doodad_idx,
                trilinear,
                num_lights,
                specularity,
                water_l_o_d_idx,
                particle_density_idx,
                unit_draw_dist_idx,
                small_cull_dist_idx,
                resolution_idx,
                base_mip_level,
                ogl_overrides,
                d3d_overrides,
                fix_lag,
                multisample,
            });
        }

        Ok(VideoHardware { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (VideoHardware) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // vendor_id: uint32
            b.write_all(&row.vendor_id.to_le_bytes())?;

            // device_id: uint32
            b.write_all(&row.device_id.to_le_bytes())?;

            // farclip_idx: uint32
            b.write_all(&row.farclip_idx.to_le_bytes())?;

            // terrain_l_o_d_dist_idx: uint32
            b.write_all(&row.terrain_l_o_d_dist_idx.to_le_bytes())?;

            // terrain_shadow_l_o_d: uint32
            b.write_all(&row.terrain_shadow_l_o_d.to_le_bytes())?;

            // detail_doodad_density_idx: uint32
            b.write_all(&row.detail_doodad_density_idx.to_le_bytes())?;

            // detail_doodad_alpha: uint32
            b.write_all(&row.detail_doodad_alpha.to_le_bytes())?;

            // animating_doodad_idx: uint32
            b.write_all(&row.animating_doodad_idx.to_le_bytes())?;

            // trilinear: uint32
            b.write_all(&row.trilinear.to_le_bytes())?;

            // num_lights: uint32
            b.write_all(&row.num_lights.to_le_bytes())?;

            // specularity: uint32
            b.write_all(&row.specularity.to_le_bytes())?;

            // water_l_o_d_idx: uint32
            b.write_all(&row.water_l_o_d_idx.to_le_bytes())?;

            // particle_density_idx: uint32
            b.write_all(&row.particle_density_idx.to_le_bytes())?;

            // unit_draw_dist_idx: uint32
            b.write_all(&row.unit_draw_dist_idx.to_le_bytes())?;

            // small_cull_dist_idx: uint32
            b.write_all(&row.small_cull_dist_idx.to_le_bytes())?;

            // resolution_idx: uint32
            b.write_all(&row.resolution_idx.to_le_bytes())?;

            // base_mip_level: uint32
            b.write_all(&row.base_mip_level.to_le_bytes())?;

            // ogl_overrides: string_ref
            if !row.ogl_overrides.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.ogl_overrides.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // d3d_overrides: string_ref
            if !row.d3d_overrides.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.d3d_overrides.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // fix_lag: uint32
            b.write_all(&row.fix_lag.to_le_bytes())?;

            // multisample: uint32
            b.write_all(&row.multisample.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for VideoHardware {
    type PrimaryKey = VideoHardwareKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl VideoHardware {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.ogl_overrides.is_empty() { b.write_all(row.ogl_overrides.as_bytes())?; b.write_all(&[0])?; };
            if !row.d3d_overrides.is_empty() { b.write_all(row.d3d_overrides.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.ogl_overrides.is_empty() { sum += row.ogl_overrides.len() + 1; };
            if !row.d3d_overrides.is_empty() { sum += row.d3d_overrides.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VideoHardwareKey {
    pub id: u32
}

impl VideoHardwareKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for VideoHardwareKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for VideoHardwareKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for VideoHardwareKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for VideoHardwareKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for VideoHardwareKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for VideoHardwareKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for VideoHardwareKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for VideoHardwareKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for VideoHardwareKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for VideoHardwareKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VideoHardwareRow {
    pub id: VideoHardwareKey,
    pub vendor_id: u32,
    pub device_id: u32,
    pub farclip_idx: u32,
    pub terrain_l_o_d_dist_idx: u32,
    pub terrain_shadow_l_o_d: u32,
    pub detail_doodad_density_idx: u32,
    pub detail_doodad_alpha: u32,
    pub animating_doodad_idx: u32,
    pub trilinear: u32,
    pub num_lights: u32,
    pub specularity: u32,
    pub water_l_o_d_idx: u32,
    pub particle_density_idx: u32,
    pub unit_draw_dist_idx: u32,
    pub small_cull_dist_idx: u32,
    pub resolution_idx: u32,
    pub base_mip_level: u32,
    pub ogl_overrides: String,
    pub d3d_overrides: String,
    pub fix_lag: u32,
    pub multisample: u32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn video_hardware() {
        let mut file = File::open("../vanilla-dbc/VideoHardware.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = VideoHardware::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = VideoHardware::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
