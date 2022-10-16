use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BannedAddOns {
    pub rows: Vec<BannedAddOnsRow>,
}

impl DbcTable for BannedAddOns {
    type Row = BannedAddOnsRow;

    fn filename() -> &'static str { "BannedAddOns.dbc" }

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

            // id: primary_key (BannedAddOns) int32
            let id = BannedAddOnsKey::new(crate::util::read_i32_le(chunk)?);

            // name_m_d5: int32[4]
            let name_m_d5 = crate::util::read_array_i32::<4>(chunk)?;

            // version_m_d5: int32[4]
            let version_m_d5 = crate::util::read_array_i32::<4>(chunk)?;

            // last_modified: int32
            let last_modified = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;


            rows.push(BannedAddOnsRow {
                id,
                name_m_d5,
                version_m_d5,
                last_modified,
                flags,
            });
        }

        Ok(BannedAddOns { rows, })
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
            // id: primary_key (BannedAddOns) int32
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

impl Indexable for BannedAddOns {
    type PrimaryKey = BannedAddOnsKey;
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
pub struct BannedAddOnsKey {
    pub id: i32
}

impl BannedAddOnsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for BannedAddOnsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for BannedAddOnsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for BannedAddOnsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for BannedAddOnsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for BannedAddOnsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BannedAddOnsRow {
    pub id: BannedAddOnsKey,
    pub name_m_d5: [i32; 4],
    pub version_m_d5: [i32; 4],
    pub last_modified: i32,
    pub flags: i32,
}

