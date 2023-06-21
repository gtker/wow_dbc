use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroundEffectTexture {
    pub rows: Vec<GroundEffectTextureRow>,
}

impl DbcTable for GroundEffectTexture {
    type Row = GroundEffectTextureRow;

    fn filename() -> &'static str { "GroundEffectTexture.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 44 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 44,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 11 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 11,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (GroundEffectTexture) int32
            let id = GroundEffectTextureKey::new(crate::util::read_i32_le(chunk)?);

            // doodad_id: int32[4]
            let doodad_id = crate::util::read_array_i32::<4>(chunk)?;

            // doodad_weight: int32[4]
            let doodad_weight = crate::util::read_array_i32::<4>(chunk)?;

            // density: int32
            let density = crate::util::read_i32_le(chunk)?;

            // sound: int32
            let sound = crate::util::read_i32_le(chunk)?;


            rows.push(GroundEffectTextureRow {
                id,
                doodad_id,
                doodad_weight,
                density,
                sound,
            });
        }

        Ok(GroundEffectTexture { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 11,
            record_size: 44,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (GroundEffectTexture) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // doodad_id: int32[4]
            for i in row.doodad_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // doodad_weight: int32[4]
            for i in row.doodad_weight {
                b.write_all(&i.to_le_bytes())?;
            }


            // density: int32
            b.write_all(&row.density.to_le_bytes())?;

            // sound: int32
            b.write_all(&row.sound.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for GroundEffectTexture {
    type PrimaryKey = GroundEffectTextureKey;
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
pub struct GroundEffectTextureKey {
    pub id: i32
}

impl GroundEffectTextureKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for GroundEffectTextureKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for GroundEffectTextureKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for GroundEffectTextureKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for GroundEffectTextureKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for GroundEffectTextureKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroundEffectTextureRow {
    pub id: GroundEffectTextureKey,
    pub doodad_id: [i32; 4],
    pub doodad_weight: [i32; 4],
    pub density: i32,
    pub sound: i32,
}

