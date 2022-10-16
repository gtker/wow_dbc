use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::world_map_area::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldMapOverlay {
    pub rows: Vec<WorldMapOverlayRow>,
}

impl DbcTable for WorldMapOverlay {
    type Row = WorldMapOverlayRow;

    fn filename() -> &'static str { "WorldMapOverlay.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 68 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 68,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 17 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 17,
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

            // id: primary_key (WorldMapOverlay) uint32
            let id = WorldMapOverlayKey::new(crate::util::read_u32_le(chunk)?);

            // world_map_area: foreign_key (WorldMapArea) uint32
            let world_map_area = WorldMapAreaKey::new(crate::util::read_u32_le(chunk)?.into());

            // area_table: uint32[4]
            let area_table = crate::util::read_array_u32::<4>(chunk)?;

            // location_x: uint32
            let location_x = crate::util::read_u32_le(chunk)?;

            // location_y: uint32
            let location_y = crate::util::read_u32_le(chunk)?;

            // texture_name: string_ref
            let texture_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // texture_width: uint32
            let texture_width = crate::util::read_u32_le(chunk)?;

            // texture_height: uint32
            let texture_height = crate::util::read_u32_le(chunk)?;

            // offset_x: uint32
            let offset_x = crate::util::read_u32_le(chunk)?;

            // offset_y: uint32
            let offset_y = crate::util::read_u32_le(chunk)?;

            // hit_rect_top: uint32
            let hit_rect_top = crate::util::read_u32_le(chunk)?;

            // hit_rect_left: uint32
            let hit_rect_left = crate::util::read_u32_le(chunk)?;

            // hit_rect_bottom: uint32
            let hit_rect_bottom = crate::util::read_u32_le(chunk)?;

            // hit_rect_right: uint32
            let hit_rect_right = crate::util::read_u32_le(chunk)?;


            rows.push(WorldMapOverlayRow {
                id,
                world_map_area,
                area_table,
                location_x,
                location_y,
                texture_name,
                texture_width,
                texture_height,
                offset_x,
                offset_y,
                hit_rect_top,
                hit_rect_left,
                hit_rect_bottom,
                hit_rect_right,
            });
        }

        Ok(WorldMapOverlay { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 17,
            record_size: 68,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (WorldMapOverlay) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // world_map_area: foreign_key (WorldMapArea) uint32
            b.write_all(&(row.world_map_area.id as u32).to_le_bytes())?;

            // area_table: uint32[4]
            for i in row.area_table {
                b.write_all(&i.to_le_bytes())?;
            }


            // location_x: uint32
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: uint32
            b.write_all(&row.location_y.to_le_bytes())?;

            // texture_name: string_ref
            if !row.texture_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.texture_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // texture_width: uint32
            b.write_all(&row.texture_width.to_le_bytes())?;

            // texture_height: uint32
            b.write_all(&row.texture_height.to_le_bytes())?;

            // offset_x: uint32
            b.write_all(&row.offset_x.to_le_bytes())?;

            // offset_y: uint32
            b.write_all(&row.offset_y.to_le_bytes())?;

            // hit_rect_top: uint32
            b.write_all(&row.hit_rect_top.to_le_bytes())?;

            // hit_rect_left: uint32
            b.write_all(&row.hit_rect_left.to_le_bytes())?;

            // hit_rect_bottom: uint32
            b.write_all(&row.hit_rect_bottom.to_le_bytes())?;

            // hit_rect_right: uint32
            b.write_all(&row.hit_rect_right.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for WorldMapOverlay {
    type PrimaryKey = WorldMapOverlayKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl WorldMapOverlay {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.texture_name.is_empty() { b.write_all(row.texture_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.texture_name.is_empty() { sum += row.texture_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct WorldMapOverlayKey {
    pub id: u32
}

impl WorldMapOverlayKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for WorldMapOverlayKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for WorldMapOverlayKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for WorldMapOverlayKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldMapOverlayRow {
    pub id: WorldMapOverlayKey,
    pub world_map_area: WorldMapAreaKey,
    pub area_table: [u32; 4],
    pub location_x: u32,
    pub location_y: u32,
    pub texture_name: String,
    pub texture_width: u32,
    pub texture_height: u32,
    pub offset_x: u32,
    pub offset_y: u32,
    pub hit_rect_top: u32,
    pub hit_rect_left: u32,
    pub hit_rect_bottom: u32,
    pub hit_rect_right: u32,
}

