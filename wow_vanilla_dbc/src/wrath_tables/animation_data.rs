use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimationData {
    pub rows: Vec<AnimationDataRow>,
}

impl DbcTable for AnimationData {
    type Row = AnimationDataRow;

    fn filename() -> &'static str { "AnimationData.dbc" }

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

            // id: primary_key (AnimationData) int32
            let id = AnimationDataKey::new(crate::util::read_i32_le(chunk)?);

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // weaponflags: int32
            let weaponflags = crate::util::read_i32_le(chunk)?;

            // bodyflags: int32
            let bodyflags = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // fallback: foreign_key (AnimationData) int32
            let fallback = AnimationDataKey::new(crate::util::read_i32_le(chunk)?.into());

            // behavior_id: foreign_key (AnimationData) int32
            let behavior_id = AnimationDataKey::new(crate::util::read_i32_le(chunk)?.into());

            // behavior_tier: int32
            let behavior_tier = crate::util::read_i32_le(chunk)?;


            rows.push(AnimationDataRow {
                id,
                name,
                weaponflags,
                bodyflags,
                flags,
                fallback,
                behavior_id,
                behavior_tier,
            });
        }

        Ok(AnimationData { rows, })
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
            // id: primary_key (AnimationData) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref
            if !row.name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // weaponflags: int32
            b.write_all(&row.weaponflags.to_le_bytes())?;

            // bodyflags: int32
            b.write_all(&row.bodyflags.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // fallback: foreign_key (AnimationData) int32
            b.write_all(&(row.fallback.id as i32).to_le_bytes())?;

            // behavior_id: foreign_key (AnimationData) int32
            b.write_all(&(row.behavior_id.id as i32).to_le_bytes())?;

            // behavior_tier: int32
            b.write_all(&row.behavior_tier.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for AnimationData {
    type PrimaryKey = AnimationDataKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl AnimationData {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.name.is_empty() { b.write_all(row.name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.name.is_empty() { sum += row.name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct AnimationDataKey {
    pub id: i32
}

impl AnimationDataKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnimationDataRow {
    pub id: AnimationDataKey,
    pub name: String,
    pub weaponflags: i32,
    pub bodyflags: i32,
    pub flags: i32,
    pub fallback: AnimationDataKey,
    pub behavior_id: AnimationDataKey,
    pub behavior_tier: i32,
}

