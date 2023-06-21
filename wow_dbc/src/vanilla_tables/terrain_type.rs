use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::sound_entries::*;
use crate::vanilla_tables::spell_visual_effect_name::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerrainType {
    pub rows: Vec<TerrainTypeRow>,
}

impl DbcTable for TerrainType {
    type Row = TerrainTypeRow;

    fn filename() -> &'static str { "TerrainType.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

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
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstTerrainType<const S: usize> {
    pub rows: [ConstTerrainTypeRow; S],
}

impl<const S: usize> ConstTerrainType<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 24 {
            panic!("invalid record size, expected 24")
        }

        if header.field_count != 6 {
            panic!("invalid field count, expected 6")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstTerrainTypeRow {
                id: TerrainTypeKey::new(0),
                description: "",
                footstep_spray_run: SpellVisualEffectNameKey::new(0),
                footstep_spray_walk: SpellVisualEffectNameKey::new(0),
                sound: SoundEntriesKey::new(0),
                display_footsteps: false,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (TerrainType) uint32
            let id = TerrainTypeKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // description: string_ref
            let description = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // footstep_spray_run: foreign_key (SpellVisualEffectName) uint32
            let footstep_spray_run = SpellVisualEffectNameKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // footstep_spray_walk: foreign_key (SpellVisualEffectName) uint32
            let footstep_spray_walk = SpellVisualEffectNameKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound: foreign_key (SoundEntries) uint32
            let sound = SoundEntriesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // display_footsteps: bool32
            let display_footsteps = if (b[b_offset + 0] | b[b_offset + 1] | b[b_offset + 2] | b[b_offset + 3]) != 0 {true} else {false};
            b_offset += 4;

            rows[i] = ConstTerrainTypeRow {
                id,
                description,
                footstep_spray_run,
                footstep_spray_walk,
                sound,
                display_footsteps,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerrainTypeRow {
    pub id: TerrainTypeKey,
    pub description: String,
    pub footstep_spray_run: SpellVisualEffectNameKey,
    pub footstep_spray_walk: SpellVisualEffectNameKey,
    pub sound: SoundEntriesKey,
    pub display_footsteps: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstTerrainTypeRow {
    pub id: TerrainTypeKey,
    pub description: &'static str,
    pub footstep_spray_run: SpellVisualEffectNameKey,
    pub footstep_spray_walk: SpellVisualEffectNameKey,
    pub sound: SoundEntriesKey,
    pub display_footsteps: bool,
}

