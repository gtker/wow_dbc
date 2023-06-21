use crate::header::{HEADER_SIZE, DbcHeader, parse_header};
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnimationData {
    pub rows: Vec<AnimationDataRow>,
}

impl DbcTable for AnimationData {
    type Row = AnimationDataRow;

    fn filename() -> &'static str { "AnimationData.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (AnimationData) uint32
            let id = AnimationDataKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // weapon_flags: WeaponFlags
            let weapon_flags = WeaponFlags::new(crate::util::read_i32_le(chunk)?);

            // body_flags: int32
            let body_flags = crate::util::read_i32_le(chunk)?;

            // unknown: int32
            let unknown = crate::util::read_i32_le(chunk)?;

            // fallback: foreign_key (AnimationData) uint32
            let fallback = AnimationDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // behaviour: foreign_key (AnimationData) uint32
            let behaviour = AnimationDataKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(AnimationDataRow {
                id,
                name,
                weapon_flags,
                body_flags,
                unknown,
                fallback,
                behaviour,
            });
        }

        Ok(AnimationData { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 7,
            record_size: 28,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (AnimationData) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref
            if !row.name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // weapon_flags: WeaponFlags
            b.write_all(&(row.weapon_flags.as_int() as i32).to_le_bytes())?;

            // body_flags: int32
            b.write_all(&row.body_flags.to_le_bytes())?;

            // unknown: int32
            b.write_all(&row.unknown.to_le_bytes())?;

            // fallback: foreign_key (AnimationData) uint32
            b.write_all(&(row.fallback.id as u32).to_le_bytes())?;

            // behaviour: foreign_key (AnimationData) uint32
            b.write_all(&(row.behaviour.id as u32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for AnimationData {
    type PrimaryKey = AnimationDataKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl AnimationData {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.name.is_empty() { b.write_all(row.name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.name.is_empty() { sum += row.name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct AnimationDataKey {
    pub id: u32
}

impl AnimationDataKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for AnimationDataKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for AnimationDataKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for AnimationDataKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct WeaponFlags {
    value: i32,
}

impl WeaponFlags {
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    pub const fn as_int(&self) -> i32 {
        self.value
    }

    pub const fn weapon_not_affected_by_animation(&self) -> bool {
        self.value == 0
    }

    pub const fn sheathe_weapons_automatically(&self) -> bool {
        (self.value & 4) != 0
    }

    pub const fn sheathe_weapons_automatically2(&self) -> bool {
        (self.value & 16) != 0
    }

    pub const fn unsheathe_weapons(&self) -> bool {
        (self.value & 32) != 0
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnimationDataRow {
    pub id: AnimationDataKey,
    pub name: String,
    pub weapon_flags: WeaponFlags,
    pub body_flags: i32,
    pub unknown: i32,
    pub fallback: AnimationDataKey,
    pub behaviour: AnimationDataKey,
}

