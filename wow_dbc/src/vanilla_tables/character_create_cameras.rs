use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CharacterCreateCameras {
    pub rows: Vec<CharacterCreateCamerasRow>,
}

impl DbcTable for CharacterCreateCameras {
    type Row = CharacterCreateCamerasRow;

    fn filename() -> &'static str { "CharacterCreateCameras.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 24 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 24,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 6 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 6,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (CharacterCreateCameras) uint32
            let id = CharacterCreateCamerasKey::new(crate::util::read_u32_le(chunk)?);

            // unknown: bool32[2]
            let unknown = {
                let mut arr = [bool::default(); 2];
                for i in arr.iter_mut() {
                    *i = crate::util::read_u32_le(chunk)? != 0;
                }

                arr
            };

            // unknown_2: float[3]
            let unknown_2 = crate::util::read_array_f32::<3>(chunk)?;


            rows.push(CharacterCreateCamerasRow {
                id,
                unknown,
                unknown_2,
            });
        }

        Ok(CharacterCreateCameras { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 6,
            record_size: 24,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (CharacterCreateCameras) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // unknown: bool32[2]
            for i in row.unknown {
                b.write_all(&u32::from(i).to_le_bytes())?;
            }


            // unknown_2: float[3]
            for i in row.unknown_2 {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for CharacterCreateCameras {
    type PrimaryKey = CharacterCreateCamerasKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CharacterCreateCamerasKey {
    pub id: u32
}

impl CharacterCreateCamerasKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for CharacterCreateCamerasKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for CharacterCreateCamerasKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for CharacterCreateCamerasKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CharacterCreateCamerasRow {
    pub id: CharacterCreateCamerasKey,
    pub unknown: [bool; 2],
    pub unknown_2: [f32; 3],
}

