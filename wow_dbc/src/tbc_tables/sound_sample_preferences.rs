use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundSamplePreferences {
    pub rows: Vec<SoundSamplePreferencesRow>,
}

impl DbcTable for SoundSamplePreferences {
    type Row = SoundSamplePreferencesRow;

    fn filename() -> &'static str { "SoundSamplePreferences.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 68 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 68,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 17 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 17,
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

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 17,
            record_size: 68,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

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

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SoundSamplePreferences {
    type PrimaryKey = SoundSamplePreferencesKey;
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
pub struct SoundSamplePreferencesKey {
    pub id: i32
}

impl SoundSamplePreferencesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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

