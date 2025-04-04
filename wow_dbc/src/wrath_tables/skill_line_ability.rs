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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkillLineAbility {
    pub rows: Vec<SkillLineAbilityRow>,
}

impl DbcTable for SkillLineAbility {
    type Row = SkillLineAbilityRow;

    const FILENAME: &'static str = "SkillLineAbility.dbc";
    const FIELD_COUNT: usize = 14;
    const ROW_SIZE: usize = 56;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
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
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
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
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkillLineAbilityKey {
    pub id: i32
}

impl SkillLineAbilityKey {
    pub const fn new(id: i32) -> Self {
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

impl TryFrom<u32> for SkillLineAbilityKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SkillLineAbilityKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for SkillLineAbilityKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SkillLineAbilityKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SkillLineAbilityKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn skill_line_ability() {
        let mut file = File::open("../wrath-dbc/SkillLineAbility.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SkillLineAbility::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SkillLineAbility::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
