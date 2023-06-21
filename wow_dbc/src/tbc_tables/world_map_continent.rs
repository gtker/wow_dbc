use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::map::*;

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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstWorldMapContinent<const S: usize> {
    pub rows: [WorldMapContinentRow; S],
}

impl<const S: usize> ConstWorldMapContinent<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 52 {
            panic!("invalid record size, expected 52")
        }

        if header.field_count != 13 {
            panic!("invalid field count, expected 13")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            WorldMapContinentRow {
                id: WorldMapContinentKey::new(0),
                map_id: MapKey::new(0),
                left_boundary: 0,
                right_boundary: 0,
                top_boundary: 0,
                bottom_boundary: 0,
                continent_offset: [0.0; 2],
                scale: 0.0,
                taxi_min: [0.0; 2],
                taxi_max: [0.0; 2],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (WorldMapContinent) int32
            let id = WorldMapContinentKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // left_boundary: int32
            let left_boundary = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // right_boundary: int32
            let right_boundary = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // top_boundary: int32
            let top_boundary = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // bottom_boundary: int32
            let bottom_boundary = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // continent_offset: float[2]
            let continent_offset = {
                let mut a = [0.0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // scale: float
            let scale = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // taxi_min: float[2]
            let taxi_min = {
                let mut a = [0.0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // taxi_max: float[2]
            let taxi_max = {
                let mut a = [0.0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = WorldMapContinentRow {
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
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> WorldMapContinent {
        WorldMapContinent {
            rows: self.rows.iter().map(|s| WorldMapContinentRow {
                id: s.id,
                map_id: s.map_id,
                left_boundary: s.left_boundary,
                right_boundary: s.right_boundary,
                top_boundary: s.top_boundary,
                bottom_boundary: s.bottom_boundary,
                continent_offset: s.continent_offset,
                scale: s.scale,
                taxi_min: s.taxi_min,
                taxi_max: s.taxi_max,
            }).collect(),
        }
    }
    // TODO: Indexable?
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

impl From<i8> for WorldMapContinentKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for WorldMapContinentKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for WorldMapContinentKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
}

