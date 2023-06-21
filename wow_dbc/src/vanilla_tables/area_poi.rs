use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::area_table::AreaTableKey;
use crate::vanilla_tables::faction::FactionKey;
use crate::vanilla_tables::map::MapKey;
use crate::vanilla_tables::world_state_ui::WorldStateUIKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AreaPOI {
    pub rows: Vec<AreaPOIRow>,
}

impl DbcTable for AreaPOI {
    type Row = AreaPOIRow;

    fn filename() -> &'static str { "AreaPOI.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 116 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 116,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 29 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 29,
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

            // id: primary_key (AreaPOI) uint32
            let id = AreaPOIKey::new(crate::util::read_u32_le(chunk)?);

            // importance: int32
            let importance = crate::util::read_i32_le(chunk)?;

            // icon: int32
            let icon = crate::util::read_i32_le(chunk)?;

            // faction: foreign_key (Faction) uint32
            let faction = FactionKey::new(crate::util::read_u32_le(chunk)?.into());

            // location_x: float
            let location_x = crate::util::read_f32_le(chunk)?;

            // location_y: float
            let location_y = crate::util::read_f32_le(chunk)?;

            // location_z: float
            let location_z = crate::util::read_f32_le(chunk)?;

            // map: foreign_key (Map) uint32
            let map = MapKey::new(crate::util::read_u32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // area_table: foreign_key (AreaTable) uint32
            let area_table = AreaTableKey::new(crate::util::read_u32_le(chunk)?.into());

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // description: string_ref_loc
            let description = crate::util::read_localized_string(chunk, &string_block)?;

            // world_state: foreign_key (WorldStateUI) uint32
            let world_state = WorldStateUIKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(AreaPOIRow {
                id,
                importance,
                icon,
                faction,
                location_x,
                location_y,
                location_z,
                map,
                flags,
                area_table,
                name,
                description,
                world_state,
            });
        }

        Ok(AreaPOI { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 29,
            record_size: 116,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (AreaPOI) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // importance: int32
            b.write_all(&row.importance.to_le_bytes())?;

            // icon: int32
            b.write_all(&row.icon.to_le_bytes())?;

            // faction: foreign_key (Faction) uint32
            b.write_all(&(row.faction.id as u32).to_le_bytes())?;

            // location_x: float
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: float
            b.write_all(&row.location_y.to_le_bytes())?;

            // location_z: float
            b.write_all(&row.location_z.to_le_bytes())?;

            // map: foreign_key (Map) uint32
            b.write_all(&(row.map.id as u32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // area_table: foreign_key (AreaTable) uint32
            b.write_all(&(row.area_table.id as u32).to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // description: string_ref_loc
            b.write_all(&row.description.string_indices_as_array(&mut string_index))?;

            // world_state: foreign_key (WorldStateUI) uint32
            b.write_all(&(row.world_state.id as u32).to_le_bytes())?;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct AreaPOIKey {
    pub id: u32
}

impl AreaPOIKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
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

impl From<u32> for AreaPOIKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AreaPOIRow {
    pub id: AreaPOIKey,
    pub importance: i32,
    pub icon: i32,
    pub faction: FactionKey,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub map: MapKey,
    pub flags: i32,
    pub area_table: AreaTableKey,
    pub name: LocalizedString,
    pub description: LocalizedString,
    pub world_state: WorldStateUIKey,
}

