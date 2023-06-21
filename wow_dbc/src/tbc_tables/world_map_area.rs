use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::area_table::*;
use crate::tbc_tables::map::*;

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

        if header.record_size != 36 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 36,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 9 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 9,
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

            // id: primary_key (WorldMapArea) int32
            let id = WorldMapAreaKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // area_id: foreign_key (AreaTable) int32
            let area_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // area_name: string_ref
            let area_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // loc_left: float
            let loc_left = crate::util::read_f32_le(chunk)?;

            // loc_right: float
            let loc_right = crate::util::read_f32_le(chunk)?;

            // loc_top: float
            let loc_top = crate::util::read_f32_le(chunk)?;

            // loc_bottom: float
            let loc_bottom = crate::util::read_f32_le(chunk)?;

            // display_map_id: foreign_key (Map) int32
            let display_map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(WorldMapAreaRow {
                id,
                map_id,
                area_id,
                area_name,
                loc_left,
                loc_right,
                loc_top,
                loc_bottom,
                display_map_id,
            });
        }

        Ok(WorldMapArea { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 9,
            record_size: 36,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (WorldMapArea) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // area_id: foreign_key (AreaTable) int32
            b.write_all(&(row.area_id.id as i32).to_le_bytes())?;

            // area_name: string_ref
            if !row.area_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.area_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // loc_left: float
            b.write_all(&row.loc_left.to_le_bytes())?;

            // loc_right: float
            b.write_all(&row.loc_right.to_le_bytes())?;

            // loc_top: float
            b.write_all(&row.loc_top.to_le_bytes())?;

            // loc_bottom: float
            b.write_all(&row.loc_bottom.to_le_bytes())?;

            // display_map_id: foreign_key (Map) int32
            b.write_all(&(row.display_map_id.id as i32).to_le_bytes())?;

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
        if header.record_size != 36 {
            panic!("invalid record size, expected 36")
        }

        if header.field_count != 9 {
            panic!("invalid field count, expected 9")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstWorldMapAreaRow {
                id: WorldMapAreaKey::new(0),
                map_id: MapKey::new(0),
                area_id: AreaTableKey::new(0),
                area_name: "",
                loc_left: 0.0,
                loc_right: 0.0,
                loc_top: 0.0,
                loc_bottom: 0.0,
                display_map_id: MapKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (WorldMapArea) int32
            let id = WorldMapAreaKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // area_id: foreign_key (AreaTable) int32
            let area_id = AreaTableKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // area_name: string_ref
            let area_name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // loc_left: float
            let loc_left = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // loc_right: float
            let loc_right = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // loc_top: float
            let loc_top = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // loc_bottom: float
            let loc_bottom = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // display_map_id: foreign_key (Map) int32
            let display_map_id = MapKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = ConstWorldMapAreaRow {
                id,
                map_id,
                area_id,
                area_name,
                loc_left,
                loc_right,
                loc_top,
                loc_bottom,
                display_map_id,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct WorldMapAreaKey {
    pub id: i32
}

impl WorldMapAreaKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for WorldMapAreaKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for WorldMapAreaKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for WorldMapAreaKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct WorldMapAreaRow {
    pub id: WorldMapAreaKey,
    pub map_id: MapKey,
    pub area_id: AreaTableKey,
    pub area_name: String,
    pub loc_left: f32,
    pub loc_right: f32,
    pub loc_top: f32,
    pub loc_bottom: f32,
    pub display_map_id: MapKey,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstWorldMapAreaRow {
    pub id: WorldMapAreaKey,
    pub map_id: MapKey,
    pub area_id: AreaTableKey,
    pub area_name: &'static str,
    pub loc_left: f32,
    pub loc_right: f32,
    pub loc_top: f32,
    pub loc_bottom: f32,
    pub display_map_id: MapKey,
}

