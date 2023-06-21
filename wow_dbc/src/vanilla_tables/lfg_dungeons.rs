use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LFGDungeons {
    pub rows: Vec<LFGDungeonsRow>,
}

impl DbcTable for LFGDungeons {
    type Row = LFGDungeonsRow;

    fn filename() -> &'static str { "LFGDungeons.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 56 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 56,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 14 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 14,
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

            // id: primary_key (LFGDungeons) uint32
            let id = LFGDungeonsKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // min_allowed_level: uint32
            let min_allowed_level = crate::util::read_u32_le(chunk)?;

            // max_allowed_level: uint32
            let max_allowed_level = crate::util::read_u32_le(chunk)?;

            // instance_type: InstanceType
            let instance_type = InstanceType::try_from(crate::util::read_i32_le(chunk)?)?;

            // faction: Faction
            let faction = Faction::try_from(crate::util::read_i32_le(chunk)?)?;


            rows.push(LFGDungeonsRow {
                id,
                name,
                min_allowed_level,
                max_allowed_level,
                instance_type,
                faction,
            });
        }

        Ok(LFGDungeons { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 14,
            record_size: 56,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (LFGDungeons) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // min_allowed_level: uint32
            b.write_all(&row.min_allowed_level.to_le_bytes())?;

            // max_allowed_level: uint32
            b.write_all(&row.max_allowed_level.to_le_bytes())?;

            // instance_type: InstanceType
            b.write_all(&(row.instance_type.as_int() as i32).to_le_bytes())?;

            // faction: Faction
            b.write_all(&(row.faction.as_int() as i32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for LFGDungeons {
    type PrimaryKey = LFGDungeonsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl LFGDungeons {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstLFGDungeons<const S: usize> {
    pub rows: [ConstLFGDungeonsRow; S],
}

impl<const S: usize> ConstLFGDungeons<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 56 {
            panic!("invalid record size, expected 56")
        }

        if header.field_count != 14 {
            panic!("invalid field count, expected 14")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstLFGDungeonsRow {
                id: LFGDungeonsKey::new(0),
                name: crate::ConstLocalizedString::empty(),
                min_allowed_level: 0,
                max_allowed_level: 0,
                instance_type: InstanceType::GroupInstance,
                faction: Faction::Neutral,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (LFGDungeons) uint32
            let id = LFGDungeonsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // name: string_ref_loc
            let name = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            // min_allowed_level: uint32
            let min_allowed_level = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // max_allowed_level: uint32
            let max_allowed_level = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // instance_type: InstanceType
            let instance_type = match InstanceType::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            // faction: Faction
            let faction = match Faction::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            rows[i] = ConstLFGDungeonsRow {
                id,
                name,
                min_allowed_level,
                max_allowed_level,
                instance_type,
                faction,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> LFGDungeons {
        LFGDungeons {
            rows: self.rows.iter().map(|s| LFGDungeonsRow {
                id: s.id,
                name: s.name.to_string(),
                min_allowed_level: s.min_allowed_level,
                max_allowed_level: s.max_allowed_level,
                instance_type: s.instance_type,
                faction: s.faction,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct LFGDungeonsKey {
    pub id: u32
}

impl LFGDungeonsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for LFGDungeonsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for LFGDungeonsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for LFGDungeonsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum InstanceType {
    GroupInstance,
    RaidInstance,
    WorldZone,
    Battleground,
}

impl InstanceType {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            1 => Self::GroupInstance,
            2 => Self::RaidInstance,
            4 => Self::WorldZone,
            5 => Self::Battleground,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for InstanceType {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("InstanceType", value as i64))
    }

}

impl InstanceType {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::GroupInstance => 1,
            Self::RaidInstance => 2,
            Self::WorldZone => 4,
            Self::Battleground => 5,
        }

    }

}

impl Default for InstanceType {
    fn default() -> Self {
        Self::GroupInstance
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Faction {
    Neutral,
    Horde,
    Alliance,
}

impl Faction {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            -1 => Self::Neutral,
            0 => Self::Horde,
            1 => Self::Alliance,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for Faction {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("Faction", value as i64))
    }

}

impl Faction {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::Neutral => -1,
            Self::Horde => 0,
            Self::Alliance => 1,
        }

    }

}

impl Default for Faction {
    fn default() -> Self {
        Self::Neutral
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LFGDungeonsRow {
    pub id: LFGDungeonsKey,
    pub name: LocalizedString,
    pub min_allowed_level: u32,
    pub max_allowed_level: u32,
    pub instance_type: InstanceType,
    pub faction: Faction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstLFGDungeonsRow {
    pub id: LFGDungeonsKey,
    pub name: ConstLocalizedString,
    pub min_allowed_level: u32,
    pub max_allowed_level: u32,
    pub instance_type: InstanceType,
    pub faction: Faction,
}

