use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::ground_effect_doodad::GroundEffectDoodadKey;
use crate::vanilla_tables::sound_entries::SoundEntriesKey;
use crate::vanilla_tables::terrain_type::TerrainTypeKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FootstepTerrainLookup {
    pub rows: Vec<FootstepTerrainLookupRow>,
}

impl DbcTable for FootstepTerrainLookup {
    type Row = FootstepTerrainLookupRow;

    const FILENAME: &'static str = "FootstepTerrainLookup.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 20 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 20,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 5 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 5,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (FootstepTerrainLookup) uint32
            let id = FootstepTerrainLookupKey::new(crate::util::read_u32_le(chunk)?);

            // creature_footstep: foreign_key (GroundEffectDoodad) uint32
            let creature_footstep = GroundEffectDoodadKey::new(crate::util::read_u32_le(chunk)?.into());

            // terrain_type: foreign_key (TerrainType) uint32
            let terrain_type = TerrainTypeKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_entry: foreign_key (SoundEntries) uint32
            let sound_entry = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound_entry_splash: foreign_key (SoundEntries) uint32
            let sound_entry_splash = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(FootstepTerrainLookupRow {
                id,
                creature_footstep,
                terrain_type,
                sound_entry,
                sound_entry_splash,
            });
        }

        Ok(FootstepTerrainLookup { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 5,
            record_size: 20,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (FootstepTerrainLookup) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // creature_footstep: foreign_key (GroundEffectDoodad) uint32
            b.write_all(&(row.creature_footstep.id as u32).to_le_bytes())?;

            // terrain_type: foreign_key (TerrainType) uint32
            b.write_all(&(row.terrain_type.id as u32).to_le_bytes())?;

            // sound_entry: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_entry.id as u32).to_le_bytes())?;

            // sound_entry_splash: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_entry_splash.id as u32).to_le_bytes())?;

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
pub struct FootstepTerrainLookupKey {
    pub id: u32
}

impl FootstepTerrainLookupKey {
    pub const fn new(id: u32) -> Self {
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

impl From<u32> for FootstepTerrainLookupKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for FootstepTerrainLookupKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for FootstepTerrainLookupKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for FootstepTerrainLookupKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for FootstepTerrainLookupKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for FootstepTerrainLookupKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for FootstepTerrainLookupKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for FootstepTerrainLookupKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FootstepTerrainLookupRow {
    pub id: FootstepTerrainLookupKey,
    pub creature_footstep: GroundEffectDoodadKey,
    pub terrain_type: TerrainTypeKey,
    pub sound_entry: SoundEntriesKey,
    pub sound_entry_splash: SoundEntriesKey,
}

