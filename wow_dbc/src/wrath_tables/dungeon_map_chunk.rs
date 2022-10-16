use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::dungeon_map::*;
use crate::wrath_tables::map::*;
use crate::wrath_tables::wmo_area_table::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DungeonMapChunk {
    pub rows: Vec<DungeonMapChunkRow>,
}

impl DbcTable for DungeonMapChunk {
    type Row = DungeonMapChunkRow;

    fn filename() -> &'static str { "DungeonMapChunk.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 20 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 20,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 5 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 5,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (DungeonMapChunk) int32
            let id = DungeonMapChunkKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // w_m_o_group_id: foreign_key (WMOAreaTable) int32
            let w_m_o_group_id = WMOAreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // dungeon_map_id: foreign_key (DungeonMap) int32
            let dungeon_map_id = DungeonMapKey::new(crate::util::read_i32_le(chunk)?.into());

            // min_z: float
            let min_z = crate::util::read_f32_le(chunk)?;


            rows.push(DungeonMapChunkRow {
                id,
                map_id,
                w_m_o_group_id,
                dungeon_map_id,
                min_z,
            });
        }

        Ok(DungeonMapChunk { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 5,
            record_size: 20,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (DungeonMapChunk) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // w_m_o_group_id: foreign_key (WMOAreaTable) int32
            b.write_all(&(row.w_m_o_group_id.id as i32).to_le_bytes())?;

            // dungeon_map_id: foreign_key (DungeonMap) int32
            b.write_all(&(row.dungeon_map_id.id as i32).to_le_bytes())?;

            // min_z: float
            b.write_all(&row.min_z.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for DungeonMapChunk {
    type PrimaryKey = DungeonMapChunkKey;
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
pub struct DungeonMapChunkKey {
    pub id: i32
}

impl DungeonMapChunkKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i32> for DungeonMapChunkKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DungeonMapChunkRow {
    pub id: DungeonMapChunkKey,
    pub map_id: MapKey,
    pub w_m_o_group_id: WMOAreaTableKey,
    pub dungeon_map_id: DungeonMapKey,
    pub min_z: f32,
}

