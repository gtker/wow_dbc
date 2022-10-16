use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::ExtendedLocalizedString;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BattlemasterList {
    pub rows: Vec<BattlemasterListRow>,
}

impl DbcTable for BattlemasterList {
    type Row = BattlemasterListRow;

    fn filename() -> &'static str { "BattlemasterList.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 128 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 128,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 32,
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

            // id: primary_key (BattlemasterList) int32
            let id = BattlemasterListKey::new(crate::util::read_i32_le(chunk)?);

            // map_id: int32[8]
            let map_id = crate::util::read_array_i32::<8>(chunk)?;

            // instance_type: int32
            let instance_type = crate::util::read_i32_le(chunk)?;

            // groups_allowed: int32
            let groups_allowed = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // max_group_size: int32
            let max_group_size = crate::util::read_i32_le(chunk)?;

            // holiday_world_state: foreign_key (WorldState) int32
            let holiday_world_state = crate::util::read_i32_le(chunk)?;

            // min_level: int32
            let min_level = crate::util::read_i32_le(chunk)?;

            // max_level: int32
            let max_level = crate::util::read_i32_le(chunk)?;


            rows.push(BattlemasterListRow {
                id,
                map_id,
                instance_type,
                groups_allowed,
                name_lang,
                max_group_size,
                holiday_world_state,
                min_level,
                max_level,
            });
        }

        Ok(BattlemasterList { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 32,
            record_size: 128,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (BattlemasterList) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map_id: int32[8]
            for i in row.map_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // instance_type: int32
            b.write_all(&row.instance_type.to_le_bytes())?;

            // groups_allowed: int32
            b.write_all(&row.groups_allowed.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // max_group_size: int32
            b.write_all(&row.max_group_size.to_le_bytes())?;

            // holiday_world_state: foreign_key (WorldState) int32
            b.write_all(&row.holiday_world_state.to_le_bytes())?;

            // min_level: int32
            b.write_all(&row.min_level.to_le_bytes())?;

            // max_level: int32
            b.write_all(&row.max_level.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for BattlemasterList {
    type PrimaryKey = BattlemasterListKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl BattlemasterList {
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
pub struct BattlemasterListKey {
    pub id: i32
}

impl BattlemasterListKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i32> for BattlemasterListKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BattlemasterListRow {
    pub id: BattlemasterListKey,
    pub map_id: [i32; 8],
    pub instance_type: i32,
    pub groups_allowed: i32,
    pub name_lang: ExtendedLocalizedString,
    pub max_group_size: i32,
    pub holiday_world_state: i32,
    pub min_level: i32,
    pub max_level: i32,
}

