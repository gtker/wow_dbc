use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::vehicle_ui_indicator::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vehicle {
    pub rows: Vec<VehicleRow>,
}

impl DbcTable for Vehicle {
    type Row = VehicleRow;

    fn filename() -> &'static str { "Vehicle.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstVehicle<const S: usize> {
    pub rows: [ConstVehicleRow; S],
}

impl<const S: usize> ConstVehicle<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 160 {
            panic!("invalid record size, expected 160")
        }

        if header.field_count != 40 {
            panic!("invalid field count, expected 40")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstVehicleRow {
                id: VehicleKey::new(0),
                flags: 0,
                turn_speed: 0.0,
                pitch_speed: 0.0,
                pitch_min: 0.0,
                pitch_max: 0.0,
                seat_id: [0; 8],
                mouse_look_offset_pitch: 0.0,
                camera_fade_dist_scalar_min: 0.0,
                camera_fade_dist_scalar_max: 0.0,
                camera_pitch_offset: 0.0,
                facing_limit_right: 0.0,
                facing_limit_left: 0.0,
                mssl_trgt_turn_lingering: 0.0,
                mssl_trgt_pitch_lingering: 0.0,
                mssl_trgt_mouse_lingering: 0.0,
                mssl_trgt_end_opacity: 0.0,
                mssl_trgt_arc_speed: 0.0,
                mssl_trgt_arc_repeat: 0.0,
                mssl_trgt_arc_width: 0.0,
                mssl_trgt_impact_radius: [0.0; 2],
                mssl_trgt_arc_texture: "",
                mssl_trgt_impact_texture: "",
                mssl_trgt_impact_model: [""; 2],
                camera_yaw_offset: 0.0,
                ui_locomotion_type: 0,
                mssl_trgt_impact_tex_radius: 0.0,
                vehicle_u_i_indicator_id: VehicleUIIndicatorKey::new(0),
                power_display_id: [0; 3],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Vehicle) int32
            let id = VehicleKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // turn_speed: float
            let turn_speed = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // pitch_speed: float
            let pitch_speed = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // pitch_min: float
            let pitch_min = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // pitch_max: float
            let pitch_max = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // seat_id: int32[8]
            let seat_id = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // mouse_look_offset_pitch: float
            let mouse_look_offset_pitch = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // camera_fade_dist_scalar_min: float
            let camera_fade_dist_scalar_min = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // camera_fade_dist_scalar_max: float
            let camera_fade_dist_scalar_max = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // camera_pitch_offset: float
            let camera_pitch_offset = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // facing_limit_right: float
            let facing_limit_right = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // facing_limit_left: float
            let facing_limit_left = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mssl_trgt_turn_lingering: float
            let mssl_trgt_turn_lingering = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mssl_trgt_pitch_lingering: float
            let mssl_trgt_pitch_lingering = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mssl_trgt_mouse_lingering: float
            let mssl_trgt_mouse_lingering = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mssl_trgt_end_opacity: float
            let mssl_trgt_end_opacity = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mssl_trgt_arc_speed: float
            let mssl_trgt_arc_speed = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mssl_trgt_arc_repeat: float
            let mssl_trgt_arc_repeat = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mssl_trgt_arc_width: float
            let mssl_trgt_arc_width = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mssl_trgt_impact_radius: float[2]
            let mssl_trgt_impact_radius = {
                let mut a = [0.0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // mssl_trgt_arc_texture: string_ref
            let mssl_trgt_arc_texture = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // mssl_trgt_impact_texture: string_ref
            let mssl_trgt_impact_texture = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // mssl_trgt_impact_model: string_ref[2]
            let mssl_trgt_impact_model = {
                let mut a = [""; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::get_string_from_block(b_offset, b, string_block);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // camera_yaw_offset: float
            let camera_yaw_offset = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // ui_locomotion_type: int32
            let ui_locomotion_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mssl_trgt_impact_tex_radius: float
            let mssl_trgt_impact_tex_radius = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // vehicle_u_i_indicator_id: foreign_key (VehicleUIIndicator) int32
            let vehicle_u_i_indicator_id = VehicleUIIndicatorKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // power_display_id: int32[3]
            let power_display_id = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = ConstVehicleRow {
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
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> Vehicle {
        Vehicle {
            rows: self.rows.iter().map(|s| VehicleRow {
                id: s.id,
                flags: s.flags,
                turn_speed: s.turn_speed,
                pitch_speed: s.pitch_speed,
                pitch_min: s.pitch_min,
                pitch_max: s.pitch_max,
                seat_id: s.seat_id,
                mouse_look_offset_pitch: s.mouse_look_offset_pitch,
                camera_fade_dist_scalar_min: s.camera_fade_dist_scalar_min,
                camera_fade_dist_scalar_max: s.camera_fade_dist_scalar_max,
                camera_pitch_offset: s.camera_pitch_offset,
                facing_limit_right: s.facing_limit_right,
                facing_limit_left: s.facing_limit_left,
                mssl_trgt_turn_lingering: s.mssl_trgt_turn_lingering,
                mssl_trgt_pitch_lingering: s.mssl_trgt_pitch_lingering,
                mssl_trgt_mouse_lingering: s.mssl_trgt_mouse_lingering,
                mssl_trgt_end_opacity: s.mssl_trgt_end_opacity,
                mssl_trgt_arc_speed: s.mssl_trgt_arc_speed,
                mssl_trgt_arc_repeat: s.mssl_trgt_arc_repeat,
                mssl_trgt_arc_width: s.mssl_trgt_arc_width,
                mssl_trgt_impact_radius: s.mssl_trgt_impact_radius,
                mssl_trgt_arc_texture: s.mssl_trgt_arc_texture.to_string(),
                mssl_trgt_impact_texture: s.mssl_trgt_impact_texture.to_string(),
                mssl_trgt_impact_model: s.mssl_trgt_impact_model.map(|a| a.to_string()),
                camera_yaw_offset: s.camera_yaw_offset,
                ui_locomotion_type: s.ui_locomotion_type,
                mssl_trgt_impact_tex_radius: s.mssl_trgt_impact_tex_radius,
                vehicle_u_i_indicator_id: s.vehicle_u_i_indicator_id,
                power_display_id: s.power_display_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstVehicleRow {
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
    pub mssl_trgt_arc_texture: &'static str,
    pub mssl_trgt_impact_texture: &'static str,
    pub mssl_trgt_impact_model: [&'static str; 2],
    pub camera_yaw_offset: f32,
    pub ui_locomotion_type: i32,
    pub mssl_trgt_impact_tex_radius: f32,
    pub vehicle_u_i_indicator_id: VehicleUIIndicatorKey,
    pub power_display_id: [i32; 3],
}

