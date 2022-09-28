use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, Eq)]
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

            // id: primary_key (WeaponImpactSounds) int32
            let id = WeaponImpactSoundsKey::new(crate::util::read_i32_le(chunk)?);

            // weapon_sub_class_id: int32
            let weapon_sub_class_id = crate::util::read_i32_le(chunk)?;

            // parry_sound_type: int32
            let parry_sound_type = crate::util::read_i32_le(chunk)?;

            // impact_sound_id: int32[10]
            let impact_sound_id = crate::util::read_array_i32::<10>(chunk)?;

            // crit_impact_sound_id: int32[10]
            let crit_impact_sound_id = crate::util::read_array_i32::<10>(chunk)?;


            rows.push(WeaponImpactSoundsRow {
                id,
                weapon_sub_class_id,
                parry_sound_type,
                impact_sound_id,
                crit_impact_sound_id,
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
            // id: primary_key (WeaponImpactSounds) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // weapon_sub_class_id: int32
            b.write_all(&row.weapon_sub_class_id.to_le_bytes())?;

            // parry_sound_type: int32
            b.write_all(&row.parry_sound_type.to_le_bytes())?;

            // impact_sound_id: int32[10]
            for i in row.impact_sound_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // crit_impact_sound_id: int32[10]
            for i in row.crit_impact_sound_id {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for WeaponImpactSounds {
    type PrimaryKey = WeaponImpactSoundsKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct WeaponImpactSoundsKey {
    pub id: i32
}

impl WeaponImpactSoundsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaponImpactSoundsRow {
    pub id: WeaponImpactSoundsKey,
    pub weapon_sub_class_id: i32,
    pub parry_sound_type: i32,
    pub impact_sound_id: [i32; 10],
    pub crit_impact_sound_id: [i32; 10],
}

