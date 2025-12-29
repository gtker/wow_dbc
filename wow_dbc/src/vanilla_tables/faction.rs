use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;
pub use wow_world_base::vanilla::{
    AllowedRace, ReputationFlags,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Faction {
    pub rows: Vec<FactionRow>,
}

impl DbcTable for Faction {
    type Row = FactionRow;

    const FILENAME: &'static str = "Faction.dbc";
    const FIELD_COUNT: usize = 37;
    const ROW_SIZE: usize = 148;

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

            // id: primary_key (Faction) uint32
            let id = FactionKey::new(crate::util::read_u32_le(chunk)?);

            // reputation_index: uint32
            let reputation_index = crate::util::read_u32_le(chunk)?;

            // reputation_race_mask: AllowedRace[4]
            let reputation_race_mask = {
                let mut arr = [AllowedRace::default(); 4];
                for i in arr.iter_mut() {
                    *i = AllowedRace::new(crate::util::read_i32_le(chunk)? as _);
                }

                arr
            };

            // reputation_class_mask: uint32[4]
            let reputation_class_mask = crate::util::read_array_u32::<4>(chunk)?;

            // reputation_base: uint32[4]
            let reputation_base = crate::util::read_array_u32::<4>(chunk)?;

            // reputation_flags: ReputationFlags[4]
            let reputation_flags = {
                let mut arr = [ReputationFlags::default(); 4];
                for i in arr.iter_mut() {
                    *i = ReputationFlags::new(crate::util::read_i32_le(chunk)? as _);
                }

                arr
            };

            // parent_faction: foreign_key (Faction) uint32
            let parent_faction = FactionKey::new(crate::util::read_u32_le(chunk)?.into());

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // description: string_ref_loc
            let description = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(FactionRow {
                id,
                reputation_index,
                reputation_race_mask,
                reputation_class_mask,
                reputation_base,
                reputation_flags,
                parent_faction,
                name,
                description,
            });
        }

        Ok(Faction { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Faction) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // reputation_index: uint32
            b.write_all(&row.reputation_index.to_le_bytes())?;

            // reputation_race_mask: AllowedRace[4]
            for i in row.reputation_race_mask {
                b.write_all(&(i.as_int() as i32).to_le_bytes())?;
            }


            // reputation_class_mask: uint32[4]
            for i in row.reputation_class_mask {
                b.write_all(&i.to_le_bytes())?;
            }


            // reputation_base: uint32[4]
            for i in row.reputation_base {
                b.write_all(&i.to_le_bytes())?;
            }


            // reputation_flags: ReputationFlags[4]
            for i in row.reputation_flags {
                b.write_all(&(i.as_int() as i32).to_le_bytes())?;
            }


            // parent_faction: foreign_key (Faction) uint32
            b.write_all(&(row.parent_faction.id as u32).to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_cache))?;

            // description: string_ref_loc
            b.write_all(&row.description.string_indices_as_array(&mut string_cache))?;

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

impl Indexable for Faction {
    type PrimaryKey = FactionKey;
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
pub struct FactionKey {
    pub id: u32
}

impl FactionKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for FactionKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for FactionKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for FactionKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for FactionKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for FactionKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for FactionKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for FactionKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for FactionKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for FactionKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for FactionKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FactionRow {
    pub id: FactionKey,
    pub reputation_index: u32,
    pub reputation_race_mask: [AllowedRace; 4],
    pub reputation_class_mask: [u32; 4],
    pub reputation_base: [u32; 4],
    pub reputation_flags: [ReputationFlags; 4],
    pub parent_faction: FactionKey,
    pub name: LocalizedString,
    pub description: LocalizedString,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn faction() {
        let mut file = File::open("../vanilla-dbc/Faction.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Faction::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Faction::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
