use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiquidMaterial {
    pub rows: Vec<LiquidMaterialRow>,
}

impl DbcTable for LiquidMaterial {
    type Row = LiquidMaterialRow;

    fn filename() -> &'static str { "LiquidMaterial.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 12 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 12,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 3 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 3,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (LiquidMaterial) int32
            let id = LiquidMaterialKey::new(crate::util::read_i32_le(chunk)?);

            // l_v_f: int32
            let l_v_f = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(LiquidMaterialRow {
                id,
                l_v_f,
                flags,
            });
        }

        Ok(LiquidMaterial { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 3,
            record_size: 12,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (LiquidMaterial) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // l_v_f: int32
            b.write_all(&row.l_v_f.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for LiquidMaterial {
    type PrimaryKey = LiquidMaterialKey;
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
pub struct ConstLiquidMaterial<const S: usize> {
    pub rows: [LiquidMaterialRow; S],
}

impl<const S: usize> ConstLiquidMaterial<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 12 {
            panic!("invalid record size, expected 12")
        }

        if header.field_count != 3 {
            panic!("invalid field count, expected 3")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            LiquidMaterialRow {
                id: LiquidMaterialKey::new(0),
                l_v_f: 0,
                flags: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (LiquidMaterial) int32
            let id = LiquidMaterialKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // l_v_f: int32
            let l_v_f = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = LiquidMaterialRow {
                id,
                l_v_f,
                flags,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> LiquidMaterial {
        LiquidMaterial {
            rows: self.rows.iter().map(|s| LiquidMaterialRow {
                id: s.id,
                l_v_f: s.l_v_f,
                flags: s.flags,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct LiquidMaterialKey {
    pub id: i32
}

impl LiquidMaterialKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for LiquidMaterialKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for LiquidMaterialKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for LiquidMaterialKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for LiquidMaterialKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for LiquidMaterialKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiquidMaterialRow {
    pub id: LiquidMaterialKey,
    pub l_v_f: i32,
    pub flags: i32,
}

