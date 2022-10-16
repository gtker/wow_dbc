use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::chr_classes::*;
use crate::wrath_tables::chr_races::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharStartOutfit {
    pub rows: Vec<CharStartOutfitRow>,
}

impl DbcTable for CharStartOutfit {
    type Row = CharStartOutfitRow;

    fn filename() -> &'static str { "CharStartOutfit.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 296 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 296,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 77 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 77,
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

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 77,
            record_size: 296,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

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

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for CharStartOutfit {
    type PrimaryKey = CharStartOutfitKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CharStartOutfitKey {
    pub id: i32
}

impl CharStartOutfitKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

