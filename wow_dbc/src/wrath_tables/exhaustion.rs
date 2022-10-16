use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::ExtendedLocalizedString;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Exhaustion {
    pub rows: Vec<ExhaustionRow>,
}

impl DbcTable for Exhaustion {
    type Row = ExhaustionRow;

    fn filename() -> &'static str { "Exhaustion.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 92 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 92,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 23 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 23,
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

            // id: primary_key (Exhaustion) int32
            let id = ExhaustionKey::new(crate::util::read_i32_le(chunk)?);

            // xp: int32
            let xp = crate::util::read_i32_le(chunk)?;

            // factor: float
            let factor = crate::util::read_f32_le(chunk)?;

            // outdoor_hours: float
            let outdoor_hours = crate::util::read_f32_le(chunk)?;

            // inn_hours: float
            let inn_hours = crate::util::read_f32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // threshold: float
            let threshold = crate::util::read_f32_le(chunk)?;


            rows.push(ExhaustionRow {
                id,
                xp,
                factor,
                outdoor_hours,
                inn_hours,
                name_lang,
                threshold,
            });
        }

        Ok(Exhaustion { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 23,
            record_size: 92,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Exhaustion) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // xp: int32
            b.write_all(&row.xp.to_le_bytes())?;

            // factor: float
            b.write_all(&row.factor.to_le_bytes())?;

            // outdoor_hours: float
            b.write_all(&row.outdoor_hours.to_le_bytes())?;

            // inn_hours: float
            b.write_all(&row.inn_hours.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // threshold: float
            b.write_all(&row.threshold.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Exhaustion {
    type PrimaryKey = ExhaustionKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl Exhaustion {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ExhaustionKey {
    pub id: i32
}

impl ExhaustionKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for ExhaustionKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for ExhaustionKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for ExhaustionKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for ExhaustionKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ExhaustionKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ExhaustionRow {
    pub id: ExhaustionKey,
    pub xp: i32,
    pub factor: f32,
    pub outdoor_hours: f32,
    pub inn_hours: f32,
    pub name_lang: ExtendedLocalizedString,
    pub threshold: f32,
}

