use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;

#[derive(Debug, Clone, PartialEq)]
pub struct ItemRandomProperties {
    pub rows: Vec<ItemRandomPropertiesRow>,
}

impl DbcTable for ItemRandomProperties {
    type Row = ItemRandomPropertiesRow;

    fn filename() -> &'static str { "ItemRandomProperties.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 64 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 64,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 64,
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

            // id: primary_key (ItemRandomProperties) uint32
            let id = ItemRandomPropertiesKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // spell_item_enchantment: uint32[5]
            let spell_item_enchantment = crate::util::read_array_u32::<5>(chunk)?;

            // suffix: string_ref_loc
            let suffix = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(ItemRandomPropertiesRow {
                id,
                name,
                spell_item_enchantment,
                suffix,
            });
        }

        Ok(ItemRandomProperties { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 16,
            record_size: 64,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ItemRandomProperties) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref
            if !row.name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // spell_item_enchantment: uint32[5]
            for i in row.spell_item_enchantment {
                b.write_all(&i.to_le_bytes())?;
            }


            // suffix: string_ref_loc
            b.write_all(&row.suffix.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ItemRandomProperties {
    type PrimaryKey = ItemRandomPropertiesKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ItemRandomProperties {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.name.is_empty() { b.write_all(row.name.as_bytes())?; b.write_all(&[0])?; };
            row.suffix.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.name.is_empty() { sum += row.name.len() + 1; };
            sum += row.suffix.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemRandomPropertiesKey {
    pub id: u32
}

impl ItemRandomPropertiesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemRandomPropertiesRow {
    pub id: ItemRandomPropertiesKey,
    pub name: String,
    pub spell_item_enchantment: [u32; 5],
    pub suffix: LocalizedString,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn item_random_properties() {
        let contents = include_bytes!("../../../dbc/ItemRandomProperties.dbc");
        let actual = ItemRandomProperties::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ItemRandomProperties::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
