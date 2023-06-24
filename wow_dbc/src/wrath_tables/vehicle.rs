use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::vehicle_ui_indicator::VehicleUIIndicatorKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vehicle {
    pub rows: Vec<VehicleRow>,
}

impl DbcTable for Vehicle {
    type Row = VehicleRow;

    const FILENAME: &'static str = "Vehicle.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 160 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 160,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 40 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 40,
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

            // id: primary_key (Vehicle) int32
            let id = VehicleKey::new(crate::util::read_i32_le(chunk)?);

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // turn_speed: float
            let turn_speed = crate::util::read_f32_le(chunk)?;

            // pitch_speed: float
            let pitch_speed = crate::util::read_f32_le(chunk)?;

            // pitch_min: float
            let pitch_min = crate::util::read_f32_le(chunk)?;

            // pitch_max: float
            let pitch_max = crate::util::read_f32_le(chunk)?;

            // seat_id: int32[8]
            let seat_id = crate::util::read_array_i32::<8>(chunk)?;

            // mouse_look_offset_pitch: float
            let mouse_look_offset_pitch = crate::util::read_f32_le(chunk)?;

            // camera_fade_dist_scalar_min: float
            let camera_fade_dist_scalar_min = crate::util::read_f32_le(chunk)?;

            // camera_fade_dist_scalar_max: float
            let camera_fade_dist_scalar_max = crate::util::read_f32_le(chunk)?;

            // camera_pitch_offset: float
            let camera_pitch_offset = crate::util::read_f32_le(chunk)?;

            // facing_limit_right: float
            let facing_limit_right = crate::util::read_f32_le(chunk)?;

            // facing_limit_left: float
            let facing_limit_left = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_turn_lingering: float
            let mssl_trgt_turn_lingering = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_pitch_lingering: float
            let mssl_trgt_pitch_lingering = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_mouse_lingering: float
            let mssl_trgt_mouse_lingering = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_end_opacity: float
            let mssl_trgt_end_opacity = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_arc_speed: float
            let mssl_trgt_arc_speed = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_arc_repeat: float
            let mssl_trgt_arc_repeat = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_arc_width: float
            let mssl_trgt_arc_width = crate::util::read_f32_le(chunk)?;

            // mssl_trgt_impact_radius: float[2]
            let mssl_trgt_impact_radius = crate::util::read_array_f32::<2>(chunk)?;

            // mssl_trgt_arc_texture: string_ref
            let mssl_trgt_arc_texture = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // mssl_trgt_impact_texture: string_ref
            let mssl_trgt_impact_texture = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // mssl_trgt_impact_model: string_ref[2]
            let mssl_trgt_impact_model = {
                let mut arr = Vec::with_capacity(2);
                for _ in 0..2 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // camera_yaw_offset: float
            let camera_yaw_offset = crate::util::read_f32_le(chunk)?;

            // ui_locomotion_type: int32
            let ui_locomotion_type = crate::util::read_i32_le(chunk)?;

            // mssl_trgt_impact_tex_radius: float
            let mssl_trgt_impact_tex_radius = crate::util::read_f32_le(chunk)?;

            // vehicle_u_i_indicator_id: foreign_key (VehicleUIIndicator) int32
            let vehicle_u_i_indicator_id = VehicleUIIndicatorKey::new(crate::util::read_i32_le(chunk)?.into());

            // power_display_id: int32[3]
            let power_display_id = crate::util::read_array_i32::<3>(chunk)?;


            rows.push(VehicleRow {
                id,
                flags,
                turn_speed,
                pitch_speed,
                pitch_min,
                pitch_max,
                seat_id,
                mouse_look_offset_pitch,
                camera_fade_dist_scalar_min,
                camera_fade_dist_scalar_max,
                camera_pitch_offset,
                facing_limit_right,
                facing_limit_left,
                mssl_trgt_turn_lingering,
                mssl_trgt_pitch_lingering,
                mssl_trgt_mouse_lingering,
                mssl_trgt_end_opacity,
                mssl_trgt_arc_speed,
                mssl_trgt_arc_repeat,
                mssl_trgt_arc_width,
                mssl_trgt_impact_radius,
                mssl_trgt_arc_texture,
                mssl_trgt_impact_texture,
                mssl_trgt_impact_model,
                camera_yaw_offset,
                ui_locomotion_type,
                mssl_trgt_impact_tex_radius,
                vehicle_u_i_indicator_id,
                power_display_id,
            });
        }

