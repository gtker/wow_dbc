use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::area_table::*;
use crate::wrath_tables::map::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DungeonMap {
    pub rows: Vec<DungeonMapRow>,
}

impl DbcTable for DungeonMap {
    type Row = DungeonMapRow;

    fn filename() -> &'static str { "DungeonMap.dbc" }

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

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (DungeonMap) int32
            let id = DungeonMapKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // floor_index: int32
            let floor_index = crate::util::read_i32_le(chunk)?;

            // min_x: float
            let min_x = crate::util::read_f32_le(chunk)?;

            // max_x: float
            let max_x = crate::util::read_f32_le(chunk)?;

            // min_y: float
            let min_y = crate::util::read_f32_le(chunk)?;

            // max_y: float
            let max_y = crate::util::read_f32_le(chunk)?;

            // parent_world_map_id: foreign_key (AreaTable) int32
            let parent_world_map_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(DungeonMapRow {
                id,
                map_id,
                floor_index,
                min_x,
                max_x,
                min_y,
                max_y,
                parent_world_map_id,
            });
        }

        Ok(DungeonMap { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 8,
            record_size: 32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (DungeonMap) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // floor_index: int32
            b.write_all(&row.floor_index.to_le_bytes())?;

            // min_x: float
            b.write_all(&row.min_x.to_le_bytes())?;

            // max_x: float
            b.write_all(&row.max_x.to_le_bytes())?;

            // min_y: float
            b.write_all(&row.min_y.to_le_bytes())?;

            // max_y: float
            b.write_all(&row.max_y.to_le_bytes())?;

            // parent_world_map_id: foreign_key (AreaTable) int32
            b.write_all(&(row.parent_world_map_id.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for DungeonMap {
    type PrimaryKey = DungeonMapKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct DungeonMapKey {
    pub id: i32
}

impl DungeonMapKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct DungeonMapRow {
    pub id: DungeonMapKey,
    pub map_id: MapKey,
    pub floor_index: i32,
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub parent_world_map_id: AreaTableKey,
}

