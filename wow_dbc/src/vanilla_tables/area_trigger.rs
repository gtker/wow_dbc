use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::map::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AreaTrigger {
    pub rows: Vec<AreaTriggerRow>,
}

impl DbcTable for AreaTrigger {
    type Row = AreaTriggerRow;

    fn filename() -> &'static str { "AreaTrigger.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 40 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 40,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 10 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 10,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (AreaTrigger) uint32
            let id = AreaTriggerKey::new(crate::util::read_u32_le(chunk)?);

            // map: foreign_key (Map) uint32
            let map = MapKey::new(crate::util::read_u32_le(chunk)?.into());

            // location_x: float
            let location_x = crate::util::read_f32_le(chunk)?;

            // location_y: float
            let location_y = crate::util::read_f32_le(chunk)?;

            // location_z: float
            let location_z = crate::util::read_f32_le(chunk)?;

            // radius: float
            let radius = crate::util::read_f32_le(chunk)?;

            // box_length: float
            let box_length = crate::util::read_f32_le(chunk)?;

            // box_width: float
            let box_width = crate::util::read_f32_le(chunk)?;

            // box_height: float
            let box_height = crate::util::read_f32_le(chunk)?;

            // box_yaw: float
            let box_yaw = crate::util::read_f32_le(chunk)?;


            rows.push(AreaTriggerRow {
                id,
                map,
                location_x,
                location_y,
                location_z,
                radius,
                box_length,
                box_width,
                box_height,
                box_yaw,
            });
        }

        Ok(AreaTrigger { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 10,
            record_size: 40,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (AreaTrigger) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map: foreign_key (Map) uint32
            b.write_all(&(row.map.id as u32).to_le_bytes())?;

            // location_x: float
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: float
            b.write_all(&row.location_y.to_le_bytes())?;

            // location_z: float
            b.write_all(&row.location_z.to_le_bytes())?;

            // radius: float
            b.write_all(&row.radius.to_le_bytes())?;

            // box_length: float
            b.write_all(&row.box_length.to_le_bytes())?;

            // box_width: float
            b.write_all(&row.box_width.to_le_bytes())?;

            // box_height: float
            b.write_all(&row.box_height.to_le_bytes())?;

            // box_yaw: float
            b.write_all(&row.box_yaw.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for AreaTrigger {
    type PrimaryKey = AreaTriggerKey;
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
pub struct AreaTriggerKey {
    pub id: u32
}

impl AreaTriggerKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u32> for AreaTriggerKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AreaTriggerRow {
    pub id: AreaTriggerKey,
    pub map: MapKey,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub radius: f32,
    pub box_length: f32,
    pub box_width: f32,
    pub box_height: f32,
    pub box_yaw: f32,
}

