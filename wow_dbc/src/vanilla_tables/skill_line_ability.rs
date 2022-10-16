use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::chr_classes::*;
use crate::vanilla_tables::chr_races::*;
use crate::vanilla_tables::skill_line::*;
use crate::vanilla_tables::spell::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillLineAbility {
    pub rows: Vec<SkillLineAbilityRow>,
}

impl DbcTable for SkillLineAbility {
    type Row = SkillLineAbilityRow;

    fn filename() -> &'static str { "SkillLineAbility.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 60 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 60,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 15 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 15,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SkillLineAbility) uint32
            let id = SkillLineAbilityKey::new(crate::util::read_u32_le(chunk)?);

            // skill_line: foreign_key (SkillLine) uint32
            let skill_line = SkillLineKey::new(crate::util::read_u32_le(chunk)?.into());

            // spell: foreign_key (Spell) uint32
            let spell = SpellKey::new(crate::util::read_u32_le(chunk)?.into());

            // race_mask: foreign_key (ChrRaces) uint32
            let race_mask = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // class_mask: foreign_key (ChrClasses) uint32
            let class_mask = ChrClassesKey::new(crate::util::read_u32_le(chunk)?.into());

            // exclude_race: foreign_key (ChrRaces) uint32
            let exclude_race = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // exclude_class: foreign_key (ChrClasses) uint32
            let exclude_class = ChrClassesKey::new(crate::util::read_u32_le(chunk)?.into());

            // superseded_by: foreign_key (Spell) uint32
            let superseded_by = SpellKey::new(crate::util::read_u32_le(chunk)?.into());

            // acquire_method: int32
            let acquire_method = crate::util::read_i32_le(chunk)?;

            // trivial_skill_line_rank_high: int32
            let trivial_skill_line_rank_high = crate::util::read_i32_le(chunk)?;

            // trivial_skill_line_rank_low: int32
            let trivial_skill_line_rank_low = crate::util::read_i32_le(chunk)?;

            // character_points: int32[2]
            let character_points = crate::util::read_array_i32::<2>(chunk)?;

            // num_skills_up: int32
            let num_skills_up = crate::util::read_i32_le(chunk)?;

            // unknown_padding: int32
            let unknown_padding = crate::util::read_i32_le(chunk)?;


            rows.push(SkillLineAbilityRow {
                id,
                skill_line,
                spell,
                race_mask,
                class_mask,
                exclude_race,
                exclude_class,
                superseded_by,
                acquire_method,
                trivial_skill_line_rank_high,
                trivial_skill_line_rank_low,
                character_points,
                num_skills_up,
                unknown_padding,
            });
        }

        Ok(SkillLineAbility { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 15,
            record_size: 60,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SkillLineAbility) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // skill_line: foreign_key (SkillLine) uint32
            b.write_all(&(row.skill_line.id as u32).to_le_bytes())?;

            // spell: foreign_key (Spell) uint32
            b.write_all(&(row.spell.id as u32).to_le_bytes())?;

            // race_mask: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race_mask.id as u32).to_le_bytes())?;

            // class_mask: foreign_key (ChrClasses) uint32
            b.write_all(&(row.class_mask.id as u32).to_le_bytes())?;

            // exclude_race: foreign_key (ChrRaces) uint32
            b.write_all(&(row.exclude_race.id as u32).to_le_bytes())?;

            // exclude_class: foreign_key (ChrClasses) uint32
            b.write_all(&(row.exclude_class.id as u32).to_le_bytes())?;

            // superseded_by: foreign_key (Spell) uint32
            b.write_all(&(row.superseded_by.id as u32).to_le_bytes())?;

            // acquire_method: int32
            b.write_all(&row.acquire_method.to_le_bytes())?;

            // trivial_skill_line_rank_high: int32
            b.write_all(&row.trivial_skill_line_rank_high.to_le_bytes())?;

            // trivial_skill_line_rank_low: int32
            b.write_all(&row.trivial_skill_line_rank_low.to_le_bytes())?;

            // character_points: int32[2]
            for i in row.character_points {
                b.write_all(&i.to_le_bytes())?;
            }


            // num_skills_up: int32
            b.write_all(&row.num_skills_up.to_le_bytes())?;

            // unknown_padding: int32
            b.write_all(&row.unknown_padding.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SkillLineAbility {
    type PrimaryKey = SkillLineAbilityKey;
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
pub struct SkillLineAbilityKey {
    pub id: u32
}

impl SkillLineAbilityKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SkillLineAbilityKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SkillLineAbilityKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SkillLineAbilityKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillLineAbilityRow {
    pub id: SkillLineAbilityKey,
    pub skill_line: SkillLineKey,
    pub spell: SpellKey,
    pub race_mask: ChrRacesKey,
    pub class_mask: ChrClassesKey,
    pub exclude_race: ChrRacesKey,
    pub exclude_class: ChrClassesKey,
    pub superseded_by: SpellKey,
    pub acquire_method: i32,
    pub trivial_skill_line_rank_high: i32,
    pub trivial_skill_line_rank_low: i32,
    pub character_points: [i32; 2],
    pub num_skills_up: i32,
    pub unknown_padding: i32,
}

