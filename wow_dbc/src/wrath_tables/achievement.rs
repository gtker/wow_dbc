use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::achievement_category::Achievement_CategoryKey;
use crate::wrath_tables::faction::FactionKey;
use crate::wrath_tables::map::MapKey;
use crate::wrath_tables::spell_icon::SpellIconKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Achievement {
    pub rows: Vec<AchievementRow>,
}

impl DbcTable for Achievement {
    type Row = AchievementRow;

    const FILENAME: &'static str = "Achievement.dbc";
    const FIELD_COUNT: usize = 62;
    const ROW_SIZE: usize = 248;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (Achievement) int32
            let id = AchievementKey::new(crate::util::read_i32_le(chunk)?);

            // faction: foreign_key (Faction) int32
            let faction = FactionKey::new(crate::util::read_i32_le(chunk)?.into());

            // instance_id: foreign_key (Map) int32
            let instance_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // supercedes: foreign_key (Achievement) int32
            let supercedes = AchievementKey::new(crate::util::read_i32_le(chunk)?.into());

            // title_lang: string_ref_loc (Extended)
            let title_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // category: foreign_key (Achievement_Category) int32
            let category = Achievement_CategoryKey::new(crate::util::read_i32_le(chunk)?.into());

            // points: int32
            let points = crate::util::read_i32_le(chunk)?;

            // ui_order: int32
            let ui_order = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // icon_id: foreign_key (SpellIcon) int32
            let icon_id = SpellIconKey::new(crate::util::read_i32_le(chunk)?.into());

            // reward_lang: string_ref_loc (Extended)
            let reward_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // minimum_criteria: int32
            let minimum_criteria = crate::util::read_i32_le(chunk)?;

            // shares_criteria: foreign_key (Achievement) int32
            let shares_criteria = AchievementKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(AchievementRow {
                id,
                faction,
                instance_id,
                supercedes,
                title_lang,
                description_lang,
                category,
                points,
                ui_order,
                flags,
                icon_id,
                reward_lang,
                minimum_criteria,
                shares_criteria,
            });
        }

        Ok(Achievement { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Achievement) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // faction: foreign_key (Faction) int32
            b.write_all(&(row.faction.id as i32).to_le_bytes())?;

            // instance_id: foreign_key (Map) int32
            b.write_all(&(row.instance_id.id as i32).to_le_bytes())?;

            // supercedes: foreign_key (Achievement) int32
            b.write_all(&(row.supercedes.id as i32).to_le_bytes())?;

            // title_lang: string_ref_loc (Extended)
            b.write_all(&row.title_lang.string_indices_as_array(&mut string_index))?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_index))?;

            // category: foreign_key (Achievement_Category) int32
            b.write_all(&(row.category.id as i32).to_le_bytes())?;

            // points: int32
            b.write_all(&row.points.to_le_bytes())?;

            // ui_order: int32
            b.write_all(&row.ui_order.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // icon_id: foreign_key (SpellIcon) int32
            b.write_all(&(row.icon_id.id as i32).to_le_bytes())?;

            // reward_lang: string_ref_loc (Extended)
            b.write_all(&row.reward_lang.string_indices_as_array(&mut string_index))?;

            // minimum_criteria: int32
            b.write_all(&row.minimum_criteria.to_le_bytes())?;

            // shares_criteria: foreign_key (Achievement) int32
            b.write_all(&(row.shares_criteria.id as i32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Achievement {
    type PrimaryKey = AchievementKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl Achievement {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.title_lang.string_block_as_array(b)?;
            row.description_lang.string_block_as_array(b)?;
            row.reward_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.title_lang.string_block_size();
            sum += row.description_lang.string_block_size();
            sum += row.reward_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AchievementKey {
    pub id: i32
}

impl AchievementKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for AchievementKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for AchievementKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for AchievementKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for AchievementKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for AchievementKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for AchievementKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for AchievementKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for AchievementKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for AchievementKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for AchievementKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AchievementRow {
    pub id: AchievementKey,
    pub faction: FactionKey,
    pub instance_id: MapKey,
    pub supercedes: AchievementKey,
    pub title_lang: ExtendedLocalizedString,
    pub description_lang: ExtendedLocalizedString,
    pub category: Achievement_CategoryKey,
    pub points: i32,
    pub ui_order: i32,
    pub flags: i32,
    pub icon_id: SpellIconKey,
    pub reward_lang: ExtendedLocalizedString,
    pub minimum_criteria: i32,
    pub shares_criteria: AchievementKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn achievement() {
        let mut file = File::open("../wrath-dbc/Achievement.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Achievement::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Achievement::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
