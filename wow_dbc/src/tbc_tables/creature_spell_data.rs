use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreatureSpellData {
    pub rows: Vec<CreatureSpellDataRow>,
}

impl DbcTable for CreatureSpellData {
    type Row = CreatureSpellDataRow;

    fn filename() -> &'static str { "CreatureSpellData.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 36 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 36,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 9 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 9,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (CreatureSpellData) int32
            let id = CreatureSpellDataKey::new(crate::util::read_i32_le(chunk)?);

            // spells: int32[4]
            let spells = crate::util::read_array_i32::<4>(chunk)?;

            // availability: int32[4]
            let availability = crate::util::read_array_i32::<4>(chunk)?;


            rows.push(CreatureSpellDataRow {
                id,
                spells,
                availability,
            });
        }

        Ok(CreatureSpellData { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 9,
            record_size: 36,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (CreatureSpellData) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // spells: int32[4]
            for i in row.spells {
                b.write_all(&i.to_le_bytes())?;
            }


            // availability: int32[4]
            for i in row.availability {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for CreatureSpellData {
    type PrimaryKey = CreatureSpellDataKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CreatureSpellDataKey {
    pub id: i32
}

impl CreatureSpellDataKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreatureSpellDataRow {
    pub id: CreatureSpellDataKey,
    pub spells: [i32; 4],
    pub availability: [i32; 4],
}

