use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use crate::wrath_tables::chr_races::ChrRacesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VocalUISounds {
    pub rows: Vec<VocalUISoundsRow>,
}

impl DbcTable for VocalUISounds {
    type Row = VocalUISoundsRow;

    const FILENAME: &'static str = "VocalUISounds.dbc";
    const FIELD_COUNT: usize = 7;
    const ROW_SIZE: usize = 28;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
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

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let  string_cache = StringCache::new();

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

        assert_eq!(b.len(), self.rows.len() * Self::ROW_SIZE);
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: string_cache.size(),
        };

        w.write_all(&header.write_header())?;
        w.write_all(&b)?;
        w.write_all(string_cache.buffer())?;
        Ok(())
    }

}

impl Indexable for VocalUISounds {
    type PrimaryKey = VocalUISoundsKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VocalUISoundsKey {
    pub id: i32
}

impl VocalUISoundsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

impl TryFrom<u32> for VocalUISoundsKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for VocalUISoundsKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for VocalUISoundsKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for VocalUISoundsKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for VocalUISoundsKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VocalUISoundsRow {
    pub id: VocalUISoundsKey,
    pub vocal_u_i_enum: i32,
    pub race_id: ChrRacesKey,
    pub normal_sound_id: [i32; 2],
    pub pissed_sound_id: [i32; 2],
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn vocal_ui_sounds() {
        let mut file = File::open("../wrath-dbc/VocalUISounds.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = VocalUISounds::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = VocalUISounds::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
