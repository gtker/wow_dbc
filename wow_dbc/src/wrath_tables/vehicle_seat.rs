use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::sound_entries::SoundEntriesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VehicleSeat {
    pub rows: Vec<VehicleSeatRow>,
}

impl DbcTable for VehicleSeat {
    type Row = VehicleSeatRow;

    const FILENAME: &'static str = "VehicleSeat.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 232 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 232,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 58 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 58,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (VehicleSeat) int32
            let id = VehicleSeatKey::new(crate::util::read_i32_le(chunk)?);

            // field_3_3_5_12213_001: float
            let field_3_3_5_12213_001 = crate::util::read_f32_le(chunk)?;

            // attachment_id: int32
            let attachment_id = crate::util::read_i32_le(chunk)?;

            // attachment_offset: float[3]
            let attachment_offset = crate::util::read_array_f32::<3>(chunk)?;

            // field_3_3_5_12213_004: float
            let field_3_3_5_12213_004 = crate::util::read_f32_le(chunk)?;

            // enter_speed: float
            let enter_speed = crate::util::read_f32_le(chunk)?;

            // enter_gravity: float
            let enter_gravity = crate::util::read_f32_le(chunk)?;

            // enter_min_duration: float
            let enter_min_duration = crate::util::read_f32_le(chunk)?;

            // enter_max_duration: float
            let enter_max_duration = crate::util::read_f32_le(chunk)?;

            // enter_min_arc_height: float
            let enter_min_arc_height = crate::util::read_f32_le(chunk)?;

            // enter_max_arc_height: float
            let enter_max_arc_height = crate::util::read_f32_le(chunk)?;

            // enter_anim_start: int32
            let enter_anim_start = crate::util::read_i32_le(chunk)?;

            // enter_anim_loop: int32
            let enter_anim_loop = crate::util::read_i32_le(chunk)?;

            // ride_anim_start: int32
            let ride_anim_start = crate::util::read_i32_le(chunk)?;

            // ride_anim_loop: int32
            let ride_anim_loop = crate::util::read_i32_le(chunk)?;

            // ride_upper_anim_start: int32
            let ride_upper_anim_start = crate::util::read_i32_le(chunk)?;

            // ride_upper_anim_loop: int32
            let ride_upper_anim_loop = crate::util::read_i32_le(chunk)?;

            // field_3_3_5_12213_017: float
            let field_3_3_5_12213_017 = crate::util::read_f32_le(chunk)?;

            // exit_speed: float
            let exit_speed = crate::util::read_f32_le(chunk)?;

            // exit_gravity: float
            let exit_gravity = crate::util::read_f32_le(chunk)?;

            // exit_min_duration: float
            let exit_min_duration = crate::util::read_f32_le(chunk)?;

            // exit_max_duration: float
            let exit_max_duration = crate::util::read_f32_le(chunk)?;

            // exit_min_arc_height: float
            let exit_min_arc_height = crate::util::read_f32_le(chunk)?;

            // exit_max_arc_height: float
            let exit_max_arc_height = crate::util::read_f32_le(chunk)?;

            // exit_anim_start: int32
            let exit_anim_start = crate::util::read_i32_le(chunk)?;

            // exit_anim_loop: int32
            let exit_anim_loop = crate::util::read_i32_le(chunk)?;

            // exit_anim_end: int32
            let exit_anim_end = crate::util::read_i32_le(chunk)?;

            // field_3_3_5_12213_027: float
            let field_3_3_5_12213_027 = crate::util::read_f32_le(chunk)?;

            // passenger_pitch: float
            let passenger_pitch = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_029: float
            let field_3_3_5_12213_029 = crate::util::read_f32_le(chunk)?;

            // passenger_attachment_id: int32
            let passenger_attachment_id = crate::util::read_i32_le(chunk)?;

            // vehicle_enter_anim: int32
            let vehicle_enter_anim = crate::util::read_i32_le(chunk)?;

            // vehicle_exit_anim: int32
            let vehicle_exit_anim = crate::util::read_i32_le(chunk)?;

            // vehicle_ride_anim_loop: int32
            let vehicle_ride_anim_loop = crate::util::read_i32_le(chunk)?;

            // field_3_3_5_12213_034: int32
            let field_3_3_5_12213_034 = crate::util::read_i32_le(chunk)?;

