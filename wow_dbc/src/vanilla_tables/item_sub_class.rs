use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::{ConstLocalizedString, LocalizedString};
use crate::vanilla_tables::item_class::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemSubClass {
    pub rows: Vec<ItemSubClassRow>,
}

impl DbcTable for ItemSubClass {
    type Row = ItemSubClassRow;

    fn filename() -> &'static str { "ItemSubClass.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 112 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 112,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 28 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 28,
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

            // item_class: foreign_key (ItemClass) uint32
            let item_class = ItemClassKey::new(crate::util::read_u32_le(chunk)?.into());

            // subclass: int32
            let subclass = crate::util::read_i32_le(chunk)?;

            // prerequisite_proficiency: int32
            let prerequisite_proficiency = crate::util::read_i32_le(chunk)?;

            // postrequisite_proficiency: int32
            let postrequisite_proficiency = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // display_flags: int32
            let display_flags = crate::util::read_i32_le(chunk)?;

            // weapon_parry_sequence: int32
            let weapon_parry_sequence = crate::util::read_i32_le(chunk)?;

            // weapon_ready_sequence: int32
            let weapon_ready_sequence = crate::util::read_i32_le(chunk)?;

            // weapon_attack_sequence: int32
            let weapon_attack_sequence = crate::util::read_i32_le(chunk)?;

            // weapon_swing_size: int32
            let weapon_swing_size = crate::util::read_i32_le(chunk)?;

            // display_name: string_ref_loc
            let display_name = crate::util::read_localized_string(chunk, &string_block)?;

            // verbose_name: string_ref_loc
            let verbose_name = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(ItemSubClassRow {
                item_class,
                subclass,
                prerequisite_proficiency,
                postrequisite_proficiency,
                flags,
                display_flags,
                weapon_parry_sequence,
                weapon_ready_sequence,
                weapon_attack_sequence,
                weapon_swing_size,
                display_name,
                verbose_name,
            });
        }

        Ok(ItemSubClass { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 28,
            record_size: 112,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // item_class: foreign_key (ItemClass) uint32
            b.write_all(&(row.item_class.id as u32).to_le_bytes())?;

            // subclass: int32
            b.write_all(&row.subclass.to_le_bytes())?;

            // prerequisite_proficiency: int32
            b.write_all(&row.prerequisite_proficiency.to_le_bytes())?;

            // postrequisite_proficiency: int32
            b.write_all(&row.postrequisite_proficiency.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // display_flags: int32
            b.write_all(&row.display_flags.to_le_bytes())?;

            // weapon_parry_sequence: int32
            b.write_all(&row.weapon_parry_sequence.to_le_bytes())?;

            // weapon_ready_sequence: int32
            b.write_all(&row.weapon_ready_sequence.to_le_bytes())?;

            // weapon_attack_sequence: int32
            b.write_all(&row.weapon_attack_sequence.to_le_bytes())?;

            // weapon_swing_size: int32
            b.write_all(&row.weapon_swing_size.to_le_bytes())?;

            // display_name: string_ref_loc
            b.write_all(&row.display_name.string_indices_as_array(&mut string_index))?;

            // verbose_name: string_ref_loc
            b.write_all(&row.verbose_name.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl ItemSubClass {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.display_name.string_block_as_array(b)?;
            row.verbose_name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.display_name.string_block_size();
            sum += row.verbose_name.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstItemSubClass<const S: usize> {
    pub rows: [ConstItemSubClassRow; S],
}

impl<const S: usize> ConstItemSubClass<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 112 {
            panic!("invalid record size, expected 112")
        }

        if header.field_count != 28 {
            panic!("invalid field count, expected 28")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstItemSubClassRow {
                item_class: ItemClassKey::new(0),
                subclass: 0,
                prerequisite_proficiency: 0,
                postrequisite_proficiency: 0,
                flags: 0,
                display_flags: 0,
                weapon_parry_sequence: 0,
                weapon_ready_sequence: 0,
                weapon_attack_sequence: 0,
                weapon_swing_size: 0,
                display_name: crate::ConstLocalizedString::empty(),
                verbose_name: crate::ConstLocalizedString::empty(),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // item_class: foreign_key (ItemClass) uint32
            let item_class = ItemClassKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // subclass: int32
            let subclass = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // prerequisite_proficiency: int32
            let prerequisite_proficiency = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // postrequisite_proficiency: int32
            let postrequisite_proficiency = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // display_flags: int32
            let display_flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // weapon_parry_sequence: int32
            let weapon_parry_sequence = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // weapon_ready_sequence: int32
            let weapon_ready_sequence = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // weapon_attack_sequence: int32
            let weapon_attack_sequence = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // weapon_swing_size: int32
            let weapon_swing_size = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // display_name: string_ref_loc
            let display_name = ConstLocalizedString::new(
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

            // verbose_name: string_ref_loc
            let verbose_name = ConstLocalizedString::new(
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

            rows[i] = ConstItemSubClassRow {
                item_class,
                subclass,
                prerequisite_proficiency,
                postrequisite_proficiency,
                flags,
                display_flags,
                weapon_parry_sequence,
                weapon_ready_sequence,
                weapon_attack_sequence,
                weapon_swing_size,
                display_name,
                verbose_name,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> ItemSubClass {
        ItemSubClass {
            rows: self.rows.iter().map(|s| ItemSubClassRow {
                item_class: s.item_class,
                subclass: s.subclass,
                prerequisite_proficiency: s.prerequisite_proficiency,
                postrequisite_proficiency: s.postrequisite_proficiency,
                flags: s.flags,
                display_flags: s.display_flags,
                weapon_parry_sequence: s.weapon_parry_sequence,
                weapon_ready_sequence: s.weapon_ready_sequence,
                weapon_attack_sequence: s.weapon_attack_sequence,
                weapon_swing_size: s.weapon_swing_size,
                display_name: s.display_name.to_string(),
                verbose_name: s.verbose_name.to_string(),
            }).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemSubClassRow {
    pub item_class: ItemClassKey,
    pub subclass: i32,
    pub prerequisite_proficiency: i32,
    pub postrequisite_proficiency: i32,
    pub flags: i32,
    pub display_flags: i32,
    pub weapon_parry_sequence: i32,
    pub weapon_ready_sequence: i32,
    pub weapon_attack_sequence: i32,
    pub weapon_swing_size: i32,
    pub display_name: LocalizedString,
    pub verbose_name: LocalizedString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstItemSubClassRow {
    pub item_class: ItemClassKey,
    pub subclass: i32,
    pub prerequisite_proficiency: i32,
    pub postrequisite_proficiency: i32,
    pub flags: i32,
    pub display_flags: i32,
    pub weapon_parry_sequence: i32,
    pub weapon_ready_sequence: i32,
    pub weapon_attack_sequence: i32,
    pub weapon_swing_size: i32,
    pub display_name: ConstLocalizedString,
    pub verbose_name: ConstLocalizedString,
}

