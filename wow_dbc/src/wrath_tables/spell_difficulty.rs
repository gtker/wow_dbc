use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellDifficulty {
    pub rows: Vec<SpellDifficultyRow>,
}

impl DbcTable for SpellDifficulty {
    type Row = SpellDifficultyRow;

    fn filename() -> &'static str { "SpellDifficulty.dbc" }

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

            // id: primary_key (SpellDifficulty) int32
            let id = SpellDifficultyKey::new(crate::util::read_i32_le(chunk)?);

            // difficulty_spell_id: int32[4]
            let difficulty_spell_id = crate::util::read_array_i32::<4>(chunk)?;


            rows.push(SpellDifficultyRow {
                id,
                difficulty_spell_id,
            });
        }

        Ok(SpellDifficulty { rows, })
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
            // id: primary_key (SpellDifficulty) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // difficulty_spell_id: int32[4]
            for i in row.difficulty_spell_id {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellDifficulty {
    type PrimaryKey = SpellDifficultyKey;
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
pub struct ConstSpellDifficulty<const S: usize> {
    pub rows: [SpellDifficultyRow; S],
}

impl<const S: usize> ConstSpellDifficulty<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 20 {
            panic!("invalid record size, expected 20")
        }

        if header.field_count != 5 {
            panic!("invalid field count, expected 5")
        }

        let mut b_offset = 20;
        let mut rows = [
            SpellDifficultyRow {
                id: SpellDifficultyKey::new(0),
                difficulty_spell_id: [0; 4],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SpellDifficulty) int32
            let id = SpellDifficultyKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // difficulty_spell_id: int32[4]
            let difficulty_spell_id = {
                let mut a = [0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = SpellDifficultyRow {
                id,
                difficulty_spell_id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SpellDifficulty {
        SpellDifficulty {
            rows: self.rows.iter().map(|s| SpellDifficultyRow {
                id: s.id,
                difficulty_spell_id: s.difficulty_spell_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellDifficultyKey {
    pub id: i32
}

impl SpellDifficultyKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SpellDifficultyKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SpellDifficultyKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SpellDifficultyKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SpellDifficultyKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SpellDifficultyKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpellDifficultyRow {
    pub id: SpellDifficultyKey,
    pub difficulty_spell_id: [i32; 4],
}

