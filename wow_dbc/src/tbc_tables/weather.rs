use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Weather {
    pub rows: Vec<WeatherRow>,
}

impl DbcTable for Weather {
    type Row = WeatherRow;

    fn filename() -> &'static str { "Weather.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 28 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 28,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 7 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 7,
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

            // id: primary_key (Weather) int32
            let id = WeatherKey::new(crate::util::read_i32_le(chunk)?);

            // ambience_id: foreign_key (SoundEntries) int32
            let ambience_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // effect_type: int32
            let effect_type = crate::util::read_i32_le(chunk)?;

            // effect_color: float[3]
            let effect_color = crate::util::read_array_f32::<3>(chunk)?;

            // effect_texture: string_ref
            let effect_texture = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(WeatherRow {
                id,
                ambience_id,
                effect_type,
                effect_color,
                effect_texture,
            });
        }

        Ok(Weather { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 7,
            record_size: 28,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Weather) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // ambience_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.ambience_id.id as i32).to_le_bytes())?;

            // effect_type: int32
            b.write_all(&row.effect_type.to_le_bytes())?;

            // effect_color: float[3]
            for i in row.effect_color {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_texture: string_ref
            if !row.effect_texture.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.effect_texture.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Weather {
    type PrimaryKey = WeatherKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl Weather {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.effect_texture.is_empty() { b.write_all(row.effect_texture.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.effect_texture.is_empty() { sum += row.effect_texture.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct WeatherKey {
    pub id: i32
}

impl WeatherKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct WeatherRow {
    pub id: WeatherKey,
    pub ambience_id: SoundEntriesKey,
    pub effect_type: i32,
    pub effect_color: [f32; 3],
    pub effect_texture: String,
}
