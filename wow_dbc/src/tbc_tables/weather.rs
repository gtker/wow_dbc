use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstWeather<const S: usize> {
    pub rows: [ConstWeatherRow; S],
}

impl<const S: usize> ConstWeather<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 28 {
            panic!("invalid record size, expected 28")
        }

        if header.field_count != 7 {
            panic!("invalid field count, expected 7")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstWeatherRow {
                id: WeatherKey::new(0),
                ambience_id: SoundEntriesKey::new(0),
                effect_type: 0,
                effect_color: [0.0; 3],
                effect_texture: "",
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Weather) int32
            let id = WeatherKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // ambience_id: foreign_key (SoundEntries) int32
            let ambience_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // effect_type: int32
            let effect_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // effect_color: float[3]
            let effect_color = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_texture: string_ref
            let effect_texture = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            rows[i] = ConstWeatherRow {
                id,
                ambience_id,
                effect_type,
                effect_color,
                effect_texture,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> Weather {
        Weather {
            rows: self.rows.iter().map(|s| WeatherRow {
                id: s.id,
                ambience_id: s.ambience_id,
                effect_type: s.effect_type,
                effect_color: s.effect_color,
                effect_texture: s.effect_texture.to_string(),
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct WeatherKey {
    pub id: i32
}

impl WeatherKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for WeatherKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for WeatherKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for WeatherKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for WeatherKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for WeatherKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct WeatherRow {
    pub id: WeatherKey,
    pub ambience_id: SoundEntriesKey,
    pub effect_type: i32,
    pub effect_color: [f32; 3],
    pub effect_texture: String,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstWeatherRow {
    pub id: WeatherKey,
    pub ambience_id: SoundEntriesKey,
    pub effect_type: i32,
    pub effect_color: [f32; 3],
    pub effect_texture: &'static str,
}

