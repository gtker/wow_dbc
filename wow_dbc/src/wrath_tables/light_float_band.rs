use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LightFloatBand {
    pub rows: Vec<LightFloatBandRow>,
}

impl DbcTable for LightFloatBand {
    type Row = LightFloatBandRow;

    fn filename() -> &'static str { "LightFloatBand.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 136 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 136,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 34 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 34,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (LightFloatBand) int32
            let id = LightFloatBandKey::new(crate::util::read_i32_le(chunk)?);

            // num: int32
            let num = crate::util::read_i32_le(chunk)?;

            // time: int32[16]
            let time = crate::util::read_array_i32::<16>(chunk)?;

            // data: float[16]
            let data = crate::util::read_array_f32::<16>(chunk)?;


            rows.push(LightFloatBandRow {
                id,
                num,
                time,
                data,
            });
        }

        Ok(LightFloatBand { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 34,
            record_size: 136,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (LightFloatBand) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // num: int32
            b.write_all(&row.num.to_le_bytes())?;

            // time: int32[16]
            for i in row.time {
                b.write_all(&i.to_le_bytes())?;
            }


            // data: float[16]
            for i in row.data {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for LightFloatBand {
    type PrimaryKey = LightFloatBandKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstLightFloatBand<const S: usize> {
    pub rows: [LightFloatBandRow; S],
}

impl<const S: usize> ConstLightFloatBand<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 136 {
            panic!("invalid record size, expected 136")
        }

        if header.field_count != 34 {
            panic!("invalid field count, expected 34")
        }

        let mut b_offset = 20;
        let mut rows = [
            LightFloatBandRow {
                id: LightFloatBandKey::new(0),
                num: 0,
                time: [0; 16],
                data: [0.0; 16],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (LightFloatBand) int32
            let id = LightFloatBandKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // num: int32
            let num = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // time: int32[16]
            let time = {
                let mut a = [0; 16];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // data: float[16]
            let data = {
                let mut a = [0.0; 16];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = LightFloatBandRow {
                id,
                num,
                time,
                data,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> LightFloatBand {
        LightFloatBand {
            rows: self.rows.iter().map(|s| LightFloatBandRow {
                id: s.id,
                num: s.num,
                time: s.time,
                data: s.data,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct LightFloatBandKey {
    pub id: i32
}

impl LightFloatBandKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for LightFloatBandKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for LightFloatBandKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for LightFloatBandKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for LightFloatBandKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for LightFloatBandKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LightFloatBandRow {
    pub id: LightFloatBandKey,
    pub num: i32,
    pub time: [i32; 16],
    pub data: [f32; 16],
}

