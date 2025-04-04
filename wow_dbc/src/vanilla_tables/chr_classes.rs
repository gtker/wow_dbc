use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;
use wow_world_base::vanilla::Power;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChrClasses {
    pub rows: Vec<ChrClassesRow>,
}

impl DbcTable for ChrClasses {
    type Row = ChrClassesRow;

    const FILENAME: &'static str = "ChrClasses.dbc";
    const FIELD_COUNT: usize = 17;
    const ROW_SIZE: usize = 68;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
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

            // power_type: Power
            let power_type = crate::util::read_i32_le(chunk)?.try_into()?;

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
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
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

            // power_type: Power
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
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl TryFrom<u64> for ChrClassesKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ChrClassesKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for ChrClassesKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for ChrClassesKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for ChrClassesKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ChrClassesKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ChrClassesKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChrClassesRow {
    pub id: ChrClassesKey,
    pub player_class: u32,
    pub damage_bonus_stat: i32,
    pub power_type: Power,
    pub pet_name_token: String,
    pub name: LocalizedString,
    pub filename: String,
    pub class_mask: i32,
    pub hybrid_class: bool,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn chr_classes() {
        let mut file = File::open("../vanilla-dbc/ChrClasses.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ChrClasses::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ChrClasses::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
