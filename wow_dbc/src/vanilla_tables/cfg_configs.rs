use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cfg_Configs {
    pub rows: Vec<Cfg_ConfigsRow>,
}

impl DbcTable for Cfg_Configs {
    type Row = Cfg_ConfigsRow;

    fn filename() -> &'static str { "Cfg_Configs.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 16,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 4 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 4,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (Cfg_Configs) uint32
            let id = Cfg_ConfigsKey::new(crate::util::read_u32_le(chunk)?);

            // realm_type: int32
            let realm_type = crate::util::read_i32_le(chunk)?;

            // pvp_allowed: int32
            let pvp_allowed = crate::util::read_i32_le(chunk)?;

            // roleplaying: int32
            let roleplaying = crate::util::read_i32_le(chunk)?;


            rows.push(Cfg_ConfigsRow {
                id,
                realm_type,
                pvp_allowed,
                roleplaying,
            });
        }

        Ok(Cfg_Configs { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 4,
            record_size: 16,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (Cfg_Configs) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // realm_type: int32
            b.write_all(&row.realm_type.to_le_bytes())?;

            // pvp_allowed: int32
            b.write_all(&row.pvp_allowed.to_le_bytes())?;

            // roleplaying: int32
            b.write_all(&row.roleplaying.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for Cfg_Configs {
    type PrimaryKey = Cfg_ConfigsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct Cfg_ConfigsKey {
    pub id: u32
}

impl Cfg_ConfigsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for Cfg_ConfigsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for Cfg_ConfigsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for Cfg_ConfigsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cfg_ConfigsRow {
    pub id: Cfg_ConfigsKey,
    pub realm_type: i32,
    pub pvp_allowed: i32,
    pub roleplaying: i32,
}

