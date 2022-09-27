use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::vehicle_ui_indicator::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VehicleUIIndSeat {
    pub rows: Vec<VehicleUIIndSeatRow>,
}

impl DbcTable for VehicleUIIndSeat {
    type Row = VehicleUIIndSeatRow;

    fn filename() -> &'static str { "VehicleUIIndSeat.dbc" }

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

            // id: primary_key (VehicleUIIndSeat) int32
            let id = VehicleUIIndSeatKey::new(crate::util::read_i32_le(chunk)?);

            // vehicle_u_i_indicator_id: foreign_key (VehicleUIIndicator) int32
            let vehicle_u_i_indicator_id = VehicleUIIndicatorKey::new(crate::util::read_i32_le(chunk)?.into());

            // virtual_seat_index: int32
            let virtual_seat_index = crate::util::read_i32_le(chunk)?;

            // x_pos: float
            let x_pos = crate::util::read_f32_le(chunk)?;

            // y_pos: float
            let y_pos = crate::util::read_f32_le(chunk)?;


            rows.push(VehicleUIIndSeatRow {
                id,
                vehicle_u_i_indicator_id,
                virtual_seat_index,
                x_pos,
                y_pos,
            });
        }

        Ok(VehicleUIIndSeat { rows, })
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
            // id: primary_key (VehicleUIIndSeat) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // vehicle_u_i_indicator_id: foreign_key (VehicleUIIndicator) int32
            b.write_all(&(row.vehicle_u_i_indicator_id.id as i32).to_le_bytes())?;

            // virtual_seat_index: int32
            b.write_all(&row.virtual_seat_index.to_le_bytes())?;

            // x_pos: float
            b.write_all(&row.x_pos.to_le_bytes())?;

            // y_pos: float
            b.write_all(&row.y_pos.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for VehicleUIIndSeat {
    type PrimaryKey = VehicleUIIndSeatKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct VehicleUIIndSeatKey {
    pub id: i32
}

impl VehicleUIIndSeatKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct VehicleUIIndSeatRow {
    pub id: VehicleUIIndSeatKey,
    pub vehicle_u_i_indicator_id: VehicleUIIndicatorKey,
    pub virtual_seat_index: i32,
    pub x_pos: f32,
    pub y_pos: f32,
}
