use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::spell_visual_effect_name::SpellVisualEffectNameKey;
use crate::wrath_tables::spell_visual_kit::SpellVisualKitKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellVisualKitModelAttach {
    pub rows: Vec<SpellVisualKitModelAttachRow>,
}

impl DbcTable for SpellVisualKitModelAttach {
    type Row = SpellVisualKitModelAttachRow;

    const FILENAME: &'static str = "SpellVisualKitModelAttach.dbc";
    const FIELD_COUNT: usize = 10;
    const ROW_SIZE: usize = 40;

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

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SpellVisualKitModelAttach) int32
            let id = SpellVisualKitModelAttachKey::new(crate::util::read_i32_le(chunk)?);

            // parent_spell_visual_kit_id: foreign_key (SpellVisualKit) int32
            let parent_spell_visual_kit_id = SpellVisualKitKey::new(crate::util::read_i32_le(chunk)?.into());

            // spell_visual_effect_name_id: foreign_key (SpellVisualEffectName) int32
            let spell_visual_effect_name_id = SpellVisualEffectNameKey::new(crate::util::read_i32_le(chunk)?.into());

            // attachment_id: int32
            let attachment_id = crate::util::read_i32_le(chunk)?;

            // offset: float[3]
            let offset = crate::util::read_array_f32::<3>(chunk)?;

            // yaw: float
            let yaw = crate::util::read_f32_le(chunk)?;

            // pitch: float
            let pitch = crate::util::read_f32_le(chunk)?;

            // roll: float
            let roll = crate::util::read_f32_le(chunk)?;


            rows.push(SpellVisualKitModelAttachRow {
                id,
                parent_spell_visual_kit_id,
                spell_visual_effect_name_id,
                attachment_id,
                offset,
                yaw,
                pitch,
                roll,
            });
        }

        Ok(SpellVisualKitModelAttach { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SpellVisualKitModelAttach) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // parent_spell_visual_kit_id: foreign_key (SpellVisualKit) int32
            b.write_all(&(row.parent_spell_visual_kit_id.id as i32).to_le_bytes())?;

            // spell_visual_effect_name_id: foreign_key (SpellVisualEffectName) int32
            b.write_all(&(row.spell_visual_effect_name_id.id as i32).to_le_bytes())?;

            // attachment_id: int32
            b.write_all(&row.attachment_id.to_le_bytes())?;

            // offset: float[3]
            for i in row.offset {
                b.write_all(&i.to_le_bytes())?;
            }


            // yaw: float
            b.write_all(&row.yaw.to_le_bytes())?;

            // pitch: float
            b.write_all(&row.pitch.to_le_bytes())?;

            // roll: float
            b.write_all(&row.roll.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellVisualKitModelAttach {
    type PrimaryKey = SpellVisualKitModelAttachKey;
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
pub struct SpellVisualKitModelAttachKey {
    pub id: i32
}

impl SpellVisualKitModelAttachKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for SpellVisualKitModelAttachKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for SpellVisualKitModelAttachKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for SpellVisualKitModelAttachKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for SpellVisualKitModelAttachKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for SpellVisualKitModelAttachKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for SpellVisualKitModelAttachKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SpellVisualKitModelAttachKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for SpellVisualKitModelAttachKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SpellVisualKitModelAttachKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SpellVisualKitModelAttachKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellVisualKitModelAttachRow {
    pub id: SpellVisualKitModelAttachKey,
    pub parent_spell_visual_kit_id: SpellVisualKitKey,
    pub spell_visual_effect_name_id: SpellVisualEffectNameKey,
    pub attachment_id: i32,
    pub offset: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell_visual_kit_model_attach() {
        let mut file = File::open("../wrath-dbc/SpellVisualKitModelAttach.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SpellVisualKitModelAttach::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellVisualKitModelAttach::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
