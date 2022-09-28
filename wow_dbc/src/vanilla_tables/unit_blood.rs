use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitBlood {
    pub rows: Vec<UnitBloodRow>,
}

impl DbcTable for UnitBlood {
    type Row = UnitBloodRow;

    fn filename() -> &'static str { "UnitBlood.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 40 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 40,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 10 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 10,
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

            // id: primary_key (UnitBlood) uint32
            let id = UnitBloodKey::new(crate::util::read_u32_le(chunk)?);

            // combat_blood_spurt_front_small: int32
            let combat_blood_spurt_front_small = crate::util::read_i32_le(chunk)?;

            // combat_blood_spurt_front_large: int32
            let combat_blood_spurt_front_large = crate::util::read_i32_le(chunk)?;

            // combat_blood_spurt_back_small: int32
            let combat_blood_spurt_back_small = crate::util::read_i32_le(chunk)?;

            // combat_blood_spurt_back_large: int32
            let combat_blood_spurt_back_large = crate::util::read_i32_le(chunk)?;

            // texture: string_ref[5]
            let texture = {
                let mut arr = Vec::with_capacity(5);
                for _ in 0..5 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };


            rows.push(UnitBloodRow {
                id,
                combat_blood_spurt_front_small,
                combat_blood_spurt_front_large,
                combat_blood_spurt_back_small,
                combat_blood_spurt_back_large,
                texture,
            });
        }

        Ok(UnitBlood { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 10,
            record_size: 40,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (UnitBlood) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // combat_blood_spurt_front_small: int32
            b.write_all(&row.combat_blood_spurt_front_small.to_le_bytes())?;

            // combat_blood_spurt_front_large: int32
            b.write_all(&row.combat_blood_spurt_front_large.to_le_bytes())?;

            // combat_blood_spurt_back_small: int32
            b.write_all(&row.combat_blood_spurt_back_small.to_le_bytes())?;

            // combat_blood_spurt_back_large: int32
            b.write_all(&row.combat_blood_spurt_back_large.to_le_bytes())?;

            // texture: string_ref[5]
            for i in &row.texture {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for UnitBlood {
    type PrimaryKey = UnitBloodKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl UnitBlood {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            for s in &row.texture {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            for s in &row.texture {
                if !s.is_empty() { sum += s.len() + 1; };
            }

        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct UnitBloodKey {
    pub id: u32
}

impl UnitBloodKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitBloodRow {
    pub id: UnitBloodKey,
    pub combat_blood_spurt_front_small: i32,
    pub combat_blood_spurt_front_large: i32,
    pub combat_blood_spurt_back_small: i32,
    pub combat_blood_spurt_back_large: i32,
    pub texture: [String; 5],
}

