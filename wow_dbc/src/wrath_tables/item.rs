use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::material::MaterialKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Item {
    pub rows: Vec<ItemRow>,
}

impl DbcTable for Item {
    type Row = ItemRow;

    const FILENAME: &'static str = "Item.dbc";
    const FIELD_COUNT: usize = 8;
    const ROW_SIZE: usize = 32;

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

            // id: primary_key (Item) int32
            let id = ItemKey::new(crate::util::read_i32_le(chunk)?);

            // class_id: int32
            let class_id = crate::util::read_i32_le(chunk)?;

            // subclass_id: int32
            let subclass_id = crate::util::read_i32_le(chunk)?;

            // sound_override_subclass_id: int32
            let sound_override_subclass_id = crate::util::read_i32_le(chunk)?;

            // material: foreign_key (Material) int32
            let material = MaterialKey::new(crate::util::read_i32_le(chunk)?.into());

            // display_info_id: int32
            let display_info_id = crate::util::read_i32_le(chunk)?;

            // inventory_type: int32
            let inventory_type = crate::util::read_i32_le(chunk)?;

            // sheathe_type: int32
            let sheathe_type = crate::util::read_i32_le(chunk)?;


            rows.push(ItemRow {
                id,
                class_id,
                subclass_id,
                sound_override_subclass_id,
                material,
                display_info_id,
                inventory_type,
                sheathe_type,
            });
        }

        Ok(Item { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (Item) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // class_id: int32
            b.write_all(&row.class_id.to_le_bytes())?;

            // subclass_id: int32
            b.write_all(&row.subclass_id.to_le_bytes())?;

            // sound_override_subclass_id: int32
            b.write_all(&row.sound_override_subclass_id.to_le_bytes())?;

            // material: foreign_key (Material) int32
            b.write_all(&(row.material.id as i32).to_le_bytes())?;

            // display_info_id: int32
            b.write_all(&row.display_info_id.to_le_bytes())?;

            // inventory_type: int32
            b.write_all(&row.inventory_type.to_le_bytes())?;

            // sheathe_type: int32
            b.write_all(&row.sheathe_type.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for Item {
    type PrimaryKey = ItemKey;
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
pub struct ItemKey {
    pub id: i32
}

impl ItemKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for ItemKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for ItemKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for ItemKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for ItemKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for ItemKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for ItemKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ItemKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for ItemKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ItemKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ItemKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemRow {
    pub id: ItemKey,
    pub class_id: i32,
    pub subclass_id: i32,
    pub sound_override_subclass_id: i32,
    pub material: MaterialKey,
    pub display_info_id: i32,
    pub inventory_type: i32,
    pub sheathe_type: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn item() {
        let mut file = File::open("../wrath-dbc/Item.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Item::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Item::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
