use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::map::*;
use crate::wrath_tables::world_map_area::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct WorldMapContinent {
    pub rows: Vec<WorldMapContinentRow>,
}

impl DbcTable for WorldMapContinent {
    type Row = WorldMapContinentRow;

    fn filename() -> &'static str { "WorldMapContinent.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 56 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 56,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 14 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 14,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (WorldMapContinent) int32
            let id = WorldMapContinentKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // left_boundary: int32
            let left_boundary = crate::util::read_i32_le(chunk)?;

            // right_boundary: int32
            let right_boundary = crate::util::read_i32_le(chunk)?;

            // top_boundary: int32
            let top_boundary = crate::util::read_i32_le(chunk)?;

            // bottom_boundary: int32
            let bottom_boundary = crate::util::read_i32_le(chunk)?;

            // continent_offset: float[2]
            let continent_offset = crate::util::read_array_f32::<2>(chunk)?;

            // scale: float
            let scale = crate::util::read_f32_le(chunk)?;

            // taxi_min: float[2]
            let taxi_min = crate::util::read_array_f32::<2>(chunk)?;

            // taxi_max: float[2]
            let taxi_max = crate::util::read_array_f32::<2>(chunk)?;

            // world_map_id: foreign_key (WorldMapArea) int32
            let world_map_id = WorldMapAreaKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(WorldMapContinentRow {
                id,
                map_id,
                left_boundary,
                right_boundary,
                top_boundary,
                bottom_boundary,
                continent_offset,
                scale,
                taxi_min,
                taxi_max,
                world_map_id,
            });
        }

        Ok(WorldMapContinent { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 14,
            record_size: 56,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (WorldMapContinent) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // left_boundary: int32
            b.write_all(&row.left_boundary.to_le_bytes())?;

            // right_boundary: int32
            b.write_all(&row.right_boundary.to_le_bytes())?;

            // top_boundary: int32
            b.write_all(&row.top_boundary.to_le_bytes())?;

            // bottom_boundary: int32
            b.write_all(&row.bottom_boundary.to_le_bytes())?;

            // continent_offset: float[2]
            for i in row.continent_offset {
                b.write_all(&i.to_le_bytes())?;
            }


            // scale: float
            b.write_all(&row.scale.to_le_bytes())?;

            // taxi_min: float[2]
            for i in row.taxi_min {
                b.write_all(&i.to_le_bytes())?;
            }


            // taxi_max: float[2]
            for i in row.taxi_max {
                b.write_all(&i.to_le_bytes())?;
            }


            // world_map_id: foreign_key (WorldMapArea) int32
            b.write_all(&(row.world_map_id.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for WorldMapContinent {
    type PrimaryKey = WorldMapContinentKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct WorldMapContinentKey {
    pub id: i32
}

impl WorldMapContinentKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct WorldMapContinentRow {
    pub id: WorldMapContinentKey,
    pub map_id: MapKey,
    pub left_boundary: i32,
    pub right_boundary: i32,
    pub top_boundary: i32,
    pub bottom_boundary: i32,
    pub continent_offset: [f32; 2],
    pub scale: f32,
    pub taxi_min: [f32; 2],
    pub taxi_max: [f32; 2],
    pub world_map_id: WorldMapAreaKey,
}

