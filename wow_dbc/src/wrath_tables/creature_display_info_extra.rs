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
pub struct CreatureDisplayInfoExtra {
    pub rows: Vec<CreatureDisplayInfoExtraRow>,
}

impl DbcTable for CreatureDisplayInfoExtra {
    type Row = CreatureDisplayInfoExtraRow;

    const FILENAME: &'static str = "CreatureDisplayInfoExtra.dbc";
    const FIELD_COUNT: usize = 21;
    const ROW_SIZE: usize = 84;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (CreatureDisplayInfoExtra) int32
            let id = CreatureDisplayInfoExtraKey::new(crate::util::read_i32_le(chunk)?);

            // display_race_id: foreign_key (ChrRaces) int32
            let display_race_id = ChrRacesKey::new(crate::util::read_i32_le(chunk)?.into());

            // display_sex_id: int32
            let display_sex_id = crate::util::read_i32_le(chunk)?;

            // skin_id: int32
            let skin_id = crate::util::read_i32_le(chunk)?;

            // face_id: int32
            let face_id = crate::util::read_i32_le(chunk)?;

            // hair_style_id: int32
            let hair_style_id = crate::util::read_i32_le(chunk)?;

            // hair_color_id: int32
            let hair_color_id = crate::util::read_i32_le(chunk)?;

            // facial_hair_id: int32
            let facial_hair_id = crate::util::read_i32_le(chunk)?;

            // n_p_c_item_display: int32[11]
            let n_p_c_item_display = crate::util::read_array_i32::<11>(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // bake_name: string_ref
            let bake_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(CreatureDisplayInfoExtraRow {
                id,
                display_race_id,
                display_sex_id,
                skin_id,
                face_id,
                hair_style_id,
                hair_color_id,
                facial_hair_id,
                n_p_c_item_display,
                flags,
                bake_name,
            });
        }

        Ok(CreatureDisplayInfoExtra { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (CreatureDisplayInfoExtra) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // display_race_id: foreign_key (ChrRaces) int32
            b.write_all(&(row.display_race_id.id as i32).to_le_bytes())?;

            // display_sex_id: int32
            b.write_all(&row.display_sex_id.to_le_bytes())?;

            // skin_id: int32
            b.write_all(&row.skin_id.to_le_bytes())?;

            // face_id: int32
            b.write_all(&row.face_id.to_le_bytes())?;

            // hair_style_id: int32
            b.write_all(&row.hair_style_id.to_le_bytes())?;

            // hair_color_id: int32
            b.write_all(&row.hair_color_id.to_le_bytes())?;

            // facial_hair_id: int32
            b.write_all(&row.facial_hair_id.to_le_bytes())?;

            // n_p_c_item_display: int32[11]
            for i in row.n_p_c_item_display {
                b.write_all(&i.to_le_bytes())?;
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // bake_name: string_ref
            b.write_all(&string_cache.add_string(&row.bake_name).to_le_bytes())?;

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

impl Indexable for CreatureDisplayInfoExtra {
    type PrimaryKey = CreatureDisplayInfoExtraKey;
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
pub struct CreatureDisplayInfoExtraKey {
    pub id: i32
}

impl CreatureDisplayInfoExtraKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for CreatureDisplayInfoExtraKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for CreatureDisplayInfoExtraKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for CreatureDisplayInfoExtraKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for CreatureDisplayInfoExtraKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for CreatureDisplayInfoExtraKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for CreatureDisplayInfoExtraKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for CreatureDisplayInfoExtraKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for CreatureDisplayInfoExtraKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for CreatureDisplayInfoExtraKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for CreatureDisplayInfoExtraKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatureDisplayInfoExtraRow {
    pub id: CreatureDisplayInfoExtraKey,
    pub display_race_id: ChrRacesKey,
    pub display_sex_id: i32,
    pub skin_id: i32,
    pub face_id: i32,
    pub hair_style_id: i32,
    pub hair_color_id: i32,
    pub facial_hair_id: i32,
    pub n_p_c_item_display: [i32; 11],
    pub flags: i32,
    pub bake_name: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn creature_display_info_extra() {
        let mut file = File::open("../wrath-dbc/CreatureDisplayInfoExtra.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = CreatureDisplayInfoExtra::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = CreatureDisplayInfoExtra::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
