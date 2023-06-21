use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
                    expected: 4,
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
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstSpellDuration<const S: usize> {
    pub rows: [SpellDurationRow; S],
}

impl<const S: usize> ConstSpellDuration<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 16 {
            panic!("invalid record size, expected 16")
        }

        if header.field_count != 4 {
            panic!("invalid field count, expected 4")
        }

        let mut b_offset = 20;
        let mut rows = [
            SpellDurationRow {
                id: SpellDurationKey::new(0),
                duration: 0,
                duration_per_level: 0,
                max_duration: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SpellDuration) uint32
            let id = SpellDurationKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // duration: int32
            let duration = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // duration_per_level: int32
            let duration_per_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // max_duration: int32
            let max_duration = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = SpellDurationRow {
                id,
                duration,
                duration_per_level,
                max_duration,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SpellDuration {
        SpellDuration {
            rows: self.rows.iter().map(|s| SpellDurationRow {
                id: s.id,
                duration: s.duration,
                duration_per_level: s.duration_per_level,
                max_duration: s.max_duration,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellDurationKey {
    pub id: u32
}

impl SpellDurationKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SpellDurationKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SpellDurationKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SpellDurationKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellDurationRow {
    pub id: SpellDurationKey,
    pub duration: i32,
    pub duration_per_level: i32,
    pub max_duration: i32,
}

