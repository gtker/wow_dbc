use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::item_visuals::ItemVisualsKey;
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

        if header.record_size != 96 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 96,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 24 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 24,
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

            // id: primary_key (SpellItemEnchantment) uint32
            let id = SpellItemEnchantmentKey::new(crate::util::read_u32_le(chunk)?);

            // enchantment_type: int32[3]
            let enchantment_type = crate::util::read_array_i32::<3>(chunk)?;

            // effect_points_min: int32[3]
            let effect_points_min = crate::util::read_array_i32::<3>(chunk)?;

            // effect_points_max: int32[3]
            let effect_points_max = crate::util::read_array_i32::<3>(chunk)?;

            // effect_arg: int32[3]
            let effect_arg = crate::util::read_array_i32::<3>(chunk)?;

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // item_visual: foreign_key (ItemVisuals) uint32
            let item_visual = ItemVisualsKey::new(crate::util::read_u32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(SpellItemEnchantmentRow {
                id,
                enchantment_type,
                effect_points_min,
                effect_points_max,
                effect_arg,
                name,
                item_visual,
                flags,
            });
        }

        Ok(SpellItemEnchantment { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 24,
            record_size: 96,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SpellItemEnchantment) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // enchantment_type: int32[3]
            for i in row.enchantment_type {
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


            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // item_visual: foreign_key (ItemVisuals) uint32
            b.write_all(&(row.item_visual.id as u32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SpellItemEnchantment {
    type PrimaryKey = SpellItemEnchantmentKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl SpellItemEnchantment {
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
pub struct SpellItemEnchantmentKey {
    pub id: u32
}

impl SpellItemEnchantmentKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
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

impl From<u32> for SpellItemEnchantmentKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for SpellItemEnchantmentKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SpellItemEnchantmentKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for SpellItemEnchantmentKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for SpellItemEnchantmentKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for SpellItemEnchantmentKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SpellItemEnchantmentKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SpellItemEnchantmentKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellItemEnchantmentRow {
    pub id: SpellItemEnchantmentKey,
    pub enchantment_type: [i32; 3],
    pub effect_points_min: [i32; 3],
    pub effect_points_max: [i32; 3],
    pub effect_arg: [i32; 3],
    pub name: LocalizedString,
    pub item_visual: ItemVisualsKey,
    pub flags: i32,
}

