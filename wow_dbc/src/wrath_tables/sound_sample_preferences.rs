use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundSamplePreferences {
    pub rows: Vec<SoundSamplePreferencesRow>,
}

impl DbcTable for SoundSamplePreferences {
    type Row = SoundSamplePreferencesRow;

    const FILENAME: &'static str = "SoundSamplePreferences.dbc";
    const FIELD_COUNT: usize = 17;
    const ROW_SIZE: usize = 68;

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

            // id: primary_key (SoundSamplePreferences) int32
            let id = SoundSamplePreferencesKey::new(crate::util::read_i32_le(chunk)?);

            // field_0_6_0_3592_001: int32
            let field_0_6_0_3592_001 = crate::util::read_i32_le(chunk)?;

            // field_0_6_0_3592_002: int32
            let field_0_6_0_3592_002 = crate::util::read_i32_le(chunk)?;

            // e_a_x2_sample_room: int32
            let e_a_x2_sample_room = crate::util::read_i32_le(chunk)?;

            // field_0_6_0_3592_004: int32
            let field_0_6_0_3592_004 = crate::util::read_i32_le(chunk)?;

            // field_0_6_0_3592_005: int32
            let field_0_6_0_3592_005 = crate::util::read_i32_le(chunk)?;

            // field_0_6_0_3592_006: float
            let field_0_6_0_3592_006 = crate::util::read_f32_le(chunk)?;

            // field_0_6_0_3592_007: int32
            let field_0_6_0_3592_007 = crate::util::read_i32_le(chunk)?;

            // e_a_x2_sample_occlusion_l_f_ratio: float
            let e_a_x2_sample_occlusion_l_f_ratio = crate::util::read_f32_le(chunk)?;

            // e_a_x2_sample_occlusion_room_ratio: float
            let e_a_x2_sample_occlusion_room_ratio = crate::util::read_f32_le(chunk)?;

            // field_0_6_0_3592_010: int32
            let field_0_6_0_3592_010 = crate::util::read_i32_le(chunk)?;

            // e_a_x1_effect_level: float
            let e_a_x1_effect_level = crate::util::read_f32_le(chunk)?;

            // field_0_6_0_3592_012: int32
            let field_0_6_0_3592_012 = crate::util::read_i32_le(chunk)?;

            // field_0_6_0_3592_013: float
            let field_0_6_0_3592_013 = crate::util::read_f32_le(chunk)?;

            // e_a_x3_sample_exclusion: float
            let e_a_x3_sample_exclusion = crate::util::read_f32_le(chunk)?;

            // field_0_6_0_3592_015: float
            let field_0_6_0_3592_015 = crate::util::read_f32_le(chunk)?;

            // field_0_6_0_3592_016: int32
            let field_0_6_0_3592_016 = crate::util::read_i32_le(chunk)?;


            rows.push(SoundSamplePreferencesRow {
                id,
                field_0_6_0_3592_001,
                field_0_6_0_3592_002,
                e_a_x2_sample_room,
                field_0_6_0_3592_004,
                field_0_6_0_3592_005,
                field_0_6_0_3592_006,
                field_0_6_0_3592_007,
                e_a_x2_sample_occlusion_l_f_ratio,
                e_a_x2_sample_occlusion_room_ratio,
                field_0_6_0_3592_010,
                e_a_x1_effect_level,
                field_0_6_0_3592_012,
                field_0_6_0_3592_013,
                e_a_x3_sample_exclusion,
                field_0_6_0_3592_015,
                field_0_6_0_3592_016,
            });
        }

        Ok(SoundSamplePreferences { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SoundSamplePreferences) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // field_0_6_0_3592_001: int32
            b.write_all(&row.field_0_6_0_3592_001.to_le_bytes())?;

            // field_0_6_0_3592_002: int32
            b.write_all(&row.field_0_6_0_3592_002.to_le_bytes())?;

            // e_a_x2_sample_room: int32
            b.write_all(&row.e_a_x2_sample_room.to_le_bytes())?;

            // field_0_6_0_3592_004: int32
            b.write_all(&row.field_0_6_0_3592_004.to_le_bytes())?;

            // field_0_6_0_3592_005: int32
            b.write_all(&row.field_0_6_0_3592_005.to_le_bytes())?;

            // field_0_6_0_3592_006: float
            b.write_all(&row.field_0_6_0_3592_006.to_le_bytes())?;

            // field_0_6_0_3592_007: int32
            b.write_all(&row.field_0_6_0_3592_007.to_le_bytes())?;

            // e_a_x2_sample_occlusion_l_f_ratio: float
            b.write_all(&row.e_a_x2_sample_occlusion_l_f_ratio.to_le_bytes())?;

            // e_a_x2_sample_occlusion_room_ratio: float
            b.write_all(&row.e_a_x2_sample_occlusion_room_ratio.to_le_bytes())?;

            // field_0_6_0_3592_010: int32
            b.write_all(&row.field_0_6_0_3592_010.to_le_bytes())?;

            // e_a_x1_effect_level: float
            b.write_all(&row.e_a_x1_effect_level.to_le_bytes())?;

            // field_0_6_0_3592_012: int32
            b.write_all(&row.field_0_6_0_3592_012.to_le_bytes())?;

            // field_0_6_0_3592_013: float
            b.write_all(&row.field_0_6_0_3592_013.to_le_bytes())?;

            // e_a_x3_sample_exclusion: float
            b.write_all(&row.e_a_x3_sample_exclusion.to_le_bytes())?;

            // field_0_6_0_3592_015: float
            b.write_all(&row.field_0_6_0_3592_015.to_le_bytes())?;

            // field_0_6_0_3592_016: int32
            b.write_all(&row.field_0_6_0_3592_016.to_le_bytes())?;

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

impl Indexable for SoundSamplePreferences {
    type PrimaryKey = SoundSamplePreferencesKey;
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
pub struct SoundSamplePreferencesKey {
    pub id: i32
}

impl SoundSamplePreferencesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for SoundSamplePreferencesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for SoundSamplePreferencesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for SoundSamplePreferencesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for SoundSamplePreferencesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for SoundSamplePreferencesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for SoundSamplePreferencesKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SoundSamplePreferencesKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for SoundSamplePreferencesKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SoundSamplePreferencesKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SoundSamplePreferencesKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundSamplePreferencesRow {
    pub id: SoundSamplePreferencesKey,
    pub field_0_6_0_3592_001: i32,
    pub field_0_6_0_3592_002: i32,
    pub e_a_x2_sample_room: i32,
    pub field_0_6_0_3592_004: i32,
    pub field_0_6_0_3592_005: i32,
    pub field_0_6_0_3592_006: f32,
    pub field_0_6_0_3592_007: i32,
    pub e_a_x2_sample_occlusion_l_f_ratio: f32,
    pub e_a_x2_sample_occlusion_room_ratio: f32,
    pub field_0_6_0_3592_010: i32,
    pub e_a_x1_effect_level: f32,
    pub field_0_6_0_3592_012: i32,
    pub field_0_6_0_3592_013: f32,
    pub e_a_x3_sample_exclusion: f32,
    pub field_0_6_0_3592_015: f32,
    pub field_0_6_0_3592_016: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn sound_sample_preferences() {
        let mut file = File::open("../wrath-dbc/SoundSamplePreferences.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SoundSamplePreferences::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SoundSamplePreferences::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
