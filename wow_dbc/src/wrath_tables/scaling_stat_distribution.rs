use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScalingStatDistribution {
    pub rows: Vec<ScalingStatDistributionRow>,
}

impl DbcTable for ScalingStatDistribution {
    type Row = ScalingStatDistributionRow;

    fn filename() -> &'static str { "ScalingStatDistribution.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 88 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 88,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 22 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 22,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (ScalingStatDistribution) int32
            let id = ScalingStatDistributionKey::new(crate::util::read_i32_le(chunk)?);

            // stat_id: int32[10]
            let stat_id = crate::util::read_array_i32::<10>(chunk)?;

            // bonus: int32[10]
            let bonus = crate::util::read_array_i32::<10>(chunk)?;

            // maxlevel: int32
            let maxlevel = crate::util::read_i32_le(chunk)?;


            rows.push(ScalingStatDistributionRow {
                id,
                stat_id,
                bonus,
                maxlevel,
            });
        }

        Ok(ScalingStatDistribution { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 22,
            record_size: 88,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (ScalingStatDistribution) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // stat_id: int32[10]
            for i in row.stat_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // bonus: int32[10]
            for i in row.bonus {
                b.write_all(&i.to_le_bytes())?;
            }


            // maxlevel: int32
            b.write_all(&row.maxlevel.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for ScalingStatDistribution {
    type PrimaryKey = ScalingStatDistributionKey;
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
pub struct ScalingStatDistributionKey {
    pub id: i32
}

impl ScalingStatDistributionKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i32> for ScalingStatDistributionKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScalingStatDistributionRow {
    pub id: ScalingStatDistributionKey,
    pub stat_id: [i32; 10],
    pub bonus: [i32; 10],
    pub maxlevel: i32,
}

