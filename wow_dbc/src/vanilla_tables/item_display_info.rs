use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::item_group_sounds::*;
use crate::vanilla_tables::item_visuals::*;
use crate::vanilla_tables::spell_visual::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemDisplayInfo {
    pub rows: Vec<ItemDisplayInfoRow>,
}

impl DbcTable for ItemDisplayInfo {
    type Row = ItemDisplayInfoRow;

    fn filename() -> &'static str { "ItemDisplayInfo.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 92 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 92,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 23 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 23,
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

            // id: primary_key (ItemDisplayInfo) uint32
            let id = ItemDisplayInfoKey::new(crate::util::read_u32_le(chunk)?);

            // model_name: string_ref[2]
            let model_name = {
                let mut arr = Vec::with_capacity(2);
                for _ in 0..2 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // model_texture: string_ref[2]
            let model_texture = {
                let mut arr = Vec::with_capacity(2);
                for _ in 0..2 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // inventory_icon: string_ref[2]
            let inventory_icon = {
                let mut arr = Vec::with_capacity(2);
                for _ in 0..2 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // geoset_group: int32[3]
            let geoset_group = crate::util::read_array_i32::<3>(chunk)?;

            // spell_visual: foreign_key (SpellVisual) uint32
            let spell_visual = SpellVisualKey::new(crate::util::read_u32_le(chunk)?.into());

            // group_sound_index: foreign_key (ItemGroupSounds) uint32
            let group_sound_index = ItemGroupSoundsKey::new(crate::util::read_u32_le(chunk)?.into());

            // helmet_geoset_vis: uint32[2]
            let helmet_geoset_vis = crate::util::read_array_u32::<2>(chunk)?;

            // textures: string_ref[8]
            let textures = {
                let mut arr = Vec::with_capacity(8);
                for _ in 0..8 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // item_visual: foreign_key (ItemVisuals) uint32
            let item_visual = ItemVisualsKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(ItemDisplayInfoRow {
                id,
                model_name,
                model_texture,
                inventory_icon,
                geoset_group,
                spell_visual,
                group_sound_index,
                helmet_geoset_vis,
                textures,
                item_visual,
            });
        }

        Ok(ItemDisplayInfo { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 23,
            record_size: 92,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ItemDisplayInfo) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model_name: string_ref[2]
            for i in &row.model_name {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // model_texture: string_ref[2]
            for i in &row.model_texture {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // inventory_icon: string_ref[2]
            for i in &row.inventory_icon {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // geoset_group: int32[3]
            for i in row.geoset_group {
                b.write_all(&i.to_le_bytes())?;
            }


            // spell_visual: foreign_key (SpellVisual) uint32
            b.write_all(&(row.spell_visual.id as u32).to_le_bytes())?;

            // group_sound_index: foreign_key (ItemGroupSounds) uint32
            b.write_all(&(row.group_sound_index.id as u32).to_le_bytes())?;

            // helmet_geoset_vis: uint32[2]
            for i in row.helmet_geoset_vis {
                b.write_all(&i.to_le_bytes())?;
            }


            // textures: string_ref[8]
            for i in &row.textures {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // item_visual: foreign_key (ItemVisuals) uint32
            b.write_all(&(row.item_visual.id as u32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ItemDisplayInfo {
    type PrimaryKey = ItemDisplayInfoKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ItemDisplayInfo {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            for s in &row.model_name {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

            for s in &row.model_texture {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

            for s in &row.inventory_icon {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

            for s in &row.textures {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            for s in &row.model_name {
                if !s.is_empty() { sum += s.len() + 1; };
            }

            for s in &row.model_texture {
                if !s.is_empty() { sum += s.len() + 1; };
            }

            for s in &row.inventory_icon {
                if !s.is_empty() { sum += s.len() + 1; };
            }

            for s in &row.textures {
                if !s.is_empty() { sum += s.len() + 1; };
            }

        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstItemDisplayInfo<const S: usize> {
    pub rows: [ConstItemDisplayInfoRow; S],
}

impl<const S: usize> ConstItemDisplayInfo<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 92 {
            panic!("invalid record size, expected 92")
        }

        if header.field_count != 23 {
            panic!("invalid field count, expected 23")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstItemDisplayInfoRow {
                id: ItemDisplayInfoKey::new(0),
                model_name: [""; 2],
                model_texture: [""; 2],
                inventory_icon: [""; 2],
                geoset_group: [0; 3],
                spell_visual: SpellVisualKey::new(0),
                group_sound_index: ItemGroupSoundsKey::new(0),
                helmet_geoset_vis: [0; 2],
                textures: [""; 8],
                item_visual: ItemVisualsKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (ItemDisplayInfo) uint32
            let id = ItemDisplayInfoKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // model_name: string_ref[2]
            let model_name = {
                let mut a = [""; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::get_string_from_block(b_offset, b, string_block);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // model_texture: string_ref[2]
            let model_texture = {
                let mut a = [""; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::get_string_from_block(b_offset, b, string_block);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // inventory_icon: string_ref[2]
            let inventory_icon = {
                let mut a = [""; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::get_string_from_block(b_offset, b, string_block);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // geoset_group: int32[3]
            let geoset_group = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // spell_visual: foreign_key (SpellVisual) uint32
            let spell_visual = SpellVisualKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // group_sound_index: foreign_key (ItemGroupSounds) uint32
            let group_sound_index = ItemGroupSoundsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // helmet_geoset_vis: uint32[2]
            let helmet_geoset_vis = {
                let mut a = [0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // textures: string_ref[8]
            let textures = {
                let mut a = [""; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::get_string_from_block(b_offset, b, string_block);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // item_visual: foreign_key (ItemVisuals) uint32
            let item_visual = ItemVisualsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = ConstItemDisplayInfoRow {
                id,
                model_name,
                model_texture,
                inventory_icon,
                geoset_group,
                spell_visual,
                group_sound_index,
                helmet_geoset_vis,
                textures,
                item_visual,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> ItemDisplayInfo {
        ItemDisplayInfo {
            rows: self.rows.iter().map(|s| ItemDisplayInfoRow {
                id: s.id,
                model_name: s.model_name.map(|a| a.to_string()),
                model_texture: s.model_texture.map(|a| a.to_string()),
                inventory_icon: s.inventory_icon.map(|a| a.to_string()),
                geoset_group: s.geoset_group,
                spell_visual: s.spell_visual,
                group_sound_index: s.group_sound_index,
                helmet_geoset_vis: s.helmet_geoset_vis,
                textures: s.textures.map(|a| a.to_string()),
                item_visual: s.item_visual,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ItemDisplayInfoKey {
    pub id: u32
}

impl ItemDisplayInfoKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for ItemDisplayInfoKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ItemDisplayInfoKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for ItemDisplayInfoKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemDisplayInfoRow {
    pub id: ItemDisplayInfoKey,
    pub model_name: [String; 2],
    pub model_texture: [String; 2],
    pub inventory_icon: [String; 2],
    pub geoset_group: [i32; 3],
    pub spell_visual: SpellVisualKey,
    pub group_sound_index: ItemGroupSoundsKey,
    pub helmet_geoset_vis: [u32; 2],
    pub textures: [String; 8],
    pub item_visual: ItemVisualsKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstItemDisplayInfoRow {
    pub id: ItemDisplayInfoKey,
    pub model_name: [&'static str; 2],
    pub model_texture: [&'static str; 2],
    pub inventory_icon: [&'static str; 2],
    pub geoset_group: [i32; 3],
    pub spell_visual: SpellVisualKey,
    pub group_sound_index: ItemGroupSoundsKey,
    pub helmet_geoset_vis: [u32; 2],
    pub textures: [&'static str; 8],
    pub item_visual: ItemVisualsKey,
}

