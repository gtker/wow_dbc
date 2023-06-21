use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemGroupSounds {
    pub rows: Vec<ItemGroupSoundsRow>,
}

impl DbcTable for ItemGroupSounds {
    type Row = ItemGroupSoundsRow;

    fn filename() -> &'static str { "ItemGroupSounds.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 20 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 20,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 5 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 5,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (ItemGroupSounds) uint32
            let id = ItemGroupSoundsKey::new(crate::util::read_u32_le(chunk)?);

            // sound_entry: uint32[4]
            let sound_entry = crate::util::read_array_u32::<4>(chunk)?;


            rows.push(ItemGroupSoundsRow {
                id,
                sound_entry,
            });
        }

        Ok(ItemGroupSounds { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 5,
            record_size: 20,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (ItemGroupSounds) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_entry: uint32[4]
            for i in row.sound_entry {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for ItemGroupSounds {
    type PrimaryKey = ItemGroupSoundsKey;
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
pub struct ConstItemGroupSounds<const S: usize> {
    pub rows: [ItemGroupSoundsRow; S],
}

impl<const S: usize> ConstItemGroupSounds<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 20 {
            panic!("invalid record size, expected 20")
        }

        if header.field_count != 5 {
            panic!("invalid field count, expected 5")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ItemGroupSoundsRow {
                id: ItemGroupSoundsKey::new(0),
                sound_entry: [0; 4],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (ItemGroupSounds) uint32
            let id = ItemGroupSoundsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_entry: uint32[4]
            let sound_entry = {
                let mut a = [0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = ItemGroupSoundsRow {
                id,
                sound_entry,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> ItemGroupSounds {
        ItemGroupSounds {
            rows: self.rows.iter().map(|s| ItemGroupSoundsRow {
                id: s.id,
                sound_entry: s.sound_entry,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ItemGroupSoundsKey {
    pub id: u32
}

impl ItemGroupSoundsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for ItemGroupSoundsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ItemGroupSoundsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for ItemGroupSoundsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemGroupSoundsRow {
    pub id: ItemGroupSoundsKey,
    pub sound_entry: [u32; 4],
}

