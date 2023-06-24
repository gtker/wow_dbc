use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::map::MapKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct WorldMapContinent {
    pub rows: Vec<WorldMapContinentRow>,
}

impl DbcTable for WorldMapContinent {
    type Row = WorldMapContinentRow;

    const FILENAME: &'static str = "WorldMapContinent.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 52 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 52,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 13 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 13,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (WorldMapContinent) uint32
            let id = WorldMapContinentKey::new(crate::util::read_u32_le(chunk)?);

            // map: foreign_key (Map) uint32
            let map = MapKey::new(crate::util::read_u32_le(chunk)?.into());

            // left_boundary: uint32
            let left_boundary = crate::util::read_u32_le(chunk)?;

            // right_boundary: uint32
            let right_boundary = crate::util::read_u32_le(chunk)?;

            // top_boundary: uint32
            let top_boundary = crate::util::read_u32_le(chunk)?;

            // bottom_boundary: uint32
            let bottom_boundary = crate::util::read_u32_le(chunk)?;

            // continent_offset_x: float
            let continent_offset_x = crate::util::read_f32_le(chunk)?;

            // continent_offset_y: float
            let continent_offset_y = crate::util::read_f32_le(chunk)?;

            // scale: float
            let scale = crate::util::read_f32_le(chunk)?;

            // taxi_min_x: float
            let taxi_min_x = crate::util::read_f32_le(chunk)?;

            // taxi_min_y: float
            let taxi_min_y = crate::util::read_f32_le(chunk)?;

            // taxi_max_x: float
            let taxi_max_x = crate::util::read_f32_le(chunk)?;

            // taxi_max_y: float
            let taxi_max_y = crate::util::read_f32_le(chunk)?;


            rows.push(WorldMapContinentRow {
                id,
                map,
                left_boundary,
                right_boundary,
                top_boundary,
                bottom_boundary,
                continent_offset_x,
                continent_offset_y,
                scale,
                taxi_min_x,
                taxi_min_y,
                taxi_max_x,
                taxi_max_y,
            });
        }

        Ok(WorldMapContinent { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 13,
            record_size: 52,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (WorldMapContinent) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map: foreign_key (Map) uint32
            b.write_all(&(row.map.id as u32).to_le_bytes())?;

            // left_boundary: uint32
            b.write_all(&row.left_boundary.to_le_bytes())?;

            // right_boundary: uint32
            b.write_all(&row.right_boundary.to_le_bytes())?;

            // top_boundary: uint32
            b.write_all(&row.top_boundary.to_le_bytes())?;

            // bottom_boundary: uint32
            b.write_all(&row.bottom_boundary.to_le_bytes())?;

            // continent_offset_x: float
            b.write_all(&row.continent_offset_x.to_le_bytes())?;

            // continent_offset_y: float
            b.write_all(&row.continent_offset_y.to_le_bytes())?;

            // scale: float
            b.write_all(&row.scale.to_le_bytes())?;

            // taxi_min_x: float
            b.write_all(&row.taxi_min_x.to_le_bytes())?;

            // taxi_min_y: float
            b.write_all(&row.taxi_min_y.to_le_bytes())?;

            // taxi_max_x: float
            b.write_all(&row.taxi_max_x.to_le_bytes())?;

            // taxi_max_y: float
            b.write_all(&row.taxi_max_y.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for WorldMapContinent {
    type PrimaryKey = WorldMapContinentKey;
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
pub struct WorldMapContinentKey {
    pub id: u32
}

impl WorldMapContinentKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for WorldMapContinentKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for WorldMapContinentKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for WorldMapContinentKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct WorldMapContinentRow {
    pub id: WorldMapContinentKey,
    pub map: MapKey,
    pub left_boundary: u32,
    pub right_boundary: u32,
    pub top_boundary: u32,
    pub bottom_boundary: u32,
    pub continent_offset_x: f32,
    pub continent_offset_y: f32,
    pub scale: f32,
    pub taxi_min_x: f32,
    pub taxi_min_y: f32,
    pub taxi_max_x: f32,
    pub taxi_max_y: f32,
}

