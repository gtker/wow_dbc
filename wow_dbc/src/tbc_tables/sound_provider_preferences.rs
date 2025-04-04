use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundProviderPreferences {
    pub rows: Vec<SoundProviderPreferencesRow>,
}

impl DbcTable for SoundProviderPreferences {
    type Row = SoundProviderPreferencesRow;

    const FILENAME: &'static str = "SoundProviderPreferences.dbc";
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

            // id: primary_key (SoundProviderPreferences) int32
            let id = SoundProviderPreferencesKey::new(crate::util::read_i32_le(chunk)?);

            // description: string_ref
            let description = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // e_a_x_environment_selection: int32
            let e_a_x_environment_selection = crate::util::read_i32_le(chunk)?;

            // e_a_x_decay_time: float
            let e_a_x_decay_time = crate::util::read_f32_le(chunk)?;

            // e_a_x2_environment_size: float
            let e_a_x2_environment_size = crate::util::read_f32_le(chunk)?;

            // e_a_x2_environment_diffusion: float
            let e_a_x2_environment_diffusion = crate::util::read_f32_le(chunk)?;

            // e_a_x2_room: int32
            let e_a_x2_room = crate::util::read_i32_le(chunk)?;

            // e_a_x2_room_h_f: int32
            let e_a_x2_room_h_f = crate::util::read_i32_le(chunk)?;

            // e_a_x2_decay_h_f_ratio: float
            let e_a_x2_decay_h_f_ratio = crate::util::read_f32_le(chunk)?;

            // e_a_x2_reflections: int32
            let e_a_x2_reflections = crate::util::read_i32_le(chunk)?;

            // e_a_x2_reflections_delay: float
            let e_a_x2_reflections_delay = crate::util::read_f32_le(chunk)?;

            // e_a_x2_reverb: int32
            let e_a_x2_reverb = crate::util::read_i32_le(chunk)?;

            // e_a_x2_reverb_delay: float
            let e_a_x2_reverb_delay = crate::util::read_f32_le(chunk)?;

            // e_a_x2_room_rolloff: float
            let e_a_x2_room_rolloff = crate::util::read_f32_le(chunk)?;

            // e_a_x2_air_absorption: float
            let e_a_x2_air_absorption = crate::util::read_f32_le(chunk)?;

            // e_a_x3_room_l_f: int32
            let e_a_x3_room_l_f = crate::util::read_i32_le(chunk)?;

            // e_a_x3_decay_l_f_ratio: float
            let e_a_x3_decay_l_f_ratio = crate::util::read_f32_le(chunk)?;

            // e_a_x3_echo_time: float
            let e_a_x3_echo_time = crate::util::read_f32_le(chunk)?;

            // e_a_x3_echo_depth: float
            let e_a_x3_echo_depth = crate::util::read_f32_le(chunk)?;

            // e_a_x3_modulation_time: float
            let e_a_x3_modulation_time = crate::util::read_f32_le(chunk)?;

            // e_a_x3_modulation_depth: float
            let e_a_x3_modulation_depth = crate::util::read_f32_le(chunk)?;

            // e_a_x3_h_f_reference: float
            let e_a_x3_h_f_reference = crate::util::read_f32_le(chunk)?;

            // e_a_x3_l_f_reference: float
            let e_a_x3_l_f_reference = crate::util::read_f32_le(chunk)?;


