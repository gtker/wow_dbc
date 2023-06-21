use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WeaponSwingSounds2 {
    pub rows: Vec<WeaponSwingSounds2Row>,
}

impl DbcTable for WeaponSwingSounds2 {
    type Row = WeaponSwingSounds2Row;

    fn filename() -> &'static str { "WeaponSwingSounds2.dbc" }

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

            // id: primary_key (WeaponSwingSounds2) uint32
            let id = WeaponSwingSounds2Key::new(crate::util::read_u32_le(chunk)?);

            // swing_type: SwingType
            let swing_type = SwingType::try_from(crate::util::read_i32_le(chunk)?)?;

            // critical: bool32
            let critical = crate::util::read_u32_le(chunk)? != 0;

            // sound: foreign_key (SoundEntries) uint32
            let sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(WeaponSwingSounds2Row {
                id,
                swing_type,
                critical,
                sound,
            });
        }

        Ok(WeaponSwingSounds2 { rows, })
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
            // id: primary_key (WeaponSwingSounds2) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // swing_type: SwingType
            b.write_all(&(row.swing_type.as_int() as i32).to_le_bytes())?;

            // critical: bool32
            b.write_all(&u32::from(row.critical).to_le_bytes())?;

            // sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound.id as u32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for WeaponSwingSounds2 {
    type PrimaryKey = WeaponSwingSounds2Key;
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
pub struct WeaponSwingSounds2Key {
    pub id: u32
}

impl WeaponSwingSounds2Key {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for WeaponSwingSounds2Key {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for WeaponSwingSounds2Key {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for WeaponSwingSounds2Key {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SwingType {
    Light,
    Medium,
    Heavy,
}

impl TryFrom<i32> for SwingType {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Light,
            1 => Self::Medium,
            2 => Self::Heavy,
            val => return Err(crate::InvalidEnumError::new("SwingType", val as i64)),
        })
    }

}

impl SwingType {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::Light => 0,
            Self::Medium => 1,
            Self::Heavy => 2,
        }

    }

}

impl Default for SwingType {
    fn default() -> Self {
        Self::Light
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WeaponSwingSounds2Row {
    pub id: WeaponSwingSounds2Key,
    pub swing_type: SwingType,
    pub critical: bool,
    pub sound: SoundEntriesKey,
}

