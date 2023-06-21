use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::creature_sound_data::*;
use crate::wrath_tables::footprint_textures::*;
use crate::wrath_tables::material::*;
use crate::wrath_tables::unit_blood::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CreatureModelData {
    pub rows: Vec<CreatureModelDataRow>,
}

impl DbcTable for CreatureModelData {
    type Row = CreatureModelDataRow;

    fn filename() -> &'static str { "CreatureModelData.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 112 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 112,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 28 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 28,
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

            // id: primary_key (CreatureModelData) int32
            let id = CreatureModelDataKey::new(crate::util::read_i32_le(chunk)?);

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // model_name: string_ref
            let model_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // size_class: int32
            let size_class = crate::util::read_i32_le(chunk)?;

            // model_scale: float
            let model_scale = crate::util::read_f32_le(chunk)?;

            // blood_id: foreign_key (UnitBlood) int32
            let blood_id = UnitBloodKey::new(crate::util::read_i32_le(chunk)?.into());

            // footprint_texture_id: foreign_key (FootprintTextures) int32
            let footprint_texture_id = FootprintTexturesKey::new(crate::util::read_i32_le(chunk)?.into());

            // footprint_texture_length: float
            let footprint_texture_length = crate::util::read_f32_le(chunk)?;

            // footprint_texture_width: float
            let footprint_texture_width = crate::util::read_f32_le(chunk)?;

            // footprint_particle_scale: float
            let footprint_particle_scale = crate::util::read_f32_le(chunk)?;

            // foley_material_id: foreign_key (Material) int32
            let foley_material_id = MaterialKey::new(crate::util::read_i32_le(chunk)?.into());

            // footstep_shake_size: int32
            let footstep_shake_size = crate::util::read_i32_le(chunk)?;

            // death_thud_shake_size: int32
            let death_thud_shake_size = crate::util::read_i32_le(chunk)?;

            // sound_id: foreign_key (CreatureSoundData) int32
            let sound_id = CreatureSoundDataKey::new(crate::util::read_i32_le(chunk)?.into());

            // collision_width: float
            let collision_width = crate::util::read_f32_le(chunk)?;

            // collision_height: float
            let collision_height = crate::util::read_f32_le(chunk)?;

            // mount_height: float
            let mount_height = crate::util::read_f32_le(chunk)?;

            // geo_box_min_x: float
            let geo_box_min_x = crate::util::read_f32_le(chunk)?;

            // geo_box_min_y: float
            let geo_box_min_y = crate::util::read_f32_le(chunk)?;

            // geo_box_min_z: float
            let geo_box_min_z = crate::util::read_f32_le(chunk)?;

            // geo_box_max_x: float
            let geo_box_max_x = crate::util::read_f32_le(chunk)?;

            // geo_box_max_y: float
            let geo_box_max_y = crate::util::read_f32_le(chunk)?;

            // geo_box_max_z: float
            let geo_box_max_z = crate::util::read_f32_le(chunk)?;

            // world_effect_scale: float
            let world_effect_scale = crate::util::read_f32_le(chunk)?;

            // attached_effect_scale: float
            let attached_effect_scale = crate::util::read_f32_le(chunk)?;

            // missile_collision_radius: float
            let missile_collision_radius = crate::util::read_f32_le(chunk)?;

            // missile_collision_push: float
            let missile_collision_push = crate::util::read_f32_le(chunk)?;

            // missile_collision_raise: float
            let missile_collision_raise = crate::util::read_f32_le(chunk)?;


