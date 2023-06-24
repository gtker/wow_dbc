use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::spell_icon::SpellIconKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellShapeshiftForm {
    pub rows: Vec<SpellShapeshiftFormRow>,
}

impl DbcTable for SpellShapeshiftForm {
    type Row = SpellShapeshiftFormRow;

    const FILENAME: &'static str = "SpellShapeshiftForm.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 56 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 56,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 14 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 14,
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

            // id: primary_key (SpellShapeshiftForm) uint32
            let id = SpellShapeshiftFormKey::new(crate::util::read_u32_le(chunk)?);

            // bonus_action_bar: int32
            let bonus_action_bar = crate::util::read_i32_le(chunk)?;

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // creature_type: int32
            let creature_type = crate::util::read_i32_le(chunk)?;

            // spell_icon: foreign_key (SpellIcon) uint32
            let spell_icon = SpellIconKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(SpellShapeshiftFormRow {
                id,
                bonus_action_bar,
                name,
                flags,
                creature_type,
                spell_icon,
            });
        }

        Ok(SpellShapeshiftForm { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 14,
            record_size: 56,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SpellShapeshiftForm) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // bonus_action_bar: int32
            b.write_all(&row.bonus_action_bar.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // creature_type: int32
            b.write_all(&row.creature_type.to_le_bytes())?;

            // spell_icon: foreign_key (SpellIcon) uint32
            b.write_all(&(row.spell_icon.id as u32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SpellShapeshiftForm {
    type PrimaryKey = SpellShapeshiftFormKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl SpellShapeshiftForm {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellShapeshiftFormKey {
    pub id: u32
}

impl SpellShapeshiftFormKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SpellShapeshiftFormKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SpellShapeshiftFormKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SpellShapeshiftFormKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellShapeshiftFormRow {
    pub id: SpellShapeshiftFormKey,
    pub bonus_action_bar: i32,
    pub name: LocalizedString,
    pub flags: i32,
    pub creature_type: i32,
    pub spell_icon: SpellIconKey,
}

