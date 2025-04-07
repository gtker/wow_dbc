use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::light::LightKey;
use crate::wrath_tables::liquid_material::LiquidMaterialKey;
use crate::wrath_tables::sound_entries::SoundEntriesKey;
use crate::wrath_tables::spell::SpellKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidType {
    pub rows: Vec<LiquidTypeRow>,
}

impl DbcTable for LiquidType {
    type Row = LiquidTypeRow;

    const FILENAME: &'static str = "LiquidType.dbc";
    const FIELD_COUNT: usize = 45;
    const ROW_SIZE: usize = 180;

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

            // id: primary_key (LiquidType) int32
            let id = LiquidTypeKey::new(crate::util::read_i32_le(chunk)?);

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // sound_bank: int32
            let sound_bank = crate::util::read_i32_le(chunk)?;

            // sound_id: foreign_key (SoundEntries) int32
            let sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // spell_id: foreign_key (Spell) int32
            let spell_id = SpellKey::new(crate::util::read_i32_le(chunk)?.into());

            // max_darken_depth: float
            let max_darken_depth = crate::util::read_f32_le(chunk)?;

            // fog_darken_intensity: float
            let fog_darken_intensity = crate::util::read_f32_le(chunk)?;

            // amb_darken_intensity: float
            let amb_darken_intensity = crate::util::read_f32_le(chunk)?;

            // dir_darken_intensity: float
            let dir_darken_intensity = crate::util::read_f32_le(chunk)?;

            // light_id: foreign_key (Light) int32
            let light_id = LightKey::new(crate::util::read_i32_le(chunk)?.into());

            // particle_scale: float
            let particle_scale = crate::util::read_f32_le(chunk)?;

            // particle_movement: int32
            let particle_movement = crate::util::read_i32_le(chunk)?;

            // particle_tex_slots: int32
            let particle_tex_slots = crate::util::read_i32_le(chunk)?;

            // material_id: foreign_key (LiquidMaterial) int32
            let material_id = LiquidMaterialKey::new(crate::util::read_i32_le(chunk)?.into());

            // texture: string_ref[6]
            let texture = {
                let mut arr = Vec::with_capacity(6);
                for _ in 0..6 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // color: int32[2]
            let color = crate::util::read_array_i32::<2>(chunk)?;

            // float: float[18]
            let float = crate::util::read_array_f32::<18>(chunk)?;

            // int: int32[4]
            let int = crate::util::read_array_i32::<4>(chunk)?;


            rows.push(LiquidTypeRow {
                id,
                name,
                flags,
                sound_bank,
                sound_id,
                spell_id,
                max_darken_depth,
                fog_darken_intensity,
                amb_darken_intensity,
                dir_darken_intensity,
                light_id,
                particle_scale,
                particle_movement,
                particle_tex_slots,
                material_id,
                texture,
                color,
                float,
                int,
            });
        }

        Ok(LiquidType { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (LiquidType) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref
            b.write_all(&string_cache.add_string(&row.name).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // sound_bank: int32
            b.write_all(&row.sound_bank.to_le_bytes())?;

            // sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_id.id as i32).to_le_bytes())?;

            // spell_id: foreign_key (Spell) int32
            b.write_all(&(row.spell_id.id as i32).to_le_bytes())?;

            // max_darken_depth: float
            b.write_all(&row.max_darken_depth.to_le_bytes())?;

            // fog_darken_intensity: float
            b.write_all(&row.fog_darken_intensity.to_le_bytes())?;

            // amb_darken_intensity: float
            b.write_all(&row.amb_darken_intensity.to_le_bytes())?;

            // dir_darken_intensity: float
            b.write_all(&row.dir_darken_intensity.to_le_bytes())?;

            // light_id: foreign_key (Light) int32
            b.write_all(&(row.light_id.id as i32).to_le_bytes())?;

            // particle_scale: float
            b.write_all(&row.particle_scale.to_le_bytes())?;

            // particle_movement: int32
            b.write_all(&row.particle_movement.to_le_bytes())?;

            // particle_tex_slots: int32
            b.write_all(&row.particle_tex_slots.to_le_bytes())?;

            // material_id: foreign_key (LiquidMaterial) int32
            b.write_all(&(row.material_id.id as i32).to_le_bytes())?;

            // texture: string_ref[6]
            for i in &row.texture {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // color: int32[2]
            for i in row.color {
                b.write_all(&i.to_le_bytes())?;
            }


            // float: float[18]
            for i in row.float {
                b.write_all(&i.to_le_bytes())?;
            }


            // int: int32[4]
            for i in row.int {
                b.write_all(&i.to_le_bytes())?;
            }


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

impl Indexable for LiquidType {
    type PrimaryKey = LiquidTypeKey;
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
pub struct LiquidTypeKey {
    pub id: i32
}

impl LiquidTypeKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for LiquidTypeKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for LiquidTypeKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for LiquidTypeKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for LiquidTypeKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for LiquidTypeKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for LiquidTypeKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for LiquidTypeKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for LiquidTypeKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for LiquidTypeKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for LiquidTypeKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiquidTypeRow {
    pub id: LiquidTypeKey,
    pub name: String,
    pub flags: i32,
    pub sound_bank: i32,
    pub sound_id: SoundEntriesKey,
    pub spell_id: SpellKey,
    pub max_darken_depth: f32,
    pub fog_darken_intensity: f32,
    pub amb_darken_intensity: f32,
    pub dir_darken_intensity: f32,
    pub light_id: LightKey,
    pub particle_scale: f32,
    pub particle_movement: i32,
    pub particle_tex_slots: i32,
    pub material_id: LiquidMaterialKey,
    pub texture: [String; 6],
    pub color: [i32; 2],
    pub float: [f32; 18],
    pub int: [i32; 4],
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn liquid_type() {
        let mut file = File::open("../wrath-dbc/LiquidType.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = LiquidType::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = LiquidType::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
