use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::spell::SpellKey;
use crate::vanilla_tables::talent_tab::TalentTabKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Talent {
    pub rows: Vec<TalentRow>,
}

impl DbcTable for Talent {
    type Row = TalentRow;

    const FILENAME: &'static str = "Talent.dbc";
    const FIELD_COUNT: usize = 21;
    const ROW_SIZE: usize = 84;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (Talent) uint32
            let id = TalentKey::new(crate::util::read_u32_le(chunk)?);

            // tab: foreign_key (TalentTab) uint32
            let tab = TalentTabKey::new(crate::util::read_u32_le(chunk)?.into());

            // tier: int32
            let tier = crate::util::read_i32_le(chunk)?;

            // column_index: int32
            let column_index = crate::util::read_i32_le(chunk)?;

            // spell_rank: uint32[9]
            let spell_rank = crate::util::read_array_u32::<9>(chunk)?;

            // prereq_talents: uint32[3]
            let prereq_talents = crate::util::read_array_u32::<3>(chunk)?;

            // prereq_ranks: int32[3]
            let prereq_ranks = crate::util::read_array_i32::<3>(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // required_spell: foreign_key (Spell) uint32
            let required_spell = SpellKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(TalentRow {
                id,
                tab,
                tier,
                column_index,
                spell_rank,
                prereq_talents,
                prereq_ranks,
                flags,
                required_spell,
            });
        }

        Ok(Talent { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (Talent) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // tab: foreign_key (TalentTab) uint32
            b.write_all(&(row.tab.id as u32).to_le_bytes())?;

            // tier: int32
            b.write_all(&row.tier.to_le_bytes())?;

            // column_index: int32
            b.write_all(&row.column_index.to_le_bytes())?;

            // spell_rank: uint32[9]
            for i in row.spell_rank {
                b.write_all(&i.to_le_bytes())?;
            }


            // prereq_talents: uint32[3]
            for i in row.prereq_talents {
                b.write_all(&i.to_le_bytes())?;
            }


            // prereq_ranks: int32[3]
            for i in row.prereq_ranks {
                b.write_all(&i.to_le_bytes())?;
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // required_spell: foreign_key (Spell) uint32
            b.write_all(&(row.required_spell.id as u32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for Talent {
    type PrimaryKey = TalentKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TalentKey {
    pub id: u32
}

impl TalentKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for TalentKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for TalentKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for TalentKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u64> for TalentKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for TalentKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i8> for TalentKey {
    type Error = i8;
    fn try_from(v: i8) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i16> for TalentKey {
    type Error = i16;
    fn try_from(v: i16) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i32> for TalentKey {
    type Error = i32;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for TalentKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for TalentKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<u32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TalentRow {
    pub id: TalentKey,
    pub tab: TalentTabKey,
    pub tier: i32,
    pub column_index: i32,
    pub spell_rank: [u32; 9],
    pub prereq_talents: [u32; 3],
    pub prereq_ranks: [i32; 3],
    pub flags: i32,
    pub required_spell: SpellKey,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn talent() {
        let mut file = File::open("../vanilla-dbc/Talent.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Talent::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Talent::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
