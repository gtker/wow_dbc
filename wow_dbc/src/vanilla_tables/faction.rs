use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Faction {
    pub rows: Vec<FactionRow>,
}

impl DbcTable for Faction {
    type Row = FactionRow;

    fn filename() -> &'static str { "Faction.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 148 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 148,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 37 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 37,
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

            // id: primary_key (Faction) uint32
            let id = FactionKey::new(crate::util::read_u32_le(chunk)?);

            // reputation_index: uint32
            let reputation_index = crate::util::read_u32_le(chunk)?;

            // reputation_race_mask: ReputationRaceMask[4]
            let reputation_race_mask = {
                let mut arr = [ReputationRaceMask::default(); 4];
                for i in arr.iter_mut() {
                    *i = ReputationRaceMask::new(crate::util::read_i32_le(chunk)?);
                }

                arr
            };

            // reputation_class_mask: uint32[4]
            let reputation_class_mask = crate::util::read_array_u32::<4>(chunk)?;

            // reputation_base: uint32[4]
            let reputation_base = crate::util::read_array_u32::<4>(chunk)?;

            // reputation_flags: ReputationFlags[4]
            let reputation_flags = {
                let mut arr = [ReputationFlags::default(); 4];
                for i in arr.iter_mut() {
                    *i = ReputationFlags::new(crate::util::read_i32_le(chunk)?);
                }

                arr
            };

            // parent_faction: foreign_key (Faction) uint32
            let parent_faction = FactionKey::new(crate::util::read_u32_le(chunk)?.into());

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // description: string_ref_loc
            let description = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(FactionRow {
                id,
                reputation_index,
                reputation_race_mask,
                reputation_class_mask,
                reputation_base,
                reputation_flags,
                parent_faction,
                name,
                description,
            });
        }

        Ok(Faction { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 37,
            record_size: 148,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Faction) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // reputation_index: uint32
            b.write_all(&row.reputation_index.to_le_bytes())?;

            // reputation_race_mask: ReputationRaceMask[4]
            for i in row.reputation_race_mask {
                b.write_all(&(i.as_int() as i32).to_le_bytes())?;
            }


            // reputation_class_mask: uint32[4]
            for i in row.reputation_class_mask {
                b.write_all(&i.to_le_bytes())?;
            }


            // reputation_base: uint32[4]
            for i in row.reputation_base {
                b.write_all(&i.to_le_bytes())?;
            }


            // reputation_flags: ReputationFlags[4]
            for i in row.reputation_flags {
                b.write_all(&(i.as_int() as i32).to_le_bytes())?;
            }


            // parent_faction: foreign_key (Faction) uint32
            b.write_all(&(row.parent_faction.id as u32).to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // description: string_ref_loc
            b.write_all(&row.description.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Faction {
    type PrimaryKey = FactionKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl Faction {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name.string_block_as_array(b)?;
            row.description.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
            sum += row.description.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstFaction<const S: usize> {
    pub rows: [ConstFactionRow; S],
}

impl<const S: usize> ConstFaction<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 148 {
            panic!("invalid record size, expected 148")
        }

        if header.field_count != 37 {
            panic!("invalid field count, expected 37")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstFactionRow {
                id: FactionKey::new(0),
                reputation_index: 0,
                reputation_race_mask: [ReputationRaceMask::new(0); 4],
                reputation_class_mask: [0; 4],
                reputation_base: [0; 4],
                reputation_flags: [ReputationFlags::new(0); 4],
                parent_faction: FactionKey::new(0),
                name: crate::ConstLocalizedString::empty(),
                description: crate::ConstLocalizedString::empty(),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Faction) uint32
            let id = FactionKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // reputation_index: uint32
            let reputation_index = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // reputation_race_mask: ReputationRaceMask[4]
            let reputation_race_mask = {
                let mut a = [ReputationRaceMask::new(0); 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = ReputationRaceMask::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // reputation_class_mask: uint32[4]
            let reputation_class_mask = {
                let mut a = [0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // reputation_base: uint32[4]
            let reputation_base = {
                let mut a = [0; 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // reputation_flags: ReputationFlags[4]
            let reputation_flags = {
                let mut a = [ReputationFlags::new(0); 4];
                let mut i = 0;
                while i < a.len() {
                    a[i] = ReputationFlags::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // parent_faction: foreign_key (Faction) uint32
            let parent_faction = FactionKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // name: string_ref_loc
            let name = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            // description: string_ref_loc
            let description = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            rows[i] = ConstFactionRow {
                id,
                reputation_index,
                reputation_race_mask,
                reputation_class_mask,
                reputation_base,
                reputation_flags,
                parent_faction,
                name,
                description,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct FactionKey {
    pub id: u32
}

impl FactionKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for FactionKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for FactionKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for FactionKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct ReputationRaceMask {
    value: i32,
}

impl ReputationRaceMask {
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    pub const fn as_int(&self) -> i32 {
        self.value
    }

    pub const fn human(&self) -> bool {
        (self.value & 1) != 0
    }

    pub const fn orc(&self) -> bool {
        (self.value & 2) != 0
    }

    pub const fn dwarf(&self) -> bool {
        (self.value & 4) != 0
    }

    pub const fn night_elf(&self) -> bool {
        (self.value & 8) != 0
    }

    pub const fn undead(&self) -> bool {
        (self.value & 16) != 0
    }

    pub const fn tauren(&self) -> bool {
        (self.value & 32) != 0
    }

    pub const fn gnome(&self) -> bool {
        (self.value & 64) != 0
    }

    pub const fn troll(&self) -> bool {
        (self.value & 128) != 0
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct ReputationFlags {
    value: i32,
}

impl ReputationFlags {
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    pub const fn as_int(&self) -> i32 {
        self.value
    }

    pub const fn visible_to_client(&self) -> bool {
        (self.value & 1) != 0
    }

    pub const fn enable_at_war(&self) -> bool {
        (self.value & 2) != 0
    }

    pub const fn hide_in_client(&self) -> bool {
        (self.value & 4) != 0
    }

    pub const fn force_hide_in_client(&self) -> bool {
        (self.value & 8) != 0
    }

    pub const fn force_at_peace(&self) -> bool {
        (self.value & 16) != 0
    }

    pub const fn faction_inactive(&self) -> bool {
        (self.value & 32) != 0
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactionRow {
    pub id: FactionKey,
    pub reputation_index: u32,
    pub reputation_race_mask: [ReputationRaceMask; 4],
    pub reputation_class_mask: [u32; 4],
    pub reputation_base: [u32; 4],
    pub reputation_flags: [ReputationFlags; 4],
    pub parent_faction: FactionKey,
    pub name: LocalizedString,
    pub description: LocalizedString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstFactionRow {
    pub id: FactionKey,
    pub reputation_index: u32,
    pub reputation_race_mask: [ReputationRaceMask; 4],
    pub reputation_class_mask: [u32; 4],
    pub reputation_base: [u32; 4],
    pub reputation_flags: [ReputationFlags; 4],
    pub parent_faction: FactionKey,
    pub name: ConstLocalizedString,
    pub description: ConstLocalizedString,
}

