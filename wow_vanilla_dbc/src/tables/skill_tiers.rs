use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq)]
pub struct SkillTiers {
    pub rows: Vec<SkillTiersRow>,
}

impl DbcTable for SkillTiers {
    type Row = SkillTiersRow;

    fn filename() -> &'static str { "SkillTiers.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 132 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 132,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 33 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 132,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SkillTiers) uint32
            let id = SkillTiersKey::new(crate::util::read_u32_le(chunk)?);

            // cost: int32[16]
            let cost = crate::util::read_array_i32::<16>(chunk)?;

            // value: int32[16]
            let value = crate::util::read_array_i32::<16>(chunk)?;


            rows.push(SkillTiersRow {
                id,
                cost,
                value,
            });
        }

        Ok(SkillTiers { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 33,
            record_size: 132,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SkillTiers) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // cost: int32[16]
            for i in row.cost {
                b.write_all(&i.to_le_bytes())?;
            }


            // value: int32[16]
            for i in row.value {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SkillTiers {
    type PrimaryKey = SkillTiersKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct SkillTiersKey {
    pub id: u32
}

impl SkillTiersKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct SkillTiersRow {
    pub id: SkillTiersKey,
    pub cost: [i32; 16],
    pub value: [i32; 16],
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn skill_tiers() {
        let contents = include_bytes!("../../../dbc/SkillTiers.dbc");
        let actual = SkillTiers::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SkillTiers::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
