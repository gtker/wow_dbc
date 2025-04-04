use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::item_group_sounds::ItemGroupSoundsKey;
use crate::vanilla_tables::item_visuals::ItemVisualsKey;
use crate::vanilla_tables::spell_visual::SpellVisualKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemDisplayInfo {
    pub rows: Vec<ItemDisplayInfoRow>,
}

impl DbcTable for ItemDisplayInfo {
    type Row = ItemDisplayInfoRow;

    const FILENAME: &'static str = "ItemDisplayInfo.dbc";
    const FIELD_COUNT: usize = 23;
    const ROW_SIZE: usize = 92;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
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
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
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
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl TryFrom<u64> for ItemDisplayInfoKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ItemDisplayInfoKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for ItemDisplayInfoKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for ItemDisplayInfoKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for ItemDisplayInfoKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ItemDisplayInfoKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ItemDisplayInfoKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn item_display_info() {
        let mut file = File::open("../vanilla-dbc/ItemDisplayInfo.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ItemDisplayInfo::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ItemDisplayInfo::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
