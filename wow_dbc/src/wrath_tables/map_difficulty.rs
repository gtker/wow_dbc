use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};
use crate::wrath_tables::map::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapDifficulty {
    pub rows: Vec<MapDifficultyRow>,
}

impl DbcTable for MapDifficulty {
    type Row = MapDifficultyRow;

    fn filename() -> &'static str { "MapDifficulty.dbc" }

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
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (MapDifficulty) int32
            let id = MapDifficultyKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // difficulty: int32
            let difficulty = crate::util::read_i32_le(chunk)?;

            // message_lang: string_ref_loc (Extended)
            let message_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // raid_duration: int32
            let raid_duration = crate::util::read_i32_le(chunk)?;

            // max_players: int32
            let max_players = crate::util::read_i32_le(chunk)?;

            // difficultystring: string_ref
            let difficultystring = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(MapDifficultyRow {
                id,
                map_id,
                difficulty,
                message_lang,
                raid_duration,
                max_players,
                difficultystring,
            });
        }

        Ok(MapDifficulty { rows, })
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
            // id: primary_key (MapDifficulty) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: foreign_key (Map) int32
            b.write_all(&(row.map_id.id as i32).to_le_bytes())?;

            // difficulty: int32
            b.write_all(&row.difficulty.to_le_bytes())?;

            // message_lang: string_ref_loc (Extended)
            b.write_all(&row.message_lang.string_indices_as_array(&mut string_index))?;

            // raid_duration: int32
            b.write_all(&row.raid_duration.to_le_bytes())?;

            // max_players: int32
            b.write_all(&row.max_players.to_le_bytes())?;

            // difficultystring: string_ref
            if !row.difficultystring.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.difficultystring.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for MapDifficulty {
    type PrimaryKey = MapDifficultyKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl MapDifficulty {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.message_lang.string_block_as_array(b)?;
            if !row.difficultystring.is_empty() { b.write_all(row.difficultystring.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.message_lang.string_block_size();
            if !row.difficultystring.is_empty() { sum += row.difficultystring.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstMapDifficulty<const S: usize> {
    pub rows: [ConstMapDifficultyRow; S],
}

impl<const S: usize> ConstMapDifficulty<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 92 {
            panic!("invalid record size, expected 92")
        }

        if header.field_count != 23 {
            panic!("invalid field count, expected 23")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstMapDifficultyRow {
                id: MapDifficultyKey::new(0),
                map_id: MapKey::new(0),
                difficulty: 0,
                message_lang: crate::ConstExtendedLocalizedString::empty(),
                raid_duration: 0,
                max_players: 0,
                difficultystring: "",
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (MapDifficulty) int32
            let id = MapDifficultyKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // map_id: foreign_key (Map) int32
            let map_id = MapKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // difficulty: int32
            let difficulty = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // message_lang: string_ref_loc (Extended)
            let message_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // raid_duration: int32
            let raid_duration = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // max_players: int32
            let max_players = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // difficultystring: string_ref
            let difficultystring = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            rows[i] = ConstMapDifficultyRow {
                id,
                map_id,
                difficulty,
                message_lang,
                raid_duration,
                max_players,
                difficultystring,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> MapDifficulty {
        MapDifficulty {
            rows: self.rows.iter().map(|s| MapDifficultyRow {
                id: s.id,
                map_id: s.map_id,
                difficulty: s.difficulty,
                message_lang: s.message_lang.to_string(),
                raid_duration: s.raid_duration,
                max_players: s.max_players,
                difficultystring: s.difficultystring.to_string(),
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct MapDifficultyKey {
    pub id: i32
}

impl MapDifficultyKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for MapDifficultyKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for MapDifficultyKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for MapDifficultyKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for MapDifficultyKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for MapDifficultyKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapDifficultyRow {
    pub id: MapDifficultyKey,
    pub map_id: MapKey,
    pub difficulty: i32,
    pub message_lang: ExtendedLocalizedString,
    pub raid_duration: i32,
    pub max_players: i32,
    pub difficultystring: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstMapDifficultyRow {
    pub id: MapDifficultyKey,
    pub map_id: MapKey,
    pub difficulty: i32,
    pub message_lang: ConstExtendedLocalizedString,
    pub raid_duration: i32,
    pub max_players: i32,
    pub difficultystring: &'static str,
}