            rows.push(SoundProviderPreferencesRow {
                id,
                description,
                flags,
                e_a_x_environment_selection,
                e_a_x_decay_time,
                e_a_x2_environment_size,
                e_a_x2_environment_diffusion,
                e_a_x2_room,
                e_a_x2_room_h_f,
                e_a_x2_decay_h_f_ratio,
                e_a_x2_reflections,
                e_a_x2_reflections_delay,
                e_a_x2_reverb,
                e_a_x2_reverb_delay,
                e_a_x2_room_rolloff,
                e_a_x2_air_absorption,
                e_a_x3_room_l_f,
                e_a_x3_decay_l_f_ratio,
                e_a_x3_echo_time,
                e_a_x3_echo_depth,
                e_a_x3_modulation_time,
                e_a_x3_modulation_depth,
                e_a_x3_h_f_reference,
                e_a_x3_l_f_reference,
            });
        }

        Ok(SoundProviderPreferences { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SoundProviderPreferences) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // description: string_ref
            if !row.description.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.description.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // e_a_x_environment_selection: int32
            b.write_all(&row.e_a_x_environment_selection.to_le_bytes())?;

            // e_a_x_decay_time: float
            b.write_all(&row.e_a_x_decay_time.to_le_bytes())?;

            // e_a_x2_environment_size: float
            b.write_all(&row.e_a_x2_environment_size.to_le_bytes())?;

            // e_a_x2_environment_diffusion: float
            b.write_all(&row.e_a_x2_environment_diffusion.to_le_bytes())?;

            // e_a_x2_room: int32
            b.write_all(&row.e_a_x2_room.to_le_bytes())?;

            // e_a_x2_room_h_f: int32
            b.write_all(&row.e_a_x2_room_h_f.to_le_bytes())?;

            // e_a_x2_decay_h_f_ratio: float
            b.write_all(&row.e_a_x2_decay_h_f_ratio.to_le_bytes())?;

            // e_a_x2_reflections: int32
            b.write_all(&row.e_a_x2_reflections.to_le_bytes())?;

            // e_a_x2_reflections_delay: float
            b.write_all(&row.e_a_x2_reflections_delay.to_le_bytes())?;

            // e_a_x2_reverb: int32
            b.write_all(&row.e_a_x2_reverb.to_le_bytes())?;

            // e_a_x2_reverb_delay: float
            b.write_all(&row.e_a_x2_reverb_delay.to_le_bytes())?;

            // e_a_x2_room_rolloff: float
            b.write_all(&row.e_a_x2_room_rolloff.to_le_bytes())?;

            // e_a_x2_air_absorption: float
            b.write_all(&row.e_a_x2_air_absorption.to_le_bytes())?;

            // e_a_x3_room_l_f: int32
            b.write_all(&row.e_a_x3_room_l_f.to_le_bytes())?;

            // e_a_x3_decay_l_f_ratio: float
            b.write_all(&row.e_a_x3_decay_l_f_ratio.to_le_bytes())?;

            // e_a_x3_echo_time: float
            b.write_all(&row.e_a_x3_echo_time.to_le_bytes())?;

            // e_a_x3_echo_depth: float
            b.write_all(&row.e_a_x3_echo_depth.to_le_bytes())?;

            // e_a_x3_modulation_time: float
            b.write_all(&row.e_a_x3_modulation_time.to_le_bytes())?;

            // e_a_x3_modulation_depth: float
            b.write_all(&row.e_a_x3_modulation_depth.to_le_bytes())?;

            // e_a_x3_h_f_reference: float
            b.write_all(&row.e_a_x3_h_f_reference.to_le_bytes())?;

            // e_a_x3_l_f_reference: float
            b.write_all(&row.e_a_x3_l_f_reference.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SoundProviderPreferences {
    type PrimaryKey = SoundProviderPreferencesKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl SoundProviderPreferences {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.description.is_empty() { b.write_all(row.description.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.description.is_empty() { sum += row.description.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundProviderPreferencesKey {
    pub id: i32
}

impl SoundProviderPreferencesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for SoundProviderPreferencesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for SoundProviderPreferencesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for SoundProviderPreferencesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for SoundProviderPreferencesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for SoundProviderPreferencesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for SoundProviderPreferencesKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SoundProviderPreferencesKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for SoundProviderPreferencesKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SoundProviderPreferencesKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SoundProviderPreferencesKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundProviderPreferencesRow {
    pub id: SoundProviderPreferencesKey,
    pub description: String,
    pub flags: i32,
    pub e_a_x_environment_selection: i32,
    pub e_a_x_decay_time: f32,
    pub e_a_x2_environment_size: f32,
    pub e_a_x2_environment_diffusion: f32,
    pub e_a_x2_room: i32,
    pub e_a_x2_room_h_f: i32,
    pub e_a_x2_decay_h_f_ratio: f32,
    pub e_a_x2_reflections: i32,
    pub e_a_x2_reflections_delay: f32,
    pub e_a_x2_reverb: i32,
    pub e_a_x2_reverb_delay: f32,
    pub e_a_x2_room_rolloff: f32,
    pub e_a_x2_air_absorption: f32,
    pub e_a_x3_room_l_f: i32,
    pub e_a_x3_decay_l_f_ratio: f32,
    pub e_a_x3_echo_time: f32,
    pub e_a_x3_echo_depth: f32,
    pub e_a_x3_modulation_time: f32,
    pub e_a_x3_modulation_depth: f32,
    pub e_a_x3_h_f_reference: f32,
    pub e_a_x3_l_f_reference: f32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn sound_provider_preferences() {
        let mut file = File::open("../tbc-dbc/SoundProviderPreferences.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SoundProviderPreferences::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SoundProviderPreferences::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
