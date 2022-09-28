use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::liquid_type::*;
use crate::vanilla_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundWaterType {
    pub rows: Vec<SoundWaterTypeRow>,
}

impl DbcTable for SoundWaterType {
    type Row = SoundWaterTypeRow;

    fn filename() -> &'static str { "SoundWaterType.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 16,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 4 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 4,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SoundWaterType) uint32
            let id = SoundWaterTypeKey::new(crate::util::read_u32_le(chunk)?);

            // liquid_type: foreign_key (LiquidType) uint32
            let liquid_type = LiquidTypeKey::new(crate::util::read_u32_le(chunk)?.into());

            // fluid_speed: FluidSpeed
            let fluid_speed = FluidSpeed::try_from(crate::util::read_i32_le(chunk)?)?;

            // sound: foreign_key (SoundEntries) uint32
            let sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(SoundWaterTypeRow {
                id,
                liquid_type,
                fluid_speed,
                sound,
            });
        }

        Ok(SoundWaterType { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 4,
            record_size: 16,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SoundWaterType) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // liquid_type: foreign_key (LiquidType) uint32
            b.write_all(&(row.liquid_type.id as u32).to_le_bytes())?;

            // fluid_speed: FluidSpeed
            b.write_all(&(row.fluid_speed.as_int() as i32).to_le_bytes())?;

            // sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound.id as u32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SoundWaterType {
    type PrimaryKey = SoundWaterTypeKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct SoundWaterTypeKey {
    pub id: u32
}

impl SoundWaterTypeKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum FluidSpeed {
    Still,
    Slow,
    Rapid,
}

impl TryFrom<i32> for FluidSpeed {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Still,
            4 => Self::Slow,
            8 => Self::Rapid,
            val => return Err(crate::InvalidEnumError::new("FluidSpeed", val as i64)),
        })
    }

}

impl FluidSpeed {
    const fn as_int(&self) -> i32 {
        match self {
            Self::Still => 0,
            Self::Slow => 4,
            Self::Rapid => 8,
        }

    }

}

impl Default for FluidSpeed {
    fn default() -> Self {
        Self::Still
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundWaterTypeRow {
    pub id: SoundWaterTypeKey,
    pub liquid_type: LiquidTypeKey,
    pub fluid_speed: FluidSpeed,
    pub sound: SoundEntriesKey,
}

