use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::chr_classes::ChrClassesKey;
use crate::vanilla_tables::chr_races::ChrRacesKey;
use std::io::Write;
use wow_world_base::vanilla::Gender;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharStartOutfit {
    pub rows: Vec<CharStartOutfitRow>,
}

impl DbcTable for CharStartOutfit {
    type Row = CharStartOutfitRow;

    const FILENAME: &'static str = "CharStartOutfit.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 152 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 152,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 41 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 41,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (CharStartOutfit) uint32
            let id = CharStartOutfitKey::new(crate::util::read_u32_le(chunk)?);

            // race: foreign_key (ChrRaces) uint8
            let race = ChrRacesKey::new(crate::util::read_u8_le(chunk)?.into());

            // class: foreign_key (ChrClasses) uint8
            let class = ChrClassesKey::new(crate::util::read_u8_le(chunk)?.into());

            // gender: Gender
            let gender = crate::util::read_i8_le(chunk)?.try_into()?;

            // outfit_id: int8
            let outfit_id = crate::util::read_i8_le(chunk)?;

            // item_id: int32[12]
            let item_id = crate::util::read_array_i32::<12>(chunk)?;

            // display_id: int32[12]
            let display_id = crate::util::read_array_i32::<12>(chunk)?;

            // inv_slot_id: int32[12]
            let inv_slot_id = crate::util::read_array_i32::<12>(chunk)?;


            rows.push(CharStartOutfitRow {
                id,
                race,
                class,
                gender,
                outfit_id,
                item_id,
                display_id,
                inv_slot_id,
            });
        }

        Ok(CharStartOutfit { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 41,
            record_size: 152,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (CharStartOutfit) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // race: foreign_key (ChrRaces) uint8
            b.write_all(&(row.race.id as u8).to_le_bytes())?;

            // class: foreign_key (ChrClasses) uint8
            b.write_all(&(row.class.id as u8).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i8).to_le_bytes())?;

            // outfit_id: int8
            b.write_all(&row.outfit_id.to_le_bytes())?;

            // item_id: int32[12]
            for i in row.item_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // display_id: int32[12]
            for i in row.display_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // inv_slot_id: int32[12]
            for i in row.inv_slot_id {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for CharStartOutfit {
    type PrimaryKey = CharStartOutfitKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CharStartOutfitKey {
    pub id: u32
}

impl CharStartOutfitKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for CharStartOutfitKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for CharStartOutfitKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for CharStartOutfitKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for CharStartOutfitKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for CharStartOutfitKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for CharStartOutfitKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for CharStartOutfitKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for CharStartOutfitKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for CharStartOutfitKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for CharStartOutfitKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharStartOutfitRow {
    pub id: CharStartOutfitKey,
    pub race: ChrRacesKey,
    pub class: ChrClassesKey,
    pub gender: Gender,
    pub outfit_id: i8,
    pub item_id: [i32; 12],
    pub display_id: [i32; 12],
    pub inv_slot_id: [i32; 12],
}

