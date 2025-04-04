use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::map::MapKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaTrigger {
    pub rows: Vec<AreaTriggerRow>,
}

impl DbcTable for AreaTrigger {
    type Row = AreaTriggerRow;

    const FILENAME: &'static str = "AreaTrigger.dbc";
    const FIELD_COUNT: usize = 10;
    const ROW_SIZE: usize = 40;

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

            // id: primary_key (AreaTrigger) int32
            let id = AreaTriggerKey::new(crate::util::read_i32_le(chunk)?);

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // pos: float[3]
            let pos = crate::util::read_array_f32::<3>(chunk)?;

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
                continent_id,
                pos,
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
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (AreaTrigger) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // continent_id: foreign_key (Map) int32
            b.write_all(&(row.continent_id.id as i32).to_le_bytes())?;

            // pos: float[3]
            for i in row.pos {
                b.write_all(&i.to_le_bytes())?;
            }


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
pub struct AreaTriggerKey {
    pub id: i32
}

impl AreaTriggerKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for AreaTriggerKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for AreaTriggerKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for AreaTriggerKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for AreaTriggerKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for AreaTriggerKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for AreaTriggerKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for AreaTriggerKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for AreaTriggerKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for AreaTriggerKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for AreaTriggerKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AreaTriggerRow {
    pub id: AreaTriggerKey,
    pub continent_id: MapKey,
    pub pos: [f32; 3],
    pub radius: f32,
    pub box_length: f32,
    pub box_width: f32,
    pub box_height: f32,
    pub box_yaw: f32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn area_trigger() {
        let mut file = File::open("../tbc-dbc/AreaTrigger.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = AreaTrigger::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = AreaTrigger::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
