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
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroundEffectTextureRow {
    pub id: GroundEffectTextureKey,
    pub doodad_id: [i32; 4],
    pub density: i32,
    pub sound: i32,
}

