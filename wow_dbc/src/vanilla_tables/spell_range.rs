use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellRange {
    pub rows: Vec<SpellRangeRow>,
}

impl DbcTable for SpellRange {
    type Row = SpellRangeRow;

    fn filename() -> &'static str { "SpellRange.dbc" }

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SpellRange) uint32
            let id = SpellRangeKey::new(crate::util::read_u32_le(chunk)?);

            // range_min: float
            let range_min = crate::util::read_f32_le(chunk)?;

            // range_max: float
            let range_max = crate::util::read_f32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // display_name: string_ref_loc
            let display_name = crate::util::read_localized_string(chunk, &string_block)?;

            // display_name_short: string_ref_loc
            let display_name_short = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(SpellRangeRow {
                id,
                range_min,
                range_max,
                flags,
                display_name,
                display_name_short,
            });
        }

        Ok(SpellRange { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 22,
            record_size: 88,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SpellRange) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // range_min: float
            b.write_all(&row.range_min.to_le_bytes())?;

            // range_max: float
            b.write_all(&row.range_max.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // display_name: string_ref_loc
            b.write_all(&row.display_name.string_indices_as_array(&mut string_index))?;

            // display_name_short: string_ref_loc
            b.write_all(&row.display_name_short.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SpellRange {
    type PrimaryKey = SpellRangeKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl SpellRange {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.display_name.string_block_as_array(b)?;
            row.display_name_short.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.display_name.string_block_size();
            sum += row.display_name_short.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstSpellRange<const S: usize> {
    pub rows: [ConstSpellRangeRow; S],
}

impl<const S: usize> ConstSpellRange<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 88 {
            panic!("invalid record size, expected 88")
        }

        if header.field_count != 22 {
            panic!("invalid field count, expected 22")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstSpellRangeRow {
                id: SpellRangeKey::new(0),
                range_min: 0.0,
                range_max: 0.0,
                flags: 0,
                display_name: crate::ConstLocalizedString::empty(),
                display_name_short: crate::ConstLocalizedString::empty(),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SpellRange) uint32
            let id = SpellRangeKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // range_min: float
            let range_min = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // range_max: float
            let range_max = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // display_name: string_ref_loc
            let display_name = ConstLocalizedString::new(
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

            // display_name_short: string_ref_loc
            let display_name_short = ConstLocalizedString::new(
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

            rows[i] = ConstSpellRangeRow {
                id,
                range_min,
                range_max,
                flags,
                display_name,
                display_name_short,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SpellRange {
        SpellRange {
            rows: self.rows.iter().map(|s| SpellRangeRow {
                id: s.id,
                range_min: s.range_min,
                range_max: s.range_max,
                flags: s.flags,
                display_name: s.display_name.to_string(),
                display_name_short: s.display_name_short.to_string(),
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellRangeKey {
    pub id: u32
}

impl SpellRangeKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SpellRangeKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SpellRangeKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SpellRangeKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellRangeRow {
    pub id: SpellRangeKey,
    pub range_min: f32,
    pub range_max: f32,
    pub flags: i32,
    pub display_name: LocalizedString,
    pub display_name_short: LocalizedString,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstSpellRangeRow {
    pub id: SpellRangeKey,
    pub range_min: f32,
    pub range_max: f32,
    pub flags: i32,
    pub display_name: ConstLocalizedString,
    pub display_name_short: ConstLocalizedString,
}

