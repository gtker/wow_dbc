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
pub struct VideoHardware {
    pub rows: Vec<VideoHardwareRow>,
}

impl DbcTable for VideoHardware {
    type Row = VideoHardwareRow;

    const FILENAME: &'static str = "VideoHardware.dbc";
    const FIELD_COUNT: usize = 23;
    const ROW_SIZE: usize = 92;

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

            // id: primary_key (VideoHardware) int32
            let id = VideoHardwareKey::new(crate::util::read_i32_le(chunk)?);

            // vendor_id: int32
            let vendor_id = crate::util::read_i32_le(chunk)?;

            // device_id: int32
            let device_id = crate::util::read_i32_le(chunk)?;

            // farclip_idx: int32
            let farclip_idx = crate::util::read_i32_le(chunk)?;

            // terrain_l_o_d_dist_idx: int32
            let terrain_l_o_d_dist_idx = crate::util::read_i32_le(chunk)?;

            // terrain_shadow_l_o_d: int32
            let terrain_shadow_l_o_d = crate::util::read_i32_le(chunk)?;

            // detail_doodad_density_idx: int32
            let detail_doodad_density_idx = crate::util::read_i32_le(chunk)?;

            // detail_doodad_alpha: int32
            let detail_doodad_alpha = crate::util::read_i32_le(chunk)?;

            // animating_doodad_idx: int32
            let animating_doodad_idx = crate::util::read_i32_le(chunk)?;

            // trilinear: int32
            let trilinear = crate::util::read_i32_le(chunk)?;

            // num_lights: int32
            let num_lights = crate::util::read_i32_le(chunk)?;

            // specularity: int32
            let specularity = crate::util::read_i32_le(chunk)?;

            // water_l_o_d_idx: int32
            let water_l_o_d_idx = crate::util::read_i32_le(chunk)?;

            // particle_density_idx: int32
            let particle_density_idx = crate::util::read_i32_le(chunk)?;

            // unit_draw_dist_idx: int32
            let unit_draw_dist_idx = crate::util::read_i32_le(chunk)?;

            // small_cull_dist_idx: int32
            let small_cull_dist_idx = crate::util::read_i32_le(chunk)?;

            // resolution_idx: int32
            let resolution_idx = crate::util::read_i32_le(chunk)?;

            // base_mip_level: int32
            let base_mip_level = crate::util::read_i32_le(chunk)?;

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

            // fix_lag: int32
            let fix_lag = crate::util::read_i32_le(chunk)?;

            // multisample: int32
            let multisample = crate::util::read_i32_le(chunk)?;

            // atlasdisable: int32
            let atlasdisable = crate::util::read_i32_le(chunk)?;


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
                atlasdisable,
            });
        }

        Ok(VideoHardware { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (VideoHardware) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // vendor_id: int32
            b.write_all(&row.vendor_id.to_le_bytes())?;

            // device_id: int32
            b.write_all(&row.device_id.to_le_bytes())?;

            // farclip_idx: int32
            b.write_all(&row.farclip_idx.to_le_bytes())?;

            // terrain_l_o_d_dist_idx: int32
            b.write_all(&row.terrain_l_o_d_dist_idx.to_le_bytes())?;

            // terrain_shadow_l_o_d: int32
            b.write_all(&row.terrain_shadow_l_o_d.to_le_bytes())?;

            // detail_doodad_density_idx: int32
            b.write_all(&row.detail_doodad_density_idx.to_le_bytes())?;

            // detail_doodad_alpha: int32
            b.write_all(&row.detail_doodad_alpha.to_le_bytes())?;

            // animating_doodad_idx: int32
            b.write_all(&row.animating_doodad_idx.to_le_bytes())?;

            // trilinear: int32
            b.write_all(&row.trilinear.to_le_bytes())?;

            // num_lights: int32
            b.write_all(&row.num_lights.to_le_bytes())?;

            // specularity: int32
            b.write_all(&row.specularity.to_le_bytes())?;

            // water_l_o_d_idx: int32
            b.write_all(&row.water_l_o_d_idx.to_le_bytes())?;

            // particle_density_idx: int32
            b.write_all(&row.particle_density_idx.to_le_bytes())?;

            // unit_draw_dist_idx: int32
            b.write_all(&row.unit_draw_dist_idx.to_le_bytes())?;

            // small_cull_dist_idx: int32
            b.write_all(&row.small_cull_dist_idx.to_le_bytes())?;

            // resolution_idx: int32
            b.write_all(&row.resolution_idx.to_le_bytes())?;

            // base_mip_level: int32
            b.write_all(&row.base_mip_level.to_le_bytes())?;

            // ogl_overrides: string_ref
            b.write_all(&string_cache.add_string(&row.ogl_overrides).to_le_bytes())?;

            // d3d_overrides: string_ref
            b.write_all(&string_cache.add_string(&row.d3d_overrides).to_le_bytes())?;

            // fix_lag: int32
            b.write_all(&row.fix_lag.to_le_bytes())?;

            // multisample: int32
            b.write_all(&row.multisample.to_le_bytes())?;

            // atlasdisable: int32
            b.write_all(&row.atlasdisable.to_le_bytes())?;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VideoHardwareKey {
    pub id: i32
}

impl VideoHardwareKey {
    pub const fn new(id: i32) -> Self {
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

impl From<i8> for VideoHardwareKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for VideoHardwareKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for VideoHardwareKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for VideoHardwareKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for VideoHardwareKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for VideoHardwareKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for VideoHardwareKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for VideoHardwareKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VideoHardwareRow {
    pub id: VideoHardwareKey,
    pub vendor_id: i32,
    pub device_id: i32,
    pub farclip_idx: i32,
    pub terrain_l_o_d_dist_idx: i32,
    pub terrain_shadow_l_o_d: i32,
    pub detail_doodad_density_idx: i32,
    pub detail_doodad_alpha: i32,
    pub animating_doodad_idx: i32,
    pub trilinear: i32,
    pub num_lights: i32,
    pub specularity: i32,
    pub water_l_o_d_idx: i32,
    pub particle_density_idx: i32,
    pub unit_draw_dist_idx: i32,
    pub small_cull_dist_idx: i32,
    pub resolution_idx: i32,
    pub base_mip_level: i32,
    pub ogl_overrides: String,
    pub d3d_overrides: String,
    pub fix_lag: i32,
    pub multisample: i32,
    pub atlasdisable: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn video_hardware() {
        let mut file = File::open("../tbc-dbc/VideoHardware.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = VideoHardware::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = VideoHardware::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
