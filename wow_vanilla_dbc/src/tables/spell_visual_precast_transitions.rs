use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq)]
pub struct SpellVisualPrecastTransitions {
    pub rows: Vec<SpellVisualPrecastTransitionsRow>,
}

impl DbcTable for SpellVisualPrecastTransitions {
    type Row = SpellVisualPrecastTransitionsRow;

    fn filename() -> &'static str { "SpellVisualPrecastTransitions.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 12 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 12,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 3 {
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

            // id: primary_key (SpellVisualPrecastTransitions) uint32
            let id = SpellVisualPrecastTransitionsKey::new(crate::util::read_u32_le(chunk)?);

            // load_animation: string_ref
            let load_animation = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // hold_animation: string_ref
            let hold_animation = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(SpellVisualPrecastTransitionsRow {
                id,
                load_animation,
                hold_animation,
            });
        }

        Ok(SpellVisualPrecastTransitions { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 3,
            record_size: 12,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SpellVisualPrecastTransitions) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // load_animation: string_ref
            if !row.load_animation.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.load_animation.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // hold_animation: string_ref
            if !row.hold_animation.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.hold_animation.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SpellVisualPrecastTransitions {
    type PrimaryKey = SpellVisualPrecastTransitionsKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl SpellVisualPrecastTransitions {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.load_animation.is_empty() { b.write_all(row.load_animation.as_bytes())?; b.write_all(&[0])?; };
            if !row.hold_animation.is_empty() { b.write_all(row.hold_animation.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.load_animation.is_empty() { sum += row.load_animation.len() + 1; };
            if !row.hold_animation.is_empty() { sum += row.hold_animation.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct SpellVisualPrecastTransitionsKey {
    pub id: u32
}

impl SpellVisualPrecastTransitionsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct SpellVisualPrecastTransitionsRow {
    pub id: SpellVisualPrecastTransitionsKey,
    pub load_animation: String,
    pub hold_animation: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spell_visual_precast_transitions() {
        let contents = include_bytes!("../../../dbc/SpellVisualPrecastTransitions.dbc");
        let actual = SpellVisualPrecastTransitions::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = SpellVisualPrecastTransitions::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
