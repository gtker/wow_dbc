use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::sound_entries::SoundEntriesKey;
use crate::vanilla_tables::spell_visual_effect_name::SpellVisualEffectNameKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerrainType {
    pub rows: Vec<TerrainTypeRow>,
}

impl DbcTable for TerrainType {
    type Row = TerrainTypeRow;

    const FILENAME: &'static str = "TerrainType.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 24 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 24,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 6 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 6,
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

            // id: primary_key (TerrainType) uint32
            let id = TerrainTypeKey::new(crate::util::read_u32_le(chunk)?);

            // description: string_ref
            let description = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // footstep_spray_run: foreign_key (SpellVisualEffectName) uint32
            let footstep_spray_run = SpellVisualEffectNameKey::new(crate::util::read_u32_le(chunk)?.into());

            // footstep_spray_walk: foreign_key (SpellVisualEffectName) uint32
            let footstep_spray_walk = SpellVisualEffectNameKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound: foreign_key (SoundEntries) uint32
            let sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // display_footsteps: bool32
            let display_footsteps = crate::util::read_u32_le(chunk)? != 0;


            rows.push(TerrainTypeRow {
                id,
                description,
                footstep_spray_run,
                footstep_spray_walk,
                sound,
                display_footsteps,
            });
        }

        Ok(TerrainType { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 6,
            record_size: 24,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (TerrainType) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // description: string_ref
            if !row.description.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.description.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // footstep_spray_run: foreign_key (SpellVisualEffectName) uint32
            b.write_all(&(row.footstep_spray_run.id as u32).to_le_bytes())?;

            // footstep_spray_walk: foreign_key (SpellVisualEffectName) uint32
            b.write_all(&(row.footstep_spray_walk.id as u32).to_le_bytes())?;

            // sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound.id as u32).to_le_bytes())?;

            // display_footsteps: bool32
            b.write_all(&u32::from(row.display_footsteps).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for TerrainType {
    type PrimaryKey = TerrainTypeKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl TerrainType {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.description.is_empty() { b.write_all(row.description.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.description.is_empty() { sum += row.description.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct TerrainTypeKey {
    pub id: u32
}

impl TerrainTypeKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for TerrainTypeKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for TerrainTypeKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for TerrainTypeKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for TerrainTypeKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for TerrainTypeKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for TerrainTypeKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for TerrainTypeKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for TerrainTypeKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for TerrainTypeKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for TerrainTypeKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerrainTypeRow {
    pub id: TerrainTypeKey,
    pub description: String,
    pub footstep_spray_run: SpellVisualEffectNameKey,
    pub footstep_spray_walk: SpellVisualEffectNameKey,
    pub sound: SoundEntriesKey,
    pub display_footsteps: bool,
}

