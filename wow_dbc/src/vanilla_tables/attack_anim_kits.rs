use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::animation_data::*;
use crate::vanilla_tables::attack_anim_types::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttackAnimKits {
    pub rows: Vec<AttackAnimKitsRow>,
}

impl DbcTable for AttackAnimKits {
    type Row = AttackAnimKitsRow;

    fn filename() -> &'static str { "AttackAnimKits.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

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

            // id: primary_key (AttackAnimKits) uint32
            let id = AttackAnimKitsKey::new(crate::util::read_u32_le(chunk)?);

            // animation_data: foreign_key (AnimationData) uint32
            let animation_data = AnimationDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // attack_anim_type: foreign_key (AttackAnimTypes) uint32
            let attack_anim_type = AttackAnimTypesKey::new(crate::util::read_u32_le(chunk)?.into());

            // animation_frequency: uint32
            let animation_frequency = crate::util::read_u32_le(chunk)?;

            // flags: AttackHand
            let flags = AttackHand::try_from(crate::util::read_i32_le(chunk)?)?;


            rows.push(AttackAnimKitsRow {
                id,
                animation_data,
                attack_anim_type,
                animation_frequency,
                flags,
            });
        }

        Ok(AttackAnimKits { rows, })
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
            // id: primary_key (AttackAnimKits) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // animation_data: foreign_key (AnimationData) uint32
            b.write_all(&(row.animation_data.id as u32).to_le_bytes())?;

            // attack_anim_type: foreign_key (AttackAnimTypes) uint32
            b.write_all(&(row.attack_anim_type.id as u32).to_le_bytes())?;

            // animation_frequency: uint32
            b.write_all(&row.animation_frequency.to_le_bytes())?;

            // flags: AttackHand
            b.write_all(&(row.flags.as_int() as i32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for AttackAnimKits {
    type PrimaryKey = AttackAnimKitsKey;
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
pub struct ConstAttackAnimKits<const S: usize> {
    pub rows: [AttackAnimKitsRow; S],
}

impl<const S: usize> ConstAttackAnimKits<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 20 {
            panic!("invalid record size, expected 20")
        }

        if header.field_count != 5 {
            panic!("invalid field count, expected 5")
        }

        let mut b_offset = 20;
        let mut rows = [
            AttackAnimKitsRow {
                id: AttackAnimKitsKey::new(0),
                animation_data: AnimationDataKey::new(0),
                attack_anim_type: AttackAnimTypesKey::new(0),
                animation_frequency: 0,
                flags: AttackHand::MainHand,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (AttackAnimKits) uint32
            let id = AttackAnimKitsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // animation_data: foreign_key (AnimationData) uint32
            let animation_data = AnimationDataKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // attack_anim_type: foreign_key (AttackAnimTypes) uint32
            let attack_anim_type = AttackAnimTypesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // animation_frequency: uint32
            let animation_frequency = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: AttackHand
            let flags = match AttackHand::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            rows[i] = AttackAnimKitsRow {
                id,
                animation_data,
                attack_anim_type,
                animation_frequency,
                flags,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> AttackAnimKits {
        AttackAnimKits {
            rows: self.rows.iter().map(|s| AttackAnimKitsRow {
                id: s.id,
                animation_data: s.animation_data,
                attack_anim_type: s.attack_anim_type,
                animation_frequency: s.animation_frequency,
                flags: s.flags,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct AttackAnimKitsKey {
    pub id: u32
}

impl AttackAnimKitsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for AttackAnimKitsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for AttackAnimKitsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for AttackAnimKitsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AttackHand {
    MainHand,
    OffHand,
}

impl AttackHand {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            0 => Self::MainHand,
            1 => Self::OffHand,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for AttackHand {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("AttackHand", value as i64))
    }

}

impl AttackHand {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::MainHand => 0,
            Self::OffHand => 1,
        }

    }

}

impl Default for AttackHand {
    fn default() -> Self {
        Self::MainHand
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttackAnimKitsRow {
    pub id: AttackAnimKitsKey,
    pub animation_data: AnimationDataKey,
    pub attack_anim_type: AttackAnimTypesKey,
    pub animation_frequency: u32,
    pub flags: AttackHand,
}

