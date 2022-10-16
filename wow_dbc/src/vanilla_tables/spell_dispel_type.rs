use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;

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

        if header.record_size != 48 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 48,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 12 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 12,
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

            // id: primary_key (SpellDispelType) uint32
            let id = SpellDispelTypeKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // mask: int32
            let mask = crate::util::read_i32_le(chunk)?;

            // immunity_possible: uint32
            let immunity_possible = crate::util::read_u32_le(chunk)?;


            rows.push(SpellDispelTypeRow {
                id,
                name,
                mask,
                immunity_possible,
            });
        }

        Ok(SpellDispelType { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 12,
            record_size: 48,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SpellDispelType) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // mask: int32
            b.write_all(&row.mask.to_le_bytes())?;

            // immunity_possible: uint32
            b.write_all(&row.immunity_possible.to_le_bytes())?;

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
            row.name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellDispelTypeKey {
    pub id: u32
}

impl SpellDispelTypeKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellDispelTypeRow {
    pub id: SpellDispelTypeKey,
    pub name: LocalizedString,
    pub mask: i32,
    pub immunity_possible: u32,
}

