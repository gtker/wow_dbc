use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::creature_sound_data::CreatureSoundDataKey;
use crate::tbc_tables::footprint_textures::FootprintTexturesKey;
use crate::tbc_tables::material::MaterialKey;
use crate::tbc_tables::unit_blood::UnitBloodKey;
use crate::util::StringCache;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureModelData {
    pub rows: Vec<CreatureModelDataRow>,
}

impl DbcTable for CreatureModelData {
    type Row = CreatureModelDataRow;

    const FILENAME: &'static str = "CreatureModelData.dbc";
    const FIELD_COUNT: usize = 24;
    const ROW_SIZE: usize = 96;

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

            // geo_box: float[6]
            let geo_box = crate::util::read_array_f32::<6>(chunk)?;

            // attached_effect_scale: float
            let attached_effect_scale = crate::util::read_f32_le(chunk)?;


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
                geo_box,
                attached_effect_scale,
            });
        }

        Ok(CreatureModelData { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CreatureModelData) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // model_name: string_ref
            b.write_all(&string_cache.add_string(&row.model_name).to_le_bytes())?;

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

            // geo_box: float[6]
            for i in row.geo_box {
                b.write_all(&i.to_le_bytes())?;
            }


            // attached_effect_scale: float
            b.write_all(&row.attached_effect_scale.to_le_bytes())?;

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

impl Indexable for CreatureModelData {
    type PrimaryKey = CreatureModelDataKey;
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
pub struct CreatureModelDataKey {
    pub id: i32
}

impl CreatureModelDataKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

impl TryFrom<u32> for CreatureModelDataKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for CreatureModelDataKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for CreatureModelDataKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for CreatureModelDataKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for CreatureModelDataKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    pub geo_box: [f32; 6],
    pub attached_effect_scale: f32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn creature_model_data() {
        let mut file = File::open("../tbc-dbc/CreatureModelData.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CreatureModelData::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CreatureModelData::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
