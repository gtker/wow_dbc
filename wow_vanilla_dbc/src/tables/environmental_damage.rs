use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tables::spell_visual_kit::*;

#[derive(Debug, Clone, PartialEq)]
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

            // id: primary_key (EnvironmentalDamage) uint32
            let id = EnvironmentalDamageKey::new(crate::util::read_u32_le(chunk)?);

            // en: int32
            let en = crate::util::read_i32_le(chunk)?;

            // spell_visual_kit: foreign_key (SpellVisualKit) uint32
            let spell_visual_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(EnvironmentalDamageRow {
                id,
                en,
                spell_visual_kit,
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
            // id: primary_key (EnvironmentalDamage) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // en: int32
            b.write_all(&row.en.to_le_bytes())?;

            // spell_visual_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.spell_visual_kit.id as u32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for EnvironmentalDamage {
    type PrimaryKey = EnvironmentalDamageKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct EnvironmentalDamageKey {
    pub id: u32
}

impl EnvironmentalDamageKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct EnvironmentalDamageRow {
    pub id: EnvironmentalDamageKey,
    pub en: i32,
    pub spell_visual_kit: SpellVisualKitKey,
}

