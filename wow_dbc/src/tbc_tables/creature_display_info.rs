use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::creature_display_info_extra::*;
use crate::tbc_tables::creature_model_data::*;
use crate::tbc_tables::creature_sound_data::*;
use crate::tbc_tables::npc_sounds::*;
use crate::tbc_tables::particle_color::*;
use crate::tbc_tables::unit_blood::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

        if header.record_size != 56 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 56,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 14 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 14,
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
            });
        }

        Ok(CreatureDisplayInfo { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 14,
            record_size: 56,
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

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for CreatureDisplayInfo {
    type PrimaryKey = CreatureDisplayInfoKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstCreatureDisplayInfo<const S: usize> {
    pub rows: [ConstCreatureDisplayInfoRow; S],
}

impl<const S: usize> ConstCreatureDisplayInfo<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 56 {
            panic!("invalid record size, expected 56")
        }

        if header.field_count != 14 {
            panic!("invalid field count, expected 14")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstCreatureDisplayInfoRow {
                id: CreatureDisplayInfoKey::new(0),
                model_id: CreatureModelDataKey::new(0),
                sound_id: CreatureSoundDataKey::new(0),
                extended_display_info_id: CreatureDisplayInfoExtraKey::new(0),
                creature_model_scale: 0.0,
                creature_model_alpha: 0,
                texture_variation: [""; 3],
                portrait_texture_name: "",
                size_class: 0,
                blood_id: UnitBloodKey::new(0),
                n_p_c_sound_id: NPCSoundsKey::new(0),
                particle_color_id: ParticleColorKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (CreatureDisplayInfo) int32
            let id = CreatureDisplayInfoKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // model_id: foreign_key (CreatureModelData) int32
            let model_id = CreatureModelDataKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_id: foreign_key (CreatureSoundData) int32
            let sound_id = CreatureSoundDataKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // extended_display_info_id: foreign_key (CreatureDisplayInfoExtra) int32
            let extended_display_info_id = CreatureDisplayInfoExtraKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // creature_model_scale: float
            let creature_model_scale = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // creature_model_alpha: int32
            let creature_model_alpha = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // texture_variation: string_ref[3]
            let texture_variation = {
                let mut a = [""; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::get_string_from_block(b_offset, b, string_block);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // portrait_texture_name: string_ref
            let portrait_texture_name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // size_class: int32
            let size_class = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // blood_id: foreign_key (UnitBlood) int32
            let blood_id = UnitBloodKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // n_p_c_sound_id: foreign_key (NPCSounds) int32
            let n_p_c_sound_id = NPCSoundsKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // particle_color_id: foreign_key (ParticleColor) int32
            let particle_color_id = ParticleColorKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = ConstCreatureDisplayInfoRow {
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
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CreatureDisplayInfo {
        CreatureDisplayInfo {
            rows: self.rows.iter().map(|s| CreatureDisplayInfoRow {
                id: s.id,
                model_id: s.model_id,
                sound_id: s.sound_id,
                extended_display_info_id: s.extended_display_info_id,
                creature_model_scale: s.creature_model_scale,
                creature_model_alpha: s.creature_model_alpha,
                texture_variation: s.texture_variation.map(|a| a.to_string()),
                portrait_texture_name: s.portrait_texture_name.to_string(),
                size_class: s.size_class,
                blood_id: s.blood_id,
                n_p_c_sound_id: s.n_p_c_sound_id,
                particle_color_id: s.particle_color_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CreatureDisplayInfoKey {
    pub id: i32
}

impl CreatureDisplayInfoKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for CreatureDisplayInfoKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for CreatureDisplayInfoKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for CreatureDisplayInfoKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for CreatureDisplayInfoKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for CreatureDisplayInfoKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstCreatureDisplayInfoRow {
    pub id: CreatureDisplayInfoKey,
    pub model_id: CreatureModelDataKey,
    pub sound_id: CreatureSoundDataKey,
    pub extended_display_info_id: CreatureDisplayInfoExtraKey,
    pub creature_model_scale: f32,
    pub creature_model_alpha: i32,
    pub texture_variation: [&'static str; 3],
    pub portrait_texture_name: &'static str,
    pub size_class: i32,
    pub blood_id: UnitBloodKey,
    pub n_p_c_sound_id: NPCSoundsKey,
    pub particle_color_id: ParticleColorKey,
}

