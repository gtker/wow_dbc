use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WeaponImpactSounds {
    pub rows: Vec<WeaponImpactSoundsRow>,
}

impl DbcTable for WeaponImpactSounds {
    type Row = WeaponImpactSoundsRow;

    fn filename() -> &'static str { "WeaponImpactSounds.dbc" }

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

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (WeaponImpactSounds) uint32
            let id = WeaponImpactSoundsKey::new(crate::util::read_u32_le(chunk)?);

            // weapon_subclass: int32
            let weapon_subclass = crate::util::read_i32_le(chunk)?;

            // parry_sound_type: int32
            let parry_sound_type = crate::util::read_i32_le(chunk)?;

            // impact_sound: uint32[10]
            let impact_sound = crate::util::read_array_u32::<10>(chunk)?;

            // crit_impact_sound: uint32[10]
            let crit_impact_sound = crate::util::read_array_u32::<10>(chunk)?;


            rows.push(WeaponImpactSoundsRow {
                id,
                weapon_subclass,
                parry_sound_type,
                impact_sound,
                crit_impact_sound,
            });
        }

        Ok(WeaponImpactSounds { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 23,
            record_size: 92,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (WeaponImpactSounds) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // weapon_subclass: int32
            b.write_all(&row.weapon_subclass.to_le_bytes())?;

            // parry_sound_type: int32
            b.write_all(&row.parry_sound_type.to_le_bytes())?;

            // impact_sound: uint32[10]
            for i in row.impact_sound {
                b.write_all(&i.to_le_bytes())?;
            }


            // crit_impact_sound: uint32[10]
            for i in row.crit_impact_sound {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for WeaponImpactSounds {
    type PrimaryKey = WeaponImpactSoundsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstWeaponImpactSounds<const S: usize> {
    pub rows: [WeaponImpactSoundsRow; S],
}

impl<const S: usize> ConstWeaponImpactSounds<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 92 {
            panic!("invalid record size, expected 92")
        }

        if header.field_count != 23 {
            panic!("invalid field count, expected 23")
        }

        let mut b_offset = 20;
        let mut rows = [
            WeaponImpactSoundsRow {
                id: WeaponImpactSoundsKey::new(0),
                weapon_subclass: 0,
                parry_sound_type: 0,
                impact_sound: [0; 10],
                crit_impact_sound: [0; 10],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (WeaponImpactSounds) uint32
            let id = WeaponImpactSoundsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // weapon_subclass: int32
            let weapon_subclass = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // parry_sound_type: int32
            let parry_sound_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // impact_sound: uint32[10]
            let impact_sound = {
                let mut a = [0; 10];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // crit_impact_sound: uint32[10]
            let crit_impact_sound = {
                let mut a = [0; 10];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = WeaponImpactSoundsRow {
                id,
                weapon_subclass,
                parry_sound_type,
                impact_sound,
                crit_impact_sound,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> WeaponImpactSounds {
        WeaponImpactSounds {
            rows: self.rows.iter().map(|s| WeaponImpactSoundsRow {
                id: s.id,
                weapon_subclass: s.weapon_subclass,
                parry_sound_type: s.parry_sound_type,
                impact_sound: s.impact_sound,
                crit_impact_sound: s.crit_impact_sound,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct WeaponImpactSoundsKey {
    pub id: u32
}

impl WeaponImpactSoundsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for WeaponImpactSoundsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for WeaponImpactSoundsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for WeaponImpactSoundsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WeaponImpactSoundsRow {
    pub id: WeaponImpactSoundsKey,
    pub weapon_subclass: i32,
    pub parry_sound_type: i32,
    pub impact_sound: [u32; 10],
    pub crit_impact_sound: [u32; 10],
}

