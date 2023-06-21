use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemRandomSuffix {
    pub rows: Vec<ItemRandomSuffixRow>,
}

impl DbcTable for ItemRandomSuffix {
    type Row = ItemRandomSuffixRow;

    fn filename() -> &'static str { "ItemRandomSuffix.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 100 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 100,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 25 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 25,
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

            // id: primary_key (ItemRandomSuffix) int32
            let id = ItemRandomSuffixKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // internal_name: string_ref
            let internal_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // enchantment: int32[3]
            let enchantment = crate::util::read_array_i32::<3>(chunk)?;

            // allocation_pct: int32[3]
            let allocation_pct = crate::util::read_array_i32::<3>(chunk)?;


            rows.push(ItemRandomSuffixRow {
                id,
                name_lang,
                internal_name,
                enchantment,
                allocation_pct,
            });
        }

        Ok(ItemRandomSuffix { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 25,
            record_size: 100,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ItemRandomSuffix) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // internal_name: string_ref
            if !row.internal_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.internal_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // enchantment: int32[3]
            for i in row.enchantment {
                b.write_all(&i.to_le_bytes())?;
            }


            // allocation_pct: int32[3]
            for i in row.allocation_pct {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ItemRandomSuffix {
    type PrimaryKey = ItemRandomSuffixKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ItemRandomSuffix {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
            if !row.internal_name.is_empty() { b.write_all(row.internal_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
            if !row.internal_name.is_empty() { sum += row.internal_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstItemRandomSuffix<const S: usize> {
    pub rows: [ConstItemRandomSuffixRow; S],
}

impl<const S: usize> ConstItemRandomSuffix<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 100 {
            panic!("invalid record size, expected 100")
        }

        if header.field_count != 25 {
            panic!("invalid field count, expected 25")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstItemRandomSuffixRow {
                id: ItemRandomSuffixKey::new(0),
                name_lang: crate::ConstExtendedLocalizedString::empty(),
                internal_name: "",
                enchantment: [0; 3],
                allocation_pct: [0; 3],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (ItemRandomSuffix) int32
            let id = ItemRandomSuffixKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
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

            // internal_name: string_ref
            let internal_name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // enchantment: int32[3]
            let enchantment = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // allocation_pct: int32[3]
            let allocation_pct = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = ConstItemRandomSuffixRow {
                id,
                name_lang,
                internal_name,
                enchantment,
                allocation_pct,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> ItemRandomSuffix {
        ItemRandomSuffix {
            rows: self.rows.iter().map(|s| ItemRandomSuffixRow {
                id: s.id,
                name_lang: s.name_lang.to_string(),
                internal_name: s.internal_name.to_string(),
                enchantment: s.enchantment,
                allocation_pct: s.allocation_pct,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ItemRandomSuffixKey {
    pub id: i32
}

impl ItemRandomSuffixKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for ItemRandomSuffixKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for ItemRandomSuffixKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for ItemRandomSuffixKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for ItemRandomSuffixKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ItemRandomSuffixKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemRandomSuffixRow {
    pub id: ItemRandomSuffixKey,
    pub name_lang: ExtendedLocalizedString,
    pub internal_name: String,
    pub enchantment: [i32; 3],
    pub allocation_pct: [i32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstItemRandomSuffixRow {
    pub id: ItemRandomSuffixKey,
    pub name_lang: ConstExtendedLocalizedString,
    pub internal_name: &'static str,
    pub enchantment: [i32; 3],
    pub allocation_pct: [i32; 3],
}