            // vehicle_exit_anim_bone: int32
            let vehicle_exit_anim_bone = crate::util::read_i32_le(chunk)?;

            // vehicle_enter_anim_bone: int32
            let vehicle_enter_anim_bone = crate::util::read_i32_le(chunk)?;

            // field_3_3_5_12213_037: float
            let field_3_3_5_12213_037 = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_038: float
            let field_3_3_5_12213_038 = crate::util::read_f32_le(chunk)?;

            // vehicle_ability_display: int32
            let vehicle_ability_display = crate::util::read_i32_le(chunk)?;

            // enter_u_i_sound_id: foreign_key (SoundEntries) int32
            let enter_u_i_sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // field_3_3_5_12213_041: int32
            let field_3_3_5_12213_041 = crate::util::read_i32_le(chunk)?;

            // ui_skin: int32
            let ui_skin = crate::util::read_i32_le(chunk)?;

            // field_3_3_5_12213_043: float
            let field_3_3_5_12213_043 = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_044: float
            let field_3_3_5_12213_044 = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_045: float
            let field_3_3_5_12213_045 = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_046: int32
            let field_3_3_5_12213_046 = crate::util::read_i32_le(chunk)?;

            // field_3_3_5_12213_047: float
            let field_3_3_5_12213_047 = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_048: float
            let field_3_3_5_12213_048 = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_049: float
            let field_3_3_5_12213_049 = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_050: float
            let field_3_3_5_12213_050 = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_051: float
            let field_3_3_5_12213_051 = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_052: float
            let field_3_3_5_12213_052 = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_053: float
            let field_3_3_5_12213_053 = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_054: float
            let field_3_3_5_12213_054 = crate::util::read_f32_le(chunk)?;

            // field_3_3_5_12213_055: float
            let field_3_3_5_12213_055 = crate::util::read_f32_le(chunk)?;


