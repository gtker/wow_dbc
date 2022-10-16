use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;
use crate::vanilla_tables::map::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct WorldSafeLocs {
    pub rows: Vec<WorldSafeLocsRow>,
}

impl DbcTable for WorldSafeLocs {
    type Row = WorldSafeLocsRow;

    fn filename() -> &'static str { "WorldSafeLocs.dbc" }

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (WorldSafeLocs) uint32
            let id = WorldSafeLocsKey::new(crate::util::read_u32_le(chunk)?);

            // map: foreign_key (Map) uint32
            let map = MapKey::new(crate::util::read_u32_le(chunk)?.into());

            // location_x: float
            let location_x = crate::util::read_f32_le(chunk)?;

            // location_y: float
            let location_y = crate::util::read_f32_le(chunk)?;

            // location_z: float
            let location_z = crate::util::read_f32_le(chunk)?;

            // area_name: string_ref_loc
            let area_name = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(WorldSafeLocsRow {
                id,
                map,
                location_x,
                location_y,
                location_z,
                area_name,
            });
        }

        Ok(WorldSafeLocs { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 14,
            record_size: 56,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (WorldSafeLocs) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map: foreign_key (Map) uint32
            b.write_all(&(row.map.id as u32).to_le_bytes())?;

            // location_x: float
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: float
            b.write_all(&row.location_y.to_le_bytes())?;

            // location_z: float
            b.write_all(&row.location_z.to_le_bytes())?;

            // area_name: string_ref_loc
            b.write_all(&row.area_name.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for WorldSafeLocs {
    type PrimaryKey = WorldSafeLocsKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl WorldSafeLocs {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.area_name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.area_name.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct WorldSafeLocsKey {
    pub id: u32
}

impl WorldSafeLocsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct WorldSafeLocsRow {
    pub id: WorldSafeLocsKey,
    pub map: MapKey,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub area_name: LocalizedString,
}

