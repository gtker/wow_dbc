use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::item_class::*;
use crate::vanilla_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SheatheSoundLookups {
    pub rows: Vec<SheatheSoundLookupsRow>,
}

impl DbcTable for SheatheSoundLookups {
    type Row = SheatheSoundLookupsRow;

    fn filename() -> &'static str { "SheatheSoundLookups.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 28 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 28,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 7 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 7,
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
            let item_env_types = ItemEnvTypes::try_from(crate::util::read_i32_le(chunk)?)?;

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

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 7,
            record_size: 28,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

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

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SheatheSoundLookups {
    type PrimaryKey = SheatheSoundLookupsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstSheatheSoundLookups<const S: usize> {
    pub rows: [SheatheSoundLookupsRow; S],
}

impl<const S: usize> ConstSheatheSoundLookups<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 28 {
            panic!("invalid record size, expected 28")
        }

        if header.field_count != 7 {
            panic!("invalid field count, expected 7")
        }

        let mut b_offset = 20;
        let mut rows = [
            SheatheSoundLookupsRow {
                id: SheatheSoundLookupsKey::new(0),
                item_class: ItemClassKey::new(0),
                item_subclass: 0,
                item_env_types: ItemEnvTypes::Shield,
                not_shield: false,
                sheathe_sound: SoundEntriesKey::new(0),
                draw_sound: SoundEntriesKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SheatheSoundLookups) uint32
            let id = SheatheSoundLookupsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // item_class: foreign_key (ItemClass) uint32
            let item_class = ItemClassKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // item_subclass: uint32
            let item_subclass = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // item_env_types: ItemEnvTypes
            let item_env_types = match ItemEnvTypes::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            // not_shield: bool32
            let not_shield = if (b[b_offset + 0] | b[b_offset + 1] | b[b_offset + 2] | b[b_offset + 3]) != 0 {true} else {false};
            b_offset += 4;

            // sheathe_sound: foreign_key (SoundEntries) uint32
            let sheathe_sound = SoundEntriesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // draw_sound: foreign_key (SoundEntries) uint32
            let draw_sound = SoundEntriesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = SheatheSoundLookupsRow {
                id,
                item_class,
                item_subclass,
                item_env_types,
                not_shield,
                sheathe_sound,
                draw_sound,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
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

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ItemEnvTypes {
    Shield,
    MetalWeapon,
    WoodWeapon,
}

impl ItemEnvTypes {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            0 => Self::Shield,
            1 => Self::MetalWeapon,
            2 => Self::WoodWeapon,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for ItemEnvTypes {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("ItemEnvTypes", value as i64))
    }

}

impl ItemEnvTypes {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::Shield => 0,
            Self::MetalWeapon => 1,
            Self::WoodWeapon => 2,
        }

    }

}

impl Default for ItemEnvTypes {
    fn default() -> Self {
        Self::Shield
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SheatheSoundLookupsRow {
    pub id: SheatheSoundLookupsKey,
    pub item_class: ItemClassKey,
    pub item_subclass: u32,
    pub item_env_types: ItemEnvTypes,
    pub not_shield: bool,
    pub sheathe_sound: SoundEntriesKey,
    pub draw_sound: SoundEntriesKey,
}

