use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::animation_data::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TransportAnimation {
    pub rows: Vec<TransportAnimationRow>,
}

impl DbcTable for TransportAnimation {
    type Row = TransportAnimationRow;

    fn filename() -> &'static str { "TransportAnimation.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 28 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 28,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 7 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 7,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (TransportAnimation) uint32
            let id = TransportAnimationKey::new(crate::util::read_u32_le(chunk)?);

            // transport: uint32
            let transport = crate::util::read_u32_le(chunk)?;

            // time_index: int32
            let time_index = crate::util::read_i32_le(chunk)?;

            // location_x: float
            let location_x = crate::util::read_f32_le(chunk)?;

            // location_y: float
            let location_y = crate::util::read_f32_le(chunk)?;

            // location_z: float
            let location_z = crate::util::read_f32_le(chunk)?;

            // sequence: foreign_key (AnimationData) uint32
            let sequence = AnimationDataKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(TransportAnimationRow {
                id,
                transport,
                time_index,
                location_x,
                location_y,
                location_z,
                sequence,
            });
        }

        Ok(TransportAnimation { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 7,
            record_size: 28,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (TransportAnimation) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // transport: uint32
            b.write_all(&row.transport.to_le_bytes())?;

            // time_index: int32
            b.write_all(&row.time_index.to_le_bytes())?;

            // location_x: float
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: float
            b.write_all(&row.location_y.to_le_bytes())?;

            // location_z: float
            b.write_all(&row.location_z.to_le_bytes())?;

            // sequence: foreign_key (AnimationData) uint32
            b.write_all(&(row.sequence.id as u32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for TransportAnimation {
    type PrimaryKey = TransportAnimationKey;
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
pub struct TransportAnimationKey {
    pub id: u32
}

impl TransportAnimationKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for TransportAnimationKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for TransportAnimationKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for TransportAnimationKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TransportAnimationRow {
    pub id: TransportAnimationKey,
    pub transport: u32,
    pub time_index: i32,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub sequence: AnimationDataKey,
}

