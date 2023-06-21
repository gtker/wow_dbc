use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::area_table::*;
use crate::vanilla_tables::world_map_continent::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct WorldMapArea {
    pub rows: Vec<WorldMapAreaRow>,
}

impl DbcTable for WorldMapArea {
    type Row = WorldMapAreaRow;

    fn filename() -> &'static str { "WorldMapArea.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 8,
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

            // id: primary_key (WorldMapArea) uint32
            let id = WorldMapAreaKey::new(crate::util::read_u32_le(chunk)?);

            // world_map_continent: foreign_key (WorldMapContinent) uint32
            let world_map_continent = WorldMapContinentKey::new(crate::util::read_u32_le(chunk)?.into());

            // area_table: foreign_key (AreaTable) uint32
            let area_table = AreaTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // area_name: string_ref
            let area_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // location_left: float
            let location_left = crate::util::read_f32_le(chunk)?;

            // location_right: float
            let location_right = crate::util::read_f32_le(chunk)?;

            // location_top: float
            let location_top = crate::util::read_f32_le(chunk)?;

            // location_bottom: float
            let location_bottom = crate::util::read_f32_le(chunk)?;


            rows.push(WorldMapAreaRow {
                id,
                world_map_continent,
                area_table,
                area_name,
                location_left,
                location_right,
                location_top,
                location_bottom,
            });
        }

        Ok(WorldMapArea { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 8,
            record_size: 32,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (WorldMapArea) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // world_map_continent: foreign_key (WorldMapContinent) uint32
            b.write_all(&(row.world_map_continent.id as u32).to_le_bytes())?;

            // area_table: foreign_key (AreaTable) uint32
            b.write_all(&(row.area_table.id as u32).to_le_bytes())?;

            // area_name: string_ref
            if !row.area_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.area_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // location_left: float
            b.write_all(&row.location_left.to_le_bytes())?;

            // location_right: float
            b.write_all(&row.location_right.to_le_bytes())?;

            // location_top: float
            b.write_all(&row.location_top.to_le_bytes())?;

            // location_bottom: float
            b.write_all(&row.location_bottom.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for WorldMapArea {
    type PrimaryKey = WorldMapAreaKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl WorldMapArea {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.area_name.is_empty() { b.write_all(row.area_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.area_name.is_empty() { sum += row.area_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstWorldMapArea<const S: usize> {
    pub rows: [ConstWorldMapAreaRow; S],
}

impl<const S: usize> ConstWorldMapArea<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 32 {
            panic!("invalid record size, expected 32")
        }

        if header.field_count != 8 {
            panic!("invalid field count, expected 8")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstWorldMapAreaRow {
                id: WorldMapAreaKey::new(0),
                world_map_continent: WorldMapContinentKey::new(0),
                area_table: AreaTableKey::new(0),
                area_name: "",
                location_left: 0.0,
                location_right: 0.0,
                location_top: 0.0,
                location_bottom: 0.0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (WorldMapArea) uint32
            let id = WorldMapAreaKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // world_map_continent: foreign_key (WorldMapContinent) uint32
            let world_map_continent = WorldMapContinentKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // area_table: foreign_key (AreaTable) uint32
            let area_table = AreaTableKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // area_name: string_ref
            let area_name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // location_left: float
            let location_left = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // location_right: float
            let location_right = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // location_top: float
            let location_top = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // location_bottom: float
            let location_bottom = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstWorldMapAreaRow {
                id,
                world_map_continent,
                area_table,
                area_name,
                location_left,
                location_right,
                location_top,
                location_bottom,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> WorldMapArea {
        WorldMapArea {
            rows: self.rows.iter().map(|s| WorldMapAreaRow {
                id: s.id,
                world_map_continent: s.world_map_continent,
                area_table: s.area_table,
                area_name: s.area_name.to_string(),
                location_left: s.location_left,
                location_right: s.location_right,
                location_top: s.location_top,
                location_bottom: s.location_bottom,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct WorldMapAreaKey {
    pub id: u32
}

impl WorldMapAreaKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for WorldMapAreaKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for WorldMapAreaKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for WorldMapAreaKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct WorldMapAreaRow {
    pub id: WorldMapAreaKey,
    pub world_map_continent: WorldMapContinentKey,
    pub area_table: AreaTableKey,
    pub area_name: String,
    pub location_left: f32,
    pub location_right: f32,
    pub location_top: f32,
    pub location_bottom: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstWorldMapAreaRow {
    pub id: WorldMapAreaKey,
    pub world_map_continent: WorldMapContinentKey,
    pub area_table: AreaTableKey,
    pub area_name: &'static str,
    pub location_left: f32,
    pub location_right: f32,
    pub location_top: f32,
    pub location_bottom: f32,
}

