use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::vanilla_tables::item_class::ItemClassKey;
use crate::vanilla_tables::sound_entries::SoundEntriesKey;
use std::io::Write;
pub use wow_world_base::vanilla::ItemEnvTypes;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SheatheSoundLookups {
    pub rows: Vec<SheatheSoundLookupsRow>,
}

impl DbcTable for SheatheSoundLookups {
    type Row = SheatheSoundLookupsRow;

    const FILENAME: &'static str = "SheatheSoundLookups.dbc";
    const FIELD_COUNT: usize = 7;
    const ROW_SIZE: usize = 28;

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

            // id: primary_key (SheatheSoundLookups) uint32
            let id = SheatheSoundLookupsKey::new(crate::util::read_u32_le(chunk)?);

            // item_class: foreign_key (ItemClass) uint32
            let item_class = ItemClassKey::new(crate::util::read_u32_le(chunk)?.into());

            // item_subclass: uint32
            let item_subclass = crate::util::read_u32_le(chunk)?;

            // item_env_types: ItemEnvTypes
            let item_env_types = crate::util::read_i32_le(chunk)?.try_into()?;

            // not_shield: bool32
            let not_shield = crate::util::read_u32_le(chunk)? != 0;

            // sheathe_sound: foreign_key (SoundEntries) uint32
            let sheathe_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // draw_sound: foreign_key (SoundEntries) uint32
            let draw_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(SheatheSoundLookupsRow {
                id,
                item_class,
                item_subclass,
                item_env_types,
                not_shield,
                sheathe_sound,
                draw_sound,
            });
        }

        Ok(SheatheSoundLookups { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (SheatheSoundLookups) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // item_class: foreign_key (ItemClass) uint32
            b.write_all(&(row.item_class.id as u32).to_le_bytes())?;

            // item_subclass: uint32
            b.write_all(&row.item_subclass.to_le_bytes())?;

            // item_env_types: ItemEnvTypes
            b.write_all(&(row.item_env_types.as_int() as i32).to_le_bytes())?;

            // not_shield: bool32
            b.write_all(&u32::from(row.not_shield).to_le_bytes())?;

            // sheathe_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sheathe_sound.id as u32).to_le_bytes())?;

            // draw_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.draw_sound.id as u32).to_le_bytes())?;

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

impl Indexable for SheatheSoundLookups {
    type PrimaryKey = SheatheSoundLookupsKey;
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
pub struct SheatheSoundLookupsKey {
    pub id: u32
}

impl SheatheSoundLookupsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SheatheSoundLookupsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for SheatheSoundLookupsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for SheatheSoundLookupsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for SheatheSoundLookupsKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SheatheSoundLookupsKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for SheatheSoundLookupsKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for SheatheSoundLookupsKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for SheatheSoundLookupsKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SheatheSoundLookupsKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SheatheSoundLookupsKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SheatheSoundLookupsRow {
    pub id: SheatheSoundLookupsKey,
    pub item_class: ItemClassKey,
    pub item_subclass: u32,
    pub item_env_types: ItemEnvTypes,
    pub not_shield: bool,
    pub sheathe_sound: SoundEntriesKey,
    pub draw_sound: SoundEntriesKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn sheathe_sound_lookups() {
        let mut file = File::open("../vanilla-dbc/SheatheSoundLookups.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = SheatheSoundLookups::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SheatheSoundLookups::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
