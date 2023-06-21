use crate::header::{HEADER_SIZE, DbcHeader, parse_header};
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::faction_template::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SummonProperties {
    pub rows: Vec<SummonPropertiesRow>,
}

impl DbcTable for SummonProperties {
    type Row = SummonPropertiesRow;

    fn filename() -> &'static str { "SummonProperties.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 24 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 24,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 6 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 6,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SummonProperties) int32
            let id = SummonPropertiesKey::new(crate::util::read_i32_le(chunk)?);

            // control: int32
            let control = crate::util::read_i32_le(chunk)?;

            // faction: foreign_key (FactionTemplate) int32
            let faction = FactionTemplateKey::new(crate::util::read_i32_le(chunk)?.into());

            // title: int32
            let title = crate::util::read_i32_le(chunk)?;

            // slot: int32
            let slot = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(SummonPropertiesRow {
                id,
                control,
                faction,
                title,
                slot,
                flags,
            });
        }

        Ok(SummonProperties { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 6,
            record_size: 24,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SummonProperties) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // control: int32
            b.write_all(&row.control.to_le_bytes())?;

            // faction: foreign_key (FactionTemplate) int32
            b.write_all(&(row.faction.id as i32).to_le_bytes())?;

            // title: int32
            b.write_all(&row.title.to_le_bytes())?;

            // slot: int32
            b.write_all(&row.slot.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SummonProperties {
    type PrimaryKey = SummonPropertiesKey;
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
pub struct SummonPropertiesKey {
    pub id: i32
}

impl SummonPropertiesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SummonPropertiesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SummonPropertiesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SummonPropertiesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SummonPropertiesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SummonPropertiesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SummonPropertiesRow {
    pub id: SummonPropertiesKey,
    pub control: i32,
    pub faction: FactionTemplateKey,
    pub title: i32,
    pub slot: i32,
    pub flags: i32,
}

