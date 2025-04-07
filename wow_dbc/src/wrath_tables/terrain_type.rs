use crate::DbcTable;
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::util::StringCache;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TerrainType {
    pub rows: Vec<TerrainTypeRow>,
}

impl DbcTable for TerrainType {
    type Row = TerrainTypeRow;

    const FILENAME: &'static str = "TerrainType.dbc";
    const FIELD_COUNT: usize = 6;
    const ROW_SIZE: usize = 24;

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

            // terrain_id: int32
            let terrain_id = crate::util::read_i32_le(chunk)?;

            // terrain_desc: string_ref
            let terrain_desc = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // footstep_spray_run: int32
            let footstep_spray_run = crate::util::read_i32_le(chunk)?;

            // footstep_spray_walk: int32
            let footstep_spray_walk = crate::util::read_i32_le(chunk)?;

            // sound_id: int32
            let sound_id = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(TerrainTypeRow {
                terrain_id,
                terrain_desc,
                footstep_spray_run,
                footstep_spray_walk,
                sound_id,
                flags,
            });
        }

        Ok(TerrainType { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // terrain_id: int32
            b.write_all(&row.terrain_id.to_le_bytes())?;

            // terrain_desc: string_ref
            b.write_all(&string_cache.add_string(&row.terrain_desc).to_le_bytes())?;

            // footstep_spray_run: int32
            b.write_all(&row.footstep_spray_run.to_le_bytes())?;

            // footstep_spray_walk: int32
            b.write_all(&row.footstep_spray_walk.to_le_bytes())?;

            // sound_id: int32
            b.write_all(&row.sound_id.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TerrainTypeRow {
    pub terrain_id: i32,
    pub terrain_desc: String,
    pub footstep_spray_run: i32,
    pub footstep_spray_walk: i32,
    pub sound_id: i32,
    pub flags: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn terrain_type() {
        let mut file = File::open("../wrath-dbc/TerrainType.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = TerrainType::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = TerrainType::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
