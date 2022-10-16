use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::currency_category::*;
use crate::wrath_tables::item::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CurrencyTypes {
    pub rows: Vec<CurrencyTypesRow>,
}

impl DbcTable for CurrencyTypes {
    type Row = CurrencyTypesRow;

    fn filename() -> &'static str { "CurrencyTypes.dbc" }

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

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (CurrencyTypes) int32
            let id = CurrencyTypesKey::new(crate::util::read_i32_le(chunk)?);

            // item_id: foreign_key (Item) int32
            let item_id = ItemKey::new(crate::util::read_i32_le(chunk)?.into());

            // category_id: foreign_key (CurrencyCategory) int32
            let category_id = CurrencyCategoryKey::new(crate::util::read_i32_le(chunk)?.into());

            // bit_index: int32
            let bit_index = crate::util::read_i32_le(chunk)?;


            rows.push(CurrencyTypesRow {
                id,
                item_id,
                category_id,
                bit_index,
            });
        }

        Ok(CurrencyTypes { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 4,
            record_size: 16,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (CurrencyTypes) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // item_id: foreign_key (Item) int32
            b.write_all(&(row.item_id.id as i32).to_le_bytes())?;

            // category_id: foreign_key (CurrencyCategory) int32
            b.write_all(&(row.category_id.id as i32).to_le_bytes())?;

            // bit_index: int32
            b.write_all(&row.bit_index.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for CurrencyTypes {
    type PrimaryKey = CurrencyTypesKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CurrencyTypesKey {
    pub id: i32
}

impl CurrencyTypesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CurrencyTypesRow {
    pub id: CurrencyTypesKey,
    pub item_id: ItemKey,
    pub category_id: CurrencyCategoryKey,
    pub bit_index: i32,
}

