use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundProviderPreferences {
    pub rows: Vec<SoundProviderPreferencesRow>,
}

impl DbcTable for SoundProviderPreferences {
    type Row = SoundProviderPreferencesRow;

    const FILENAME: &'static str = "SoundProviderPreferences.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 96 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 96,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 24 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 24,
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

            // id: primary_key (SoundProviderPreferences) uint32
            let id = SoundProviderPreferencesKey::new(crate::util::read_u32_le(chunk)?);

            // description: string_ref
            let description = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // eax_environment_selection: int32
            let eax_environment_selection = crate::util::read_i32_le(chunk)?;

            // eax_decay_time: float
            let eax_decay_time = crate::util::read_f32_le(chunk)?;

            // eax2_environment_size: float
            let eax2_environment_size = crate::util::read_f32_le(chunk)?;

            // eax_environment_diffusion: float
            let eax_environment_diffusion = crate::util::read_f32_le(chunk)?;

            // eax2_room: int32
            let eax2_room = crate::util::read_i32_le(chunk)?;

            // eax2_room_hf: int32
            let eax2_room_hf = crate::util::read_i32_le(chunk)?;

            // eax2_decay_hf_ratio: float
            let eax2_decay_hf_ratio = crate::util::read_f32_le(chunk)?;

            // eax2_reflections: int32
            let eax2_reflections = crate::util::read_i32_le(chunk)?;

            // eax2_reflections_delay: float
            let eax2_reflections_delay = crate::util::read_f32_le(chunk)?;

            // eax2_reverb: int32
            let eax2_reverb = crate::util::read_i32_le(chunk)?;

            // eax2_reverb_delay: float
            let eax2_reverb_delay = crate::util::read_f32_le(chunk)?;

            // eax2_room_rolloff: float
            let eax2_room_rolloff = crate::util::read_f32_le(chunk)?;

            // eax2_air_absorption: float
            let eax2_air_absorption = crate::util::read_f32_le(chunk)?;

            // eax3_room_lf: int32
            let eax3_room_lf = crate::util::read_i32_le(chunk)?;

            // eax3_delay_lf_ratio: float
            let eax3_delay_lf_ratio = crate::util::read_f32_le(chunk)?;

            // eax3_echo_time: float
            let eax3_echo_time = crate::util::read_f32_le(chunk)?;

            // eax3_echo_depth: float
            let eax3_echo_depth = crate::util::read_f32_le(chunk)?;

            // eax3_modulation_time: float
            let eax3_modulation_time = crate::util::read_f32_le(chunk)?;

            // eax3_modulation_depth: float
            let eax3_modulation_depth = crate::util::read_f32_le(chunk)?;

            // eax3_hf_reference: float
            let eax3_hf_reference = crate::util::read_f32_le(chunk)?;

            // eax3_lf_reference: float
            let eax3_lf_reference = crate::util::read_f32_le(chunk)?;


            rows.push(SoundProviderPreferencesRow {
                id,
                description,
                flags,
                eax_environment_selection,
                eax_decay_time,
                eax2_environment_size,
                eax_environment_diffusion,
                eax2_room,
                eax2_room_hf,
                eax2_decay_hf_ratio,
                eax2_reflections,
                eax2_reflections_delay,
                eax2_reverb,
                eax2_reverb_delay,
                eax2_room_rolloff,
                eax2_air_absorption,
                eax3_room_lf,
                eax3_delay_lf_ratio,
                eax3_echo_time,
                eax3_echo_depth,
                eax3_modulation_time,
                eax3_modulation_depth,
                eax3_hf_reference,
                eax3_lf_reference,
            });
        }

        Ok(SoundProviderPreferences { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 24,
            record_size: 96,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SoundProviderPreferences) uint32
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

            // eax_environment_selection: int32
            b.write_all(&row.eax_environment_selection.to_le_bytes())?;

            // eax_decay_time: float
            b.write_all(&row.eax_decay_time.to_le_bytes())?;

            // eax2_environment_size: float
            b.write_all(&row.eax2_environment_size.to_le_bytes())?;

            // eax_environment_diffusion: float
            b.write_all(&row.eax_environment_diffusion.to_le_bytes())?;

            // eax2_room: int32
            b.write_all(&row.eax2_room.to_le_bytes())?;

            // eax2_room_hf: int32
            b.write_all(&row.eax2_room_hf.to_le_bytes())?;

            // eax2_decay_hf_ratio: float
            b.write_all(&row.eax2_decay_hf_ratio.to_le_bytes())?;

            // eax2_reflections: int32
            b.write_all(&row.eax2_reflections.to_le_bytes())?;

            // eax2_reflections_delay: float
            b.write_all(&row.eax2_reflections_delay.to_le_bytes())?;

            // eax2_reverb: int32
            b.write_all(&row.eax2_reverb.to_le_bytes())?;

            // eax2_reverb_delay: float
            b.write_all(&row.eax2_reverb_delay.to_le_bytes())?;

            // eax2_room_rolloff: float
            b.write_all(&row.eax2_room_rolloff.to_le_bytes())?;

            // eax2_air_absorption: float
            b.write_all(&row.eax2_air_absorption.to_le_bytes())?;

            // eax3_room_lf: int32
            b.write_all(&row.eax3_room_lf.to_le_bytes())?;

            // eax3_delay_lf_ratio: float
            b.write_all(&row.eax3_delay_lf_ratio.to_le_bytes())?;

            // eax3_echo_time: float
            b.write_all(&row.eax3_echo_time.to_le_bytes())?;

            // eax3_echo_depth: float
            b.write_all(&row.eax3_echo_depth.to_le_bytes())?;

            // eax3_modulation_time: float
            b.write_all(&row.eax3_modulation_time.to_le_bytes())?;

            // eax3_modulation_depth: float
            b.write_all(&row.eax3_modulation_depth.to_le_bytes())?;

            // eax3_hf_reference: float
            b.write_all(&row.eax3_hf_reference.to_le_bytes())?;

            // eax3_lf_reference: float
            b.write_all(&row.eax3_lf_reference.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SoundProviderPreferences {
    type PrimaryKey = SoundProviderPreferencesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
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
pub struct SoundProviderPreferencesKey {
    pub id: u32
}

impl SoundProviderPreferencesKey {
    pub const fn new(id: u32) -> Self {
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

impl From<u32> for SoundProviderPreferencesKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundProviderPreferencesRow {
    pub id: SoundProviderPreferencesKey,
    pub description: String,
    pub flags: i32,
    pub eax_environment_selection: i32,
    pub eax_decay_time: f32,
    pub eax2_environment_size: f32,
    pub eax_environment_diffusion: f32,
    pub eax2_room: i32,
    pub eax2_room_hf: i32,
    pub eax2_decay_hf_ratio: f32,
    pub eax2_reflections: i32,
    pub eax2_reflections_delay: f32,
    pub eax2_reverb: i32,
    pub eax2_reverb_delay: f32,
    pub eax2_room_rolloff: f32,
    pub eax2_air_absorption: f32,
    pub eax3_room_lf: i32,
    pub eax3_delay_lf_ratio: f32,
    pub eax3_echo_time: f32,
    pub eax3_echo_depth: f32,
    pub eax3_modulation_time: f32,
    pub eax3_modulation_depth: f32,
    pub eax3_hf_reference: f32,
    pub eax3_lf_reference: f32,
}

