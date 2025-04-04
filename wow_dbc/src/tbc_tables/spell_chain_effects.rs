use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellChainEffects {
    pub rows: Vec<SpellChainEffectsRow>,
}

impl DbcTable for SpellChainEffects {
    type Row = SpellChainEffectsRow;

    const FILENAME: &'static str = "SpellChainEffects.dbc";
    const FIELD_COUNT: usize = 47;
    const ROW_SIZE: usize = 173;

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

            // id: primary_key (SpellChainEffects) int32
            let id = SpellChainEffectsKey::new(crate::util::read_i32_le(chunk)?);

            // avg_seg_len: float
            let avg_seg_len = crate::util::read_f32_le(chunk)?;

            // width: float
            let width = crate::util::read_f32_le(chunk)?;

            // noise_scale: float
            let noise_scale = crate::util::read_f32_le(chunk)?;

            // tex_coord_scale: float
            let tex_coord_scale = crate::util::read_f32_le(chunk)?;

            // seg_duration: int32
            let seg_duration = crate::util::read_i32_le(chunk)?;

            // seg_delay: int32
            let seg_delay = crate::util::read_i32_le(chunk)?;

            // texture: string_ref
            let texture = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // joint_count: int32
            let joint_count = crate::util::read_i32_le(chunk)?;

            // joint_offset_radius: float
            let joint_offset_radius = crate::util::read_f32_le(chunk)?;

            // joints_per_minor_joint: int32
            let joints_per_minor_joint = crate::util::read_i32_le(chunk)?;

            // minor_joints_per_major_joint: int32
            let minor_joints_per_major_joint = crate::util::read_i32_le(chunk)?;

            // minor_joint_scale: float
            let minor_joint_scale = crate::util::read_f32_le(chunk)?;

            // major_joint_scale: float
            let major_joint_scale = crate::util::read_f32_le(chunk)?;

            // joint_move_speed: float
            let joint_move_speed = crate::util::read_f32_le(chunk)?;

            // joint_smoothness: float
            let joint_smoothness = crate::util::read_f32_le(chunk)?;

            // min_duration_between_joint_jumps: float
            let min_duration_between_joint_jumps = crate::util::read_f32_le(chunk)?;

            // max_duration_between_joint_jumps: float
            let max_duration_between_joint_jumps = crate::util::read_f32_le(chunk)?;

            // wave_height: float
            let wave_height = crate::util::read_f32_le(chunk)?;

            // wave_freq: float
            let wave_freq = crate::util::read_f32_le(chunk)?;

            // wave_speed: float
            let wave_speed = crate::util::read_f32_le(chunk)?;

            // min_wave_angle: float
            let min_wave_angle = crate::util::read_f32_le(chunk)?;

            // max_wave_angle: float
            let max_wave_angle = crate::util::read_f32_le(chunk)?;

            // min_wave_spin: float
            let min_wave_spin = crate::util::read_f32_le(chunk)?;

            // max_wave_spin: float
            let max_wave_spin = crate::util::read_f32_le(chunk)?;

            // arc_height: float
            let arc_height = crate::util::read_f32_le(chunk)?;

            // min_arc_angle: float
            let min_arc_angle = crate::util::read_f32_le(chunk)?;

            // max_arc_angle: float
            let max_arc_angle = crate::util::read_f32_le(chunk)?;

            // min_arc_spin: float
            let min_arc_spin = crate::util::read_f32_le(chunk)?;

            // max_arc_spin: float
            let max_arc_spin = crate::util::read_f32_le(chunk)?;

            // delay_between_effects: float
            let delay_between_effects = crate::util::read_f32_le(chunk)?;

            // min_flicker_on_duration: float
            let min_flicker_on_duration = crate::util::read_f32_le(chunk)?;

            // max_flicker_on_duration: float
            let max_flicker_on_duration = crate::util::read_f32_le(chunk)?;

            // min_flicker_off_duration: float
            let min_flicker_off_duration = crate::util::read_f32_le(chunk)?;

            // max_flicker_off_duration: float
            let max_flicker_off_duration = crate::util::read_f32_le(chunk)?;

            // pulse_speed: float
            let pulse_speed = crate::util::read_f32_le(chunk)?;

            // pulse_on_length: float
            let pulse_on_length = crate::util::read_f32_le(chunk)?;

            // pulse_fade_length: float
            let pulse_fade_length = crate::util::read_f32_le(chunk)?;

