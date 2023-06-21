use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoundSamplePreferences {
    pub rows: Vec<SoundSamplePreferencesRow>,
}

impl DbcTable for SoundSamplePreferences {
    type Row = SoundSamplePreferencesRow;

    fn filename() -> &'static str { "SoundSamplePreferences.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 68 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 68,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 17 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 17,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SoundSamplePreferences) uint32
            let id = SoundSamplePreferencesKey::new(crate::util::read_u32_le(chunk)?);

            // unknown: int32[16]
            let unknown = crate::util::read_array_i32::<16>(chunk)?;


            rows.push(SoundSamplePreferencesRow {
                id,
                unknown,
            });
        }

        Ok(SoundSamplePreferences { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 17,
            record_size: 68,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SoundSamplePreferences) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // unknown: int32[16]
            for i in row.unknown {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SoundSamplePreferences {
    type PrimaryKey = SoundSamplePreferencesKey;
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
pub struct ConstSoundSamplePreferences<const S: usize> {
    pub rows: [SoundSamplePreferencesRow; S],
}

impl<const S: usize> ConstSoundSamplePreferences<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 68 {
            panic!("invalid record size, expected 68")
        }

        if header.field_count != 17 {
            panic!("invalid field count, expected 17")
        }

        let mut b_offset = 20;
        let mut rows = [
            SoundSamplePreferencesRow {
                id: SoundSamplePreferencesKey::new(0),
                unknown: [0; 16],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SoundSamplePreferences) uint32
            let id = SoundSamplePreferencesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // unknown: int32[16]
            let unknown = {
                let mut a = [0; 16];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = SoundSamplePreferencesRow {
                id,
                unknown,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SoundSamplePreferences {
        SoundSamplePreferences {
            rows: self.rows.iter().map(|s| SoundSamplePreferencesRow {
                id: s.id,
                unknown: s.unknown,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SoundSamplePreferencesKey {
    pub id: u32
}

impl SoundSamplePreferencesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SoundSamplePreferencesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SoundSamplePreferencesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SoundSamplePreferencesKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SoundSamplePreferencesRow {
    pub id: SoundSamplePreferencesKey,
    pub unknown: [i32; 16],
}