            rows.push(VehicleSeatRow {
                id,
                field_3_3_5_12213_001,
                attachment_id,
                attachment_offset,
                field_3_3_5_12213_004,
                enter_speed,
                enter_gravity,
                enter_min_duration,
                enter_max_duration,
                enter_min_arc_height,
                enter_max_arc_height,
                enter_anim_start,
                enter_anim_loop,
                ride_anim_start,
                ride_anim_loop,
                ride_upper_anim_start,
                ride_upper_anim_loop,
                field_3_3_5_12213_017,
                exit_speed,
                exit_gravity,
                exit_min_duration,
                exit_max_duration,
                exit_min_arc_height,
                exit_max_arc_height,
                exit_anim_start,
                exit_anim_loop,
                exit_anim_end,
                field_3_3_5_12213_027,
                passenger_pitch,
                field_3_3_5_12213_029,
                passenger_attachment_id,
                vehicle_enter_anim,
                vehicle_exit_anim,
                vehicle_ride_anim_loop,
                field_3_3_5_12213_034,
                vehicle_exit_anim_bone,
                vehicle_enter_anim_bone,
                field_3_3_5_12213_037,
                field_3_3_5_12213_038,
                vehicle_ability_display,
                enter_u_i_sound_id,
                field_3_3_5_12213_041,
                ui_skin,
                field_3_3_5_12213_043,
                field_3_3_5_12213_044,
                field_3_3_5_12213_045,
                field_3_3_5_12213_046,
                field_3_3_5_12213_047,
                field_3_3_5_12213_048,
                field_3_3_5_12213_049,
                field_3_3_5_12213_050,
                field_3_3_5_12213_051,
                field_3_3_5_12213_052,
                field_3_3_5_12213_053,
                field_3_3_5_12213_054,
                field_3_3_5_12213_055,
            });
        }

        Ok(VehicleSeat { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 58,
            record_size: 232,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (VehicleSeat) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // field_3_3_5_12213_001: float
            b.write_all(&row.field_3_3_5_12213_001.to_le_bytes())?;

            // attachment_id: int32
            b.write_all(&row.attachment_id.to_le_bytes())?;

            // attachment_offset: float[3]
            for i in row.attachment_offset {
                b.write_all(&i.to_le_bytes())?;
            }


            // field_3_3_5_12213_004: float
            b.write_all(&row.field_3_3_5_12213_004.to_le_bytes())?;

            // enter_speed: float
            b.write_all(&row.enter_speed.to_le_bytes())?;

            // enter_gravity: float
            b.write_all(&row.enter_gravity.to_le_bytes())?;

            // enter_min_duration: float
            b.write_all(&row.enter_min_duration.to_le_bytes())?;

            // enter_max_duration: float
            b.write_all(&row.enter_max_duration.to_le_bytes())?;

            // enter_min_arc_height: float
            b.write_all(&row.enter_min_arc_height.to_le_bytes())?;

            // enter_max_arc_height: float
            b.write_all(&row.enter_max_arc_height.to_le_bytes())?;

            // enter_anim_start: int32
            b.write_all(&row.enter_anim_start.to_le_bytes())?;

            // enter_anim_loop: int32
            b.write_all(&row.enter_anim_loop.to_le_bytes())?;

            // ride_anim_start: int32
            b.write_all(&row.ride_anim_start.to_le_bytes())?;

            // ride_anim_loop: int32
            b.write_all(&row.ride_anim_loop.to_le_bytes())?;

            // ride_upper_anim_start: int32
            b.write_all(&row.ride_upper_anim_start.to_le_bytes())?;

            // ride_upper_anim_loop: int32
            b.write_all(&row.ride_upper_anim_loop.to_le_bytes())?;

            // field_3_3_5_12213_017: float
            b.write_all(&row.field_3_3_5_12213_017.to_le_bytes())?;

            // exit_speed: float
            b.write_all(&row.exit_speed.to_le_bytes())?;

            // exit_gravity: float
            b.write_all(&row.exit_gravity.to_le_bytes())?;

            // exit_min_duration: float
            b.write_all(&row.exit_min_duration.to_le_bytes())?;

            // exit_max_duration: float
            b.write_all(&row.exit_max_duration.to_le_bytes())?;

            // exit_min_arc_height: float
            b.write_all(&row.exit_min_arc_height.to_le_bytes())?;

            // exit_max_arc_height: float
            b.write_all(&row.exit_max_arc_height.to_le_bytes())?;

            // exit_anim_start: int32
            b.write_all(&row.exit_anim_start.to_le_bytes())?;

            // exit_anim_loop: int32
            b.write_all(&row.exit_anim_loop.to_le_bytes())?;

            // exit_anim_end: int32
            b.write_all(&row.exit_anim_end.to_le_bytes())?;

            // field_3_3_5_12213_027: float
            b.write_all(&row.field_3_3_5_12213_027.to_le_bytes())?;

            // passenger_pitch: float
            b.write_all(&row.passenger_pitch.to_le_bytes())?;

            // field_3_3_5_12213_029: float
            b.write_all(&row.field_3_3_5_12213_029.to_le_bytes())?;

            // passenger_attachment_id: int32
            b.write_all(&row.passenger_attachment_id.to_le_bytes())?;

            // vehicle_enter_anim: int32
            b.write_all(&row.vehicle_enter_anim.to_le_bytes())?;

            // vehicle_exit_anim: int32
            b.write_all(&row.vehicle_exit_anim.to_le_bytes())?;

            // vehicle_ride_anim_loop: int32
            b.write_all(&row.vehicle_ride_anim_loop.to_le_bytes())?;

            // field_3_3_5_12213_034: int32
            b.write_all(&row.field_3_3_5_12213_034.to_le_bytes())?;

            // vehicle_exit_anim_bone: int32
            b.write_all(&row.vehicle_exit_anim_bone.to_le_bytes())?;

            // vehicle_enter_anim_bone: int32
            b.write_all(&row.vehicle_enter_anim_bone.to_le_bytes())?;

            // field_3_3_5_12213_037: float
            b.write_all(&row.field_3_3_5_12213_037.to_le_bytes())?;

            // field_3_3_5_12213_038: float
            b.write_all(&row.field_3_3_5_12213_038.to_le_bytes())?;

            // vehicle_ability_display: int32
            b.write_all(&row.vehicle_ability_display.to_le_bytes())?;

            // enter_u_i_sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.enter_u_i_sound_id.id as i32).to_le_bytes())?;

            // field_3_3_5_12213_041: int32
            b.write_all(&row.field_3_3_5_12213_041.to_le_bytes())?;

            // ui_skin: int32
            b.write_all(&row.ui_skin.to_le_bytes())?;

            // field_3_3_5_12213_043: float
            b.write_all(&row.field_3_3_5_12213_043.to_le_bytes())?;

            // field_3_3_5_12213_044: float
            b.write_all(&row.field_3_3_5_12213_044.to_le_bytes())?;

            // field_3_3_5_12213_045: float
            b.write_all(&row.field_3_3_5_12213_045.to_le_bytes())?;

            // field_3_3_5_12213_046: int32
            b.write_all(&row.field_3_3_5_12213_046.to_le_bytes())?;

            // field_3_3_5_12213_047: float
            b.write_all(&row.field_3_3_5_12213_047.to_le_bytes())?;

            // field_3_3_5_12213_048: float
            b.write_all(&row.field_3_3_5_12213_048.to_le_bytes())?;

            // field_3_3_5_12213_049: float
            b.write_all(&row.field_3_3_5_12213_049.to_le_bytes())?;

            // field_3_3_5_12213_050: float
            b.write_all(&row.field_3_3_5_12213_050.to_le_bytes())?;

            // field_3_3_5_12213_051: float
            b.write_all(&row.field_3_3_5_12213_051.to_le_bytes())?;

            // field_3_3_5_12213_052: float
            b.write_all(&row.field_3_3_5_12213_052.to_le_bytes())?;

            // field_3_3_5_12213_053: float
            b.write_all(&row.field_3_3_5_12213_053.to_le_bytes())?;

            // field_3_3_5_12213_054: float
            b.write_all(&row.field_3_3_5_12213_054.to_le_bytes())?;

            // field_3_3_5_12213_055: float
            b.write_all(&row.field_3_3_5_12213_055.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for VehicleSeat {
    type PrimaryKey = VehicleSeatKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct VehicleSeatKey {
    pub id: i32
}

impl VehicleSeatKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for VehicleSeatKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for VehicleSeatKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for VehicleSeatKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl From<u8> for VehicleSeatKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for VehicleSeatKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VehicleSeatRow {
    pub id: VehicleSeatKey,
    pub field_3_3_5_12213_001: f32,
    pub attachment_id: i32,
    pub attachment_offset: [f32; 3],
    pub field_3_3_5_12213_004: f32,
    pub enter_speed: f32,
    pub enter_gravity: f32,
    pub enter_min_duration: f32,
    pub enter_max_duration: f32,
    pub enter_min_arc_height: f32,
    pub enter_max_arc_height: f32,
    pub enter_anim_start: i32,
    pub enter_anim_loop: i32,
    pub ride_anim_start: i32,
    pub ride_anim_loop: i32,
    pub ride_upper_anim_start: i32,
    pub ride_upper_anim_loop: i32,
    pub field_3_3_5_12213_017: f32,
    pub exit_speed: f32,
    pub exit_gravity: f32,
    pub exit_min_duration: f32,
    pub exit_max_duration: f32,
    pub exit_min_arc_height: f32,
    pub exit_max_arc_height: f32,
    pub exit_anim_start: i32,
    pub exit_anim_loop: i32,
    pub exit_anim_end: i32,
    pub field_3_3_5_12213_027: f32,
    pub passenger_pitch: f32,
    pub field_3_3_5_12213_029: f32,
    pub passenger_attachment_id: i32,
    pub vehicle_enter_anim: i32,
    pub vehicle_exit_anim: i32,
    pub vehicle_ride_anim_loop: i32,
    pub field_3_3_5_12213_034: i32,
    pub vehicle_exit_anim_bone: i32,
    pub vehicle_enter_anim_bone: i32,
    pub field_3_3_5_12213_037: f32,
    pub field_3_3_5_12213_038: f32,
    pub vehicle_ability_display: i32,
    pub enter_u_i_sound_id: SoundEntriesKey,
    pub field_3_3_5_12213_041: i32,
    pub ui_skin: i32,
    pub field_3_3_5_12213_043: f32,
    pub field_3_3_5_12213_044: f32,
    pub field_3_3_5_12213_045: f32,
    pub field_3_3_5_12213_046: i32,
    pub field_3_3_5_12213_047: f32,
    pub field_3_3_5_12213_048: f32,
    pub field_3_3_5_12213_049: f32,
    pub field_3_3_5_12213_050: f32,
    pub field_3_3_5_12213_051: f32,
    pub field_3_3_5_12213_052: f32,
    pub field_3_3_5_12213_053: f32,
    pub field_3_3_5_12213_054: f32,
    pub field_3_3_5_12213_055: f32,
}

