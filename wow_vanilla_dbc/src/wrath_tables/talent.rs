use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::spell::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Talent {
    pub rows: Vec<TalentRow>,
}

impl DbcTable for Talent {
    type Row = TalentRow;

    fn filename() -> &'static str { "Talent.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 92 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 92,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 23 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 23,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (Talent) int32
            let id = TalentKey::new(crate::util::read_i32_le(chunk)?);

            // tab_id: int32
            let tab_id = crate::util::read_i32_le(chunk)?;

            // tier_id: int32
            let tier_id = crate::util::read_i32_le(chunk)?;

            // column_index: int32
            let column_index = crate::util::read_i32_le(chunk)?;

            // spell_rank: int32[9]
            let spell_rank = crate::util::read_array_i32::<9>(chunk)?;

            // prereq_talent: int32[3]
            let prereq_talent = crate::util::read_array_i32::<3>(chunk)?;

            // prereq_rank: int32[3]
            let prereq_rank = crate::util::read_array_i32::<3>(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // required_spell_id: foreign_key (Spell) int32
            let required_spell_id = SpellKey::new(crate::util::read_i32_le(chunk)?.into());

            // category_mask: int32[2]
            let category_mask = crate::util::read_array_i32::<2>(chunk)?;


            rows.push(TalentRow {
                id,
                tab_id,
                tier_id,
                column_index,
                spell_rank,
                prereq_talent,
                prereq_rank,
                flags,
                required_spell_id,
                category_mask,
            });
        }

        Ok(Talent { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 23,
            record_size: 92,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (Talent) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // tab_id: int32
            b.write_all(&row.tab_id.to_le_bytes())?;

            // tier_id: int32
            b.write_all(&row.tier_id.to_le_bytes())?;

            // column_index: int32
            b.write_all(&row.column_index.to_le_bytes())?;

            // spell_rank: int32[9]
            for i in row.spell_rank {
                b.write_all(&i.to_le_bytes())?;
            }


            // prereq_talent: int32[3]
            for i in row.prereq_talent {
                b.write_all(&i.to_le_bytes())?;
            }


            // prereq_rank: int32[3]
            for i in row.prereq_rank {
                b.write_all(&i.to_le_bytes())?;
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // required_spell_id: foreign_key (Spell) int32
            b.write_all(&(row.required_spell_id.id as i32).to_le_bytes())?;

            // category_mask: int32[2]
            for i in row.category_mask {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for Talent {
    type PrimaryKey = TalentKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct TalentKey {
    pub id: i32
}

impl TalentKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TalentRow {
    pub id: TalentKey,
    pub tab_id: i32,
    pub tier_id: i32,
    pub column_index: i32,
    pub spell_rank: [i32; 9],
    pub prereq_talent: [i32; 3],
    pub prereq_rank: [i32; 3],
    pub flags: i32,
    pub required_spell_id: SpellKey,
    pub category_mask: [i32; 2],
}
