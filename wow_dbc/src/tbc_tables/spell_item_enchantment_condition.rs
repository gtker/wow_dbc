use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellItemEnchantmentCondition {
    pub rows: Vec<SpellItemEnchantmentConditionRow>,
}

impl DbcTable for SpellItemEnchantmentCondition {
    type Row = SpellItemEnchantmentConditionRow;

    fn filename() -> &'static str { "SpellItemEnchantmentCondition.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 64 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 64,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 31 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 31,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SpellItemEnchantmentCondition) int32
            let id = SpellItemEnchantmentConditionKey::new(crate::util::read_i32_le(chunk)?);

            // lt_operand_type: int8[5]
            let lt_operand_type = {
                let mut arr = [i8::default(); 5];
                for i in arr.iter_mut() {
                    *i = crate::util::read_i8_le(chunk)?;
                }

                arr
            };

            // lt_operand: int32[5]
            let lt_operand = crate::util::read_array_i32::<5>(chunk)?;

            // operator: int8[5]
            let operator = {
                let mut arr = [i8::default(); 5];
                for i in arr.iter_mut() {
                    *i = crate::util::read_i8_le(chunk)?;
                }

                arr
            };

            // rt_operand_type: int8[5]
            let rt_operand_type = {
                let mut arr = [i8::default(); 5];
                for i in arr.iter_mut() {
                    *i = crate::util::read_i8_le(chunk)?;
                }

                arr
            };

            // rt_operand: int32[5]
            let rt_operand = crate::util::read_array_i32::<5>(chunk)?;

            // logic: int8[5]
            let logic = {
                let mut arr = [i8::default(); 5];
                for i in arr.iter_mut() {
                    *i = crate::util::read_i8_le(chunk)?;
                }

                arr
            };


            rows.push(SpellItemEnchantmentConditionRow {
                id,
                lt_operand_type,
                lt_operand,
                operator,
                rt_operand_type,
                rt_operand,
                logic,
            });
        }

        Ok(SpellItemEnchantmentCondition { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 31,
            record_size: 64,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SpellItemEnchantmentCondition) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // lt_operand_type: int8[5]
            for i in row.lt_operand_type {
                b.write_all(&i.to_le_bytes())?;
            }


            // lt_operand: int32[5]
            for i in row.lt_operand {
                b.write_all(&i.to_le_bytes())?;
            }


            // operator: int8[5]
            for i in row.operator {
                b.write_all(&i.to_le_bytes())?;
            }


            // rt_operand_type: int8[5]
            for i in row.rt_operand_type {
                b.write_all(&i.to_le_bytes())?;
            }


            // rt_operand: int32[5]
            for i in row.rt_operand {
                b.write_all(&i.to_le_bytes())?;
            }


            // logic: int8[5]
            for i in row.logic {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellItemEnchantmentCondition {
    type PrimaryKey = SpellItemEnchantmentConditionKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellItemEnchantmentConditionKey {
    pub id: i32
}

impl SpellItemEnchantmentConditionKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellItemEnchantmentConditionRow {
    pub id: SpellItemEnchantmentConditionKey,
    pub lt_operand_type: [i8; 5],
    pub lt_operand: [i32; 5],
    pub operator: [i8; 5],
    pub rt_operand_type: [i8; 5],
    pub rt_operand: [i32; 5],
    pub logic: [i8; 5],
}

