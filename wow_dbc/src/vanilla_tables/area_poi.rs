use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};
use crate::vanilla_tables::area_table::*;
use crate::vanilla_tables::faction::*;
use crate::vanilla_tables::map::*;
use crate::vanilla_tables::world_state_ui::*;

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
        let header = header::parse_header(&header)?;

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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstAreaPOI<const S: usize> {
    pub rows: [ConstAreaPOIRow; S],
}

impl<const S: usize> ConstAreaPOI<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 116 {
            panic!("invalid record size, expected 116")
        }

        if header.field_count != 29 {
            panic!("invalid field count, expected 29")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstAreaPOIRow {
                id: AreaPOIKey::new(0),
                importance: 0,
                icon: 0,
                faction: FactionKey::new(0),
                location_x: 0.0,
                location_y: 0.0,
                location_z: 0.0,
                map: MapKey::new(0),
                flags: 0,
                area_table: AreaTableKey::new(0),
                name: crate::ConstLocalizedString::empty(),
                description: crate::ConstLocalizedString::empty(),
                world_state: WorldStateUIKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (AreaPOI) uint32
            let id = AreaPOIKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // importance: int32
            let importance = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // icon: int32
            let icon = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // faction: foreign_key (Faction) uint32
            let faction = FactionKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // location_x: float
            let location_x = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // location_y: float
            let location_y = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // location_z: float
            let location_z = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // map: foreign_key (Map) uint32
            let map = MapKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // area_table: foreign_key (AreaTable) uint32
            let area_table = AreaTableKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
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

            // world_state: foreign_key (WorldStateUI) uint32
            let world_state = WorldStateUIKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = ConstAreaPOIRow {
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
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstAreaPOIRow {
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
    pub name: ConstLocalizedString,
    pub description: ConstLocalizedString,
    pub world_state: WorldStateUIKey,
}

