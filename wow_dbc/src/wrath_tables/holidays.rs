use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::holiday_descriptions::*;
use crate::wrath_tables::holiday_names::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Holidays {
    pub rows: Vec<HolidaysRow>,
}

impl DbcTable for Holidays {
    type Row = HolidaysRow;

    fn filename() -> &'static str { "Holidays.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 220 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 220,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 55 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 55,
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

            // id: primary_key (Holidays) int32
            let id = HolidaysKey::new(crate::util::read_i32_le(chunk)?);

            // duration: int32[10]
            let duration = crate::util::read_array_i32::<10>(chunk)?;

            // date: int32[26]
            let date = crate::util::read_array_i32::<26>(chunk)?;

            // region: int32
            let region = crate::util::read_i32_le(chunk)?;

            // looping: int32
            let looping = crate::util::read_i32_le(chunk)?;

            // calendar_flags: int32[10]
            let calendar_flags = crate::util::read_array_i32::<10>(chunk)?;

            // holiday_name_id: foreign_key (HolidayNames) int32
            let holiday_name_id = HolidayNamesKey::new(crate::util::read_i32_le(chunk)?.into());

            // holiday_description_id: foreign_key (HolidayDescriptions) int32
            let holiday_description_id = HolidayDescriptionsKey::new(crate::util::read_i32_le(chunk)?.into());

            // texture_file_name: string_ref
            let texture_file_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // priority: int32
            let priority = crate::util::read_i32_le(chunk)?;

            // calendar_filter_type: int32
            let calendar_filter_type = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(HolidaysRow {
                id,
                duration,
                date,
                region,
                looping,
                calendar_flags,
                holiday_name_id,
                holiday_description_id,
                texture_file_name,
                priority,
                calendar_filter_type,
                flags,
            });
        }

        Ok(Holidays { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 55,
            record_size: 220,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Holidays) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // duration: int32[10]
            for i in row.duration {
                b.write_all(&i.to_le_bytes())?;
            }


            // date: int32[26]
            for i in row.date {
                b.write_all(&i.to_le_bytes())?;
            }


            // region: int32
            b.write_all(&row.region.to_le_bytes())?;

            // looping: int32
            b.write_all(&row.looping.to_le_bytes())?;

            // calendar_flags: int32[10]
            for i in row.calendar_flags {
                b.write_all(&i.to_le_bytes())?;
            }


            // holiday_name_id: foreign_key (HolidayNames) int32
            b.write_all(&(row.holiday_name_id.id as i32).to_le_bytes())?;

            // holiday_description_id: foreign_key (HolidayDescriptions) int32
            b.write_all(&(row.holiday_description_id.id as i32).to_le_bytes())?;

            // texture_file_name: string_ref
            if !row.texture_file_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.texture_file_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // priority: int32
            b.write_all(&row.priority.to_le_bytes())?;

            // calendar_filter_type: int32
            b.write_all(&row.calendar_filter_type.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Holidays {
    type PrimaryKey = HolidaysKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl Holidays {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.texture_file_name.is_empty() { b.write_all(row.texture_file_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.texture_file_name.is_empty() { sum += row.texture_file_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstHolidays<const S: usize> {
    pub rows: [ConstHolidaysRow; S],
}

impl<const S: usize> ConstHolidays<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 220 {
            panic!("invalid record size, expected 220")
        }

        if header.field_count != 55 {
            panic!("invalid field count, expected 55")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstHolidaysRow {
                id: HolidaysKey::new(0),
                duration: [0; 10],
                date: [0; 26],
                region: 0,
                looping: 0,
                calendar_flags: [0; 10],
                holiday_name_id: HolidayNamesKey::new(0),
                holiday_description_id: HolidayDescriptionsKey::new(0),
                texture_file_name: "",
                priority: 0,
                calendar_filter_type: 0,
                flags: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Holidays) int32
            let id = HolidaysKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // duration: int32[10]
            let duration = {
                let mut a = [0; 10];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // date: int32[26]
            let date = {
                let mut a = [0; 26];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // region: int32
            let region = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // looping: int32
            let looping = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // calendar_flags: int32[10]
            let calendar_flags = {
                let mut a = [0; 10];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // holiday_name_id: foreign_key (HolidayNames) int32
            let holiday_name_id = HolidayNamesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // holiday_description_id: foreign_key (HolidayDescriptions) int32
            let holiday_description_id = HolidayDescriptionsKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // texture_file_name: string_ref
            let texture_file_name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // priority: int32
            let priority = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // calendar_filter_type: int32
            let calendar_filter_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstHolidaysRow {
                id,
                duration,
                date,
                region,
                looping,
                calendar_flags,
                holiday_name_id,
                holiday_description_id,
                texture_file_name,
                priority,
                calendar_filter_type,
                flags,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct HolidaysKey {
    pub id: i32
}

impl HolidaysKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for HolidaysKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for HolidaysKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for HolidaysKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for HolidaysKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for HolidaysKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HolidaysRow {
    pub id: HolidaysKey,
    pub duration: [i32; 10],
    pub date: [i32; 26],
    pub region: i32,
    pub looping: i32,
    pub calendar_flags: [i32; 10],
    pub holiday_name_id: HolidayNamesKey,
    pub holiday_description_id: HolidayDescriptionsKey,
    pub texture_file_name: String,
    pub priority: i32,
    pub calendar_filter_type: i32,
    pub flags: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstHolidaysRow {
    pub id: HolidaysKey,
    pub duration: [i32; 10],
    pub date: [i32; 26],
    pub region: i32,
    pub looping: i32,
    pub calendar_flags: [i32; 10],
    pub holiday_name_id: HolidayNamesKey,
    pub holiday_description_id: HolidayDescriptionsKey,
    pub texture_file_name: &'static str,
    pub priority: i32,
    pub calendar_filter_type: i32,
    pub flags: i32,
}