            rows.push(CreatureModelDataRow {
                id,
                flags,
                model_name,
                size_class,
                model_scale,
                blood_id,
                footprint_texture_id,
                footprint_texture_length,
                footprint_texture_width,
                footprint_particle_scale,
                foley_material_id,
                footstep_shake_size,
                death_thud_shake_size,
                sound_id,
                collision_width,
                collision_height,
                mount_height,
                geo_box_min_x,
                geo_box_min_y,
                geo_box_min_z,
                geo_box_max_x,
                geo_box_max_y,
                geo_box_max_z,
                world_effect_scale,
                attached_effect_scale,
                missile_collision_radius,
                missile_collision_push,
                missile_collision_raise,
            });
        }

        Ok(CreatureModelData { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 28,
            record_size: 112,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (CreatureModelData) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // model_name: string_ref
            if !row.model_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.model_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // size_class: int32
            b.write_all(&row.size_class.to_le_bytes())?;

            // model_scale: float
            b.write_all(&row.model_scale.to_le_bytes())?;

            // blood_id: foreign_key (UnitBlood) int32
            b.write_all(&(row.blood_id.id as i32).to_le_bytes())?;

            // footprint_texture_id: foreign_key (FootprintTextures) int32
            b.write_all(&(row.footprint_texture_id.id as i32).to_le_bytes())?;

            // footprint_texture_length: float
            b.write_all(&row.footprint_texture_length.to_le_bytes())?;

            // footprint_texture_width: float
            b.write_all(&row.footprint_texture_width.to_le_bytes())?;

            // footprint_particle_scale: float
            b.write_all(&row.footprint_particle_scale.to_le_bytes())?;

            // foley_material_id: foreign_key (Material) int32
            b.write_all(&(row.foley_material_id.id as i32).to_le_bytes())?;

            // footstep_shake_size: int32
            b.write_all(&row.footstep_shake_size.to_le_bytes())?;

            // death_thud_shake_size: int32
            b.write_all(&row.death_thud_shake_size.to_le_bytes())?;

            // sound_id: foreign_key (CreatureSoundData) int32
            b.write_all(&(row.sound_id.id as i32).to_le_bytes())?;

            // collision_width: float
            b.write_all(&row.collision_width.to_le_bytes())?;

            // collision_height: float
            b.write_all(&row.collision_height.to_le_bytes())?;

            // mount_height: float
            b.write_all(&row.mount_height.to_le_bytes())?;

            // geo_box_min_x: float
            b.write_all(&row.geo_box_min_x.to_le_bytes())?;

            // geo_box_min_y: float
            b.write_all(&row.geo_box_min_y.to_le_bytes())?;

            // geo_box_min_z: float
            b.write_all(&row.geo_box_min_z.to_le_bytes())?;

            // geo_box_max_x: float
            b.write_all(&row.geo_box_max_x.to_le_bytes())?;

            // geo_box_max_y: float
            b.write_all(&row.geo_box_max_y.to_le_bytes())?;

            // geo_box_max_z: float
            b.write_all(&row.geo_box_max_z.to_le_bytes())?;

            // world_effect_scale: float
            b.write_all(&row.world_effect_scale.to_le_bytes())?;

            // attached_effect_scale: float
            b.write_all(&row.attached_effect_scale.to_le_bytes())?;

            // missile_collision_radius: float
            b.write_all(&row.missile_collision_radius.to_le_bytes())?;

            // missile_collision_push: float
            b.write_all(&row.missile_collision_push.to_le_bytes())?;

            // missile_collision_raise: float
            b.write_all(&row.missile_collision_raise.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for CreatureModelData {
    type PrimaryKey = CreatureModelDataKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl CreatureModelData {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.model_name.is_empty() { b.write_all(row.model_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.model_name.is_empty() { sum += row.model_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstCreatureModelData<const S: usize> {
    pub rows: [ConstCreatureModelDataRow; S],
}

impl<const S: usize> ConstCreatureModelData<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 112 {
            panic!("invalid record size, expected 112")
        }

        if header.field_count != 28 {
            panic!("invalid field count, expected 28")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstCreatureModelDataRow {
                id: CreatureModelDataKey::new(0),
                flags: 0,
                model_name: "",
                size_class: 0,
                model_scale: 0.0,
                blood_id: UnitBloodKey::new(0),
                footprint_texture_id: FootprintTexturesKey::new(0),
                footprint_texture_length: 0.0,
                footprint_texture_width: 0.0,
                footprint_particle_scale: 0.0,
                foley_material_id: MaterialKey::new(0),
                footstep_shake_size: 0,
                death_thud_shake_size: 0,
                sound_id: CreatureSoundDataKey::new(0),
                collision_width: 0.0,
                collision_height: 0.0,
                mount_height: 0.0,
                geo_box_min_x: 0.0,
                geo_box_min_y: 0.0,
                geo_box_min_z: 0.0,
                geo_box_max_x: 0.0,
                geo_box_max_y: 0.0,
                geo_box_max_z: 0.0,
                world_effect_scale: 0.0,
                attached_effect_scale: 0.0,
                missile_collision_radius: 0.0,
                missile_collision_push: 0.0,
                missile_collision_raise: 0.0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (CreatureModelData) int32
            let id = CreatureModelDataKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // model_name: string_ref
            let model_name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // size_class: int32
            let size_class = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // model_scale: float
            let model_scale = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // blood_id: foreign_key (UnitBlood) int32
            let blood_id = UnitBloodKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // footprint_texture_id: foreign_key (FootprintTextures) int32
            let footprint_texture_id = FootprintTexturesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // footprint_texture_length: float
            let footprint_texture_length = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // footprint_texture_width: float
            let footprint_texture_width = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // footprint_particle_scale: float
            let footprint_particle_scale = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // foley_material_id: foreign_key (Material) int32
            let foley_material_id = MaterialKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // footstep_shake_size: int32
            let footstep_shake_size = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // death_thud_shake_size: int32
            let death_thud_shake_size = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // sound_id: foreign_key (CreatureSoundData) int32
            let sound_id = CreatureSoundDataKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // collision_width: float
            let collision_width = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // collision_height: float
            let collision_height = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mount_height: float
            let mount_height = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // geo_box_min_x: float
            let geo_box_min_x = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // geo_box_min_y: float
            let geo_box_min_y = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // geo_box_min_z: float
            let geo_box_min_z = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // geo_box_max_x: float
            let geo_box_max_x = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // geo_box_max_y: float
            let geo_box_max_y = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // geo_box_max_z: float
            let geo_box_max_z = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // world_effect_scale: float
            let world_effect_scale = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // attached_effect_scale: float
            let attached_effect_scale = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_collision_radius: float
            let missile_collision_radius = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_collision_push: float
            let missile_collision_push = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // missile_collision_raise: float
            let missile_collision_raise = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstCreatureModelDataRow {
                id,
                flags,
                model_name,
                size_class,
                model_scale,
                blood_id,
                footprint_texture_id,
                footprint_texture_length,
                footprint_texture_width,
                footprint_particle_scale,
                foley_material_id,
                footstep_shake_size,
                death_thud_shake_size,
                sound_id,
                collision_width,
                collision_height,
                mount_height,
                geo_box_min_x,
                geo_box_min_y,
                geo_box_min_z,
                geo_box_max_x,
                geo_box_max_y,
                geo_box_max_z,
                world_effect_scale,
                attached_effect_scale,
                missile_collision_radius,
                missile_collision_push,
                missile_collision_raise,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CreatureModelData {
        CreatureModelData {
            rows: self.rows.iter().map(|s| CreatureModelDataRow {
                id: s.id,
                flags: s.flags,
                model_name: s.model_name.to_string(),
                size_class: s.size_class,
                model_scale: s.model_scale,
                blood_id: s.blood_id,
                footprint_texture_id: s.footprint_texture_id,
                footprint_texture_length: s.footprint_texture_length,
                footprint_texture_width: s.footprint_texture_width,
                footprint_particle_scale: s.footprint_particle_scale,
                foley_material_id: s.foley_material_id,
                footstep_shake_size: s.footstep_shake_size,
                death_thud_shake_size: s.death_thud_shake_size,
                sound_id: s.sound_id,
                collision_width: s.collision_width,
                collision_height: s.collision_height,
                mount_height: s.mount_height,
                geo_box_min_x: s.geo_box_min_x,
                geo_box_min_y: s.geo_box_min_y,
                geo_box_min_z: s.geo_box_min_z,
                geo_box_max_x: s.geo_box_max_x,
                geo_box_max_y: s.geo_box_max_y,
                geo_box_max_z: s.geo_box_max_z,
                world_effect_scale: s.world_effect_scale,
                attached_effect_scale: s.attached_effect_scale,
                missile_collision_radius: s.missile_collision_radius,
                missile_collision_push: s.missile_collision_push,
                missile_collision_raise: s.missile_collision_raise,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CreatureModelDataKey {
    pub id: i32
}

impl CreatureModelDataKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for CreatureModelDataKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for CreatureModelDataKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for CreatureModelDataKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for CreatureModelDataKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for CreatureModelDataKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CreatureModelDataRow {
    pub id: CreatureModelDataKey,
    pub flags: i32,
    pub model_name: String,
    pub size_class: i32,
    pub model_scale: f32,
    pub blood_id: UnitBloodKey,
    pub footprint_texture_id: FootprintTexturesKey,
    pub footprint_texture_length: f32,
    pub footprint_texture_width: f32,
    pub footprint_particle_scale: f32,
    pub foley_material_id: MaterialKey,
    pub footstep_shake_size: i32,
    pub death_thud_shake_size: i32,
    pub sound_id: CreatureSoundDataKey,
    pub collision_width: f32,
    pub collision_height: f32,
    pub mount_height: f32,
    pub geo_box_min_x: f32,
    pub geo_box_min_y: f32,
    pub geo_box_min_z: f32,
    pub geo_box_max_x: f32,
    pub geo_box_max_y: f32,
    pub geo_box_max_z: f32,
    pub world_effect_scale: f32,
    pub attached_effect_scale: f32,
    pub missile_collision_radius: f32,
    pub missile_collision_push: f32,
    pub missile_collision_raise: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstCreatureModelDataRow {
    pub id: CreatureModelDataKey,
    pub flags: i32,
    pub model_name: &'static str,
    pub size_class: i32,
    pub model_scale: f32,
    pub blood_id: UnitBloodKey,
    pub footprint_texture_id: FootprintTexturesKey,
    pub footprint_texture_length: f32,
    pub footprint_texture_width: f32,
    pub footprint_particle_scale: f32,
    pub foley_material_id: MaterialKey,
    pub footstep_shake_size: i32,
    pub death_thud_shake_size: i32,
    pub sound_id: CreatureSoundDataKey,
    pub collision_width: f32,
    pub collision_height: f32,
    pub mount_height: f32,
    pub geo_box_min_x: f32,
    pub geo_box_min_y: f32,
    pub geo_box_min_z: f32,
    pub geo_box_max_x: f32,
    pub geo_box_max_y: f32,
    pub geo_box_max_z: f32,
    pub world_effect_scale: f32,
    pub attached_effect_scale: f32,
    pub missile_collision_radius: f32,
    pub missile_collision_push: f32,
    pub missile_collision_raise: f32,
}

