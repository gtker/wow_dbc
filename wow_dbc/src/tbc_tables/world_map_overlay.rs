use crate::header::{HEADER_SIZE, DbcHeader, parse_header};
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::world_map_area::*;

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
        let header = parse_header(&header)?;

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

            // id: primary_key (WorldMapOverlay) int32
            let id = WorldMapOverlayKey::new(crate::util::read_i32_le(chunk)?);

            // map_area_id: foreign_key (WorldMapArea) int32
            let map_area_id = WorldMapAreaKey::new(crate::util::read_i32_le(chunk)?.into());

            // area_id: int32[4]
            let area_id = crate::util::read_array_i32::<4>(chunk)?;

            // map_point_x: int32
            let map_point_x = crate::util::read_i32_le(chunk)?;

            // map_point_y: int32
            let map_point_y = crate::util::read_i32_le(chunk)?;

            // texture_name: string_ref
            let texture_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // texture_width: int32
            let texture_width = crate::util::read_i32_le(chunk)?;

            // texture_height: int32
            let texture_height = crate::util::read_i32_le(chunk)?;

            // offset_x: int32
            let offset_x = crate::util::read_i32_le(chunk)?;

            // offset_y: int32
            let offset_y = crate::util::read_i32_le(chunk)?;

            // hit_rect_top: int32
            let hit_rect_top = crate::util::read_i32_le(chunk)?;

            // hit_rect_left: int32
            let hit_rect_left = crate::util::read_i32_le(chunk)?;

            // hit_rect_bottom: int32
            let hit_rect_bottom = crate::util::read_i32_le(chunk)?;

            // hit_rect_right: int32
            let hit_rect_right = crate::util::read_i32_le(chunk)?;


            rows.push(WorldMapOverlayRow {
                id,
                map_area_id,
                area_id,
                map_point_x,
                map_point_y,
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
            // id: primary_key (WorldMapOverlay) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_area_id: foreign_key (WorldMapArea) int32
            b.write_all(&(row.map_area_id.id as i32).to_le_bytes())?;

            // area_id: int32[4]
            for i in row.area_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // map_point_x: int32
            b.write_all(&row.map_point_x.to_le_bytes())?;

            // map_point_y: int32
            b.write_all(&row.map_point_y.to_le_bytes())?;

            // texture_name: string_ref
            if !row.texture_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.texture_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // texture_width: int32
            b.write_all(&row.texture_width.to_le_bytes())?;

            // texture_height: int32
            b.write_all(&row.texture_height.to_le_bytes())?;

            // offset_x: int32
            b.write_all(&row.offset_x.to_le_bytes())?;

            // offset_y: int32
            b.write_all(&row.offset_y.to_le_bytes())?;

            // hit_rect_top: int32
            b.write_all(&row.hit_rect_top.to_le_bytes())?;

            // hit_rect_left: int32
            b.write_all(&row.hit_rect_left.to_le_bytes())?;

            // hit_rect_bottom: int32
            b.write_all(&row.hit_rect_bottom.to_le_bytes())?;

            // hit_rect_right: int32
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
    pub id: i32
}

impl WorldMapOverlayKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for WorldMapOverlayKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for WorldMapOverlayKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for WorldMapOverlayKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldMapOverlayRow {
    pub id: WorldMapOverlayKey,
    pub map_area_id: WorldMapAreaKey,
    pub area_id: [i32; 4],
    pub map_point_x: i32,
    pub map_point_y: i32,
    pub texture_name: String,
    pub texture_width: i32,
    pub texture_height: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub hit_rect_top: i32,
    pub hit_rect_left: i32,
    pub hit_rect_bottom: i32,
    pub hit_rect_right: i32,
}

