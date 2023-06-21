use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TransportRotation {
    pub rows: Vec<TransportRotationRow>,
}

impl DbcTable for TransportRotation {
    type Row = TransportRotationRow;

    fn filename() -> &'static str { "TransportRotation.dbc" }

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

            // id: primary_key (TransportRotation) int32
            let id = TransportRotationKey::new(crate::util::read_i32_le(chunk)?);

            // game_objects_id: foreign_key (GameObjects) int32
            let game_objects_id = crate::util::read_i32_le(chunk)?;

            // time_index: int32
            let time_index = crate::util::read_i32_le(chunk)?;

            // rot: float[4]
            let rot = crate::util::read_array_f32::<4>(chunk)?;


            rows.push(TransportRotationRow {
                id,
                game_objects_id,
                time_index,
                rot,
            });
        }

        Ok(TransportRotation { rows, })
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
            // id: primary_key (TransportRotation) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // game_objects_id: foreign_key (GameObjects) int32
            b.write_all(&row.game_objects_id.to_le_bytes())?;

            // time_index: int32
            b.write_all(&row.time_index.to_le_bytes())?;

            // rot: float[4]
            for i in row.rot {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for TransportRotation {
    type PrimaryKey = TransportRotationKey;
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
pub struct ConstTransportRotation<const S: usize> {
    pub rows: [TransportRotationRow; S],
}

impl<const S: usize> ConstTransportRotation<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 28 {
            panic!("invalid record size, expected 28")
        }

        if header.field_count != 7 {
            panic!("invalid field count, expected 7")
        }

        let mut b_offset = 20;
        let mut rows = [
            TransportRotationRow {
                id: TransportRotationKey::new(0),
                game_objects_id: 0,
                time_index: 0,
                rot: [0.0; 4],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (TransportRotation) int32
            let id = TransportRotationKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // game_objects_id: foreign_key (GameObjects) int32
            let game_objects_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // time_index: int32
            let time_index = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // rot: float[4]
            let rot = {
                let mut a = [0.0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = TransportRotationRow {
                id,
                game_objects_id,
                time_index,
                rot,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct TransportRotationKey {
    pub id: i32
}

impl TransportRotationKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for TransportRotationKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for TransportRotationKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for TransportRotationKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for TransportRotationKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for TransportRotationKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TransportRotationRow {
    pub id: TransportRotationKey,
    pub game_objects_id: i32,
    pub time_index: i32,
    pub rot: [f32; 4],
}

