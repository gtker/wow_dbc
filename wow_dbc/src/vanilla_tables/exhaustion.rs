use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Exhaustion {
    pub rows: Vec<ExhaustionRow>,
}

impl DbcTable for Exhaustion {
    type Row = ExhaustionRow;

    const FILENAME: &'static str = "Exhaustion.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 60 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 60,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 15 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 15,
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

            // id: primary_key (Exhaustion) uint32
            let id = ExhaustionKey::new(crate::util::read_u32_le(chunk)?);

            // experience: int32
            let experience = crate::util::read_i32_le(chunk)?;

            // factor: float
            let factor = crate::util::read_f32_le(chunk)?;

            // outdoor_hours: float
            let outdoor_hours = crate::util::read_f32_le(chunk)?;

            // inn_hours: float
            let inn_hours = crate::util::read_f32_le(chunk)?;

            // state_name: string_ref_loc
            let state_name = crate::util::read_localized_string(chunk, &string_block)?;

            // threshold: float
            let threshold = crate::util::read_f32_le(chunk)?;


            rows.push(ExhaustionRow {
                id,
                experience,
                factor,
                outdoor_hours,
                inn_hours,
                state_name,
                threshold,
            });
        }

        Ok(Exhaustion { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 15,
            record_size: 60,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Exhaustion) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // experience: int32
            b.write_all(&row.experience.to_le_bytes())?;

            // factor: float
            b.write_all(&row.factor.to_le_bytes())?;

            // outdoor_hours: float
            b.write_all(&row.outdoor_hours.to_le_bytes())?;

            // inn_hours: float
            b.write_all(&row.inn_hours.to_le_bytes())?;

            // state_name: string_ref_loc
            b.write_all(&row.state_name.string_indices_as_array(&mut string_index))?;

            // threshold: float
            b.write_all(&row.threshold.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Exhaustion {
    type PrimaryKey = ExhaustionKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl Exhaustion {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.state_name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.state_name.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ExhaustionKey {
    pub id: u32
}

impl ExhaustionKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for ExhaustionKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for ExhaustionKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for ExhaustionKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for ExhaustionKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ExhaustionKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for ExhaustionKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for ExhaustionKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for ExhaustionKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ExhaustionKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ExhaustionKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ExhaustionRow {
    pub id: ExhaustionKey,
    pub experience: i32,
    pub factor: f32,
    pub outdoor_hours: f32,
    pub inn_hours: f32,
    pub state_name: LocalizedString,
    pub threshold: f32,
}

