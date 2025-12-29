use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::creature_display_info_extra::CreatureDisplayInfoExtraKey;
use crate::vanilla_tables::creature_model_data::CreatureModelDataKey;
use crate::vanilla_tables::creature_sound_data::CreatureSoundDataKey;
use crate::vanilla_tables::npc_sounds::NPCSoundsKey;
use crate::vanilla_tables::unit_blood::UnitBloodKey;
use std::io::Write;
pub use wow_world_base::vanilla::SizeClass;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureDisplayInfo {
    pub rows: Vec<CreatureDisplayInfoRow>,
}

impl DbcTable for CreatureDisplayInfo {
    type Row = CreatureDisplayInfoRow;

    const FILENAME: &'static str = "CreatureDisplayInfo.dbc";
    const FIELD_COUNT: usize = 12;
    const ROW_SIZE: usize = 48;

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

            // id: primary_key (CreatureDisplayInfo) uint32
            let id = CreatureDisplayInfoKey::new(crate::util::read_u32_le(chunk)?);

            // model: foreign_key (CreatureModelData) uint32
            let model = CreatureModelDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound: foreign_key (CreatureSoundData) uint32
            let sound = CreatureSoundDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // extended_display_info: foreign_key (CreatureDisplayInfoExtra) uint32
            let extended_display_info = CreatureDisplayInfoExtraKey::new(crate::util::read_u32_le(chunk)?.into());

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

            // size: SizeClass
            let size = crate::util::read_i32_le(chunk)?.try_into()?;

            // blood: foreign_key (UnitBlood) uint32
            let blood = UnitBloodKey::new(crate::util::read_u32_le(chunk)?.into());

            // npc_sound: foreign_key (NPCSounds) uint32
            let npc_sound = NPCSoundsKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(CreatureDisplayInfoRow {
                id,
                model,
                sound,
                extended_display_info,
                creature_model_scale,
                creature_model_alpha,
                texture_variation,
                size,
                blood,
                npc_sound,
            });
        }

        Ok(CreatureDisplayInfo { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CreatureDisplayInfo) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model: foreign_key (CreatureModelData) uint32
            b.write_all(&(row.model.id as u32).to_le_bytes())?;

            // sound: foreign_key (CreatureSoundData) uint32
            b.write_all(&(row.sound.id as u32).to_le_bytes())?;

            // extended_display_info: foreign_key (CreatureDisplayInfoExtra) uint32
            b.write_all(&(row.extended_display_info.id as u32).to_le_bytes())?;

            // creature_model_scale: float
            b.write_all(&row.creature_model_scale.to_le_bytes())?;

            // creature_model_alpha: int32
            b.write_all(&row.creature_model_alpha.to_le_bytes())?;

            // texture_variation: string_ref[3]
            for i in &row.texture_variation {
                b.write_all(&string_cache.add_string(i).to_le_bytes())?;
            }


            // size: SizeClass
            b.write_all(&(row.size.as_int() as i32).to_le_bytes())?;

            // blood: foreign_key (UnitBlood) uint32
            b.write_all(&(row.blood.id as u32).to_le_bytes())?;

            // npc_sound: foreign_key (NPCSounds) uint32
            b.write_all(&(row.npc_sound.id as u32).to_le_bytes())?;

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

impl Indexable for CreatureDisplayInfo {
    type PrimaryKey = CreatureDisplayInfoKey;
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
pub struct CreatureDisplayInfoKey {
    pub id: u32
}

impl CreatureDisplayInfoKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
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

impl From<u32> for CreatureDisplayInfoKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for CreatureDisplayInfoKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for CreatureDisplayInfoKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for CreatureDisplayInfoKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for CreatureDisplayInfoKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for CreatureDisplayInfoKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for CreatureDisplayInfoKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for CreatureDisplayInfoKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureDisplayInfoRow {
    pub id: CreatureDisplayInfoKey,
    pub model: CreatureModelDataKey,
    pub sound: CreatureSoundDataKey,
    pub extended_display_info: CreatureDisplayInfoExtraKey,
    pub creature_model_scale: f32,
    pub creature_model_alpha: i32,
    pub texture_variation: [String; 3],
    pub size: SizeClass,
    pub blood: UnitBloodKey,
    pub npc_sound: NPCSoundsKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn creature_display_info() {
        let mut file = File::open("../vanilla-dbc/CreatureDisplayInfo.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CreatureDisplayInfo::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CreatureDisplayInfo::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
