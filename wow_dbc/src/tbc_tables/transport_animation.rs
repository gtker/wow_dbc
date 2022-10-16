use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

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

            // id: primary_key (TransportAnimation) int32
            let id = TransportAnimationKey::new(crate::util::read_i32_le(chunk)?);

            // transport_id: int32
            let transport_id = crate::util::read_i32_le(chunk)?;

            // time_index: int32
            let time_index = crate::util::read_i32_le(chunk)?;

            // pos: float[3]
            let pos = crate::util::read_array_f32::<3>(chunk)?;

            // sequence_id: int32
            let sequence_id = crate::util::read_i32_le(chunk)?;


            rows.push(TransportAnimationRow {
                id,
                transport_id,
                time_index,
                pos,
                sequence_id,
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
            // id: primary_key (TransportAnimation) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // transport_id: int32
            b.write_all(&row.transport_id.to_le_bytes())?;

            // time_index: int32
            b.write_all(&row.time_index.to_le_bytes())?;

            // pos: float[3]
            for i in row.pos {
                b.write_all(&i.to_le_bytes())?;
            }


            // sequence_id: int32
            b.write_all(&row.sequence_id.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for TransportAnimation {
    type PrimaryKey = TransportAnimationKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct TransportAnimationKey {
    pub id: i32
}

impl TransportAnimationKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TransportAnimationRow {
    pub id: TransportAnimationKey,
    pub transport_id: i32,
    pub time_index: i32,
    pub pos: [f32; 3],
    pub sequence_id: i32,
}

