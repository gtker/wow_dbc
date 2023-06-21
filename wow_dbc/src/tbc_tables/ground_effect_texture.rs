use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

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

            // id: primary_key (GroundEffectTexture) int32
            let id = GroundEffectTextureKey::new(crate::util::read_i32_le(chunk)?);

            // doodad_id: int32[4]
            let doodad_id = crate::util::read_array_i32::<4>(chunk)?;

            // density: int32
            let density = crate::util::read_i32_le(chunk)?;

            // sound: int32
            let sound = crate::util::read_i32_le(chunk)?;


            rows.push(GroundEffectTextureRow {
                id,
                doodad_id,
                density,
                sound,
            });
        }

        Ok(GroundEffectTexture { rows, })
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
            // id: primary_key (GroundEffectTexture) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // doodad_id: int32[4]
            for i in row.doodad_id {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstGroundEffectTexture<const S: usize> {
    pub rows: [GroundEffectTextureRow; S],
}

impl<const S: usize> ConstGroundEffectTexture<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 28 {
            panic!("invalid record size, expected 28")
        }

        if header.field_count != 7 {
            panic!("invalid field count, expected 7")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            GroundEffectTextureRow {
                id: GroundEffectTextureKey::new(0),
                doodad_id: [0; 4],
                density: 0,
                sound: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (GroundEffectTexture) int32
            let id = GroundEffectTextureKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // doodad_id: int32[4]
            let doodad_id = {
                let mut a = [0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // density: int32
            let density = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // sound: int32
            let sound = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = GroundEffectTextureRow {
                id,
                doodad_id,
                density,
                sound,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> GroundEffectTexture {
        GroundEffectTexture {
            rows: self.rows.iter().map(|s| GroundEffectTextureRow {
                id: s.id,
                doodad_id: s.doodad_id,
                density: s.density,
                sound: s.sound,
            }).collect(),
        }
    }
    // TODO: Indexable?
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
    pub density: i32,
    pub sound: i32,
}

