use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChrClasses {
    pub rows: Vec<ChrClassesRow>,
}

impl DbcTable for ChrClasses {
    type Row = ChrClassesRow;

    fn filename() -> &'static str { "ChrClasses.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 68 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 68,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 17 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 17,
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

            // id: primary_key (ChrClasses) uint32
            let id = ChrClassesKey::new(crate::util::read_u32_le(chunk)?);

            // player_class: uint32
            let player_class = crate::util::read_u32_le(chunk)?;

            // damage_bonus_stat: int32
            let damage_bonus_stat = crate::util::read_i32_le(chunk)?;

            // power_type: PowerType
            let power_type = PowerType::try_from(crate::util::read_i32_le(chunk)?)?;

            // pet_name_token: string_ref
            let pet_name_token = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // filename: string_ref
            let filename = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // class_mask: int32
            let class_mask = crate::util::read_i32_le(chunk)?;

            // hybrid_class: bool32
            let hybrid_class = crate::util::read_u32_le(chunk)? != 0;


            rows.push(ChrClassesRow {
                id,
                player_class,
                damage_bonus_stat,
                power_type,
                pet_name_token,
                name,
                filename,
                class_mask,
                hybrid_class,
            });
        }

        Ok(ChrClasses { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 17,
            record_size: 68,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ChrClasses) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // player_class: uint32
            b.write_all(&row.player_class.to_le_bytes())?;

            // damage_bonus_stat: int32
            b.write_all(&row.damage_bonus_stat.to_le_bytes())?;

            // power_type: PowerType
            b.write_all(&(row.power_type.as_int() as i32).to_le_bytes())?;

            // pet_name_token: string_ref
            if !row.pet_name_token.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.pet_name_token.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // filename: string_ref
            if !row.filename.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.filename.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // class_mask: int32
            b.write_all(&row.class_mask.to_le_bytes())?;

            // hybrid_class: bool32
            b.write_all(&u32::from(row.hybrid_class).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ChrClasses {
    type PrimaryKey = ChrClassesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ChrClasses {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.pet_name_token.is_empty() { b.write_all(row.pet_name_token.as_bytes())?; b.write_all(&[0])?; };
            row.name.string_block_as_array(b)?;
            if !row.filename.is_empty() { b.write_all(row.filename.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.pet_name_token.is_empty() { sum += row.pet_name_token.len() + 1; };
            sum += row.name.string_block_size();
            if !row.filename.is_empty() { sum += row.filename.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ChrClassesKey {
    pub id: u32
}

impl ChrClassesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for ChrClassesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ChrClassesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for ChrClassesKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PowerType {
    Mana,
    Rage,
    Focus,
    Energy,
    Happiness,
}

impl PowerType {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            0 => Self::Mana,
            1 => Self::Rage,
            2 => Self::Focus,
            3 => Self::Energy,
            4 => Self::Happiness,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for PowerType {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("PowerType", value as i64))
    }

}

impl PowerType {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::Mana => 0,
            Self::Rage => 1,
            Self::Focus => 2,
            Self::Energy => 3,
            Self::Happiness => 4,
        }

    }

}

impl Default for PowerType {
    fn default() -> Self {
        Self::Mana
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChrClassesRow {
    pub id: ChrClassesKey,
    pub player_class: u32,
    pub damage_bonus_stat: i32,
    pub power_type: PowerType,
    pub pet_name_token: String,
    pub name: LocalizedString,
    pub filename: String,
    pub class_mask: i32,
    pub hybrid_class: bool,
}

