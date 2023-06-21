use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellMissile {
    pub rows: Vec<SpellMissileRow>,
}

impl DbcTable for SpellMissile {
    type Row = SpellMissileRow;

    fn filename() -> &'static str { "SpellMissile.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 60 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 60,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 15 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 15,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SpellMissile) int32
            let id = SpellMissileKey::new(crate::util::read_i32_le(chunk)?);

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // default_pitch_min: float
            let default_pitch_min = crate::util::read_f32_le(chunk)?;

            // default_pitch_max: float
            let default_pitch_max = crate::util::read_f32_le(chunk)?;

            // default_speed_min: float
            let default_speed_min = crate::util::read_f32_le(chunk)?;

            // default_speed_max: float
            let default_speed_max = crate::util::read_f32_le(chunk)?;

            // randomize_facing_min: float
            let randomize_facing_min = crate::util::read_f32_le(chunk)?;

            // randomize_facing_max: float
            let randomize_facing_max = crate::util::read_f32_le(chunk)?;

            // randomize_pitch_min: float
            let randomize_pitch_min = crate::util::read_f32_le(chunk)?;

            // randomize_pitch_max: float
            let randomize_pitch_max = crate::util::read_f32_le(chunk)?;

            // randomize_speed_min: float
            let randomize_speed_min = crate::util::read_f32_le(chunk)?;

            // randomize_speed_max: float
            let randomize_speed_max = crate::util::read_f32_le(chunk)?;

            // gravity: float
            let gravity = crate::util::read_f32_le(chunk)?;

            // max_duration: float
            let max_duration = crate::util::read_f32_le(chunk)?;

            // collision_radius: float
            let collision_radius = crate::util::read_f32_le(chunk)?;


            rows.push(SpellMissileRow {
                id,
                flags,
                default_pitch_min,
                default_pitch_max,
                default_speed_min,
                default_speed_max,
                randomize_facing_min,
                randomize_facing_max,
                randomize_pitch_min,
                randomize_pitch_max,
                randomize_speed_min,
                randomize_speed_max,
                gravity,
                max_duration,
                collision_radius,
            });
        }

        Ok(SpellMissile { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 15,
            record_size: 60,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SpellMissile) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // default_pitch_min: float
            b.write_all(&row.default_pitch_min.to_le_bytes())?;

            // default_pitch_max: float
            b.write_all(&row.default_pitch_max.to_le_bytes())?;

            // default_speed_min: float
            b.write_all(&row.default_speed_min.to_le_bytes())?;

            // default_speed_max: float
            b.write_all(&row.default_speed_max.to_le_bytes())?;

            // randomize_facing_min: float
            b.write_all(&row.randomize_facing_min.to_le_bytes())?;

            // randomize_facing_max: float
            b.write_all(&row.randomize_facing_max.to_le_bytes())?;

            // randomize_pitch_min: float
            b.write_all(&row.randomize_pitch_min.to_le_bytes())?;

            // randomize_pitch_max: float
            b.write_all(&row.randomize_pitch_max.to_le_bytes())?;

            // randomize_speed_min: float
            b.write_all(&row.randomize_speed_min.to_le_bytes())?;

            // randomize_speed_max: float
            b.write_all(&row.randomize_speed_max.to_le_bytes())?;

            // gravity: float
            b.write_all(&row.gravity.to_le_bytes())?;

            // max_duration: float
            b.write_all(&row.max_duration.to_le_bytes())?;

            // collision_radius: float
            b.write_all(&row.collision_radius.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellMissile {
    type PrimaryKey = SpellMissileKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstSpellMissile<const S: usize> {
    pub rows: [SpellMissileRow; S],
}

impl<const S: usize> ConstSpellMissile<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 60 {
            panic!("invalid record size, expected 60")
        }

        if header.field_count != 15 {
            panic!("invalid field count, expected 15")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            SpellMissileRow {
                id: SpellMissileKey::new(0),
                flags: 0,
                default_pitch_min: 0.0,
                default_pitch_max: 0.0,
                default_speed_min: 0.0,
                default_speed_max: 0.0,
                randomize_facing_min: 0.0,
                randomize_facing_max: 0.0,
                randomize_pitch_min: 0.0,
                randomize_pitch_max: 0.0,
                randomize_speed_min: 0.0,
                randomize_speed_max: 0.0,
                gravity: 0.0,
                max_duration: 0.0,
                collision_radius: 0.0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SpellMissile) int32
            let id = SpellMissileKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // default_pitch_min: float
            let default_pitch_min = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // default_pitch_max: float
            let default_pitch_max = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // default_speed_min: float
            let default_speed_min = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // default_speed_max: float
            let default_speed_max = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // randomize_facing_min: float
            let randomize_facing_min = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // randomize_facing_max: float
            let randomize_facing_max = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // randomize_pitch_min: float
            let randomize_pitch_min = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // randomize_pitch_max: float
            let randomize_pitch_max = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // randomize_speed_min: float
            let randomize_speed_min = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // randomize_speed_max: float
            let randomize_speed_max = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // gravity: float
            let gravity = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // max_duration: float
            let max_duration = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // collision_radius: float
            let collision_radius = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = SpellMissileRow {
                id,
                flags,
                default_pitch_min,
                default_pitch_max,
                default_speed_min,
                default_speed_max,
                randomize_facing_min,
                randomize_facing_max,
                randomize_pitch_min,
                randomize_pitch_max,
                randomize_speed_min,
                randomize_speed_max,
                gravity,
                max_duration,
                collision_radius,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SpellMissile {
        SpellMissile {
            rows: self.rows.iter().map(|s| SpellMissileRow {
                id: s.id,
                flags: s.flags,
                default_pitch_min: s.default_pitch_min,
                default_pitch_max: s.default_pitch_max,
                default_speed_min: s.default_speed_min,
                default_speed_max: s.default_speed_max,
                randomize_facing_min: s.randomize_facing_min,
                randomize_facing_max: s.randomize_facing_max,
                randomize_pitch_min: s.randomize_pitch_min,
                randomize_pitch_max: s.randomize_pitch_max,
                randomize_speed_min: s.randomize_speed_min,
                randomize_speed_max: s.randomize_speed_max,
                gravity: s.gravity,
                max_duration: s.max_duration,
                collision_radius: s.collision_radius,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellMissileKey {
    pub id: i32
}

impl SpellMissileKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for SpellMissileKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for SpellMissileKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for SpellMissileKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for SpellMissileKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SpellMissileKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SpellMissileRow {
    pub id: SpellMissileKey,
    pub flags: i32,
    pub default_pitch_min: f32,
    pub default_pitch_max: f32,
    pub default_speed_min: f32,
    pub default_speed_max: f32,
    pub randomize_facing_min: f32,
    pub randomize_facing_max: f32,
    pub randomize_pitch_min: f32,
    pub randomize_pitch_max: f32,
    pub randomize_speed_min: f32,
    pub randomize_speed_max: f32,
    pub gravity: f32,
    pub max_duration: f32,
    pub collision_radius: f32,
}

