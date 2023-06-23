use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::animation_data::AnimationDataKey;
use crate::vanilla_tables::attack_anim_types::AttackAnimTypesKey;
use std::io::Write;
use wow_world_base::vanilla::AttackHand;

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

            // id: primary_key (AttackAnimKits) uint32
            let id = AttackAnimKitsKey::new(crate::util::read_u32_le(chunk)?);

            // animation_data: foreign_key (AnimationData) uint32
            let animation_data = AnimationDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // attack_anim_type: foreign_key (AttackAnimTypes) uint32
            let attack_anim_type = AttackAnimTypesKey::new(crate::util::read_u32_le(chunk)?.into());

            // animation_frequency: uint32
            let animation_frequency = crate::util::read_u32_le(chunk)?;

            // flags: AttackHand
            let flags = crate::util::read_i32_le(chunk)?.try_into()?;


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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttackAnimKitsRow {
    pub id: AttackAnimKitsKey,
    pub animation_data: AnimationDataKey,
    pub attack_anim_type: AttackAnimTypesKey,
    pub animation_frequency: u32,
    pub flags: AttackHand,
}

