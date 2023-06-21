use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};
use crate::vanilla_tables::item_visuals::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellItemEnchantment {
    pub rows: Vec<SpellItemEnchantmentRow>,
}

impl DbcTable for SpellItemEnchantment {
    type Row = SpellItemEnchantmentRow;

    fn filename() -> &'static str { "SpellItemEnchantment.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstSpellItemEnchantment<const S: usize> {
    pub rows: [ConstSpellItemEnchantmentRow; S],
}

impl<const S: usize> ConstSpellItemEnchantment<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 96 {
            panic!("invalid record size, expected 96")
        }

        if header.field_count != 24 {
            panic!("invalid field count, expected 24")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstSpellItemEnchantmentRow {
                id: SpellItemEnchantmentKey::new(0),
                enchantment_type: [0; 3],
                effect_points_min: [0; 3],
                effect_points_max: [0; 3],
                effect_arg: [0; 3],
                name: crate::ConstLocalizedString::empty(),
                item_visual: ItemVisualsKey::new(0),
                flags: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SpellItemEnchantment) uint32
            let id = SpellItemEnchantmentKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // enchantment_type: int32[3]
            let enchantment_type = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_points_min: int32[3]
            let effect_points_min = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_points_max: int32[3]
            let effect_points_max = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_arg: int32[3]
            let effect_arg = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // name: string_ref_loc
            let name = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            // item_visual: foreign_key (ItemVisuals) uint32
            let item_visual = ItemVisualsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstSpellItemEnchantmentRow {
                id,
                enchantment_type,
                effect_points_min,
                effect_points_max,
                effect_arg,
                name,
                item_visual,
                flags,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstSpellItemEnchantmentRow {
    pub id: SpellItemEnchantmentKey,
    pub enchantment_type: [i32; 3],
    pub effect_points_min: [i32; 3],
    pub effect_points_max: [i32; 3],
    pub effect_arg: [i32; 3],
    pub name: ConstLocalizedString,
    pub item_visual: ItemVisualsKey,
    pub flags: i32,
}

