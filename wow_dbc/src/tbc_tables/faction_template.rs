use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::faction::FactionKey;
use crate::tbc_tables::faction_group::FactionGroupKey;
use crate::util::StringCache;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FactionTemplate {
    pub rows: Vec<FactionTemplateRow>,
}

impl DbcTable for FactionTemplate {
    type Row = FactionTemplateRow;

    const FILENAME: &'static str = "FactionTemplate.dbc";
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

            // id: primary_key (FactionTemplate) int32
            let id = FactionTemplateKey::new(crate::util::read_i32_le(chunk)?);

            // faction: foreign_key (Faction) int32
            let faction = FactionKey::new(crate::util::read_i32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // faction_group: foreign_key (FactionGroup) int32
            let faction_group = FactionGroupKey::new(crate::util::read_i32_le(chunk)?.into());

            // friend_group: int32
            let friend_group = crate::util::read_i32_le(chunk)?;

            // enemy_group: int32
            let enemy_group = crate::util::read_i32_le(chunk)?;

            // enemies: int32[4]
            let enemies = crate::util::read_array_i32::<4>(chunk)?;

            // friend: int32[4]
            let friend = crate::util::read_array_i32::<4>(chunk)?;


            rows.push(FactionTemplateRow {
                id,
                faction,
                flags,
                faction_group,
                friend_group,
                enemy_group,
                enemies,
                friend,
            });
        }

        Ok(FactionTemplate { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (FactionTemplate) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // faction: foreign_key (Faction) int32
            b.write_all(&(row.faction.id as i32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // faction_group: foreign_key (FactionGroup) int32
            b.write_all(&(row.faction_group.id as i32).to_le_bytes())?;

            // friend_group: int32
            b.write_all(&row.friend_group.to_le_bytes())?;

            // enemy_group: int32
            b.write_all(&row.enemy_group.to_le_bytes())?;

            // enemies: int32[4]
            for i in row.enemies {
                b.write_all(&i.to_le_bytes())?;
            }


            // friend: int32[4]
            for i in row.friend {
                b.write_all(&i.to_le_bytes())?;
            }


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

impl Indexable for FactionTemplate {
    type PrimaryKey = FactionTemplateKey;
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
pub struct FactionTemplateKey {
    pub id: i32
}

impl FactionTemplateKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for FactionTemplateKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for FactionTemplateKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for FactionTemplateKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for FactionTemplateKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for FactionTemplateKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for FactionTemplateKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for FactionTemplateKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for FactionTemplateKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for FactionTemplateKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for FactionTemplateKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FactionTemplateRow {
    pub id: FactionTemplateKey,
    pub faction: FactionKey,
    pub flags: i32,
    pub faction_group: FactionGroupKey,
    pub friend_group: i32,
    pub enemy_group: i32,
    pub enemies: [i32; 4],
    pub friend: [i32; 4],
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn faction_template() {
        let mut file = File::open("../tbc-dbc/FactionTemplate.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = FactionTemplate::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = FactionTemplate::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
