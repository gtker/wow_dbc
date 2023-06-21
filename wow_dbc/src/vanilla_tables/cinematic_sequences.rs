use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::sound_entries::*;

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

            // id: primary_key (CinematicSequences) uint32
            let id = CinematicSequencesKey::new(crate::util::read_u32_le(chunk)?);

            // sound_entry: foreign_key (SoundEntries) uint32
            let sound_entry = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // cinematic_camera_1: uint32[8]
            let cinematic_camera_1 = crate::util::read_array_u32::<8>(chunk)?;


            rows.push(CinematicSequencesRow {
                id,
                sound_entry,
                cinematic_camera_1,
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
            // id: primary_key (CinematicSequences) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_entry: foreign_key (SoundEntries) uint32
            b.write_all(&(row.sound_entry.id as u32).to_le_bytes())?;

            // cinematic_camera_1: uint32[8]
            for i in row.cinematic_camera_1 {
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

        let mut b_offset = 20;
        let mut rows = [
            CinematicSequencesRow {
                id: CinematicSequencesKey::new(0),
                sound_entry: SoundEntriesKey::new(0),
                cinematic_camera_1: [0; 8],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (CinematicSequences) uint32
            let id = CinematicSequencesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_entry: foreign_key (SoundEntries) uint32
            let sound_entry = SoundEntriesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // cinematic_camera_1: uint32[8]
            let cinematic_camera_1 = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = CinematicSequencesRow {
                id,
                sound_entry,
                cinematic_camera_1,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CinematicSequences {
        CinematicSequences {
            rows: self.rows.iter().map(|s| CinematicSequencesRow {
                id: s.id,
                sound_entry: s.sound_entry,
                cinematic_camera_1: s.cinematic_camera_1,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CinematicSequencesKey {
    pub id: u32
}

impl CinematicSequencesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
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

impl From<u32> for CinematicSequencesKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CinematicSequencesRow {
    pub id: CinematicSequencesKey,
    pub sound_entry: SoundEntriesKey,
    pub cinematic_camera_1: [u32; 8],
}

