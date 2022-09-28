use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::creature_display_info_extra::*;
use crate::wrath_tables::creature_model_data::*;
use crate::wrath_tables::creature_sound_data::*;
use crate::wrath_tables::npc_sounds::*;
use crate::wrath_tables::object_effect_package::*;
use crate::wrath_tables::particle_color::*;
use crate::wrath_tables::unit_blood::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CreatureDisplayInfo {
    pub rows: Vec<CreatureDisplayInfoRow>,
}

impl DbcTable for CreatureDisplayInfo {
    type Row = CreatureDisplayInfoRow;

    fn filename() -> &'static str { "CreatureDisplayInfo.dbc" }

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

            // id: primary_key (CreatureDisplayInfo) int32
            let id = CreatureDisplayInfoKey::new(crate::util::read_i32_le(chunk)?);

            // model_id: foreign_key (CreatureModelData) int32
            let model_id = CreatureModelDataKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_id: foreign_key (CreatureSoundData) int32
            let sound_id = CreatureSoundDataKey::new(crate::util::read_i32_le(chunk)?.into());

            // extended_display_info_id: foreign_key (CreatureDisplayInfoExtra) int32
            let extended_display_info_id = CreatureDisplayInfoExtraKey::new(crate::util::read_i32_le(chunk)?.into());

            // creature_model_scale: float
            let creature_model_scale = crate::util::read_f32_le(chunk)?;

            // creature_model_alpha: int32
            let creature_model_alpha = crate::util::read_i32_le(chunk)?;

            // texture_variation: string_ref[3]
            let texture_variation = {
                let mut arr = Vec::with_capacity(3);
                for _ in 0..3 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // portrait_texture_name: string_ref
            let portrait_texture_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // size_class: int32
            let size_class = crate::util::read_i32_le(chunk)?;

            // blood_id: foreign_key (UnitBlood) int32
            let blood_id = UnitBloodKey::new(crate::util::read_i32_le(chunk)?.into());

            // n_p_c_sound_id: foreign_key (NPCSounds) int32
            let n_p_c_sound_id = NPCSoundsKey::new(crate::util::read_i32_le(chunk)?.into());

            // particle_color_id: foreign_key (ParticleColor) int32
            let particle_color_id = ParticleColorKey::new(crate::util::read_i32_le(chunk)?.into());

            // creature_geoset_data: int32
            let creature_geoset_data = crate::util::read_i32_le(chunk)?;

            // object_effect_package_id: foreign_key (ObjectEffectPackage) int32
            let object_effect_package_id = ObjectEffectPackageKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(CreatureDisplayInfoRow {
                id,
                model_id,
                sound_id,
                extended_display_info_id,
                creature_model_scale,
                creature_model_alpha,
                texture_variation,
                portrait_texture_name,
                size_class,
                blood_id,
                n_p_c_sound_id,
                particle_color_id,
                creature_geoset_data,
                object_effect_package_id,
            });
        }

        Ok(CreatureDisplayInfo { rows, })
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
            // id: primary_key (CreatureDisplayInfo) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model_id: foreign_key (CreatureModelData) int32
            b.write_all(&(row.model_id.id as i32).to_le_bytes())?;

            // sound_id: foreign_key (CreatureSoundData) int32
            b.write_all(&(row.sound_id.id as i32).to_le_bytes())?;

            // extended_display_info_id: foreign_key (CreatureDisplayInfoExtra) int32
            b.write_all(&(row.extended_display_info_id.id as i32).to_le_bytes())?;

            // creature_model_scale: float
            b.write_all(&row.creature_model_scale.to_le_bytes())?;

            // creature_model_alpha: int32
            b.write_all(&row.creature_model_alpha.to_le_bytes())?;

            // texture_variation: string_ref[3]
            for i in &row.texture_variation {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // portrait_texture_name: string_ref
            if !row.portrait_texture_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.portrait_texture_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // size_class: int32
            b.write_all(&row.size_class.to_le_bytes())?;

            // blood_id: foreign_key (UnitBlood) int32
            b.write_all(&(row.blood_id.id as i32).to_le_bytes())?;

            // n_p_c_sound_id: foreign_key (NPCSounds) int32
            b.write_all(&(row.n_p_c_sound_id.id as i32).to_le_bytes())?;

            // particle_color_id: foreign_key (ParticleColor) int32
            b.write_all(&(row.particle_color_id.id as i32).to_le_bytes())?;

            // creature_geoset_data: int32
            b.write_all(&row.creature_geoset_data.to_le_bytes())?;

            // object_effect_package_id: foreign_key (ObjectEffectPackage) int32
            b.write_all(&(row.object_effect_package_id.id as i32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for CreatureDisplayInfo {
    type PrimaryKey = CreatureDisplayInfoKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl CreatureDisplayInfo {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            for s in &row.texture_variation {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

            if !row.portrait_texture_name.is_empty() { b.write_all(row.portrait_texture_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            for s in &row.texture_variation {
                if !s.is_empty() { sum += s.len() + 1; };
            }

            if !row.portrait_texture_name.is_empty() { sum += row.portrait_texture_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct CreatureDisplayInfoKey {
    pub id: i32
}

impl CreatureDisplayInfoKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatureDisplayInfoRow {
    pub id: CreatureDisplayInfoKey,
    pub model_id: CreatureModelDataKey,
    pub sound_id: CreatureSoundDataKey,
    pub extended_display_info_id: CreatureDisplayInfoExtraKey,
    pub creature_model_scale: f32,
    pub creature_model_alpha: i32,
    pub texture_variation: [String; 3],
    pub portrait_texture_name: String,
    pub size_class: i32,
    pub blood_id: UnitBloodKey,
    pub n_p_c_sound_id: NPCSoundsKey,
    pub particle_color_id: ParticleColorKey,
    pub creature_geoset_data: i32,
    pub object_effect_package_id: ObjectEffectPackageKey,
}

