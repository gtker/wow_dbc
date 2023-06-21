use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::tbc_tables::chr_races::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VocalUISounds {
    pub rows: Vec<VocalUISoundsRow>,
}

impl DbcTable for VocalUISounds {
    type Row = VocalUISoundsRow;

    fn filename() -> &'static str { "VocalUISounds.dbc" }

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

            // id: primary_key (VocalUISounds) int32
            let id = VocalUISoundsKey::new(crate::util::read_i32_le(chunk)?);

            // vocal_u_i_enum: int32
            let vocal_u_i_enum = crate::util::read_i32_le(chunk)?;

            // race_id: foreign_key (ChrRaces) int32
            let race_id = ChrRacesKey::new(crate::util::read_i32_le(chunk)?.into());

            // normal_sound_id: int32[2]
            let normal_sound_id = crate::util::read_array_i32::<2>(chunk)?;

            // pissed_sound_id: int32[2]
            let pissed_sound_id = crate::util::read_array_i32::<2>(chunk)?;


            rows.push(VocalUISoundsRow {
                id,
                vocal_u_i_enum,
                race_id,
                normal_sound_id,
                pissed_sound_id,
            });
        }

        Ok(VocalUISounds { rows, })
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
            // id: primary_key (VocalUISounds) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // vocal_u_i_enum: int32
            b.write_all(&row.vocal_u_i_enum.to_le_bytes())?;

            // race_id: foreign_key (ChrRaces) int32
            b.write_all(&(row.race_id.id as i32).to_le_bytes())?;

            // normal_sound_id: int32[2]
            for i in row.normal_sound_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // pissed_sound_id: int32[2]
            for i in row.pissed_sound_id {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for VocalUISounds {
    type PrimaryKey = VocalUISoundsKey;
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
pub struct VocalUISoundsKey {
    pub id: i32
}

impl VocalUISoundsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for VocalUISoundsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for VocalUISoundsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for VocalUISoundsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for VocalUISoundsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for VocalUISoundsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VocalUISoundsRow {
    pub id: VocalUISoundsKey,
    pub vocal_u_i_enum: i32,
    pub race_id: ChrRacesKey,
    pub normal_sound_id: [i32; 2],
    pub pissed_sound_id: [i32; 2],
}