        Ok(Vehicle { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 40,
            record_size: 160,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Vehicle) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // turn_speed: float
            b.write_all(&row.turn_speed.to_le_bytes())?;

            // pitch_speed: float
            b.write_all(&row.pitch_speed.to_le_bytes())?;

            // pitch_min: float
            b.write_all(&row.pitch_min.to_le_bytes())?;

            // pitch_max: float
            b.write_all(&row.pitch_max.to_le_bytes())?;

            // seat_id: int32[8]
            for i in row.seat_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // mouse_look_offset_pitch: float
            b.write_all(&row.mouse_look_offset_pitch.to_le_bytes())?;

            // camera_fade_dist_scalar_min: float
            b.write_all(&row.camera_fade_dist_scalar_min.to_le_bytes())?;

            // camera_fade_dist_scalar_max: float
            b.write_all(&row.camera_fade_dist_scalar_max.to_le_bytes())?;

            // camera_pitch_offset: float
            b.write_all(&row.camera_pitch_offset.to_le_bytes())?;

            // facing_limit_right: float
            b.write_all(&row.facing_limit_right.to_le_bytes())?;

            // facing_limit_left: float
            b.write_all(&row.facing_limit_left.to_le_bytes())?;

            // mssl_trgt_turn_lingering: float
            b.write_all(&row.mssl_trgt_turn_lingering.to_le_bytes())?;

            // mssl_trgt_pitch_lingering: float
            b.write_all(&row.mssl_trgt_pitch_lingering.to_le_bytes())?;

            // mssl_trgt_mouse_lingering: float
            b.write_all(&row.mssl_trgt_mouse_lingering.to_le_bytes())?;

            // mssl_trgt_end_opacity: float
            b.write_all(&row.mssl_trgt_end_opacity.to_le_bytes())?;

            // mssl_trgt_arc_speed: float
            b.write_all(&row.mssl_trgt_arc_speed.to_le_bytes())?;

            // mssl_trgt_arc_repeat: float
            b.write_all(&row.mssl_trgt_arc_repeat.to_le_bytes())?;

            // mssl_trgt_arc_width: float
            b.write_all(&row.mssl_trgt_arc_width.to_le_bytes())?;

            // mssl_trgt_impact_radius: float[2]
            for i in row.mssl_trgt_impact_radius {
                b.write_all(&i.to_le_bytes())?;
            }


            // mssl_trgt_arc_texture: string_ref
            if !row.mssl_trgt_arc_texture.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.mssl_trgt_arc_texture.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // mssl_trgt_impact_texture: string_ref
            if !row.mssl_trgt_impact_texture.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.mssl_trgt_impact_texture.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // mssl_trgt_impact_model: string_ref[2]
            for i in &row.mssl_trgt_impact_model {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // camera_yaw_offset: float
            b.write_all(&row.camera_yaw_offset.to_le_bytes())?;

            // ui_locomotion_type: int32
            b.write_all(&row.ui_locomotion_type.to_le_bytes())?;

            // mssl_trgt_impact_tex_radius: float
            b.write_all(&row.mssl_trgt_impact_tex_radius.to_le_bytes())?;

            // vehicle_u_i_indicator_id: foreign_key (VehicleUIIndicator) int32
            b.write_all(&(row.vehicle_u_i_indicator_id.id as i32).to_le_bytes())?;

            // power_display_id: int32[3]
            for i in row.power_display_id {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Vehicle {
    type PrimaryKey = VehicleKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl Vehicle {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.mssl_trgt_arc_texture.is_empty() { b.write_all(row.mssl_trgt_arc_texture.as_bytes())?; b.write_all(&[0])?; };
            if !row.mssl_trgt_impact_texture.is_empty() { b.write_all(row.mssl_trgt_impact_texture.as_bytes())?; b.write_all(&[0])?; };
            for s in &row.mssl_trgt_impact_model {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.mssl_trgt_arc_texture.is_empty() { sum += row.mssl_trgt_arc_texture.len() + 1; };
            if !row.mssl_trgt_impact_texture.is_empty() { sum += row.mssl_trgt_impact_texture.len() + 1; };
            for s in &row.mssl_trgt_impact_model {
                if !s.is_empty() { sum += s.len() + 1; };
            }

        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct VehicleKey {
    pub id: i32
}

impl VehicleKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for VehicleKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for VehicleKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for VehicleKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl From<u8> for VehicleKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for VehicleKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VehicleRow {
    pub id: VehicleKey,
    pub flags: i32,
    pub turn_speed: f32,
    pub pitch_speed: f32,
    pub pitch_min: f32,
    pub pitch_max: f32,
    pub seat_id: [i32; 8],
    pub mouse_look_offset_pitch: f32,
    pub camera_fade_dist_scalar_min: f32,
    pub camera_fade_dist_scalar_max: f32,
    pub camera_pitch_offset: f32,
    pub facing_limit_right: f32,
    pub facing_limit_left: f32,
    pub mssl_trgt_turn_lingering: f32,
    pub mssl_trgt_pitch_lingering: f32,
    pub mssl_trgt_mouse_lingering: f32,
    pub mssl_trgt_end_opacity: f32,
    pub mssl_trgt_arc_speed: f32,
    pub mssl_trgt_arc_repeat: f32,
    pub mssl_trgt_arc_width: f32,
    pub mssl_trgt_impact_radius: [f32; 2],
    pub mssl_trgt_arc_texture: String,
    pub mssl_trgt_impact_texture: String,
    pub mssl_trgt_impact_model: [String; 2],
    pub camera_yaw_offset: f32,
    pub ui_locomotion_type: i32,
    pub mssl_trgt_impact_tex_radius: f32,
    pub vehicle_u_i_indicator_id: VehicleUIIndicatorKey,
    pub power_display_id: [i32; 3],
}

