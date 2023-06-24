use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct gtOCTClassCombatRatingScalar {
    pub rows: Vec<gtOCTClassCombatRatingScalarRow>,
}

impl DbcTable for gtOCTClassCombatRatingScalar {
    type Row = gtOCTClassCombatRatingScalarRow;

    const FILENAME: &'static str = "gtOCTClassCombatRatingScalar.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 8 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 8,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 2 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 2,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (gtOCTClassCombatRatingScalar) int32
            let id = gtOCTClassCombatRatingScalarKey::new(crate::util::read_i32_le(chunk)?);

            // data: float
            let data = crate::util::read_f32_le(chunk)?;


            rows.push(gtOCTClassCombatRatingScalarRow {
                id,
                data,
            });
        }

        Ok(gtOCTClassCombatRatingScalar { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 2,
            record_size: 8,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (gtOCTClassCombatRatingScalar) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // data: float
            b.write_all(&row.data.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for gtOCTClassCombatRatingScalar {
    type PrimaryKey = gtOCTClassCombatRatingScalarKey;
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
pub struct gtOCTClassCombatRatingScalarKey {
    pub id: i32
}

impl gtOCTClassCombatRatingScalarKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for gtOCTClassCombatRatingScalarKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for gtOCTClassCombatRatingScalarKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for gtOCTClassCombatRatingScalarKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for gtOCTClassCombatRatingScalarKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for gtOCTClassCombatRatingScalarKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct gtOCTClassCombatRatingScalarRow {
    pub id: gtOCTClassCombatRatingScalarKey,
    pub data: f32,
}

