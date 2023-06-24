use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::skill_line::SkillLineKey;
use crate::wrath_tables::spell::SpellKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillLineAbility {
    pub rows: Vec<SkillLineAbilityRow>,
}

impl DbcTable for SkillLineAbility {
    type Row = SkillLineAbilityRow;

    const FILENAME: &'static str = "SkillLineAbility.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 56 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 56,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 14 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 14,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SkillLineAbility) int32
            let id = SkillLineAbilityKey::new(crate::util::read_i32_le(chunk)?);

            // skill_line: foreign_key (SkillLine) int32
            let skill_line = SkillLineKey::new(crate::util::read_i32_le(chunk)?.into());

            // spell: foreign_key (Spell) int32
            let spell = SpellKey::new(crate::util::read_i32_le(chunk)?.into());

            // race_mask: int32
            let race_mask = crate::util::read_i32_le(chunk)?;

            // class_mask: int32
            let class_mask = crate::util::read_i32_le(chunk)?;

            // exclude_race: int32
            let exclude_race = crate::util::read_i32_le(chunk)?;

            // exclude_class: int32
            let exclude_class = crate::util::read_i32_le(chunk)?;

            // min_skill_line_rank: int32
            let min_skill_line_rank = crate::util::read_i32_le(chunk)?;

            // superceded_by_spell: foreign_key (Spell) int32
            let superceded_by_spell = SpellKey::new(crate::util::read_i32_le(chunk)?.into());

            // acquire_method: int32
            let acquire_method = crate::util::read_i32_le(chunk)?;

            // trivial_skill_line_rank_high: int32
            let trivial_skill_line_rank_high = crate::util::read_i32_le(chunk)?;

            // trivial_skill_line_rank_low: int32
            let trivial_skill_line_rank_low = crate::util::read_i32_le(chunk)?;

            // character_points: int32[2]
            let character_points = crate::util::read_array_i32::<2>(chunk)?;


            rows.push(SkillLineAbilityRow {
                id,
                skill_line,
                spell,
                race_mask,
                class_mask,
                exclude_race,
                exclude_class,
                min_skill_line_rank,
                superceded_by_spell,
                acquire_method,
                trivial_skill_line_rank_high,
                trivial_skill_line_rank_low,
                character_points,
            });
        }

        Ok(SkillLineAbility { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 14,
            record_size: 56,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SkillLineAbility) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // skill_line: foreign_key (SkillLine) int32
            b.write_all(&(row.skill_line.id as i32).to_le_bytes())?;

            // spell: foreign_key (Spell) int32
            b.write_all(&(row.spell.id as i32).to_le_bytes())?;

            // race_mask: int32
            b.write_all(&row.race_mask.to_le_bytes())?;

            // class_mask: int32
            b.write_all(&row.class_mask.to_le_bytes())?;

            // exclude_race: int32
            b.write_all(&row.exclude_race.to_le_bytes())?;

            // exclude_class: int32
            b.write_all(&row.exclude_class.to_le_bytes())?;

            // min_skill_line_rank: int32
            b.write_all(&row.min_skill_line_rank.to_le_bytes())?;

            // superceded_by_spell: foreign_key (Spell) int32
            b.write_all(&(row.superceded_by_spell.id as i32).to_le_bytes())?;

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
    pub id: i32
}

impl SkillLineAbilityKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SkillLineAbilityKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SkillLineAbilityKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SkillLineAbilityKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillLineAbilityRow {
    pub id: SkillLineAbilityKey,
    pub skill_line: SkillLineKey,
    pub spell: SpellKey,
    pub race_mask: i32,
    pub class_mask: i32,
    pub exclude_race: i32,
    pub exclude_class: i32,
    pub min_skill_line_rank: i32,
    pub superceded_by_spell: SpellKey,
    pub acquire_method: i32,
    pub trivial_skill_line_rank_high: i32,
    pub trivial_skill_line_rank_low: i32,
    pub character_points: [i32; 2],
}