            // alpha: int8
            let alpha = crate::util::read_i8_le(chunk)?;

            // red: int8
            let red = crate::util::read_i8_le(chunk)?;

            // green: int8
            let green = crate::util::read_i8_le(chunk)?;

            // blue: int8
            let blue = crate::util::read_i8_le(chunk)?;

            // blend_mode: int8
            let blend_mode = crate::util::read_i8_le(chunk)?;

            // combo: string_ref
            let combo = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // render_layer: int32
            let render_layer = crate::util::read_i32_le(chunk)?;

            // texture_length: float
            let texture_length = crate::util::read_f32_le(chunk)?;


            rows.push(SpellChainEffectsRow {
                id,
                avg_seg_len,
                width,
                noise_scale,
                tex_coord_scale,
                seg_duration,
                seg_delay,
                texture,
                flags,
                joint_count,
                joint_offset_radius,
                joints_per_minor_joint,
                minor_joints_per_major_joint,
                minor_joint_scale,
                major_joint_scale,
                joint_move_speed,
                joint_smoothness,
                min_duration_between_joint_jumps,
                max_duration_between_joint_jumps,
                wave_height,
                wave_freq,
                wave_speed,
                min_wave_angle,
                max_wave_angle,
                min_wave_spin,
                max_wave_spin,
                arc_height,
                min_arc_angle,
                max_arc_angle,
                min_arc_spin,
                max_arc_spin,
                delay_between_effects,
                min_flicker_on_duration,
                max_flicker_on_duration,
                min_flicker_off_duration,
                max_flicker_off_duration,
                pulse_speed,
                pulse_on_length,
                pulse_fade_length,
                alpha,
                red,
                green,
                blue,
                blend_mode,
                combo,
                render_layer,
                texture_length,
            });
        }

        Ok(SpellChainEffects { rows, })
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
            // id: primary_key (SpellChainEffects) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // avg_seg_len: float
            b.write_all(&row.avg_seg_len.to_le_bytes())?;

            // width: float
            b.write_all(&row.width.to_le_bytes())?;

            // noise_scale: float
            b.write_all(&row.noise_scale.to_le_bytes())?;

            // tex_coord_scale: float
            b.write_all(&row.tex_coord_scale.to_le_bytes())?;

            // seg_duration: int32
            b.write_all(&row.seg_duration.to_le_bytes())?;

            // seg_delay: int32
            b.write_all(&row.seg_delay.to_le_bytes())?;

            // texture: string_ref
            if !row.texture.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.texture.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // joint_count: int32
            b.write_all(&row.joint_count.to_le_bytes())?;

            // joint_offset_radius: float
            b.write_all(&row.joint_offset_radius.to_le_bytes())?;

            // joints_per_minor_joint: int32
            b.write_all(&row.joints_per_minor_joint.to_le_bytes())?;

            // minor_joints_per_major_joint: int32
            b.write_all(&row.minor_joints_per_major_joint.to_le_bytes())?;

            // minor_joint_scale: float
            b.write_all(&row.minor_joint_scale.to_le_bytes())?;

            // major_joint_scale: float
            b.write_all(&row.major_joint_scale.to_le_bytes())?;

            // joint_move_speed: float
            b.write_all(&row.joint_move_speed.to_le_bytes())?;

            // joint_smoothness: float
            b.write_all(&row.joint_smoothness.to_le_bytes())?;

            // min_duration_between_joint_jumps: float
            b.write_all(&row.min_duration_between_joint_jumps.to_le_bytes())?;

            // max_duration_between_joint_jumps: float
            b.write_all(&row.max_duration_between_joint_jumps.to_le_bytes())?;

            // wave_height: float
            b.write_all(&row.wave_height.to_le_bytes())?;

            // wave_freq: float
            b.write_all(&row.wave_freq.to_le_bytes())?;

            // wave_speed: float
            b.write_all(&row.wave_speed.to_le_bytes())?;

            // min_wave_angle: float
            b.write_all(&row.min_wave_angle.to_le_bytes())?;

            // max_wave_angle: float
            b.write_all(&row.max_wave_angle.to_le_bytes())?;

            // min_wave_spin: float
            b.write_all(&row.min_wave_spin.to_le_bytes())?;

            // max_wave_spin: float
            b.write_all(&row.max_wave_spin.to_le_bytes())?;

            // arc_height: float
            b.write_all(&row.arc_height.to_le_bytes())?;

            // min_arc_angle: float
            b.write_all(&row.min_arc_angle.to_le_bytes())?;

            // max_arc_angle: float
            b.write_all(&row.max_arc_angle.to_le_bytes())?;

            // min_arc_spin: float
            b.write_all(&row.min_arc_spin.to_le_bytes())?;

            // max_arc_spin: float
            b.write_all(&row.max_arc_spin.to_le_bytes())?;

            // delay_between_effects: float
            b.write_all(&row.delay_between_effects.to_le_bytes())?;

            // min_flicker_on_duration: float
            b.write_all(&row.min_flicker_on_duration.to_le_bytes())?;

            // max_flicker_on_duration: float
            b.write_all(&row.max_flicker_on_duration.to_le_bytes())?;

            // min_flicker_off_duration: float
            b.write_all(&row.min_flicker_off_duration.to_le_bytes())?;

            // max_flicker_off_duration: float
            b.write_all(&row.max_flicker_off_duration.to_le_bytes())?;

            // pulse_speed: float
            b.write_all(&row.pulse_speed.to_le_bytes())?;

            // pulse_on_length: float
            b.write_all(&row.pulse_on_length.to_le_bytes())?;

            // pulse_fade_length: float
            b.write_all(&row.pulse_fade_length.to_le_bytes())?;

            // alpha: int8
            b.write_all(&row.alpha.to_le_bytes())?;

            // red: int8
            b.write_all(&row.red.to_le_bytes())?;

            // green: int8
            b.write_all(&row.green.to_le_bytes())?;

            // blue: int8
            b.write_all(&row.blue.to_le_bytes())?;

            // blend_mode: int8
            b.write_all(&row.blend_mode.to_le_bytes())?;

            // combo: string_ref
            if !row.combo.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.combo.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // render_layer: int32
            b.write_all(&row.render_layer.to_le_bytes())?;

            // texture_length: float
            b.write_all(&row.texture_length.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SpellChainEffects {
    type PrimaryKey = SpellChainEffectsKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl SpellChainEffects {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.texture.is_empty() { b.write_all(row.texture.as_bytes())?; b.write_all(&[0])?; };
            if !row.combo.is_empty() { b.write_all(row.combo.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.texture.is_empty() { sum += row.texture.len() + 1; };
            if !row.combo.is_empty() { sum += row.combo.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellChainEffectsKey {
    pub id: i32
}

impl SpellChainEffectsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for SpellChainEffectsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for SpellChainEffectsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for SpellChainEffectsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for SpellChainEffectsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for SpellChainEffectsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for SpellChainEffectsKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SpellChainEffectsKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for SpellChainEffectsKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SpellChainEffectsKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SpellChainEffectsKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellChainEffectsRow {
    pub id: SpellChainEffectsKey,
    pub avg_seg_len: f32,
    pub width: f32,
    pub noise_scale: f32,
    pub tex_coord_scale: f32,
    pub seg_duration: i32,
    pub seg_delay: i32,
    pub texture: String,
    pub flags: i32,
    pub joint_count: i32,
    pub joint_offset_radius: f32,
    pub joints_per_minor_joint: i32,
    pub minor_joints_per_major_joint: i32,
    pub minor_joint_scale: f32,
    pub major_joint_scale: f32,
    pub joint_move_speed: f32,
    pub joint_smoothness: f32,
    pub min_duration_between_joint_jumps: f32,
    pub max_duration_between_joint_jumps: f32,
    pub wave_height: f32,
    pub wave_freq: f32,
    pub wave_speed: f32,
    pub min_wave_angle: f32,
    pub max_wave_angle: f32,
    pub min_wave_spin: f32,
    pub max_wave_spin: f32,
    pub arc_height: f32,
    pub min_arc_angle: f32,
    pub max_arc_angle: f32,
    pub min_arc_spin: f32,
    pub max_arc_spin: f32,
    pub delay_between_effects: f32,
    pub min_flicker_on_duration: f32,
    pub max_flicker_on_duration: f32,
    pub min_flicker_off_duration: f32,
    pub max_flicker_off_duration: f32,
    pub pulse_speed: f32,
    pub pulse_on_length: f32,
    pub pulse_fade_length: f32,
    pub alpha: i8,
    pub red: i8,
    pub green: i8,
    pub blue: i8,
    pub blend_mode: i8,
    pub combo: String,
    pub render_layer: i32,
    pub texture_length: f32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell_chain_effects() {
        let mut file = File::open("../tbc-dbc/SpellChainEffects.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SpellChainEffects::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellChainEffects::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
