use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OverrideSpellData {
    pub rows: Vec<OverrideSpellDataRow>,
}

impl DbcTable for OverrideSpellData {
    type Row = OverrideSpellDataRow;

    fn filename() -> &'static str { "OverrideSpellData.dbc" }

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

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (OverrideSpellData) int32
            let id = OverrideSpellDataKey::new(crate::util::read_i32_le(chunk)?);

            // spells: int32[10]
            let spells = crate::util::read_array_i32::<10>(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(OverrideSpellDataRow {
                id,
                spells,
                flags,
            });
        }

        Ok(OverrideSpellData { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 12,
            record_size: 48,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (OverrideSpellData) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // spells: int32[10]
            for i in row.spells {
                b.write_all(&i.to_le_bytes())?;
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for OverrideSpellData {
    type PrimaryKey = OverrideSpellDataKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct OverrideSpellDataKey {
    pub id: i32
}

impl OverrideSpellDataKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OverrideSpellDataRow {
    pub id: OverrideSpellDataKey,
    pub spells: [i32; 10],
    pub flags: i32,
}

