use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillCostsData {
    pub rows: Vec<SkillCostsDataRow>,
}

impl DbcTable for SkillCostsData {
    type Row = SkillCostsDataRow;

    fn filename() -> &'static str { "SkillCostsData.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 20 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 20,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 5 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 5,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SkillCostsData) uint32
            let id = SkillCostsDataKey::new(crate::util::read_u32_le(chunk)?);

            // skill_costs: int32
            let skill_costs = crate::util::read_i32_le(chunk)?;

            // cost: int32[3]
            let cost = crate::util::read_array_i32::<3>(chunk)?;


            rows.push(SkillCostsDataRow {
                id,
                skill_costs,
                cost,
            });
        }

        Ok(SkillCostsData { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 5,
            record_size: 20,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SkillCostsData) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // skill_costs: int32
            b.write_all(&row.skill_costs.to_le_bytes())?;

            // cost: int32[3]
            for i in row.cost {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SkillCostsData {
    type PrimaryKey = SkillCostsDataKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstSkillCostsData<const S: usize> {
    pub rows: [SkillCostsDataRow; S],
}

impl<const S: usize> ConstSkillCostsData<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 20 {
            panic!("invalid record size, expected 20")
        }

        if header.field_count != 5 {
            panic!("invalid field count, expected 5")
        }

        let mut b_offset = 20;
        let mut rows = [
            SkillCostsDataRow {
                id: SkillCostsDataKey::new(0),
                skill_costs: 0,
                cost: [0; 3],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SkillCostsData) uint32
            let id = SkillCostsDataKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // skill_costs: int32
            let skill_costs = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // cost: int32[3]
            let cost = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = SkillCostsDataRow {
                id,
                skill_costs,
                cost,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SkillCostsData {
        SkillCostsData {
            rows: self.rows.iter().map(|s| SkillCostsDataRow {
                id: s.id,
                skill_costs: s.skill_costs,
                cost: s.cost,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SkillCostsDataKey {
    pub id: u32
}

impl SkillCostsDataKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SkillCostsDataKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SkillCostsDataKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SkillCostsDataKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SkillCostsDataRow {
    pub id: SkillCostsDataKey,
    pub skill_costs: i32,
    pub cost: [i32; 3],
}

