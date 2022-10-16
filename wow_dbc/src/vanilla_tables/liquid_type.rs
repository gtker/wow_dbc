use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::spell::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiquidType {
    pub rows: Vec<LiquidTypeRow>,
}

impl DbcTable for LiquidType {
    type Row = LiquidTypeRow;

    fn filename() -> &'static str { "LiquidType.dbc" }

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (LiquidType) uint32
            let id = LiquidTypeKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // ty: Type
            let ty = Type::try_from(crate::util::read_i32_le(chunk)?)?;

            // spell: foreign_key (Spell) uint32
            let spell = SpellKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(LiquidTypeRow {
                id,
                name,
                ty,
                spell,
            });
        }

        Ok(LiquidType { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 4,
            record_size: 16,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (LiquidType) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref
            if !row.name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // ty: Type
            b.write_all(&(row.ty.as_int() as i32).to_le_bytes())?;

            // spell: foreign_key (Spell) uint32
            b.write_all(&(row.spell.id as u32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for LiquidType {
    type PrimaryKey = LiquidTypeKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl LiquidType {
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
pub struct LiquidTypeKey {
    pub id: u32
}

impl LiquidTypeKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Type {
    Fire,
    Slime,
    Water,
}

impl TryFrom<i32> for Type {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Fire,
            2 => Self::Slime,
            3 => Self::Water,
            val => return Err(crate::InvalidEnumError::new("Type", val as i64)),
        })
    }

}

impl Type {
    const fn as_int(&self) -> i32 {
        match self {
            Self::Fire => 0,
            Self::Slime => 2,
            Self::Water => 3,
        }

    }

}

impl Default for Type {
    fn default() -> Self {
        Self::Fire
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiquidTypeRow {
    pub id: LiquidTypeKey,
    pub name: String,
    pub ty: Type,
    pub spell: SpellKey,
}

