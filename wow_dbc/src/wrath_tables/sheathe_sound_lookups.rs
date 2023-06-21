use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::material::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SheatheSoundLookups {
    pub rows: Vec<SheatheSoundLookupsRow>,
}

impl DbcTable for SheatheSoundLookups {
    type Row = SheatheSoundLookupsRow;

    fn filename() -> &'static str { "SheatheSoundLookups.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 28 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 28,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 7 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 7,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SheatheSoundLookups) int32
            let id = SheatheSoundLookupsKey::new(crate::util::read_i32_le(chunk)?);

            // class_id: int32
            let class_id = crate::util::read_i32_le(chunk)?;

            // subclass_id: int32
            let subclass_id = crate::util::read_i32_le(chunk)?;

            // material: foreign_key (Material) int32
            let material = MaterialKey::new(crate::util::read_i32_le(chunk)?.into());

            // check_material: int32
            let check_material = crate::util::read_i32_le(chunk)?;

            // sheathe_sound: int32
            let sheathe_sound = crate::util::read_i32_le(chunk)?;

            // unsheathe_sound: int32
            let unsheathe_sound = crate::util::read_i32_le(chunk)?;


            rows.push(SheatheSoundLookupsRow {
                id,
                class_id,
                subclass_id,
                material,
                check_material,
                sheathe_sound,
                unsheathe_sound,
            });
        }

        Ok(SheatheSoundLookups { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 7,
            record_size: 28,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SheatheSoundLookups) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // class_id: int32
            b.write_all(&row.class_id.to_le_bytes())?;

            // subclass_id: int32
            b.write_all(&row.subclass_id.to_le_bytes())?;

            // material: foreign_key (Material) int32
            b.write_all(&(row.material.id as i32).to_le_bytes())?;

            // check_material: int32
            b.write_all(&row.check_material.to_le_bytes())?;

            // sheathe_sound: int32
            b.write_all(&row.sheathe_sound.to_le_bytes())?;

            // unsheathe_sound: int32
            b.write_all(&row.unsheathe_sound.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SheatheSoundLookups {
    type PrimaryKey = SheatheSoundLookupsKey;
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
pub struct ConstSheatheSoundLookups<const S: usize> {
    pub rows: [SheatheSoundLookupsRow; S],
}

impl<const S: usize> ConstSheatheSoundLookups<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 28 {
            panic!("invalid record size, expected 28")
        }

        if header.field_count != 7 {
            panic!("invalid field count, expected 7")
        }

        let mut b_offset = 20;
        let mut rows = [
            SheatheSoundLookupsRow {
                id: SheatheSoundLookupsKey::new(0),
                class_id: 0,
                subclass_id: 0,
                material: MaterialKey::new(0),
                check_material: 0,
                sheathe_sound: 0,
                unsheathe_sound: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SheatheSoundLookups) int32
            let id = SheatheSoundLookupsKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // class_id: int32
            let class_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // subclass_id: int32
            let subclass_id = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // material: foreign_key (Material) int32
            let material = MaterialKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // check_material: int32
            let check_material = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // sheathe_sound: int32
            let sheathe_sound = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // unsheathe_sound: int32
            let unsheathe_sound = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = SheatheSoundLookupsRow {
                id,
                class_id,
                subclass_id,
                material,
                check_material,
                sheathe_sound,
                unsheathe_sound,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SheatheSoundLookupsKey {
    pub id: i32
}

impl SheatheSoundLookupsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SheatheSoundLookupsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SheatheSoundLookupsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SheatheSoundLookupsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SheatheSoundLookupsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SheatheSoundLookupsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SheatheSoundLookupsRow {
    pub id: SheatheSoundLookupsKey,
    pub class_id: i32,
    pub subclass_id: i32,
    pub material: MaterialKey,
    pub check_material: i32,
    pub sheathe_sound: i32,
    pub unsheathe_sound: i32,
}

