use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellChainEffects {
    pub rows: Vec<SpellChainEffectsRow>,
}

impl DbcTable for SpellChainEffects {
    type Row = SpellChainEffectsRow;

    fn filename() -> &'static str { "SpellChainEffects.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 8,
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

            // id: primary_key (SpellChainEffects) uint32
            let id = SpellChainEffectsKey::new(crate::util::read_u32_le(chunk)?);

            // average_seg_len: float
            let average_seg_len = crate::util::read_f32_le(chunk)?;

            // width: float
            let width = crate::util::read_f32_le(chunk)?;

            // noise_scale: float
            let noise_scale = crate::util::read_f32_le(chunk)?;

            // tex_coord_scale: float
            let tex_coord_scale = crate::util::read_f32_le(chunk)?;

            // seg_duration: int32
            let seg_duration = crate::util::read_i32_le(chunk)?;

            // seg_delay: int32
            let seg_delay = crate::util::read_i32_le(chunk)?;

            // texture: string_ref
            let texture = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(SpellChainEffectsRow {
                id,
                average_seg_len,
                width,
                noise_scale,
                tex_coord_scale,
                seg_duration,
                seg_delay,
                texture,
            });
        }

        Ok(SpellChainEffects { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 8,
            record_size: 32,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SpellChainEffects) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // average_seg_len: float
            b.write_all(&row.average_seg_len.to_le_bytes())?;

            // width: float
            b.write_all(&row.width.to_le_bytes())?;

            // noise_scale: float
            b.write_all(&row.noise_scale.to_le_bytes())?;

            // tex_coord_scale: float
            b.write_all(&row.tex_coord_scale.to_le_bytes())?;

            // seg_duration: int32
            b.write_all(&row.seg_duration.to_le_bytes())?;

            // seg_delay: int32
            b.write_all(&row.seg_delay.to_le_bytes())?;

            // texture: string_ref
            if !row.texture.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.texture.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SpellChainEffects {
    type PrimaryKey = SpellChainEffectsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl SpellChainEffects {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.texture.is_empty() { b.write_all(row.texture.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.texture.is_empty() { sum += row.texture.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellChainEffectsKey {
    pub id: u32
}

impl SpellChainEffectsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SpellChainEffectsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SpellChainEffectsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SpellChainEffectsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellChainEffectsRow {
    pub id: SpellChainEffectsKey,
    pub average_seg_len: f32,
    pub width: f32,
    pub noise_scale: f32,
    pub tex_coord_scale: f32,
    pub seg_duration: i32,
    pub seg_delay: i32,
    pub texture: String,
}

