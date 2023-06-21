use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Material {
    pub rows: Vec<MaterialRow>,
}

impl DbcTable for Material {
    type Row = MaterialRow;

    fn filename() -> &'static str { "Material.dbc" }

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

            // id: primary_key (Material) int32
            let id = MaterialKey::new(crate::util::read_i32_le(chunk)?);

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // foley_sound_id: foreign_key (SoundKit) int32
            let foley_sound_id = crate::util::read_i32_le(chunk)?;

            // sheathe_sound_id: foreign_key (SoundKit) int32
            let sheathe_sound_id = crate::util::read_i32_le(chunk)?;

            // unsheathe_sound_id: foreign_key (SoundKit) int32
            let unsheathe_sound_id = crate::util::read_i32_le(chunk)?;


            rows.push(MaterialRow {
                id,
                flags,
                foley_sound_id,
                sheathe_sound_id,
                unsheathe_sound_id,
            });
        }

        Ok(Material { rows, })
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
            // id: primary_key (Material) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // foley_sound_id: foreign_key (SoundKit) int32
            b.write_all(&row.foley_sound_id.to_le_bytes())?;

            // sheathe_sound_id: foreign_key (SoundKit) int32
            b.write_all(&row.sheathe_sound_id.to_le_bytes())?;

            // unsheathe_sound_id: foreign_key (SoundKit) int32
            b.write_all(&row.unsheathe_sound_id.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for Material {
    type PrimaryKey = MaterialKey;
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
pub struct ConstMaterial<const S: usize> {
    pub rows: [MaterialRow; S],
}

impl<const S: usize> ConstMaterial<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 20 {
            panic!("invalid record size, expected 20")
        }

        if header.field_count != 5 {
            panic!("invalid field count, expected 5")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            MaterialRow {
                id: MaterialKey::new(0),
                flags: 0,
                foley_sound_id: 0,
                sheathe_sound_id: 0,
                unsheathe_sound_id: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Material) int32
            let id = MaterialKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // foley_sound_id: foreign_key (SoundKit) int32
            let foley_sound_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // sheathe_sound_id: foreign_key (SoundKit) int32
            let sheathe_sound_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // unsheathe_sound_id: foreign_key (SoundKit) int32
            let unsheathe_sound_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = MaterialRow {
                id,
                flags,
                foley_sound_id,
                sheathe_sound_id,
                unsheathe_sound_id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> Material {
        Material {
            rows: self.rows.iter().map(|s| MaterialRow {
                id: s.id,
                flags: s.flags,
                foley_sound_id: s.foley_sound_id,
                sheathe_sound_id: s.sheathe_sound_id,
                unsheathe_sound_id: s.unsheathe_sound_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct MaterialKey {
    pub id: i32
}

impl MaterialKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for MaterialKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for MaterialKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for MaterialKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for MaterialKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for MaterialKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaterialRow {
    pub id: MaterialKey,
    pub flags: i32,
    pub foley_sound_id: i32,
    pub sheathe_sound_id: i32,
    pub unsheathe_sound_id: i32,
}

