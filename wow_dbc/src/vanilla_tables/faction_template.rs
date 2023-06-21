use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::faction::*;
use crate::vanilla_tables::faction_group::*;

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

            // id: primary_key (FactionTemplate) uint32
            let id = FactionTemplateKey::new(crate::util::read_u32_le(chunk)?);

            // faction: foreign_key (Faction) uint32
            let faction = FactionKey::new(crate::util::read_u32_le(chunk)?.into());

            // flags: Flags
            let flags = Flags::new(crate::util::read_u32_le(chunk)?);

            // faction_group: foreign_key (FactionGroup) uint32
            let faction_group = FactionGroupKey::new(crate::util::read_u32_le(chunk)?.into());

            // friend_group: foreign_key (FactionGroup) uint32
            let friend_group = FactionGroupKey::new(crate::util::read_u32_le(chunk)?.into());

            // enemy_group: foreign_key (FactionGroup) uint32
            let enemy_group = FactionGroupKey::new(crate::util::read_u32_le(chunk)?.into());

            // enemies: uint32[4]
            let enemies = crate::util::read_array_u32::<4>(chunk)?;

            // friends: uint32[4]
            let friends = crate::util::read_array_u32::<4>(chunk)?;


            rows.push(FactionTemplateRow {
                id,
                faction,
                flags,
                faction_group,
                friend_group,
                enemy_group,
                enemies,
                friends,
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
            // id: primary_key (FactionTemplate) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // faction: foreign_key (Faction) uint32
            b.write_all(&(row.faction.id as u32).to_le_bytes())?;

            // flags: Flags
            b.write_all(&(row.flags.as_int() as u32).to_le_bytes())?;

            // faction_group: foreign_key (FactionGroup) uint32
            b.write_all(&(row.faction_group.id as u32).to_le_bytes())?;

            // friend_group: foreign_key (FactionGroup) uint32
            b.write_all(&(row.friend_group.id as u32).to_le_bytes())?;

            // enemy_group: foreign_key (FactionGroup) uint32
            b.write_all(&(row.enemy_group.id as u32).to_le_bytes())?;

            // enemies: uint32[4]
            for i in row.enemies {
                b.write_all(&i.to_le_bytes())?;
            }


            // friends: uint32[4]
            for i in row.friends {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct FactionTemplateKey {
    pub id: u32
}

impl FactionTemplateKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
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

impl From<u32> for FactionTemplateKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Flags {
    value: u32,
}

impl Flags {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }

    pub const fn as_int(&self) -> u32 {
        self.value
    }

    pub const fn pvp_flagged(&self) -> bool {
        (self.value & 2048) != 0
    }

    pub const fn attack_pvping_players(&self) -> bool {
        (self.value & 4096) != 0
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactionTemplateRow {
    pub id: FactionTemplateKey,
    pub faction: FactionKey,
    pub flags: Flags,
    pub faction_group: FactionGroupKey,
    pub friend_group: FactionGroupKey,
    pub enemy_group: FactionGroupKey,
    pub enemies: [u32; 4],
    pub friends: [u32; 4],
}

