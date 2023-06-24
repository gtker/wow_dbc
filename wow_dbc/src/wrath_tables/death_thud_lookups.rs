use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::sound_entries::SoundEntriesKey;
use crate::wrath_tables::terrain_type_sounds::TerrainTypeSoundsKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeathThudLookups {
    pub rows: Vec<DeathThudLookupsRow>,
}

impl DbcTable for DeathThudLookups {
    type Row = DeathThudLookupsRow;

    const FILENAME: &'static str = "DeathThudLookups.dbc";

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
            field_count: 5,
            record_size: 20,
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
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct DeathThudLookupsKey {
    pub id: i32
}

impl DeathThudLookupsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

impl TryFrom<u32> for DeathThudLookupsKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeathThudLookupsRow {
    pub id: DeathThudLookupsKey,
    pub size_class: i32,
    pub terrain_type_sound_id: TerrainTypeSoundsKey,
    pub sound_entry_id: SoundEntriesKey,
    pub sound_entry_id_water: SoundEntriesKey,
}

