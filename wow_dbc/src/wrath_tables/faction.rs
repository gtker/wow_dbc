use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::ExtendedLocalizedString;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

        if header.record_size != 228 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 228,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 57 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 57,
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

            // id: primary_key (Faction) int32
            let id = FactionKey::new(crate::util::read_i32_le(chunk)?);

            // reputation_index: int32
            let reputation_index = crate::util::read_i32_le(chunk)?;

            // reputation_race_mask: int32[4]
            let reputation_race_mask = crate::util::read_array_i32::<4>(chunk)?;

            // reputation_class_mask: int32[4]
            let reputation_class_mask = crate::util::read_array_i32::<4>(chunk)?;

            // reputation_base: int32[4]
            let reputation_base = crate::util::read_array_i32::<4>(chunk)?;

            // reputation_flags: int32[4]
            let reputation_flags = crate::util::read_array_i32::<4>(chunk)?;

            // parent_faction_id: foreign_key (Faction) int32
            let parent_faction_id = FactionKey::new(crate::util::read_i32_le(chunk)?.into());

            // parent_faction_mod: float[2]
            let parent_faction_mod = crate::util::read_array_f32::<2>(chunk)?;

            // parent_faction_cap: int32[2]
            let parent_faction_cap = crate::util::read_array_i32::<2>(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(FactionRow {
                id,
                reputation_index,
                reputation_race_mask,
                reputation_class_mask,
                reputation_base,
                reputation_flags,
                parent_faction_id,
                parent_faction_mod,
                parent_faction_cap,
                name_lang,
                description_lang,
            });
        }

        Ok(Faction { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 57,
            record_size: 228,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Faction) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // reputation_index: int32
            b.write_all(&row.reputation_index.to_le_bytes())?;

            // reputation_race_mask: int32[4]
            for i in row.reputation_race_mask {
                b.write_all(&i.to_le_bytes())?;
            }


            // reputation_class_mask: int32[4]
            for i in row.reputation_class_mask {
                b.write_all(&i.to_le_bytes())?;
            }


            // reputation_base: int32[4]
            for i in row.reputation_base {
                b.write_all(&i.to_le_bytes())?;
            }


            // reputation_flags: int32[4]
            for i in row.reputation_flags {
                b.write_all(&i.to_le_bytes())?;
            }


            // parent_faction_id: foreign_key (Faction) int32
            b.write_all(&(row.parent_faction_id.id as i32).to_le_bytes())?;

            // parent_faction_mod: float[2]
            for i in row.parent_faction_mod {
                b.write_all(&i.to_le_bytes())?;
            }


            // parent_faction_cap: int32[2]
            for i in row.parent_faction_cap {
                b.write_all(&i.to_le_bytes())?;
            }


            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_index))?;

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
            row.name_lang.string_block_as_array(b)?;
            row.description_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
            sum += row.description_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct FactionKey {
    pub id: i32
}

impl FactionKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for FactionKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for FactionKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for FactionKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FactionRow {
    pub id: FactionKey,
    pub reputation_index: i32,
    pub reputation_race_mask: [i32; 4],
    pub reputation_class_mask: [i32; 4],
    pub reputation_base: [i32; 4],
    pub reputation_flags: [i32; 4],
    pub parent_faction_id: FactionKey,
    pub parent_faction_mod: [f32; 2],
    pub parent_faction_cap: [i32; 2],
    pub name_lang: ExtendedLocalizedString,
    pub description_lang: ExtendedLocalizedString,
}

