use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::chr_classes::ChrClassesKey;
use crate::wrath_tables::chr_races::ChrRacesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharStartOutfit {
    pub rows: Vec<CharStartOutfitRow>,
}

impl DbcTable for CharStartOutfit {
    type Row = CharStartOutfitRow;

    const FILENAME: &'static str = "CharStartOutfit.dbc";
    const FIELD_COUNT: usize = 77;
    const ROW_SIZE: usize = 296;

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

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (CharStartOutfit) int32
            let id = CharStartOutfitKey::new(crate::util::read_i32_le(chunk)?);

            // race_id: foreign_key (ChrRaces) int8
            let race_id = ChrRacesKey::new(crate::util::read_i8_le(chunk)?.into());

            // class_id: foreign_key (ChrClasses) int8
            let class_id = ChrClassesKey::new(crate::util::read_i8_le(chunk)?.into());

            // sex_id: int8
            let sex_id = crate::util::read_i8_le(chunk)?;

            // outfit_id: int8
            let outfit_id = crate::util::read_i8_le(chunk)?;

            // item_id: int32[24]
            let item_id = crate::util::read_array_i32::<24>(chunk)?;

            // display_item_id: int32[24]
            let display_item_id = crate::util::read_array_i32::<24>(chunk)?;

            // inventory_type: int32[24]
            let inventory_type = crate::util::read_array_i32::<24>(chunk)?;


            rows.push(CharStartOutfitRow {
                id,
                race_id,
                class_id,
                sex_id,
                outfit_id,
                item_id,
                display_item_id,
                inventory_type,
            });
        }

        Ok(CharStartOutfit { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CharStartOutfit) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // race_id: foreign_key (ChrRaces) int8
            b.write_all(&(row.race_id.id as i8).to_le_bytes())?;

            // class_id: foreign_key (ChrClasses) int8
            b.write_all(&(row.class_id.id as i8).to_le_bytes())?;

            // sex_id: int8
            b.write_all(&row.sex_id.to_le_bytes())?;

            // outfit_id: int8
            b.write_all(&row.outfit_id.to_le_bytes())?;

            // item_id: int32[24]
            for i in row.item_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // display_item_id: int32[24]
            for i in row.display_item_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // inventory_type: int32[24]
            for i in row.inventory_type {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        assert_eq!(b.len(), self.rows.len() * Self::ROW_SIZE);
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: string_cache.size(),
        };

        w.write_all(&header.write_header())?;
        w.write_all(&b)?;
        w.write_all(string_cache.buffer())?;
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharStartOutfitKey {
    pub id: i32
}

impl CharStartOutfitKey {
    pub const fn new(id: i32) -> Self {
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

impl From<i8> for CharStartOutfitKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for CharStartOutfitKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for CharStartOutfitKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for CharStartOutfitKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for CharStartOutfitKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for CharStartOutfitKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for CharStartOutfitKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for CharStartOutfitKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharStartOutfitRow {
    pub id: CharStartOutfitKey,
    pub race_id: ChrRacesKey,
    pub class_id: ChrClassesKey,
    pub sex_id: i8,
    pub outfit_id: i8,
    pub item_id: [i32; 24],
    pub display_item_id: [i32; 24],
    pub inventory_type: [i32; 24],
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn char_start_outfit() {
        let mut file = File::open("../wrath-dbc/CharStartOutfit.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CharStartOutfit::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CharStartOutfit::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
