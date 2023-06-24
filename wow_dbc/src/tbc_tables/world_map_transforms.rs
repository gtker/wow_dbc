use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::map::MapKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct WorldMapTransforms {
    pub rows: Vec<WorldMapTransformsRow>,
}

impl DbcTable for WorldMapTransforms {
    type Row = WorldMapTransformsRow;

    const FILENAME: &'static str = "WorldMapTransforms.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (WorldMapTransforms) int32
            let id = WorldMapTransformsKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // region_min: float[2]
            let region_min = crate::util::read_array_f32::<2>(chunk)?;

            // region_max: float[2]
            let region_max = crate::util::read_array_f32::<2>(chunk)?;

            // new_map_id: foreign_key (Map) int32
            let new_map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // region_offset: float[2]
            let region_offset = crate::util::read_array_f32::<2>(chunk)?;


            rows.push(WorldMapTransformsRow {
                id,
                map_id,
                region_min,
                region_max,
                new_map_id,
                region_offset,
            });
        }

        Ok(WorldMapTransforms { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 9,
            record_size: 36,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (WorldMapTransforms) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // region_min: float[2]
            for i in row.region_min {
                b.write_all(&i.to_le_bytes())?;
            }


            // region_max: float[2]
            for i in row.region_max {
                b.write_all(&i.to_le_bytes())?;
            }


            // new_map_id: foreign_key (Map) int32
            b.write_all(&(row.new_map_id.id as i32).to_le_bytes())?;

            // region_offset: float[2]
            for i in row.region_offset {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for WorldMapTransforms {
    type PrimaryKey = WorldMapTransformsKey;
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
pub struct WorldMapTransformsKey {
    pub id: i32
}

impl WorldMapTransformsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for WorldMapTransformsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for WorldMapTransformsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for WorldMapTransformsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for WorldMapTransformsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for WorldMapTransformsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct WorldMapTransformsRow {
    pub id: WorldMapTransformsKey,
    pub map_id: MapKey,
    pub region_min: [f32; 2],
    pub region_max: [f32; 2],
    pub new_map_id: MapKey,
    pub region_offset: [f32; 2],
}

