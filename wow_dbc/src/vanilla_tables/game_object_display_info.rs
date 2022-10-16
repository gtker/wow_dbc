use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameObjectDisplayInfo {
    pub rows: Vec<GameObjectDisplayInfoRow>,
}

impl DbcTable for GameObjectDisplayInfo {
    type Row = GameObjectDisplayInfoRow;

    fn filename() -> &'static str { "GameObjectDisplayInfo.dbc" }

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

            // id: primary_key (GameObjectDisplayInfo) uint32
            let id = GameObjectDisplayInfoKey::new(crate::util::read_u32_le(chunk)?);

            // model_name: string_ref
            let model_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // sound_entry: uint32[10]
            let sound_entry = crate::util::read_array_u32::<10>(chunk)?;


            rows.push(GameObjectDisplayInfoRow {
                id,
                model_name,
                sound_entry,
            });
        }

        Ok(GameObjectDisplayInfo { rows, })
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
            // id: primary_key (GameObjectDisplayInfo) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model_name: string_ref
            if !row.model_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.model_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // sound_entry: uint32[10]
            for i in row.sound_entry {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for GameObjectDisplayInfo {
    type PrimaryKey = GameObjectDisplayInfoKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl GameObjectDisplayInfo {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.model_name.is_empty() { b.write_all(row.model_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.model_name.is_empty() { sum += row.model_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct GameObjectDisplayInfoKey {
    pub id: u32
}

impl GameObjectDisplayInfoKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameObjectDisplayInfoRow {
    pub id: GameObjectDisplayInfoKey,
    pub model_name: String,
    pub sound_entry: [u32; 10],
}

