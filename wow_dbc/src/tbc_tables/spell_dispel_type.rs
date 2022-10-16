use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::ExtendedLocalizedString;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellDispelType {
    pub rows: Vec<SpellDispelTypeRow>,
}

impl DbcTable for SpellDispelType {
    type Row = SpellDispelTypeRow;

    fn filename() -> &'static str { "SpellDispelType.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 84 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 84,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 21 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 21,
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

            // id: primary_key (SpellDispelType) int32
            let id = SpellDispelTypeKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // mask: int32
            let mask = crate::util::read_i32_le(chunk)?;

            // immunity_possible: int32
            let immunity_possible = crate::util::read_i32_le(chunk)?;

            // internal_name: string_ref
            let internal_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(SpellDispelTypeRow {
                id,
                name_lang,
                mask,
                immunity_possible,
                internal_name,
            });
        }

        Ok(SpellDispelType { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 21,
            record_size: 84,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SpellDispelType) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // mask: int32
            b.write_all(&row.mask.to_le_bytes())?;

            // immunity_possible: int32
            b.write_all(&row.immunity_possible.to_le_bytes())?;

            // internal_name: string_ref
            if !row.internal_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.internal_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SpellDispelType {
    type PrimaryKey = SpellDispelTypeKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl SpellDispelType {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
            if !row.internal_name.is_empty() { b.write_all(row.internal_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
            if !row.internal_name.is_empty() { sum += row.internal_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellDispelTypeKey {
    pub id: i32
}

impl SpellDispelTypeKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellDispelTypeRow {
    pub id: SpellDispelTypeKey,
    pub name_lang: ExtendedLocalizedString,
    pub mask: i32,
    pub immunity_possible: i32,
    pub internal_name: String,
}

