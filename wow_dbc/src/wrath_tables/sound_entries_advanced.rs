use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundEntriesAdvanced {
    pub rows: Vec<SoundEntriesAdvancedRow>,
}

impl DbcTable for SoundEntriesAdvanced {
    type Row = SoundEntriesAdvancedRow;

    fn filename() -> &'static str { "SoundEntriesAdvanced.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

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

            // id: primary_key (SoundEntriesAdvanced) int32
            let id = SoundEntriesAdvancedKey::new(crate::util::read_i32_le(chunk)?);

            // sound_entry_id: foreign_key (SoundEntries) int32
            let sound_entry_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // inner_radius2_d: float
            let inner_radius2_d = crate::util::read_f32_le(chunk)?;

            // time_a: int32
            let time_a = crate::util::read_i32_le(chunk)?;

            // time_b: int32
            let time_b = crate::util::read_i32_le(chunk)?;

            // time_c: int32
            let time_c = crate::util::read_i32_le(chunk)?;

            // time_d: int32
            let time_d = crate::util::read_i32_le(chunk)?;

            // random_offset_range: int32
            let random_offset_range = crate::util::read_i32_le(chunk)?;

            // usage: int32
            let usage = crate::util::read_i32_le(chunk)?;

            // time_interval_min: int32
            let time_interval_min = crate::util::read_i32_le(chunk)?;

            // time_interval_max: int32
            let time_interval_max = crate::util::read_i32_le(chunk)?;

            // volume_slider_category: int32
            let volume_slider_category = crate::util::read_i32_le(chunk)?;

            // duck_to_s_f_x: float
            let duck_to_s_f_x = crate::util::read_f32_le(chunk)?;

            // duck_to_music: float
            let duck_to_music = crate::util::read_f32_le(chunk)?;

            // duck_to_ambience: float
            let duck_to_ambience = crate::util::read_f32_le(chunk)?;

            // inner_radius_of_influence: float
            let inner_radius_of_influence = crate::util::read_f32_le(chunk)?;

            // outer_radius_of_influence: float
            let outer_radius_of_influence = crate::util::read_f32_le(chunk)?;

            // time_to_duck: int32
            let time_to_duck = crate::util::read_i32_le(chunk)?;

            // time_to_unduck: int32
            let time_to_unduck = crate::util::read_i32_le(chunk)?;

            // inside_angle: float
            let inside_angle = crate::util::read_f32_le(chunk)?;

            // outside_angle: float
            let outside_angle = crate::util::read_f32_le(chunk)?;

            // outside_volume: float
            let outside_volume = crate::util::read_f32_le(chunk)?;

            // outer_radius2_d: float
            let outer_radius2_d = crate::util::read_f32_le(chunk)?;

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(SoundEntriesAdvancedRow {
                id,
                sound_entry_id,
                inner_radius2_d,
                time_a,
                time_b,
                time_c,
                time_d,
                random_offset_range,
                usage,
                time_interval_min,
                time_interval_max,
                volume_slider_category,
                duck_to_s_f_x,
                duck_to_music,
                duck_to_ambience,
                inner_radius_of_influence,
                outer_radius_of_influence,
                time_to_duck,
                time_to_unduck,
                inside_angle,
                outside_angle,
                outside_volume,
                outer_radius2_d,
                name,
            });
        }

        Ok(SoundEntriesAdvanced { rows, })
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
            // id: primary_key (SoundEntriesAdvanced) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_entry_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_entry_id.id as i32).to_le_bytes())?;

            // inner_radius2_d: float
            b.write_all(&row.inner_radius2_d.to_le_bytes())?;

            // time_a: int32
            b.write_all(&row.time_a.to_le_bytes())?;

            // time_b: int32
            b.write_all(&row.time_b.to_le_bytes())?;

            // time_c: int32
            b.write_all(&row.time_c.to_le_bytes())?;

            // time_d: int32
            b.write_all(&row.time_d.to_le_bytes())?;

            // random_offset_range: int32
            b.write_all(&row.random_offset_range.to_le_bytes())?;

            // usage: int32
            b.write_all(&row.usage.to_le_bytes())?;

            // time_interval_min: int32
            b.write_all(&row.time_interval_min.to_le_bytes())?;

            // time_interval_max: int32
            b.write_all(&row.time_interval_max.to_le_bytes())?;

            // volume_slider_category: int32
            b.write_all(&row.volume_slider_category.to_le_bytes())?;

            // duck_to_s_f_x: float
            b.write_all(&row.duck_to_s_f_x.to_le_bytes())?;

            // duck_to_music: float
            b.write_all(&row.duck_to_music.to_le_bytes())?;

            // duck_to_ambience: float
            b.write_all(&row.duck_to_ambience.to_le_bytes())?;

            // inner_radius_of_influence: float
            b.write_all(&row.inner_radius_of_influence.to_le_bytes())?;

            // outer_radius_of_influence: float
            b.write_all(&row.outer_radius_of_influence.to_le_bytes())?;

            // time_to_duck: int32
            b.write_all(&row.time_to_duck.to_le_bytes())?;

            // time_to_unduck: int32
            b.write_all(&row.time_to_unduck.to_le_bytes())?;

            // inside_angle: float
            b.write_all(&row.inside_angle.to_le_bytes())?;

            // outside_angle: float
            b.write_all(&row.outside_angle.to_le_bytes())?;

            // outside_volume: float
            b.write_all(&row.outside_volume.to_le_bytes())?;

            // outer_radius2_d: float
            b.write_all(&row.outer_radius2_d.to_le_bytes())?;

            // name: string_ref
            if !row.name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SoundEntriesAdvanced {
    type PrimaryKey = SoundEntriesAdvancedKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl SoundEntriesAdvanced {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.name.is_empty() { b.write_all(row.name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.name.is_empty() { sum += row.name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SoundEntriesAdvancedKey {
    pub id: i32
}

impl SoundEntriesAdvancedKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i32> for SoundEntriesAdvancedKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundEntriesAdvancedRow {
    pub id: SoundEntriesAdvancedKey,
    pub sound_entry_id: SoundEntriesKey,
    pub inner_radius2_d: f32,
    pub time_a: i32,
    pub time_b: i32,
    pub time_c: i32,
    pub time_d: i32,
    pub random_offset_range: i32,
    pub usage: i32,
    pub time_interval_min: i32,
    pub time_interval_max: i32,
    pub volume_slider_category: i32,
    pub duck_to_s_f_x: f32,
    pub duck_to_music: f32,
    pub duck_to_ambience: f32,
    pub inner_radius_of_influence: f32,
    pub outer_radius_of_influence: f32,
    pub time_to_duck: i32,
    pub time_to_unduck: i32,
    pub inside_angle: f32,
    pub outside_angle: f32,
    pub outside_volume: f32,
    pub outer_radius2_d: f32,
    pub name: String,
}

