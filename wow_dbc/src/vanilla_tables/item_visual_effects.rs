use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemVisualEffects {
    pub rows: Vec<ItemVisualEffectsRow>,
}

impl DbcTable for ItemVisualEffects {
    type Row = ItemVisualEffectsRow;

    fn filename() -> &'static str { "ItemVisualEffects.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 8,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 2 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 2,
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

            // id: primary_key (ItemVisualEffects) uint32
            let id = ItemVisualEffectsKey::new(crate::util::read_u32_le(chunk)?);

            // model_path: string_ref
            let model_path = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(ItemVisualEffectsRow {
                id,
                model_path,
            });
        }

        Ok(ItemVisualEffects { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 2,
            record_size: 8,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ItemVisualEffects) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model_path: string_ref
            if !row.model_path.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.model_path.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ItemVisualEffects {
    type PrimaryKey = ItemVisualEffectsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ItemVisualEffects {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.model_path.is_empty() { b.write_all(row.model_path.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.model_path.is_empty() { sum += row.model_path.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstItemVisualEffects<const S: usize> {
    pub rows: [ConstItemVisualEffectsRow; S],
}

impl<const S: usize> ConstItemVisualEffects<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 8 {
            panic!("invalid record size, expected 8")
        }

        if header.field_count != 2 {
            panic!("invalid field count, expected 2")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstItemVisualEffectsRow {
                id: ItemVisualEffectsKey::new(0),
                model_path: "",
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (ItemVisualEffects) uint32
            let id = ItemVisualEffectsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // model_path: string_ref
            let model_path = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            rows[i] = ConstItemVisualEffectsRow {
                id,
                model_path,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ItemVisualEffectsKey {
    pub id: u32
}

impl ItemVisualEffectsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for ItemVisualEffectsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ItemVisualEffectsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for ItemVisualEffectsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemVisualEffectsRow {
    pub id: ItemVisualEffectsKey,
    pub model_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstItemVisualEffectsRow {
    pub id: ItemVisualEffectsKey,
    pub model_path: &'static str,
}

