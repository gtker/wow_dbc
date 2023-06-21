use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};
use crate::tbc_tables::creature_type::*;
use crate::tbc_tables::spell_icon::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellShapeshiftForm {
    pub rows: Vec<SpellShapeshiftFormRow>,
}

impl DbcTable for SpellShapeshiftForm {
    type Row = SpellShapeshiftFormRow;

    fn filename() -> &'static str { "SpellShapeshiftForm.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 140 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 140,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 35 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 35,
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

            // id: primary_key (SpellShapeshiftForm) int32
            let id = SpellShapeshiftFormKey::new(crate::util::read_i32_le(chunk)?);

            // bonus_action_bar: int32
            let bonus_action_bar = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // creature_type: foreign_key (CreatureType) int32
            let creature_type = CreatureTypeKey::new(crate::util::read_i32_le(chunk)?.into());

            // attack_icon_id: foreign_key (SpellIcon) int32
            let attack_icon_id = SpellIconKey::new(crate::util::read_i32_le(chunk)?.into());

            // combat_round_time: int32
            let combat_round_time = crate::util::read_i32_le(chunk)?;

            // creature_display_id: int32[4]
            let creature_display_id = crate::util::read_array_i32::<4>(chunk)?;

            // preset_spell_id: int32[8]
            let preset_spell_id = crate::util::read_array_i32::<8>(chunk)?;


            rows.push(SpellShapeshiftFormRow {
                id,
                bonus_action_bar,
                name_lang,
                flags,
                creature_type,
                attack_icon_id,
                combat_round_time,
                creature_display_id,
                preset_spell_id,
            });
        }

        Ok(SpellShapeshiftForm { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 35,
            record_size: 140,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SpellShapeshiftForm) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // bonus_action_bar: int32
            b.write_all(&row.bonus_action_bar.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // creature_type: foreign_key (CreatureType) int32
            b.write_all(&(row.creature_type.id as i32).to_le_bytes())?;

            // attack_icon_id: foreign_key (SpellIcon) int32
            b.write_all(&(row.attack_icon_id.id as i32).to_le_bytes())?;

            // combat_round_time: int32
            b.write_all(&row.combat_round_time.to_le_bytes())?;

            // creature_display_id: int32[4]
            for i in row.creature_display_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // preset_spell_id: int32[8]
            for i in row.preset_spell_id {
                b.write_all(&i.to_le_bytes())?;
            }


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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstSpellShapeshiftForm<const S: usize> {
    pub rows: [ConstSpellShapeshiftFormRow; S],
}

impl<const S: usize> ConstSpellShapeshiftForm<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 140 {
            panic!("invalid record size, expected 140")
        }

        if header.field_count != 35 {
            panic!("invalid field count, expected 35")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstSpellShapeshiftFormRow {
                id: SpellShapeshiftFormKey::new(0),
                bonus_action_bar: 0,
                name_lang: crate::ConstExtendedLocalizedString::empty(),
                flags: 0,
                creature_type: CreatureTypeKey::new(0),
                attack_icon_id: SpellIconKey::new(0),
                combat_round_time: 0,
                creature_display_id: [0; 4],
                preset_spell_id: [0; 8],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SpellShapeshiftForm) int32
            let id = SpellShapeshiftFormKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // bonus_action_bar: int32
            let bonus_action_bar = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // name_lang: string_ref_loc (Extended)
            let name_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // creature_type: foreign_key (CreatureType) int32
            let creature_type = CreatureTypeKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // attack_icon_id: foreign_key (SpellIcon) int32
            let attack_icon_id = SpellIconKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // combat_round_time: int32
            let combat_round_time = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // creature_display_id: int32[4]
            let creature_display_id = {
                let mut a = [0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // preset_spell_id: int32[8]
            let preset_spell_id = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = ConstSpellShapeshiftFormRow {
                id,
                bonus_action_bar,
                name_lang,
                flags,
                creature_type,
                attack_icon_id,
                combat_round_time,
                creature_display_id,
                preset_spell_id,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellShapeshiftFormKey {
    pub id: i32
}

impl SpellShapeshiftFormKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SpellShapeshiftFormKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SpellShapeshiftFormKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SpellShapeshiftFormKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellShapeshiftFormRow {
    pub id: SpellShapeshiftFormKey,
    pub bonus_action_bar: i32,
    pub name_lang: ExtendedLocalizedString,
    pub flags: i32,
    pub creature_type: CreatureTypeKey,
    pub attack_icon_id: SpellIconKey,
    pub combat_round_time: i32,
    pub creature_display_id: [i32; 4],
    pub preset_spell_id: [i32; 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstSpellShapeshiftFormRow {
    pub id: SpellShapeshiftFormKey,
    pub bonus_action_bar: i32,
    pub name_lang: ConstExtendedLocalizedString,
    pub flags: i32,
    pub creature_type: CreatureTypeKey,
    pub attack_icon_id: SpellIconKey,
    pub combat_round_time: i32,
    pub creature_display_id: [i32; 4],
    pub preset_spell_id: [i32; 8],
}

