use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq)]
pub struct SpellCastTimes {
    pub rows: Vec<SpellCastTimesRow>,
}

impl DbcTable for SpellCastTimes {
    type Row = SpellCastTimesRow;

    fn filename() -> &'static str { "SpellCastTimes.dbc" }

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

            // id: primary_key (SpellCastTimes) uint32
            let id = SpellCastTimesKey::new(crate::util::read_u32_le(chunk)?);

            // base: int32
            let base = crate::util::read_i32_le(chunk)?;

            // per_level_increase: int32
            let per_level_increase = crate::util::read_i32_le(chunk)?;

            // minimum: int32
            let minimum = crate::util::read_i32_le(chunk)?;


            rows.push(SpellCastTimesRow {
                id,
                base,
                per_level_increase,
                minimum,
            });
        }

        Ok(SpellCastTimes { rows, })
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
            // id: primary_key (SpellCastTimes) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // base: int32
            b.write_all(&row.base.to_le_bytes())?;

            // per_level_increase: int32
            b.write_all(&row.per_level_increase.to_le_bytes())?;

            // minimum: int32
            b.write_all(&row.minimum.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellCastTimes {
    type PrimaryKey = SpellCastTimesKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct SpellCastTimesKey {
    pub id: u32
}

impl SpellCastTimesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct SpellCastTimesRow {
    pub id: SpellCastTimesKey,
    pub base: i32,
    pub per_level_increase: i32,
    pub minimum: i32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spell_cast_times() {
        let contents = include_bytes!("../../../dbc/SpellCastTimes.dbc");
        let actual = SpellCastTimes::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellCastTimes::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
