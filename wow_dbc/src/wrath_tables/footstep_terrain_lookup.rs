use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::sound_entries::SoundEntriesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FootstepTerrainLookup {
    pub rows: Vec<FootstepTerrainLookupRow>,
}

impl DbcTable for FootstepTerrainLookup {
    type Row = FootstepTerrainLookupRow;

    const FILENAME: &'static str = "FootstepTerrainLookup.dbc";
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

            // id: primary_key (FootstepTerrainLookup) int32
            let id = FootstepTerrainLookupKey::new(crate::util::read_i32_le(chunk)?);

            // creature_footstep_id: int32
            let creature_footstep_id = crate::util::read_i32_le(chunk)?;

            // terrain_sound_id: int32
            let terrain_sound_id = crate::util::read_i32_le(chunk)?;

            // sound_id: foreign_key (SoundEntries) int32
            let sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sound_id_splash: foreign_key (SoundEntries) int32
            let sound_id_splash = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(FootstepTerrainLookupRow {
                id,
                creature_footstep_id,
                terrain_sound_id,
                sound_id,
                sound_id_splash,
            });
        }

        Ok(FootstepTerrainLookup { rows, })
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
            // id: primary_key (FootstepTerrainLookup) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // creature_footstep_id: int32
            b.write_all(&row.creature_footstep_id.to_le_bytes())?;

            // terrain_sound_id: int32
            b.write_all(&row.terrain_sound_id.to_le_bytes())?;

            // sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_id.id as i32).to_le_bytes())?;

            // sound_id_splash: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_id_splash.id as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for FootstepTerrainLookup {
    type PrimaryKey = FootstepTerrainLookupKey;
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
pub struct FootstepTerrainLookupKey {
    pub id: i32
}

impl FootstepTerrainLookupKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for FootstepTerrainLookupKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for FootstepTerrainLookupKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for FootstepTerrainLookupKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for FootstepTerrainLookupKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for FootstepTerrainLookupKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for FootstepTerrainLookupKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for FootstepTerrainLookupKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for FootstepTerrainLookupKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for FootstepTerrainLookupKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for FootstepTerrainLookupKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FootstepTerrainLookupRow {
    pub id: FootstepTerrainLookupKey,
    pub creature_footstep_id: i32,
    pub terrain_sound_id: i32,
    pub sound_id: SoundEntriesKey,
    pub sound_id_splash: SoundEntriesKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn footstep_terrain_lookup() {
        let mut file = File::open("../wrath-dbc/FootstepTerrainLookup.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = FootstepTerrainLookup::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = FootstepTerrainLookup::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
