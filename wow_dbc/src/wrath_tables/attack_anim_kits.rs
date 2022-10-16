use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

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

            // id: primary_key (AttackAnimKits) int32
            let id = AttackAnimKitsKey::new(crate::util::read_i32_le(chunk)?);

            // item_subclass_id: int32
            let item_subclass_id = crate::util::read_i32_le(chunk)?;

            // anim_type_id: int32
            let anim_type_id = crate::util::read_i32_le(chunk)?;

            // anim_frequency: int32
            let anim_frequency = crate::util::read_i32_le(chunk)?;

            // which_hand: int32
            let which_hand = crate::util::read_i32_le(chunk)?;


            rows.push(AttackAnimKitsRow {
                id,
                item_subclass_id,
                anim_type_id,
                anim_frequency,
                which_hand,
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
            // id: primary_key (AttackAnimKits) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // item_subclass_id: int32
            b.write_all(&row.item_subclass_id.to_le_bytes())?;

            // anim_type_id: int32
            b.write_all(&row.anim_type_id.to_le_bytes())?;

            // anim_frequency: int32
            b.write_all(&row.anim_frequency.to_le_bytes())?;

            // which_hand: int32
            b.write_all(&row.which_hand.to_le_bytes())?;

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
    pub id: i32
}

impl AttackAnimKitsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for AttackAnimKitsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for AttackAnimKitsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for AttackAnimKitsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttackAnimKitsRow {
    pub id: AttackAnimKitsKey,
    pub item_subclass_id: i32,
    pub anim_type_id: i32,
    pub anim_frequency: i32,
    pub which_hand: i32,
}

