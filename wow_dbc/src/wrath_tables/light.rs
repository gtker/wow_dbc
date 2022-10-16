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
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LightRow {
    pub id: LightKey,
    pub continent_id: MapKey,
    pub game_coords: [f32; 3],
    pub game_falloff_start: f32,
    pub game_falloff_end: f32,
    pub light_params_id: [i32; 8],
}

