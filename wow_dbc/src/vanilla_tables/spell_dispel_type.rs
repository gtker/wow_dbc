use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellDispelType {
    pub rows: Vec<SpellDispelTypeRow>,
}

impl DbcTable for SpellDispelType {
    type Row = SpellDispelTypeRow;

    fn filename() -> &'static str { "SpellDispelType.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 48 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 48,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 12 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 12,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SpellDispelType) uint32
            let id = SpellDispelTypeKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // mask: int32
            let mask = crate::util::read_i32_le(chunk)?;

            // immunity_possible: uint32
            let immunity_possible = crate::util::read_u32_le(chunk)?;


            rows.push(SpellDispelTypeRow {
                id,
                name,
                mask,
                immunity_possible,
            });
        }

        Ok(SpellDispelType { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 12,
            record_size: 48,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SpellDispelType) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // mask: int32
            b.write_all(&row.mask.to_le_bytes())?;

            // immunity_possible: uint32
            b.write_all(&row.immunity_possible.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SpellDispelType {
    type PrimaryKey = SpellDispelTypeKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl SpellDispelType {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstSpellDispelType<const S: usize> {
    pub rows: [ConstSpellDispelTypeRow; S],
}

impl<const S: usize> ConstSpellDispelType<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 48 {
            panic!("invalid record size, expected 48")
        }

        if header.field_count != 12 {
            panic!("invalid field count, expected 12")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstSpellDispelTypeRow {
                id: SpellDispelTypeKey::new(0),
                name: crate::ConstLocalizedString::empty(),
                mask: 0,
                immunity_possible: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SpellDispelType) uint32
            let id = SpellDispelTypeKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // name: string_ref_loc
            let name = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            // mask: int32
            let mask = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // immunity_possible: uint32
            let immunity_possible = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstSpellDispelTypeRow {
                id,
                name,
                mask,
                immunity_possible,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SpellDispelType {
        SpellDispelType {
            rows: self.rows.iter().map(|s| SpellDispelTypeRow {
                id: s.id,
                name: s.name.to_string(),
                mask: s.mask,
                immunity_possible: s.immunity_possible,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellDispelTypeKey {
    pub id: u32
}

impl SpellDispelTypeKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SpellDispelTypeKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SpellDispelTypeKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SpellDispelTypeKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellDispelTypeRow {
    pub id: SpellDispelTypeKey,
    pub name: LocalizedString,
    pub mask: i32,
    pub immunity_possible: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstSpellDispelTypeRow {
    pub id: SpellDispelTypeKey,
    pub name: ConstLocalizedString,
    pub mask: i32,
    pub immunity_possible: u32,
}

