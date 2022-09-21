use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BannedAddons {
    pub rows: Vec<BannedAddonsRow>,
}

impl DbcTable for BannedAddons {
    type Row = BannedAddonsRow;

    fn filename() -> &'static str { "BannedAddons.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 44 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 44,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 11 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 11,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (BannedAddons) int32
            let id = BannedAddonsKey::new(crate::util::read_i32_le(chunk)?);

            // name_m_d5: int32[4]
            let name_m_d5 = crate::util::read_array_i32::<4>(chunk)?;

            // version_m_d5: int32[4]
            let version_m_d5 = crate::util::read_array_i32::<4>(chunk)?;

            // last_modified: int32
            let last_modified = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(BannedAddonsRow {
                id,
                name_m_d5,
                version_m_d5,
                last_modified,
                flags,
            });
        }

        Ok(BannedAddons { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 11,
            record_size: 44,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (BannedAddons) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_m_d5: int32[4]
            for i in row.name_m_d5 {
                b.write_all(&i.to_le_bytes())?;
            }


            // version_m_d5: int32[4]
            for i in row.version_m_d5 {
                b.write_all(&i.to_le_bytes())?;
            }


            // last_modified: int32
            b.write_all(&row.last_modified.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for BannedAddons {
    type PrimaryKey = BannedAddonsKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct BannedAddonsKey {
    pub id: i32
}

impl BannedAddonsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BannedAddonsRow {
    pub id: BannedAddonsKey,
    pub name_m_d5: [i32; 4],
    pub version_m_d5: [i32; 4],
    pub last_modified: i32,
    pub flags: i32,
}

