use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};
use crate::wrath_tables::faction::*;
use crate::wrath_tables::map::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LFGDungeons {
    pub rows: Vec<LFGDungeonsRow>,
}

impl DbcTable for LFGDungeons {
    type Row = LFGDungeonsRow;

    fn filename() -> &'static str { "LFGDungeons.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 196 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 196,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 49 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 49,
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

            // id: primary_key (LFGDungeons) int32
            let id = LFGDungeonsKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;

            // max_level: int32
            let max_level = crate::util::read_i32_le(chunk)?;

            // target_level: int32
            let target_level = crate::util::read_i32_le(chunk)?;

            // target_level_min: int32
            let target_level_min = crate::util::read_i32_le(chunk)?;

            // target_level_max: int32
            let target_level_max = crate::util::read_i32_le(chunk)?;

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // difficulty: int32
            let difficulty = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // type_id: int32
            let type_id = crate::util::read_i32_le(chunk)?;

            // faction: foreign_key (Faction) int32
            let faction = FactionKey::new(crate::util::read_i32_le(chunk)?.into());

            // texture_filename: string_ref
            let texture_filename = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // expansion_level: int32
            let expansion_level = crate::util::read_i32_le(chunk)?;

            // order_index: int32
            let order_index = crate::util::read_i32_le(chunk)?;

            // group_id: int32
            let group_id = crate::util::read_i32_le(chunk)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(LFGDungeonsRow {
                id,
                name_lang,
                min_level,
                max_level,
                target_level,
                target_level_min,
                target_level_max,
                map_id,
                difficulty,
                flags,
                type_id,
                faction,
                texture_filename,
                expansion_level,
                order_index,
                group_id,
                description_lang,
            });
        }

        Ok(LFGDungeons { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 49,
            record_size: 196,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (LFGDungeons) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

            // max_level: int32
            b.write_all(&row.max_level.to_le_bytes())?;

            // target_level: int32
            b.write_all(&row.target_level.to_le_bytes())?;

            // target_level_min: int32
            b.write_all(&row.target_level_min.to_le_bytes())?;

            // target_level_max: int32
            b.write_all(&row.target_level_max.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // difficulty: int32
            b.write_all(&row.difficulty.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // type_id: int32
            b.write_all(&row.type_id.to_le_bytes())?;

            // faction: foreign_key (Faction) int32
            b.write_all(&(row.faction.id as i32).to_le_bytes())?;

            // texture_filename: string_ref
            if !row.texture_filename.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.texture_filename.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // expansion_level: int32
            b.write_all(&row.expansion_level.to_le_bytes())?;

            // order_index: int32
            b.write_all(&row.order_index.to_le_bytes())?;

            // group_id: int32
            b.write_all(&row.group_id.to_le_bytes())?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for LFGDungeons {
    type PrimaryKey = LFGDungeonsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl LFGDungeons {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
            if !row.texture_filename.is_empty() { b.write_all(row.texture_filename.as_bytes())?; b.write_all(&[0])?; };
            row.description_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
            if !row.texture_filename.is_empty() { sum += row.texture_filename.len() + 1; };
            sum += row.description_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstLFGDungeons<const S: usize> {
    pub rows: [ConstLFGDungeonsRow; S],
}

impl<const S: usize> ConstLFGDungeons<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 196 {
            panic!("invalid record size, expected 196")
        }

        if header.field_count != 49 {
            panic!("invalid field count, expected 49")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstLFGDungeonsRow {
                id: LFGDungeonsKey::new(0),
                name_lang: crate::ConstExtendedLocalizedString::empty(),
                min_level: 0,
                max_level: 0,
                target_level: 0,
                target_level_min: 0,
                target_level_max: 0,
                map_id: MapKey::new(0),
                difficulty: 0,
                flags: 0,
                type_id: 0,
                faction: FactionKey::new(0),
                texture_filename: "",
                expansion_level: 0,
                order_index: 0,
                group_id: 0,
                description_lang: crate::ConstExtendedLocalizedString::empty(),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (LFGDungeons) int32
            let id = LFGDungeonsKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
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

            // min_level: int32
            let min_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // max_level: int32
            let max_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // target_level: int32
            let target_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // target_level_min: int32
            let target_level_min = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // target_level_max: int32
            let target_level_max = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // difficulty: int32
            let difficulty = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // type_id: int32
            let type_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // faction: foreign_key (Faction) int32
            let faction = FactionKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // texture_filename: string_ref
            let texture_filename = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // expansion_level: int32
            let expansion_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // order_index: int32
            let order_index = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // group_id: int32
            let group_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // description_lang: string_ref_loc (Extended)
            let description_lang = ConstExtendedLocalizedString::new(
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

            rows[i] = ConstLFGDungeonsRow {
                id,
                name_lang,
                min_level,
                max_level,
                target_level,
                target_level_min,
                target_level_max,
                map_id,
                difficulty,
                flags,
                type_id,
                faction,
                texture_filename,
                expansion_level,
                order_index,
                group_id,
                description_lang,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> LFGDungeons {
        LFGDungeons {
            rows: self.rows.iter().map(|s| LFGDungeonsRow {
                id: s.id,
                name_lang: s.name_lang.to_string(),
                min_level: s.min_level,
                max_level: s.max_level,
                target_level: s.target_level,
                target_level_min: s.target_level_min,
                target_level_max: s.target_level_max,
                map_id: s.map_id,
                difficulty: s.difficulty,
                flags: s.flags,
                type_id: s.type_id,
                faction: s.faction,
                texture_filename: s.texture_filename.to_string(),
                expansion_level: s.expansion_level,
                order_index: s.order_index,
                group_id: s.group_id,
                description_lang: s.description_lang.to_string(),
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct LFGDungeonsKey {
    pub id: i32
}

impl LFGDungeonsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for LFGDungeonsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for LFGDungeonsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for LFGDungeonsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for LFGDungeonsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for LFGDungeonsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LFGDungeonsRow {
    pub id: LFGDungeonsKey,
    pub name_lang: ExtendedLocalizedString,
    pub min_level: i32,
    pub max_level: i32,
    pub target_level: i32,
    pub target_level_min: i32,
    pub target_level_max: i32,
    pub map_id: MapKey,
    pub difficulty: i32,
    pub flags: i32,
    pub type_id: i32,
    pub faction: FactionKey,
    pub texture_filename: String,
    pub expansion_level: i32,
    pub order_index: i32,
    pub group_id: i32,
    pub description_lang: ExtendedLocalizedString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstLFGDungeonsRow {
    pub id: LFGDungeonsKey,
    pub name_lang: ConstExtendedLocalizedString,
    pub min_level: i32,
    pub max_level: i32,
    pub target_level: i32,
    pub target_level_min: i32,
    pub target_level_max: i32,
    pub map_id: MapKey,
    pub difficulty: i32,
    pub flags: i32,
    pub type_id: i32,
    pub faction: FactionKey,
    pub texture_filename: &'static str,
    pub expansion_level: i32,
    pub order_index: i32,
    pub group_id: i32,
    pub description_lang: ConstExtendedLocalizedString,
}

