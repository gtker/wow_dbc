use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::skill_line::*;
use crate::wrath_tables::skill_tiers::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillRaceClassInfo {
    pub rows: Vec<SkillRaceClassInfoRow>,
}

impl DbcTable for SkillRaceClassInfo {
    type Row = SkillRaceClassInfoRow;

    fn filename() -> &'static str { "SkillRaceClassInfo.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 8,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SkillRaceClassInfo) int32
            let id = SkillRaceClassInfoKey::new(crate::util::read_i32_le(chunk)?);

            // skill_id: foreign_key (SkillLine) int32
            let skill_id = SkillLineKey::new(crate::util::read_i32_le(chunk)?.into());

            // race_mask: int32
            let race_mask = crate::util::read_i32_le(chunk)?;

            // class_mask: int32
            let class_mask = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;

            // skill_tier_id: foreign_key (SkillTiers) int32
            let skill_tier_id = SkillTiersKey::new(crate::util::read_i32_le(chunk)?.into());

            // skill_cost_index: int32
            let skill_cost_index = crate::util::read_i32_le(chunk)?;


            rows.push(SkillRaceClassInfoRow {
                id,
                skill_id,
                race_mask,
                class_mask,
                flags,
                min_level,
                skill_tier_id,
                skill_cost_index,
            });
        }

        Ok(SkillRaceClassInfo { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 8,
            record_size: 32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SkillRaceClassInfo) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // skill_id: foreign_key (SkillLine) int32
            b.write_all(&(row.skill_id.id as i32).to_le_bytes())?;

            // race_mask: int32
            b.write_all(&row.race_mask.to_le_bytes())?;

            // class_mask: int32
            b.write_all(&row.class_mask.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

            // skill_tier_id: foreign_key (SkillTiers) int32
            b.write_all(&(row.skill_tier_id.id as i32).to_le_bytes())?;

            // skill_cost_index: int32
            b.write_all(&row.skill_cost_index.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SkillRaceClassInfo {
    type PrimaryKey = SkillRaceClassInfoKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct SkillRaceClassInfoKey {
    pub id: i32
}

impl SkillRaceClassInfoKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillRaceClassInfoRow {
    pub id: SkillRaceClassInfoKey,
    pub skill_id: SkillLineKey,
    pub race_mask: i32,
    pub class_mask: i32,
    pub flags: i32,
    pub min_level: i32,
    pub skill_tier_id: SkillTiersKey,
    pub skill_cost_index: i32,
}
