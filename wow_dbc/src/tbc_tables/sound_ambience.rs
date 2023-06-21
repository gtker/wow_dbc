use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoundAmbience {
    pub rows: Vec<SoundAmbienceRow>,
}

impl DbcTable for SoundAmbience {
    type Row = SoundAmbienceRow;

    fn filename() -> &'static str { "SoundAmbience.dbc" }

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

            // id: primary_key (SoundAmbience) int32
            let id = SoundAmbienceKey::new(crate::util::read_i32_le(chunk)?);

            // ambience_id: int32[2]
            let ambience_id = crate::util::read_array_i32::<2>(chunk)?;


            rows.push(SoundAmbienceRow {
                id,
                ambience_id,
            });
        }

        Ok(SoundAmbience { rows, })
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
            // id: primary_key (SoundAmbience) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // ambience_id: int32[2]
            for i in row.ambience_id {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SoundAmbience {
    type PrimaryKey = SoundAmbienceKey;
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
pub struct ConstSoundAmbience<const S: usize> {
    pub rows: [SoundAmbienceRow; S],
}

impl<const S: usize> ConstSoundAmbience<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 12 {
            panic!("invalid record size, expected 12")
        }

        if header.field_count != 3 {
            panic!("invalid field count, expected 3")
        }

        let mut b_offset = 20;
        let mut rows = [
            SoundAmbienceRow {
                id: SoundAmbienceKey::new(0),
                ambience_id: [0; 2],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SoundAmbience) int32
            let id = SoundAmbienceKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // ambience_id: int32[2]
            let ambience_id = {
                let mut a = [0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = SoundAmbienceRow {
                id,
                ambience_id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SoundAmbience {
        SoundAmbience {
            rows: self.rows.iter().map(|s| SoundAmbienceRow {
                id: s.id,
                ambience_id: s.ambience_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SoundAmbienceKey {
    pub id: i32
}

impl SoundAmbienceKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SoundAmbienceKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SoundAmbienceKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SoundAmbienceKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SoundAmbienceKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SoundAmbienceKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoundAmbienceRow {
    pub id: SoundAmbienceKey,
    pub ambience_id: [i32; 2],
}

