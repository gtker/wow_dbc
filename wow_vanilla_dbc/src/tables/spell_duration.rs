use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq)]
pub struct SpellDuration {
    pub rows: Vec<SpellDurationRow>,
}

impl DbcTable for SpellDuration {
    type Row = SpellDurationRow;

    fn filename() -> &'static str { "SpellDuration.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 16,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 4 {
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

            // id: primary_key (SpellDuration) uint32
            let id = SpellDurationKey::new(crate::util::read_u32_le(chunk)?);

            // duration: int32
            let duration = crate::util::read_i32_le(chunk)?;

            // duration_per_level: int32
            let duration_per_level = crate::util::read_i32_le(chunk)?;

            // max_duration: int32
            let max_duration = crate::util::read_i32_le(chunk)?;


            rows.push(SpellDurationRow {
                id,
                duration,
                duration_per_level,
                max_duration,
            });
        }

        Ok(SpellDuration { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 4,
            record_size: 16,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SpellDuration) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // duration: int32
            b.write_all(&row.duration.to_le_bytes())?;

            // duration_per_level: int32
            b.write_all(&row.duration_per_level.to_le_bytes())?;

            // max_duration: int32
            b.write_all(&row.max_duration.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellDuration {
    type PrimaryKey = SpellDurationKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct SpellDurationKey {
    pub id: u32
}

impl SpellDurationKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct SpellDurationRow {
    pub id: SpellDurationKey,
    pub duration: i32,
    pub duration_per_level: i32,
    pub max_duration: i32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spell_duration() {
        let contents = include_bytes!("../../../dbc/SpellDuration.dbc");
        let actual = SpellDuration::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellDuration::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
