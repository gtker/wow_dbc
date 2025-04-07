use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use crate::wrath_tables::achievement::AchievementKey;
use std::io::Write;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Achievement_Criteria {
    pub rows: Vec<Achievement_CriteriaRow>,
}

impl DbcTable for Achievement_Criteria {
    type Row = Achievement_CriteriaRow;

    const FILENAME: &'static str = "Achievement_Criteria.dbc";
    const FIELD_COUNT: usize = 31;
    const ROW_SIZE: usize = 124;

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

            // id: primary_key (Achievement_Criteria) int32
            let id = Achievement_CriteriaKey::new(crate::util::read_i32_le(chunk)?);

            // achievement_id: foreign_key (Achievement) int32
            let achievement_id = AchievementKey::new(crate::util::read_i32_le(chunk)?.into());

            // ty: int32
            let ty = crate::util::read_i32_le(chunk)?;

            // asset_id: int32
            let asset_id = crate::util::read_i32_le(chunk)?;

            // quantity: int32
            let quantity = crate::util::read_i32_le(chunk)?;

            // start_event: int32
            let start_event = crate::util::read_i32_le(chunk)?;

            // start_asset: int32
            let start_asset = crate::util::read_i32_le(chunk)?;

            // fail_event: int32
            let fail_event = crate::util::read_i32_le(chunk)?;

            // fail_asset: int32
            let fail_asset = crate::util::read_i32_le(chunk)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // timer_start_event: int32
            let timer_start_event = crate::util::read_i32_le(chunk)?;

            // timer_asset_id: int32
            let timer_asset_id = crate::util::read_i32_le(chunk)?;

            // timer_time: int32
            let timer_time = crate::util::read_i32_le(chunk)?;

            // ui_order: int32
            let ui_order = crate::util::read_i32_le(chunk)?;


            rows.push(Achievement_CriteriaRow {
                id,
                achievement_id,
                ty,
                asset_id,
                quantity,
                start_event,
                start_asset,
                fail_event,
                fail_asset,
                description_lang,
                flags,
                timer_start_event,
                timer_asset_id,
                timer_time,
                ui_order,
            });
        }

        Ok(Achievement_Criteria { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Achievement_Criteria) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // achievement_id: foreign_key (Achievement) int32
            b.write_all(&(row.achievement_id.id as i32).to_le_bytes())?;

            // ty: int32
            b.write_all(&row.ty.to_le_bytes())?;

            // asset_id: int32
            b.write_all(&row.asset_id.to_le_bytes())?;

            // quantity: int32
            b.write_all(&row.quantity.to_le_bytes())?;

            // start_event: int32
            b.write_all(&row.start_event.to_le_bytes())?;

            // start_asset: int32
            b.write_all(&row.start_asset.to_le_bytes())?;

            // fail_event: int32
            b.write_all(&row.fail_event.to_le_bytes())?;

            // fail_asset: int32
            b.write_all(&row.fail_asset.to_le_bytes())?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_cache))?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // timer_start_event: int32
            b.write_all(&row.timer_start_event.to_le_bytes())?;

            // timer_asset_id: int32
            b.write_all(&row.timer_asset_id.to_le_bytes())?;

            // timer_time: int32
            b.write_all(&row.timer_time.to_le_bytes())?;

            // ui_order: int32
            b.write_all(&row.ui_order.to_le_bytes())?;

        }

        assert_eq!(b.len(), self.rows.len() * Self::ROW_SIZE);
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: string_cache.size(),
        };

        w.write_all(&header.write_header())?;
        w.write_all(&b)?;
        w.write_all(string_cache.buffer())?;
        Ok(())
    }

}

impl Indexable for Achievement_Criteria {
    type PrimaryKey = Achievement_CriteriaKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Achievement_CriteriaKey {
    pub id: i32
}

impl Achievement_CriteriaKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for Achievement_CriteriaKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for Achievement_CriteriaKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for Achievement_CriteriaKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for Achievement_CriteriaKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for Achievement_CriteriaKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for Achievement_CriteriaKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for Achievement_CriteriaKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for Achievement_CriteriaKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for Achievement_CriteriaKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for Achievement_CriteriaKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Achievement_CriteriaRow {
    pub id: Achievement_CriteriaKey,
    pub achievement_id: AchievementKey,
    pub ty: i32,
    pub asset_id: i32,
    pub quantity: i32,
    pub start_event: i32,
    pub start_asset: i32,
    pub fail_event: i32,
    pub fail_asset: i32,
    pub description_lang: ExtendedLocalizedString,
    pub flags: i32,
    pub timer_start_event: i32,
    pub timer_asset_id: i32,
    pub timer_time: i32,
    pub ui_order: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn achievement_criteria() {
        let mut file = File::open("../wrath-dbc/Achievement_Criteria.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Achievement_Criteria::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Achievement_Criteria::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
