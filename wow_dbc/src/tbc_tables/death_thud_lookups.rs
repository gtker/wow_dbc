use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::sound_entries::SoundEntriesKey;
use crate::tbc_tables::terrain_type_sounds::TerrainTypeSoundsKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeathThudLookups {
    pub rows: Vec<DeathThudLookupsRow>,
}

impl DbcTable for DeathThudLookups {
    type Row = DeathThudLookupsRow;

    const FILENAME: &'static str = "DeathThudLookups.dbc";
    const FIELD_COUNT: usize = 5;
    const ROW_SIZE: usize = 20;

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

            // id: primary_key (DeathThudLookups) int32
            let id = DeathThudLookupsKey::new(crate::util::read_i32_le(chunk)?);

            // size_class: int32
            let size_class = crate::util::read_i32_le(chunk)?;

            // terrain_type_sound_id: foreign_key (TerrainTypeSounds) int32
            let terrain_type_sound_id = TerrainTypeSoundsKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_entry_id: foreign_key (SoundEntries) int32
            let sound_entry_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_entry_id_water: foreign_key (SoundEntries) int32
            let sound_entry_id_water = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(DeathThudLookupsRow {
                id,
                size_class,
                terrain_type_sound_id,
                sound_entry_id,
                sound_entry_id_water,
            });
        }

        Ok(DeathThudLookups { rows, })
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
            // id: primary_key (DeathThudLookups) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // size_class: int32
            b.write_all(&row.size_class.to_le_bytes())?;

            // terrain_type_sound_id: foreign_key (TerrainTypeSounds) int32
            b.write_all(&(row.terrain_type_sound_id.id as i32).to_le_bytes())?;

            // sound_entry_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_entry_id.id as i32).to_le_bytes())?;

            // sound_entry_id_water: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_entry_id_water.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for DeathThudLookups {
    type PrimaryKey = DeathThudLookupsKey;
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
pub struct DeathThudLookupsKey {
    pub id: i32
}

impl DeathThudLookupsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for DeathThudLookupsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for DeathThudLookupsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for DeathThudLookupsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for DeathThudLookupsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for DeathThudLookupsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for DeathThudLookupsKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for DeathThudLookupsKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for DeathThudLookupsKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for DeathThudLookupsKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for DeathThudLookupsKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeathThudLookupsRow {
    pub id: DeathThudLookupsKey,
    pub size_class: i32,
    pub terrain_type_sound_id: TerrainTypeSoundsKey,
    pub sound_entry_id: SoundEntriesKey,
    pub sound_entry_id_water: SoundEntriesKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn death_thud_lookups() {
        let mut file = File::open("../tbc-dbc/DeathThudLookups.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = DeathThudLookups::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = DeathThudLookups::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
