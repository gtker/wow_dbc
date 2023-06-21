use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::spell_visual_kit::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnvironmentalDamage {
    pub rows: Vec<EnvironmentalDamageRow>,
}

impl DbcTable for EnvironmentalDamage {
    type Row = EnvironmentalDamageRow;

    fn filename() -> &'static str { "EnvironmentalDamage.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 12 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 12,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 3 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 3,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (EnvironmentalDamage) int32
            let id = EnvironmentalDamageKey::new(crate::util::read_i32_le(chunk)?);

            // enum_id: int32
            let enum_id = crate::util::read_i32_le(chunk)?;

            // visualkit_id: foreign_key (SpellVisualKit) int32
            let visualkit_id = SpellVisualKitKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(EnvironmentalDamageRow {
                id,
                enum_id,
                visualkit_id,
            });
        }

        Ok(EnvironmentalDamage { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 3,
            record_size: 12,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (EnvironmentalDamage) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // enum_id: int32
            b.write_all(&row.enum_id.to_le_bytes())?;

            // visualkit_id: foreign_key (SpellVisualKit) int32
            b.write_all(&(row.visualkit_id.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for EnvironmentalDamage {
    type PrimaryKey = EnvironmentalDamageKey;
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
pub struct ConstEnvironmentalDamage<const S: usize> {
    pub rows: [EnvironmentalDamageRow; S],
}

impl<const S: usize> ConstEnvironmentalDamage<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 12 {
            panic!("invalid record size, expected 12")
        }

        if header.field_count != 3 {
            panic!("invalid field count, expected 3")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            EnvironmentalDamageRow {
                id: EnvironmentalDamageKey::new(0),
                enum_id: 0,
                visualkit_id: SpellVisualKitKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (EnvironmentalDamage) int32
            let id = EnvironmentalDamageKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // enum_id: int32
            let enum_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // visualkit_id: foreign_key (SpellVisualKit) int32
            let visualkit_id = SpellVisualKitKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = EnvironmentalDamageRow {
                id,
                enum_id,
                visualkit_id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> EnvironmentalDamage {
        EnvironmentalDamage {
            rows: self.rows.iter().map(|s| EnvironmentalDamageRow {
                id: s.id,
                enum_id: s.enum_id,
                visualkit_id: s.visualkit_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct EnvironmentalDamageKey {
    pub id: i32
}

impl EnvironmentalDamageKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for EnvironmentalDamageKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for EnvironmentalDamageKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for EnvironmentalDamageKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for EnvironmentalDamageKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for EnvironmentalDamageKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnvironmentalDamageRow {
    pub id: EnvironmentalDamageKey,
    pub enum_id: i32,
    pub visualkit_id: SpellVisualKitKey,
}

