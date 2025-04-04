use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::vehicle_ui_indicator::VehicleUIIndicatorKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VehicleUIIndSeat {
    pub rows: Vec<VehicleUIIndSeatRow>,
}

impl DbcTable for VehicleUIIndSeat {
    type Row = VehicleUIIndSeatRow;

    const FILENAME: &'static str = "VehicleUIIndSeat.dbc";
    const FIELD_COUNT: usize = 5;
    const ROW_SIZE: usize = 20;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
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
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
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
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VehicleUIIndSeatKey {
    pub id: i32
}

impl VehicleUIIndSeatKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for VehicleUIIndSeatKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for VehicleUIIndSeatKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for VehicleUIIndSeatKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for VehicleUIIndSeatKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for VehicleUIIndSeatKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for VehicleUIIndSeatKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for VehicleUIIndSeatKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for VehicleUIIndSeatKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for VehicleUIIndSeatKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for VehicleUIIndSeatKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VehicleUIIndSeatRow {
    pub id: VehicleUIIndSeatKey,
    pub vehicle_u_i_indicator_id: VehicleUIIndicatorKey,
    pub virtual_seat_index: i32,
    pub x_pos: f32,
    pub y_pos: f32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn vehicle_ui_ind_seat() {
        let mut file = File::open("../wrath-dbc/VehicleUIIndSeat.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = VehicleUIIndSeat::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = VehicleUIIndSeat::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
