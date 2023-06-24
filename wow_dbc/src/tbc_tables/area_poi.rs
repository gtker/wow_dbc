use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::area_table::AreaTableKey;
use crate::tbc_tables::faction_template::FactionTemplateKey;
use crate::tbc_tables::map::MapKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AreaPOI {
    pub rows: Vec<AreaPOIRow>,
}

impl DbcTable for AreaPOI {
    type Row = AreaPOIRow;

    const FILENAME: &'static str = "AreaPOI.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 180 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 180,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 45 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 45,
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

            // id: primary_key (AreaPOI) int32
            let id = AreaPOIKey::new(crate::util::read_i32_le(chunk)?);

            // importance: int32
            let importance = crate::util::read_i32_le(chunk)?;

            // icon: int32
            let icon = crate::util::read_i32_le(chunk)?;

            // faction_id: foreign_key (FactionTemplate) int32
            let faction_id = FactionTemplateKey::new(crate::util::read_i32_le(chunk)?.into());

            // pos: float[3]
            let pos = crate::util::read_array_f32::<3>(chunk)?;

            // continent_id: foreign_key (Map) int32
            let continent_id = MapKey::new(crate::util::read_i32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // area_id: foreign_key (AreaTable) int32
            let area_id = AreaTableKey::new(crate::util::read_i32_le(chunk)?.into());

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // world_state_id: foreign_key (WorldState) int32
            let world_state_id = crate::util::read_i32_le(chunk)?;


            rows.push(AreaPOIRow {
                id,
                importance,
                icon,
                faction_id,
                pos,
                continent_id,
                flags,
                area_id,
                name_lang,
                description_lang,
                world_state_id,
            });
        }

        Ok(AreaPOI { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 45,
            record_size: 180,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (AreaPOI) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // importance: int32
            b.write_all(&row.importance.to_le_bytes())?;

            // icon: int32
            b.write_all(&row.icon.to_le_bytes())?;

            // faction_id: foreign_key (FactionTemplate) int32
            b.write_all(&(row.faction_id.id as i32).to_le_bytes())?;

            // pos: float[3]
            for i in row.pos {
                b.write_all(&i.to_le_bytes())?;
            }


            // continent_id: foreign_key (Map) int32
            b.write_all(&(row.continent_id.id as i32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // area_id: foreign_key (AreaTable) int32
            b.write_all(&(row.area_id.id as i32).to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_index))?;

            // world_state_id: foreign_key (WorldState) int32
            b.write_all(&row.world_state_id.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for AreaPOI {
    type PrimaryKey = AreaPOIKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl AreaPOI {
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
pub struct AreaPOIKey {
    pub id: i32
}

impl AreaPOIKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for AreaPOIKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for AreaPOIKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for AreaPOIKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl From<u8> for AreaPOIKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for AreaPOIKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl TryFrom<u32> for AreaPOIKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AreaPOIRow {
    pub id: AreaPOIKey,
    pub importance: i32,
    pub icon: i32,
    pub faction_id: FactionTemplateKey,
    pub pos: [f32; 3],
    pub continent_id: MapKey,
    pub flags: i32,
    pub area_id: AreaTableKey,
    pub name_lang: ExtendedLocalizedString,
    pub description_lang: ExtendedLocalizedString,
    pub world_state_id: i32,
}

