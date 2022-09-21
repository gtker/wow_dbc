use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;
use crate::vanilla_tables::chr_classes::*;
use crate::vanilla_tables::chr_races::*;
use crate::vanilla_tables::spell_icon::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TalentTab {
    pub rows: Vec<TalentTabRow>,
}

impl DbcTable for TalentTab {
    type Row = TalentTabRow;

    fn filename() -> &'static str { "TalentTab.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 60 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 60,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 15 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 15,
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

            // id: primary_key (TalentTab) uint32
            let id = TalentTabKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // spell_icon: foreign_key (SpellIcon) uint32
            let spell_icon = SpellIconKey::new(crate::util::read_u32_le(chunk)?.into());

            // race_mask: foreign_key (ChrRaces) uint32
            let race_mask = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // class_mask: foreign_key (ChrClasses) uint32
            let class_mask = ChrClassesKey::new(crate::util::read_u32_le(chunk)?.into());

            // order_index: uint32
            let order_index = crate::util::read_u32_le(chunk)?;

            // background_file: string_ref
            let background_file = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(TalentTabRow {
                id,
                name,
                spell_icon,
                race_mask,
                class_mask,
                order_index,
                background_file,
            });
        }

        Ok(TalentTab { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 15,
            record_size: 60,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (TalentTab) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // spell_icon: foreign_key (SpellIcon) uint32
            b.write_all(&(row.spell_icon.id as u32).to_le_bytes())?;

            // race_mask: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race_mask.id as u32).to_le_bytes())?;

            // class_mask: foreign_key (ChrClasses) uint32
            b.write_all(&(row.class_mask.id as u32).to_le_bytes())?;

            // order_index: uint32
            b.write_all(&row.order_index.to_le_bytes())?;

            // background_file: string_ref
            if !row.background_file.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.background_file.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for TalentTab {
    type PrimaryKey = TalentTabKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl TalentTab {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name.string_block_as_array(b)?;
            if !row.background_file.is_empty() { b.write_all(row.background_file.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
            if !row.background_file.is_empty() { sum += row.background_file.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct TalentTabKey {
    pub id: u32
}

impl TalentTabKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TalentTabRow {
    pub id: TalentTabKey,
    pub name: LocalizedString,
    pub spell_icon: SpellIconKey,
    pub race_mask: ChrRacesKey,
    pub class_mask: ChrClassesKey,
    pub order_index: u32,
    pub background_file: String,
}

