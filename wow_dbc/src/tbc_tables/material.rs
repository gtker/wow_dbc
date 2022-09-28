use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Material {
    pub rows: Vec<MaterialRow>,
}

impl DbcTable for Material {
    type Row = MaterialRow;

    fn filename() -> &'static str { "Material.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 20 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 20,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 5 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 5,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (Material) int32
            let id = MaterialKey::new(crate::util::read_i32_le(chunk)?);

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // foley_sound_id: foreign_key (SoundKit) int32
            let foley_sound_id = crate::util::read_i32_le(chunk)?;

            // sheathe_sound_id: foreign_key (SoundKit) int32
            let sheathe_sound_id = crate::util::read_i32_le(chunk)?;

            // unsheathe_sound_id: foreign_key (SoundKit) int32
            let unsheathe_sound_id = crate::util::read_i32_le(chunk)?;


            rows.push(MaterialRow {
                id,
                flags,
                foley_sound_id,
                sheathe_sound_id,
                unsheathe_sound_id,
            });
        }

        Ok(Material { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 5,
            record_size: 20,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (Material) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // foley_sound_id: foreign_key (SoundKit) int32
            b.write_all(&row.foley_sound_id.to_le_bytes())?;

            // sheathe_sound_id: foreign_key (SoundKit) int32
            b.write_all(&row.sheathe_sound_id.to_le_bytes())?;

            // unsheathe_sound_id: foreign_key (SoundKit) int32
            b.write_all(&row.unsheathe_sound_id.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for Material {
    type PrimaryKey = MaterialKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct MaterialKey {
    pub id: i32
}

impl MaterialKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialRow {
    pub id: MaterialKey,
    pub flags: i32,
    pub foley_sound_id: i32,
    pub sheathe_sound_id: i32,
    pub unsheathe_sound_id: i32,
}

