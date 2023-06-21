use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::wrath_tables::faction::*;
use crate::wrath_tables::faction_group::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactionTemplate {
    pub rows: Vec<FactionTemplateRow>,
}

impl DbcTable for FactionTemplate {
    type Row = FactionTemplateRow;

    fn filename() -> &'static str { "FactionTemplate.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 56 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 56,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 14 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 14,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (FactionTemplate) int32
            let id = FactionTemplateKey::new(crate::util::read_i32_le(chunk)?);

            // faction: foreign_key (Faction) int32
            let faction = FactionKey::new(crate::util::read_i32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // faction_group: foreign_key (FactionGroup) int32
            let faction_group = FactionGroupKey::new(crate::util::read_i32_le(chunk)?.into());

            // friend_group: int32
            let friend_group = crate::util::read_i32_le(chunk)?;

            // enemy_group: int32
            let enemy_group = crate::util::read_i32_le(chunk)?;

            // enemies: int32[4]
            let enemies = crate::util::read_array_i32::<4>(chunk)?;

            // friend: int32[4]
            let friend = crate::util::read_array_i32::<4>(chunk)?;


            rows.push(FactionTemplateRow {
                id,
                faction,
                flags,
                faction_group,
                friend_group,
                enemy_group,
                enemies,
                friend,
            });
        }

        Ok(FactionTemplate { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 14,
            record_size: 56,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (FactionTemplate) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // faction: foreign_key (Faction) int32
            b.write_all(&(row.faction.id as i32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // faction_group: foreign_key (FactionGroup) int32
            b.write_all(&(row.faction_group.id as i32).to_le_bytes())?;

            // friend_group: int32
            b.write_all(&row.friend_group.to_le_bytes())?;

            // enemy_group: int32
            b.write_all(&row.enemy_group.to_le_bytes())?;

            // enemies: int32[4]
            for i in row.enemies {
                b.write_all(&i.to_le_bytes())?;
            }


            // friend: int32[4]
            for i in row.friend {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for FactionTemplate {
    type PrimaryKey = FactionTemplateKey;
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
pub struct ConstFactionTemplate<const S: usize> {
    pub rows: [FactionTemplateRow; S],
}

impl<const S: usize> ConstFactionTemplate<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 56 {
            panic!("invalid record size, expected 56")
        }

        if header.field_count != 14 {
            panic!("invalid field count, expected 14")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            FactionTemplateRow {
                id: FactionTemplateKey::new(0),
                faction: FactionKey::new(0),
                flags: 0,
                faction_group: FactionGroupKey::new(0),
                friend_group: 0,
                enemy_group: 0,
                enemies: [0; 4],
                friend: [0; 4],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (FactionTemplate) int32
            let id = FactionTemplateKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // faction: foreign_key (Faction) int32
            let faction = FactionKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // faction_group: foreign_key (FactionGroup) int32
            let faction_group = FactionGroupKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // friend_group: int32
            let friend_group = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // enemy_group: int32
            let enemy_group = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // enemies: int32[4]
            let enemies = {
                let mut a = [0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // friend: int32[4]
            let friend = {
                let mut a = [0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = FactionTemplateRow {
                id,
                faction,
                flags,
                faction_group,
                friend_group,
                enemy_group,
                enemies,
                friend,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> FactionTemplate {
        FactionTemplate {
            rows: self.rows.iter().map(|s| FactionTemplateRow {
                id: s.id,
                faction: s.faction,
                flags: s.flags,
                faction_group: s.faction_group,
                friend_group: s.friend_group,
                enemy_group: s.enemy_group,
                enemies: s.enemies,
                friend: s.friend,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct FactionTemplateKey {
    pub id: i32
}

impl FactionTemplateKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for FactionTemplateKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for FactionTemplateKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for FactionTemplateKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for FactionTemplateKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for FactionTemplateKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactionTemplateRow {
    pub id: FactionTemplateKey,
    pub faction: FactionKey,
    pub flags: i32,
    pub faction_group: FactionGroupKey,
    pub friend_group: i32,
    pub enemy_group: i32,
    pub enemies: [i32; 4],
    pub friend: [i32; 4],
}

