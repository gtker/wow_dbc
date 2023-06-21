use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};
use crate::vanilla_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Resistances {
    pub rows: Vec<ResistancesRow>,
}

impl DbcTable for Resistances {
    type Row = ResistancesRow;

    fn filename() -> &'static str { "Resistances.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 48 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 48,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 12 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 12,
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

            // id: primary_key (Resistances) uint32
            let id = ResistancesKey::new(crate::util::read_u32_le(chunk)?);

            // physical_damage: bool32
            let physical_damage = crate::util::read_u32_le(chunk)? != 0;

            // fizzle_sound_entry: foreign_key (SoundEntries) uint32
            let fizzle_sound_entry = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(ResistancesRow {
                id,
                physical_damage,
                fizzle_sound_entry,
                name,
            });
        }

        Ok(Resistances { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 12,
            record_size: 48,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Resistances) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // physical_damage: bool32
            b.write_all(&u32::from(row.physical_damage).to_le_bytes())?;

            // fizzle_sound_entry: foreign_key (SoundEntries) uint32
            b.write_all(&(row.fizzle_sound_entry.id as u32).to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Resistances {
    type PrimaryKey = ResistancesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl Resistances {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstResistances<const S: usize> {
    pub rows: [ConstResistancesRow; S],
}

impl<const S: usize> ConstResistances<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 48 {
            panic!("invalid record size, expected 48")
        }

        if header.field_count != 12 {
            panic!("invalid field count, expected 12")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstResistancesRow {
                id: ResistancesKey::new(0),
                physical_damage: false,
                fizzle_sound_entry: SoundEntriesKey::new(0),
                name: crate::ConstLocalizedString::empty(),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Resistances) uint32
            let id = ResistancesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // physical_damage: bool32
            let physical_damage = if (b[b_offset + 0] | b[b_offset + 1] | b[b_offset + 2] | b[b_offset + 3]) != 0 {true} else {false};
            b_offset += 4;

            // fizzle_sound_entry: foreign_key (SoundEntries) uint32
            let fizzle_sound_entry = SoundEntriesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
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

            rows[i] = ConstResistancesRow {
                id,
                physical_damage,
                fizzle_sound_entry,
                name,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> Resistances {
        Resistances {
            rows: self.rows.iter().map(|s| ResistancesRow {
                id: s.id,
                physical_damage: s.physical_damage,
                fizzle_sound_entry: s.fizzle_sound_entry,
                name: s.name.to_string(),
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ResistancesKey {
    pub id: u32
}

impl ResistancesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for ResistancesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for ResistancesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for ResistancesKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResistancesRow {
    pub id: ResistancesKey,
    pub physical_damage: bool,
    pub fizzle_sound_entry: SoundEntriesKey,
    pub name: LocalizedString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstResistancesRow {
    pub id: ResistancesKey,
    pub physical_damage: bool,
    pub fizzle_sound_entry: SoundEntriesKey,
    pub name: ConstLocalizedString,
}

