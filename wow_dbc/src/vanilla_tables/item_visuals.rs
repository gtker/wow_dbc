use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemVisuals {
    pub rows: Vec<ItemVisualsRow>,
}

impl DbcTable for ItemVisuals {
    type Row = ItemVisualsRow;

    fn filename() -> &'static str { "ItemVisuals.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 24 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 24,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 6 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 6,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (ItemVisuals) uint32
            let id = ItemVisualsKey::new(crate::util::read_u32_le(chunk)?);

            // item_visual_effects: uint32[5]
            let item_visual_effects = crate::util::read_array_u32::<5>(chunk)?;


            rows.push(ItemVisualsRow {
                id,
                item_visual_effects,
            });
        }

        Ok(ItemVisuals { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 6,
            record_size: 24,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (ItemVisuals) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // item_visual_effects: uint32[5]
            for i in row.item_visual_effects {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for ItemVisuals {
    type PrimaryKey = ItemVisualsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstItemVisuals<const S: usize> {
    pub rows: [ItemVisualsRow; S],
}

impl<const S: usize> ConstItemVisuals<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 24 {
            panic!("invalid record size, expected 24")
        }

        if header.field_count != 6 {
            panic!("invalid field count, expected 6")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ItemVisualsRow {
                id: ItemVisualsKey::new(0),
                item_visual_effects: [0; 5],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (ItemVisuals) uint32
            let id = ItemVisualsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // item_visual_effects: uint32[5]
            let item_visual_effects = {
                let mut a = [0; 5];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = ItemVisualsRow {
                id,
                item_visual_effects,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> ItemVisuals {
        ItemVisuals {
            rows: self.rows.iter().map(|s| ItemVisualsRow {
                id: s.id,
                item_visual_effects: s.item_visual_effects,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ItemVisualsKey {
    pub id: u32
}

impl ItemVisualsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for ItemVisualsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ItemVisualsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for ItemVisualsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemVisualsRow {
    pub id: ItemVisualsKey,
    pub item_visual_effects: [u32; 5],
}

