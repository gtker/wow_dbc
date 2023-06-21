use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};
use crate::wrath_tables::lock::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DanceMoves {
    pub rows: Vec<DanceMovesRow>,
}

impl DbcTable for DanceMoves {
    type Row = DanceMovesRow;

    fn filename() -> &'static str { "DanceMoves.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 96 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 96,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 24 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 24,
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

            // id: primary_key (DanceMoves) int32
            let id = DanceMovesKey::new(crate::util::read_i32_le(chunk)?);

            // ty: int32
            let ty = crate::util::read_i32_le(chunk)?;

            // param: int32
            let param = crate::util::read_i32_le(chunk)?;

            // fallback: int32
            let fallback = crate::util::read_i32_le(chunk)?;

            // racemask: int32
            let racemask = crate::util::read_i32_le(chunk)?;

            // internal_name: string_ref
            let internal_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // lock_id: foreign_key (Lock) int32
            let lock_id = LockKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(DanceMovesRow {
                id,
                ty,
                param,
                fallback,
                racemask,
                internal_name,
                name_lang,
                lock_id,
            });
        }

        Ok(DanceMoves { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 24,
            record_size: 96,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (DanceMoves) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // ty: int32
            b.write_all(&row.ty.to_le_bytes())?;

            // param: int32
            b.write_all(&row.param.to_le_bytes())?;

            // fallback: int32
            b.write_all(&row.fallback.to_le_bytes())?;

            // racemask: int32
            b.write_all(&row.racemask.to_le_bytes())?;

            // internal_name: string_ref
            if !row.internal_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.internal_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // lock_id: foreign_key (Lock) int32
            b.write_all(&(row.lock_id.id as i32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for DanceMoves {
    type PrimaryKey = DanceMovesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl DanceMoves {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.internal_name.is_empty() { b.write_all(row.internal_name.as_bytes())?; b.write_all(&[0])?; };
            row.name_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.internal_name.is_empty() { sum += row.internal_name.len() + 1; };
            sum += row.name_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstDanceMoves<const S: usize> {
    pub rows: [ConstDanceMovesRow; S],
}

impl<const S: usize> ConstDanceMoves<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 96 {
            panic!("invalid record size, expected 96")
        }

        if header.field_count != 24 {
            panic!("invalid field count, expected 24")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstDanceMovesRow {
                id: DanceMovesKey::new(0),
                ty: 0,
                param: 0,
                fallback: 0,
                racemask: 0,
                internal_name: "",
                name_lang: crate::ConstExtendedLocalizedString::empty(),
                lock_id: LockKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (DanceMoves) int32
            let id = DanceMovesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // ty: int32
            let ty = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // param: int32
            let param = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // fallback: int32
            let fallback = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // racemask: int32
            let racemask = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // internal_name: string_ref
            let internal_name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // name_lang: string_ref_loc (Extended)
            let name_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // lock_id: foreign_key (Lock) int32
            let lock_id = LockKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = ConstDanceMovesRow {
                id,
                ty,
                param,
                fallback,
                racemask,
                internal_name,
                name_lang,
                lock_id,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct DanceMovesKey {
    pub id: i32
}

impl DanceMovesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for DanceMovesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for DanceMovesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for DanceMovesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for DanceMovesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for DanceMovesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DanceMovesRow {
    pub id: DanceMovesKey,
    pub ty: i32,
    pub param: i32,
    pub fallback: i32,
    pub racemask: i32,
    pub internal_name: String,
    pub name_lang: ExtendedLocalizedString,
    pub lock_id: LockKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstDanceMovesRow {
    pub id: DanceMovesKey,
    pub ty: i32,
    pub param: i32,
    pub fallback: i32,
    pub racemask: i32,
    pub internal_name: &'static str,
    pub name_lang: ConstExtendedLocalizedString,
    pub lock_id: LockKey,
}

