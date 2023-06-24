use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::chr_classes::ChrClassesKey;
use crate::vanilla_tables::chr_races::ChrRacesKey;
use crate::vanilla_tables::skill_costs_data::SkillCostsDataKey;
use crate::vanilla_tables::skill_line::SkillLineKey;
use crate::vanilla_tables::skill_tiers::SkillTiersKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillRaceClassInfo {
    pub rows: Vec<SkillRaceClassInfoRow>,
}

impl DbcTable for SkillRaceClassInfo {
    type Row = SkillRaceClassInfoRow;

    const FILENAME: &'static str = "SkillRaceClassInfo.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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

            // id: primary_key (SkillRaceClassInfo) uint32
            let id = SkillRaceClassInfoKey::new(crate::util::read_u32_le(chunk)?);

            // skill_line: foreign_key (SkillLine) uint32
            let skill_line = SkillLineKey::new(crate::util::read_u32_le(chunk)?.into());

            // race_mask: foreign_key (ChrRaces) uint32
            let race_mask = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // class_mask: foreign_key (ChrClasses) uint32
            let class_mask = ChrClassesKey::new(crate::util::read_u32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;

            // skill_tier: foreign_key (SkillTiers) uint32
            let skill_tier = SkillTiersKey::new(crate::util::read_u32_le(chunk)?.into());

            // skill_cost: foreign_key (SkillCostsData) uint32
            let skill_cost = SkillCostsDataKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(SkillRaceClassInfoRow {
                id,
                skill_line,
                race_mask,
                class_mask,
                flags,
                min_level,
                skill_tier,
                skill_cost,
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
            // id: primary_key (SkillRaceClassInfo) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // skill_line: foreign_key (SkillLine) uint32
            b.write_all(&(row.skill_line.id as u32).to_le_bytes())?;

            // race_mask: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race_mask.id as u32).to_le_bytes())?;

            // class_mask: foreign_key (ChrClasses) uint32
            b.write_all(&(row.class_mask.id as u32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

            // skill_tier: foreign_key (SkillTiers) uint32
            b.write_all(&(row.skill_tier.id as u32).to_le_bytes())?;

            // skill_cost: foreign_key (SkillCostsData) uint32
            b.write_all(&(row.skill_cost.id as u32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SkillRaceClassInfo {
    type PrimaryKey = SkillRaceClassInfoKey;
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
pub struct SkillRaceClassInfoKey {
    pub id: u32
}

impl SkillRaceClassInfoKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SkillRaceClassInfoKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for SkillRaceClassInfoKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for SkillRaceClassInfoKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<i8> for SkillRaceClassInfoKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for SkillRaceClassInfoKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for SkillRaceClassInfoKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillRaceClassInfoRow {
    pub id: SkillRaceClassInfoKey,
    pub skill_line: SkillLineKey,
    pub race_mask: ChrRacesKey,
    pub class_mask: ChrClassesKey,
    pub flags: i32,
    pub min_level: i32,
    pub skill_tier: SkillTiersKey,
    pub skill_cost: SkillCostsDataKey,
}

