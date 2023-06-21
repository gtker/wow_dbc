use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};
use crate::vanilla_tables::map::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TaxiNodes {
    pub rows: Vec<TaxiNodesRow>,
}

impl DbcTable for TaxiNodes {
    type Row = TaxiNodesRow;

    fn filename() -> &'static str { "TaxiNodes.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 64 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 64,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 16,
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

            // id: primary_key (TaxiNodes) uint32
            let id = TaxiNodesKey::new(crate::util::read_u32_le(chunk)?);

            // map: foreign_key (Map) uint32
            let map = MapKey::new(crate::util::read_u32_le(chunk)?.into());

            // location_x: float
            let location_x = crate::util::read_f32_le(chunk)?;

            // location_y: float
            let location_y = crate::util::read_f32_le(chunk)?;

            // location_z: float
            let location_z = crate::util::read_f32_le(chunk)?;

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // mount_creature_display_info: uint32[2]
            let mount_creature_display_info = crate::util::read_array_u32::<2>(chunk)?;


            rows.push(TaxiNodesRow {
                id,
                map,
                location_x,
                location_y,
                location_z,
                name,
                mount_creature_display_info,
            });
        }

        Ok(TaxiNodes { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 16,
            record_size: 64,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (TaxiNodes) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // map: foreign_key (Map) uint32
            b.write_all(&(row.map.id as u32).to_le_bytes())?;

            // location_x: float
            b.write_all(&row.location_x.to_le_bytes())?;

            // location_y: float
            b.write_all(&row.location_y.to_le_bytes())?;

            // location_z: float
            b.write_all(&row.location_z.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // mount_creature_display_info: uint32[2]
            for i in row.mount_creature_display_info {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for TaxiNodes {
    type PrimaryKey = TaxiNodesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl TaxiNodes {
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstTaxiNodes<const S: usize> {
    pub rows: [ConstTaxiNodesRow; S],
}

impl<const S: usize> ConstTaxiNodes<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 64 {
            panic!("invalid record size, expected 64")
        }

        if header.field_count != 16 {
            panic!("invalid field count, expected 16")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstTaxiNodesRow {
                id: TaxiNodesKey::new(0),
                map: MapKey::new(0),
                location_x: 0.0,
                location_y: 0.0,
                location_z: 0.0,
                name: crate::ConstLocalizedString::empty(),
                mount_creature_display_info: [0; 2],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (TaxiNodes) uint32
            let id = TaxiNodesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // map: foreign_key (Map) uint32
            let map = MapKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
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

            // mount_creature_display_info: uint32[2]
            let mount_creature_display_info = {
                let mut a = [0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = ConstTaxiNodesRow {
                id,
                map,
                location_x,
                location_y,
                location_z,
                name,
                mount_creature_display_info,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> TaxiNodes {
        TaxiNodes {
            rows: self.rows.iter().map(|s| TaxiNodesRow {
                id: s.id,
                map: s.map,
                location_x: s.location_x,
                location_y: s.location_y,
                location_z: s.location_z,
                name: s.name.to_string(),
                mount_creature_display_info: s.mount_creature_display_info,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct TaxiNodesKey {
    pub id: u32
}

impl TaxiNodesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for TaxiNodesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for TaxiNodesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for TaxiNodesKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TaxiNodesRow {
    pub id: TaxiNodesKey,
    pub map: MapKey,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub name: LocalizedString,
    pub mount_creature_display_info: [u32; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstTaxiNodesRow {
    pub id: TaxiNodesKey,
    pub map: MapKey,
    pub location_x: f32,
    pub location_y: f32,
    pub location_z: f32,
    pub name: ConstLocalizedString,
    pub mount_creature_display_info: [u32; 2],
}

