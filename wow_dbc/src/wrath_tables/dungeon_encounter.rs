use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::map::MapKey;
use crate::wrath_tables::spell_icon::SpellIconKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DungeonEncounter {
    pub rows: Vec<DungeonEncounterRow>,
}

impl DbcTable for DungeonEncounter {
    type Row = DungeonEncounterRow;

    fn filename() -> &'static str { "DungeonEncounter.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (DungeonEncounter) int32
            let id = DungeonEncounterKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // difficulty: int32
            let difficulty = crate::util::read_i32_le(chunk)?;

            // order_index: int32
            let order_index = crate::util::read_i32_le(chunk)?;

            // bit: int32
            let bit = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // spell_icon_id: foreign_key (SpellIcon) int32
            let spell_icon_id = SpellIconKey::new(crate::util::read_i32_le(chunk)?.into());


            rows.push(DungeonEncounterRow {
                id,
                map_id,
                difficulty,
                order_index,
                bit,
                name_lang,
                spell_icon_id,
            });
        }

        Ok(DungeonEncounter { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 23,
            record_size: 92,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (DungeonEncounter) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // difficulty: int32
            b.write_all(&row.difficulty.to_le_bytes())?;

            // order_index: int32
            b.write_all(&row.order_index.to_le_bytes())?;

            // bit: int32
            b.write_all(&row.bit.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // spell_icon_id: foreign_key (SpellIcon) int32
            b.write_all(&(row.spell_icon_id.id as i32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for DungeonEncounter {
    type PrimaryKey = DungeonEncounterKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl DungeonEncounter {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct DungeonEncounterKey {
    pub id: i32
}

impl DungeonEncounterKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for DungeonEncounterKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for DungeonEncounterKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for DungeonEncounterKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for DungeonEncounterKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for DungeonEncounterKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DungeonEncounterRow {
    pub id: DungeonEncounterKey,
    pub map_id: MapKey,
    pub difficulty: i32,
    pub order_index: i32,
    pub bit: i32,
    pub name_lang: ExtendedLocalizedString,
    pub spell_icon_id: SpellIconKey,
}

