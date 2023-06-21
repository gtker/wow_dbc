use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CinematicSequences {
    pub rows: Vec<CinematicSequencesRow>,
}

impl DbcTable for CinematicSequences {
    type Row = CinematicSequencesRow;

    fn filename() -> &'static str { "CinematicSequences.dbc" }

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

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (CinematicSequences) int32
            let id = CinematicSequencesKey::new(crate::util::read_i32_le(chunk)?);

            // sound_id: foreign_key (SoundEntries) int32
            let sound_id = SoundEntriesKey::new(crate::util::read_i32_le(chunk)?.into());

            // camera: int32[8]
            let camera = crate::util::read_array_i32::<8>(chunk)?;


            rows.push(CinematicSequencesRow {
                id,
                sound_id,
                camera,
            });
        }

        Ok(CinematicSequences { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 10,
            record_size: 40,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (CinematicSequences) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_id: foreign_key (SoundEntries) int32
            b.write_all(&(row.sound_id.id as i32).to_le_bytes())?;

            // camera: int32[8]
            for i in row.camera {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for CinematicSequences {
    type PrimaryKey = CinematicSequencesKey;
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
pub struct ConstCinematicSequences<const S: usize> {
    pub rows: [CinematicSequencesRow; S],
}

impl<const S: usize> ConstCinematicSequences<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 40 {
            panic!("invalid record size, expected 40")
        }

        if header.field_count != 10 {
            panic!("invalid field count, expected 10")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            CinematicSequencesRow {
                id: CinematicSequencesKey::new(0),
                sound_id: SoundEntriesKey::new(0),
                camera: [0; 8],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (CinematicSequences) int32
            let id = CinematicSequencesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_id: foreign_key (SoundEntries) int32
            let sound_id = SoundEntriesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // camera: int32[8]
            let camera = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = CinematicSequencesRow {
                id,
                sound_id,
                camera,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CinematicSequences {
        CinematicSequences {
            rows: self.rows.iter().map(|s| CinematicSequencesRow {
                id: s.id,
                sound_id: s.sound_id,
                camera: s.camera,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CinematicSequencesKey {
    pub id: i32
}

impl CinematicSequencesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for CinematicSequencesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for CinematicSequencesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for CinematicSequencesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for CinematicSequencesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for CinematicSequencesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CinematicSequencesRow {
    pub id: CinematicSequencesKey,
    pub sound_id: SoundEntriesKey,
    pub camera: [i32; 8],
}

