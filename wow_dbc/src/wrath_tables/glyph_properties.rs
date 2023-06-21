use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::spell::*;
use crate::wrath_tables::spell_icon::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlyphProperties {
    pub rows: Vec<GlyphPropertiesRow>,
}

impl DbcTable for GlyphProperties {
    type Row = GlyphPropertiesRow;

    fn filename() -> &'static str { "GlyphProperties.dbc" }

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

            // id: primary_key (GlyphProperties) int32
            let id = GlyphPropertiesKey::new(crate::util::read_i32_le(chunk)?);

            // spell_id: foreign_key (Spell) int32
            let spell_id = SpellKey::new(crate::util::read_i32_le(chunk)?.into());

            // glyph_slot_flags: int32
            let glyph_slot_flags = crate::util::read_i32_le(chunk)?;

            // spell_icon_id: foreign_key (SpellIcon) int32
            let spell_icon_id = SpellIconKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(GlyphPropertiesRow {
                id,
                spell_id,
                glyph_slot_flags,
                spell_icon_id,
            });
        }

        Ok(GlyphProperties { rows, })
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
            // id: primary_key (GlyphProperties) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // spell_id: foreign_key (Spell) int32
            b.write_all(&(row.spell_id.id as i32).to_le_bytes())?;

            // glyph_slot_flags: int32
            b.write_all(&row.glyph_slot_flags.to_le_bytes())?;

            // spell_icon_id: foreign_key (SpellIcon) int32
            b.write_all(&(row.spell_icon_id.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for GlyphProperties {
    type PrimaryKey = GlyphPropertiesKey;
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
pub struct GlyphPropertiesKey {
    pub id: i32
}

impl GlyphPropertiesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for GlyphPropertiesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for GlyphPropertiesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for GlyphPropertiesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for GlyphPropertiesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for GlyphPropertiesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlyphPropertiesRow {
    pub id: GlyphPropertiesKey,
    pub spell_id: SpellKey,
    pub glyph_slot_flags: i32,
    pub spell_icon_id: SpellIconKey,
}

