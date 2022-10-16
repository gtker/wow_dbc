use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RandPropPoints {
    pub rows: Vec<RandPropPointsRow>,
}

impl DbcTable for RandPropPoints {
    type Row = RandPropPointsRow;

    fn filename() -> &'static str { "RandPropPoints.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 64 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 64,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 16,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (RandPropPoints) int32
            let id = RandPropPointsKey::new(crate::util::read_i32_le(chunk)?);

            // epic: int32[5]
            let epic = crate::util::read_array_i32::<5>(chunk)?;

            // superior: int32[5]
            let superior = crate::util::read_array_i32::<5>(chunk)?;

            // good: int32[5]
            let good = crate::util::read_array_i32::<5>(chunk)?;


            rows.push(RandPropPointsRow {
                id,
                epic,
                superior,
                good,
            });
        }

        Ok(RandPropPoints { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 16,
            record_size: 64,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (RandPropPoints) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // epic: int32[5]
            for i in row.epic {
                b.write_all(&i.to_le_bytes())?;
            }


            // superior: int32[5]
            for i in row.superior {
                b.write_all(&i.to_le_bytes())?;
            }


            // good: int32[5]
            for i in row.good {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for RandPropPoints {
    type PrimaryKey = RandPropPointsKey;
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
pub struct RandPropPointsKey {
    pub id: i32
}

impl RandPropPointsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i32> for RandPropPointsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RandPropPointsRow {
    pub id: RandPropPointsKey,
    pub epic: [i32; 5],
    pub superior: [i32; 5],
    pub good: [i32; 5],
}

