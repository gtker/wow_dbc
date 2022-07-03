use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq)]
pub struct SpellEffectCameraShakes {
    pub rows: Vec<SpellEffectCameraShakesRow>,
}

impl DbcTable for SpellEffectCameraShakes {
    type Row = SpellEffectCameraShakesRow;

    fn filename() -> &'static str { "SpellEffectCameraShakes.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 16,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 4 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 16,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SpellEffectCameraShakes) uint32
            let id = SpellEffectCameraShakesKey::new(crate::util::read_u32_le(chunk)?);

            // camera_shake: uint32[3]
            let camera_shake = crate::util::read_array_u32::<3>(chunk)?;


            rows.push(SpellEffectCameraShakesRow {
                id,
                camera_shake,
            });
        }

        Ok(SpellEffectCameraShakes { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 4,
            record_size: 16,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SpellEffectCameraShakes) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // camera_shake: uint32[3]
            for i in row.camera_shake {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellEffectCameraShakes {
    type PrimaryKey = SpellEffectCameraShakesKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct SpellEffectCameraShakesKey {
    pub id: u32
}

impl SpellEffectCameraShakesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct SpellEffectCameraShakesRow {
    pub id: SpellEffectCameraShakesKey,
    pub camera_shake: [u32; 3],
}

