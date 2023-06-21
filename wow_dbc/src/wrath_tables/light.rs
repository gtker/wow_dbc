use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::map::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Light {
    pub rows: Vec<LightRow>,
}

impl DbcTable for Light {
    type Row = LightRow;

    fn filename() -> &'static str { "Light.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 60 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 60,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 15 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 15,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (Light) int32
            let id = LightKey::new(crate::util::read_i32_le(chunk)?);

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // game_coords: float[3]
            let game_coords = crate::util::read_array_f32::<3>(chunk)?;

            // game_falloff_start: float
            let game_falloff_start = crate::util::read_f32_le(chunk)?;

            // game_falloff_end: float
            let game_falloff_end = crate::util::read_f32_le(chunk)?;

            // light_params_id: int32[8]
            let light_params_id = crate::util::read_array_i32::<8>(chunk)?;


            rows.push(LightRow {
                id,
                continent_id,
                game_coords,
                game_falloff_start,
                game_falloff_end,
                light_params_id,
            });
        }

        Ok(Light { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 15,
            record_size: 60,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (Light) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // continent_id: foreign_key (Map) int32
            b.write_all(&(row.continent_id.id as i32).to_le_bytes())?;

            // game_coords: float[3]
            for i in row.game_coords {
                b.write_all(&i.to_le_bytes())?;
            }


            // game_falloff_start: float
            b.write_all(&row.game_falloff_start.to_le_bytes())?;

            // game_falloff_end: float
            b.write_all(&row.game_falloff_end.to_le_bytes())?;

            // light_params_id: int32[8]
            for i in row.light_params_id {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for Light {
    type PrimaryKey = LightKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstLight<const S: usize> {
    pub rows: [LightRow; S],
}

impl<const S: usize> ConstLight<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 60 {
            panic!("invalid record size, expected 60")
        }

        if header.field_count != 15 {
            panic!("invalid field count, expected 15")
        }

        let mut b_offset = 20;
        let mut rows = [
            LightRow {
                id: LightKey::new(0),
                continent_id: MapKey::new(0),
                game_coords: [0.0; 3],
                game_falloff_start: 0.0,
                game_falloff_end: 0.0,
                light_params_id: [0; 8],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Light) int32
            let id = LightKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // game_coords: float[3]
            let game_coords = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // game_falloff_start: float
            let game_falloff_start = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // game_falloff_end: float
            let game_falloff_end = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // light_params_id: int32[8]
            let light_params_id = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = LightRow {
                id,
                continent_id,
                game_coords,
                game_falloff_start,
                game_falloff_end,
                light_params_id,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct LightKey {
    pub id: i32
}

impl LightKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for LightKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for LightKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for LightKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for LightKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for LightKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LightRow {
    pub id: LightKey,
    pub continent_id: MapKey,
    pub game_coords: [f32; 3],
    pub game_falloff_start: f32,
    pub game_falloff_end: f32,
    pub light_params_id: [i32; 8],
}

