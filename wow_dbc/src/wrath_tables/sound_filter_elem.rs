use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::sound_filter::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundFilterElem {
    pub rows: Vec<SoundFilterElemRow>,
}

impl DbcTable for SoundFilterElem {
    type Row = SoundFilterElemRow;

    fn filename() -> &'static str { "SoundFilterElem.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 52 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 52,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 13 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 13,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SoundFilterElem) int32
            let id = SoundFilterElemKey::new(crate::util::read_i32_le(chunk)?);

            // sound_filter_id: foreign_key (SoundFilter) int32
            let sound_filter_id = SoundFilterKey::new(crate::util::read_i32_le(chunk)?.into());

            // order_index: int32
            let order_index = crate::util::read_i32_le(chunk)?;

            // filter_type: int32
            let filter_type = crate::util::read_i32_le(chunk)?;

            // params: float[9]
            let params = crate::util::read_array_f32::<9>(chunk)?;


            rows.push(SoundFilterElemRow {
                id,
                sound_filter_id,
                order_index,
                filter_type,
                params,
            });
        }

        Ok(SoundFilterElem { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 13,
            record_size: 52,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SoundFilterElem) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_filter_id: foreign_key (SoundFilter) int32
            b.write_all(&(row.sound_filter_id.id as i32).to_le_bytes())?;

            // order_index: int32
            b.write_all(&row.order_index.to_le_bytes())?;

            // filter_type: int32
            b.write_all(&row.filter_type.to_le_bytes())?;

            // params: float[9]
            for i in row.params {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SoundFilterElem {
    type PrimaryKey = SoundFilterElemKey;
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
pub struct ConstSoundFilterElem<const S: usize> {
    pub rows: [SoundFilterElemRow; S],
}

impl<const S: usize> ConstSoundFilterElem<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 52 {
            panic!("invalid record size, expected 52")
        }

        if header.field_count != 13 {
            panic!("invalid field count, expected 13")
        }

        let mut b_offset = 20;
        let mut rows = [
            SoundFilterElemRow {
                id: SoundFilterElemKey::new(0),
                sound_filter_id: SoundFilterKey::new(0),
                order_index: 0,
                filter_type: 0,
                params: [0.0; 9],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SoundFilterElem) int32
            let id = SoundFilterElemKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_filter_id: foreign_key (SoundFilter) int32
            let sound_filter_id = SoundFilterKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // order_index: int32
            let order_index = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // filter_type: int32
            let filter_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // params: float[9]
            let params = {
                let mut a = [0.0; 9];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = SoundFilterElemRow {
                id,
                sound_filter_id,
                order_index,
                filter_type,
                params,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SoundFilterElemKey {
    pub id: i32
}

impl SoundFilterElemKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SoundFilterElemKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SoundFilterElemKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SoundFilterElemKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SoundFilterElemKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SoundFilterElemKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SoundFilterElemRow {
    pub id: SoundFilterElemKey,
    pub sound_filter_id: SoundFilterKey,
    pub order_index: i32,
    pub filter_type: i32,
    pub params: [f32; 9],
}

