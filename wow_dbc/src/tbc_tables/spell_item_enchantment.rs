use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::item_visuals::ItemVisualsKey;
use crate::tbc_tables::spell_item_enchantment_condition::SpellItemEnchantmentConditionKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellItemEnchantment {
    pub rows: Vec<SpellItemEnchantmentRow>,
}

impl DbcTable for SpellItemEnchantment {
    type Row = SpellItemEnchantmentRow;

    const FILENAME: &'static str = "SpellItemEnchantment.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 136 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 136,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 34 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 34,
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

            // id: primary_key (SpellItemEnchantment) int32
            let id = SpellItemEnchantmentKey::new(crate::util::read_i32_le(chunk)?);

            // effect: int32[3]
            let effect = crate::util::read_array_i32::<3>(chunk)?;

            // effect_points_min: int32[3]
            let effect_points_min = crate::util::read_array_i32::<3>(chunk)?;

            // effect_points_max: int32[3]
            let effect_points_max = crate::util::read_array_i32::<3>(chunk)?;

            // effect_arg: int32[3]
            let effect_arg = crate::util::read_array_i32::<3>(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // item_visual: foreign_key (ItemVisuals) int32
            let item_visual = ItemVisualsKey::new(crate::util::read_i32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // src_item_id: int32
            let src_item_id = crate::util::read_i32_le(chunk)?;

            // condition_id: foreign_key (SpellItemEnchantmentCondition) int32
            let condition_id = SpellItemEnchantmentConditionKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(SpellItemEnchantmentRow {
                id,
                effect,
                effect_points_min,
                effect_points_max,
                effect_arg,
                name_lang,
                item_visual,
                flags,
                src_item_id,
                condition_id,
            });
        }

        Ok(SpellItemEnchantment { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 34,
            record_size: 136,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SpellItemEnchantment) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // effect: int32[3]
            for i in row.effect {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_points_min: int32[3]
            for i in row.effect_points_min {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_points_max: int32[3]
            for i in row.effect_points_max {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_arg: int32[3]
            for i in row.effect_arg {
                b.write_all(&i.to_le_bytes())?;
            }


            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // item_visual: foreign_key (ItemVisuals) int32
            b.write_all(&(row.item_visual.id as i32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // src_item_id: int32
            b.write_all(&row.src_item_id.to_le_bytes())?;

            // condition_id: foreign_key (SpellItemEnchantmentCondition) int32
            b.write_all(&(row.condition_id.id as i32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SpellItemEnchantment {
    type PrimaryKey = SpellItemEnchantmentKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl SpellItemEnchantment {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellItemEnchantmentKey {
    pub id: i32
}

impl SpellItemEnchantmentKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SpellItemEnchantmentKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for SpellItemEnchantmentKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for SpellItemEnchantmentKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl From<u8> for SpellItemEnchantmentKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for SpellItemEnchantmentKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellItemEnchantmentRow {
    pub id: SpellItemEnchantmentKey,
    pub effect: [i32; 3],
    pub effect_points_min: [i32; 3],
    pub effect_points_max: [i32; 3],
    pub effect_arg: [i32; 3],
    pub name_lang: ExtendedLocalizedString,
    pub item_visual: ItemVisualsKey,
    pub flags: i32,
    pub src_item_id: i32,
    pub condition_id: SpellItemEnchantmentConditionKey,
}

