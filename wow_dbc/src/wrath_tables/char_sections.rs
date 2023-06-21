use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::chr_races::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharSections {
    pub rows: Vec<CharSectionsRow>,
}

impl DbcTable for CharSections {
    type Row = CharSectionsRow;

    fn filename() -> &'static str { "CharSections.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 40 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 40,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 10 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 10,
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

            // id: primary_key (CharSections) int32
            let id = CharSectionsKey::new(crate::util::read_i32_le(chunk)?);

            // race_id: foreign_key (ChrRaces) int32
            let race_id = ChrRacesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sex_id: int32
            let sex_id = crate::util::read_i32_le(chunk)?;

            // base_section: int32
            let base_section = crate::util::read_i32_le(chunk)?;

            // texture_name: string_ref[3]
            let texture_name = {
                let mut arr = Vec::with_capacity(3);
                for _ in 0..3 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // variation_index: int32
            let variation_index = crate::util::read_i32_le(chunk)?;

            // color_index: int32
            let color_index = crate::util::read_i32_le(chunk)?;


            rows.push(CharSectionsRow {
                id,
                race_id,
                sex_id,
                base_section,
                texture_name,
                flags,
                variation_index,
                color_index,
            });
        }

        Ok(CharSections { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 10,
            record_size: 40,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (CharSections) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // race_id: foreign_key (ChrRaces) int32
            b.write_all(&(row.race_id.id as i32).to_le_bytes())?;

            // sex_id: int32
            b.write_all(&row.sex_id.to_le_bytes())?;

            // base_section: int32
            b.write_all(&row.base_section.to_le_bytes())?;

            // texture_name: string_ref[3]
            for i in &row.texture_name {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // variation_index: int32
            b.write_all(&row.variation_index.to_le_bytes())?;

            // color_index: int32
            b.write_all(&row.color_index.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for CharSections {
    type PrimaryKey = CharSectionsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl CharSections {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            for s in &row.texture_name {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            for s in &row.texture_name {
                if !s.is_empty() { sum += s.len() + 1; };
            }

        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCharSections<const S: usize> {
    pub rows: [ConstCharSectionsRow; S],
}

impl<const S: usize> ConstCharSections<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 40 {
            panic!("invalid record size, expected 40")
        }

        if header.field_count != 10 {
            panic!("invalid field count, expected 10")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstCharSectionsRow {
                id: CharSectionsKey::new(0),
                race_id: ChrRacesKey::new(0),
                sex_id: 0,
                base_section: 0,
                texture_name: [""; 3],
                flags: 0,
                variation_index: 0,
                color_index: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (CharSections) int32
            let id = CharSectionsKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // race_id: foreign_key (ChrRaces) int32
            let race_id = ChrRacesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sex_id: int32
            let sex_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // base_section: int32
            let base_section = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // texture_name: string_ref[3]
            let texture_name = {
                let mut a = [""; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::get_string_from_block(b_offset, b, string_block);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // variation_index: int32
            let variation_index = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // color_index: int32
            let color_index = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstCharSectionsRow {
                id,
                race_id,
                sex_id,
                base_section,
                texture_name,
                flags,
                variation_index,
                color_index,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CharSections {
        CharSections {
            rows: self.rows.iter().map(|s| CharSectionsRow {
                id: s.id,
                race_id: s.race_id,
                sex_id: s.sex_id,
                base_section: s.base_section,
                texture_name: s.texture_name.map(|a| a.to_string()),
                flags: s.flags,
                variation_index: s.variation_index,
                color_index: s.color_index,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CharSectionsKey {
    pub id: i32
}

impl CharSectionsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for CharSectionsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for CharSectionsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for CharSectionsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for CharSectionsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for CharSectionsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharSectionsRow {
    pub id: CharSectionsKey,
    pub race_id: ChrRacesKey,
    pub sex_id: i32,
    pub base_section: i32,
    pub texture_name: [String; 3],
    pub flags: i32,
    pub variation_index: i32,
    pub color_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstCharSectionsRow {
    pub id: CharSectionsKey,
    pub race_id: ChrRacesKey,
    pub sex_id: i32,
    pub base_section: i32,
    pub texture_name: [&'static str; 3],
    pub flags: i32,
    pub variation_index: i32,
    pub color_index: i32,
}

