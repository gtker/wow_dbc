use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};
use crate::wrath_tables::achievement_category::*;
use crate::wrath_tables::faction::*;
use crate::wrath_tables::map::*;
use crate::wrath_tables::spell_icon::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Achievement {
    pub rows: Vec<AchievementRow>,
}

impl DbcTable for Achievement {
    type Row = AchievementRow;

    fn filename() -> &'static str { "Achievement.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 248 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 248,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 62 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 62,
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
            field_count: 62,
            record_size: 248,
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
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstAchievement<const S: usize> {
    pub rows: [ConstAchievementRow; S],
}

impl<const S: usize> ConstAchievement<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 248 {
            panic!("invalid record size, expected 248")
        }

        if header.field_count != 62 {
            panic!("invalid field count, expected 62")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstAchievementRow {
                id: AchievementKey::new(0),
                faction: FactionKey::new(0),
                instance_id: MapKey::new(0),
                supercedes: AchievementKey::new(0),
                title_lang: crate::ConstExtendedLocalizedString::empty(),
                description_lang: crate::ConstExtendedLocalizedString::empty(),
                category: Achievement_CategoryKey::new(0),
                points: 0,
                ui_order: 0,
                flags: 0,
                icon_id: SpellIconKey::new(0),
                reward_lang: crate::ConstExtendedLocalizedString::empty(),
                minimum_criteria: 0,
                shares_criteria: AchievementKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Achievement) int32
            let id = AchievementKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // faction: foreign_key (Faction) int32
            let faction = FactionKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // instance_id: foreign_key (Map) int32
            let instance_id = MapKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // supercedes: foreign_key (Achievement) int32
            let supercedes = AchievementKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // title_lang: string_ref_loc (Extended)
            let title_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // description_lang: string_ref_loc (Extended)
            let description_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // category: foreign_key (Achievement_Category) int32
            let category = Achievement_CategoryKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // points: int32
            let points = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // ui_order: int32
            let ui_order = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // icon_id: foreign_key (SpellIcon) int32
            let icon_id = SpellIconKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // reward_lang: string_ref_loc (Extended)
            let reward_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // minimum_criteria: int32
            let minimum_criteria = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // shares_criteria: foreign_key (Achievement) int32
            let shares_criteria = AchievementKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = ConstAchievementRow {
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
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> Achievement {
        Achievement {
            rows: self.rows.iter().map(|s| AchievementRow {
                id: s.id,
                faction: s.faction,
                instance_id: s.instance_id,
                supercedes: s.supercedes,
                title_lang: s.title_lang.to_string(),
                description_lang: s.description_lang.to_string(),
                category: s.category,
                points: s.points,
                ui_order: s.ui_order,
                flags: s.flags,
                icon_id: s.icon_id,
                reward_lang: s.reward_lang.to_string(),
                minimum_criteria: s.minimum_criteria,
                shares_criteria: s.shares_criteria,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct AchievementKey {
    pub id: i32
}

impl AchievementKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstAchievementRow {
    pub id: AchievementKey,
    pub faction: FactionKey,
    pub instance_id: MapKey,
    pub supercedes: AchievementKey,
    pub title_lang: ConstExtendedLocalizedString,
    pub description_lang: ConstExtendedLocalizedString,
    pub category: Achievement_CategoryKey,
    pub points: i32,
    pub ui_order: i32,
    pub flags: i32,
    pub icon_id: SpellIconKey,
    pub reward_lang: ConstExtendedLocalizedString,
    pub minimum_criteria: i32,
    pub shares_criteria: AchievementKey,
}

