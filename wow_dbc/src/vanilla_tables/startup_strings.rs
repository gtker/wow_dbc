use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Startup_Strings {
    pub rows: Vec<Startup_StringsRow>,
}

impl DbcTable for Startup_Strings {
    type Row = Startup_StringsRow;

    fn filename() -> &'static str { "Startup_Strings.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 44 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 44,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 11 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 11,
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

            // id: primary_key (Startup_Strings) uint32
            let id = Startup_StringsKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // startup_string: string_ref_loc
            let startup_string = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(Startup_StringsRow {
                id,
                name,
                startup_string,
            });
        }

        Ok(Startup_Strings { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 11,
            record_size: 44,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Startup_Strings) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref
            if !row.name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // startup_string: string_ref_loc
            b.write_all(&row.startup_string.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Startup_Strings {
    type PrimaryKey = Startup_StringsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl Startup_Strings {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.name.is_empty() { b.write_all(row.name.as_bytes())?; b.write_all(&[0])?; };
            row.startup_string.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.name.is_empty() { sum += row.name.len() + 1; };
            sum += row.startup_string.string_block_size();
        }

        sum as u32
    }

}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstStartup_Strings<const S: usize> {
    pub rows: [ConstStartup_StringsRow; S],
}

impl<const S: usize> ConstStartup_Strings<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 44 {
            panic!("invalid record size, expected 44")
        }

        if header.field_count != 11 {
            panic!("invalid field count, expected 11")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstStartup_StringsRow {
                id: Startup_StringsKey::new(0),
                name: "",
                startup_string: crate::ConstLocalizedString::empty(),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Startup_Strings) uint32
            let id = Startup_StringsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // name: string_ref
            let name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // startup_string: string_ref_loc
            let startup_string = ConstLocalizedString::new(
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

            rows[i] = ConstStartup_StringsRow {
                id,
                name,
                startup_string,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> Startup_Strings {
        Startup_Strings {
            rows: self.rows.iter().map(|s| Startup_StringsRow {
                id: s.id,
                name: s.name.to_string(),
                startup_string: s.startup_string.to_string(),
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct Startup_StringsKey {
    pub id: u32
}

impl Startup_StringsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for Startup_StringsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for Startup_StringsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for Startup_StringsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Startup_StringsRow {
    pub id: Startup_StringsKey,
    pub name: String,
    pub startup_string: LocalizedString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstStartup_StringsRow {
    pub id: Startup_StringsKey,
    pub name: &'static str,
    pub startup_string: ConstLocalizedString,
}

