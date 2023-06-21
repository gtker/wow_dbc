use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::Gender;
use crate::vanilla_tables::chr_races::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharSections {
    pub rows: Vec<CharSectionsRow>,
}

impl DbcTable for CharSections {
    type Row = CharSectionsRow;

    fn filename() -> &'static str { "CharSections.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 40 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 40,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 10 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 10,
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

            // id: primary_key (CharSections) uint32
            let id = CharSectionsKey::new(crate::util::read_u32_le(chunk)?);

            // race: foreign_key (ChrRaces) uint32
            let race = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // gender: Gender
            let gender = Gender::try_from(crate::util::read_i32_le(chunk)?)?;

            // ty: SelectionType
            let ty = SelectionType::try_from(crate::util::read_i32_le(chunk)?)?;

            // variation_index: int32
            let variation_index = crate::util::read_i32_le(chunk)?;

            // colour_index: int32
            let colour_index = crate::util::read_i32_le(chunk)?;

            // texture_name: string_ref[3]
            let texture_name = {
                let mut arr = Vec::with_capacity(3);
                for _ in 0..3 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // npc_only: bool32
            let npc_only = crate::util::read_u32_le(chunk)? != 0;


            rows.push(CharSectionsRow {
                id,
                race,
                gender,
                ty,
                variation_index,
                colour_index,
                texture_name,
                npc_only,
            });
        }

        Ok(CharSections { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 10,
            record_size: 40,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (CharSections) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // race: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race.id as u32).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i32).to_le_bytes())?;

            // ty: SelectionType
            b.write_all(&(row.ty.as_int() as i32).to_le_bytes())?;

            // variation_index: int32
            b.write_all(&row.variation_index.to_le_bytes())?;

            // colour_index: int32
            b.write_all(&row.colour_index.to_le_bytes())?;

            // texture_name: string_ref[3]
            for i in &row.texture_name {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // npc_only: bool32
            b.write_all(&u32::from(row.npc_only).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for CharSections {
    type PrimaryKey = CharSectionsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl CharSections {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            for s in &row.texture_name {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            for s in &row.texture_name {
                if !s.is_empty() { sum += s.len() + 1; };
            }

        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CharSectionsKey {
    pub id: u32
}

impl CharSectionsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for CharSectionsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for CharSectionsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for CharSectionsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SelectionType {
    BaseSkin,
    Face,
    FacialHair,
    Hair,
    Underwear,
}

impl SelectionType {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            0 => Self::BaseSkin,
            1 => Self::Face,
            2 => Self::FacialHair,
            3 => Self::Hair,
            4 => Self::Underwear,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for SelectionType {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("SelectionType", value as i64))
    }

}

impl SelectionType {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::BaseSkin => 0,
            Self::Face => 1,
            Self::FacialHair => 2,
            Self::Hair => 3,
            Self::Underwear => 4,
        }

    }

}

impl Default for SelectionType {
    fn default() -> Self {
        Self::BaseSkin
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharSectionsRow {
    pub id: CharSectionsKey,
    pub race: ChrRacesKey,
    pub gender: Gender,
    pub ty: SelectionType,
    pub variation_index: i32,
    pub colour_index: i32,
    pub texture_name: [String; 3],
    pub npc_only: bool,
}

