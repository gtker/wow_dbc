use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::SizeClass;
use crate::vanilla_tables::footprint_textures::*;
use crate::vanilla_tables::unit_blood::*;

#[derive(Debug, Clone, PartialEq)]
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

        if header.record_size != 64 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 64,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 16,
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

            // id: primary_key (CreatureModelData) uint32
            let id = CreatureModelDataKey::new(crate::util::read_u32_le(chunk)?);

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // model_path: string_ref
            let model_path = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // size: SizeClass
            let size = SizeClass::try_from(crate::util::read_i32_le(chunk)?)?;

            // model_scale: float
            let model_scale = crate::util::read_f32_le(chunk)?;

            // blood: foreign_key (UnitBlood) uint32
            let blood = UnitBloodKey::new(crate::util::read_u32_le(chunk)?.into());

            // footprint_texture: foreign_key (FootprintTextures) uint32
            let footprint_texture = FootprintTexturesKey::new(crate::util::read_u32_le(chunk)?.into());

            // footprint_texture_length: float
            let footprint_texture_length = crate::util::read_f32_le(chunk)?;

            // footprint_texture_width: float
            let footprint_texture_width = crate::util::read_f32_le(chunk)?;

            // footprint_texture_scale: float
            let footprint_texture_scale = crate::util::read_f32_le(chunk)?;

            // foley_material: int32
            let foley_material = crate::util::read_i32_le(chunk)?;

            // footstep_shake_size: int32
            let footstep_shake_size = crate::util::read_i32_le(chunk)?;

            // death_thud_shake_size: int32
            let death_thud_shake_size = crate::util::read_i32_le(chunk)?;

            // collision_width: float
            let collision_width = crate::util::read_f32_le(chunk)?;

            // collision_height: float
            let collision_height = crate::util::read_f32_le(chunk)?;

            // mount_height: float
            let mount_height = crate::util::read_f32_le(chunk)?;


            rows.push(CreatureModelDataRow {
                id,
                flags,
                model_path,
                size,
                model_scale,
                blood,
                footprint_texture,
                footprint_texture_length,
                footprint_texture_width,
                footprint_texture_scale,
                foley_material,
                footstep_shake_size,
                death_thud_shake_size,
                collision_width,
                collision_height,
                mount_height,
            });
        }

        Ok(CreatureModelData { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 16,
            record_size: 64,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (CreatureModelData) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // model_path: string_ref
            if !row.model_path.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.model_path.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // size: SizeClass
            b.write_all(&(row.size.as_int() as i32).to_le_bytes())?;

            // model_scale: float
            b.write_all(&row.model_scale.to_le_bytes())?;

            // blood: foreign_key (UnitBlood) uint32
            b.write_all(&(row.blood.id as u32).to_le_bytes())?;

            // footprint_texture: foreign_key (FootprintTextures) uint32
            b.write_all(&(row.footprint_texture.id as u32).to_le_bytes())?;

            // footprint_texture_length: float
            b.write_all(&row.footprint_texture_length.to_le_bytes())?;

            // footprint_texture_width: float
            b.write_all(&row.footprint_texture_width.to_le_bytes())?;

            // footprint_texture_scale: float
            b.write_all(&row.footprint_texture_scale.to_le_bytes())?;

            // foley_material: int32
            b.write_all(&row.foley_material.to_le_bytes())?;

            // footstep_shake_size: int32
            b.write_all(&row.footstep_shake_size.to_le_bytes())?;

            // death_thud_shake_size: int32
            b.write_all(&row.death_thud_shake_size.to_le_bytes())?;

            // collision_width: float
            b.write_all(&row.collision_width.to_le_bytes())?;

            // collision_height: float
            b.write_all(&row.collision_height.to_le_bytes())?;

            // mount_height: float
            b.write_all(&row.mount_height.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for CreatureModelData {
    type PrimaryKey = CreatureModelDataKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl CreatureModelData {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.model_path.is_empty() { b.write_all(row.model_path.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.model_path.is_empty() { sum += row.model_path.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct CreatureModelDataKey {
    pub id: u32
}

impl CreatureModelDataKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatureModelDataRow {
    pub id: CreatureModelDataKey,
    pub flags: i32,
    pub model_path: String,
    pub size: SizeClass,
    pub model_scale: f32,
    pub blood: UnitBloodKey,
    pub footprint_texture: FootprintTexturesKey,
    pub footprint_texture_length: f32,
    pub footprint_texture_width: f32,
    pub footprint_texture_scale: f32,
    pub foley_material: i32,
    pub footstep_shake_size: i32,
    pub death_thud_shake_size: i32,
    pub collision_width: f32,
    pub collision_height: f32,
    pub mount_height: f32,
}

