use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScalingStatValues {
    pub rows: Vec<ScalingStatValuesRow>,
}

impl DbcTable for ScalingStatValues {
    type Row = ScalingStatValuesRow;

    const FILENAME: &'static str = "ScalingStatValues.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 96 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 96,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 24 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 24,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (ScalingStatValues) int32
            let id = ScalingStatValuesKey::new(crate::util::read_i32_le(chunk)?);

            // charlevel: int32
            let charlevel = crate::util::read_i32_le(chunk)?;

            // shoulder_budget: int32
            let shoulder_budget = crate::util::read_i32_le(chunk)?;

            // trinket_budget: int32
            let trinket_budget = crate::util::read_i32_le(chunk)?;

            // weapon_budget1_h: int32
            let weapon_budget1_h = crate::util::read_i32_le(chunk)?;

            // ranged_budget: int32
            let ranged_budget = crate::util::read_i32_le(chunk)?;

            // cloth_shoulder_armor: int32
            let cloth_shoulder_armor = crate::util::read_i32_le(chunk)?;

            // leather_shoulder_armor: int32
            let leather_shoulder_armor = crate::util::read_i32_le(chunk)?;

            // mail_shoulder_armor: int32
            let mail_shoulder_armor = crate::util::read_i32_le(chunk)?;

            // plate_shoulder_armor: int32
            let plate_shoulder_armor = crate::util::read_i32_le(chunk)?;

            // weapon_d_p_s1_h: int32
            let weapon_d_p_s1_h = crate::util::read_i32_le(chunk)?;

            // weapon_d_p_s2_h: int32
            let weapon_d_p_s2_h = crate::util::read_i32_le(chunk)?;

            // spellcaster_d_p_s1_h: int32
            let spellcaster_d_p_s1_h = crate::util::read_i32_le(chunk)?;

            // spellcaster_d_p_s2_h: int32
            let spellcaster_d_p_s2_h = crate::util::read_i32_le(chunk)?;

            // ranged_d_p_s: int32
            let ranged_d_p_s = crate::util::read_i32_le(chunk)?;

            // wand_d_p_s: int32
            let wand_d_p_s = crate::util::read_i32_le(chunk)?;

            // spell_power: int32
            let spell_power = crate::util::read_i32_le(chunk)?;

            // primary_budget: int32
            let primary_budget = crate::util::read_i32_le(chunk)?;

            // tertiary_budget: int32
            let tertiary_budget = crate::util::read_i32_le(chunk)?;

            // cloth_cloak_armor: int32
            let cloth_cloak_armor = crate::util::read_i32_le(chunk)?;

            // cloth_chest_armor: int32
            let cloth_chest_armor = crate::util::read_i32_le(chunk)?;

            // leather_chest_armor: int32
            let leather_chest_armor = crate::util::read_i32_le(chunk)?;

            // mail_chest_armor: int32
            let mail_chest_armor = crate::util::read_i32_le(chunk)?;

            // plate_chest_armor: int32
            let plate_chest_armor = crate::util::read_i32_le(chunk)?;


            rows.push(ScalingStatValuesRow {
                id,
                charlevel,
                shoulder_budget,
                trinket_budget,
                weapon_budget1_h,
                ranged_budget,
                cloth_shoulder_armor,
                leather_shoulder_armor,
                mail_shoulder_armor,
                plate_shoulder_armor,
                weapon_d_p_s1_h,
                weapon_d_p_s2_h,
                spellcaster_d_p_s1_h,
                spellcaster_d_p_s2_h,
                ranged_d_p_s,
                wand_d_p_s,
                spell_power,
                primary_budget,
                tertiary_budget,
                cloth_cloak_armor,
                cloth_chest_armor,
                leather_chest_armor,
                mail_chest_armor,
                plate_chest_armor,
            });
        }

        Ok(ScalingStatValues { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 24,
            record_size: 96,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (ScalingStatValues) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // charlevel: int32
            b.write_all(&row.charlevel.to_le_bytes())?;

            // shoulder_budget: int32
            b.write_all(&row.shoulder_budget.to_le_bytes())?;

            // trinket_budget: int32
            b.write_all(&row.trinket_budget.to_le_bytes())?;

            // weapon_budget1_h: int32
            b.write_all(&row.weapon_budget1_h.to_le_bytes())?;

            // ranged_budget: int32
            b.write_all(&row.ranged_budget.to_le_bytes())?;

            // cloth_shoulder_armor: int32
            b.write_all(&row.cloth_shoulder_armor.to_le_bytes())?;

            // leather_shoulder_armor: int32
            b.write_all(&row.leather_shoulder_armor.to_le_bytes())?;

            // mail_shoulder_armor: int32
            b.write_all(&row.mail_shoulder_armor.to_le_bytes())?;

            // plate_shoulder_armor: int32
            b.write_all(&row.plate_shoulder_armor.to_le_bytes())?;

            // weapon_d_p_s1_h: int32
            b.write_all(&row.weapon_d_p_s1_h.to_le_bytes())?;

            // weapon_d_p_s2_h: int32
            b.write_all(&row.weapon_d_p_s2_h.to_le_bytes())?;

            // spellcaster_d_p_s1_h: int32
            b.write_all(&row.spellcaster_d_p_s1_h.to_le_bytes())?;

            // spellcaster_d_p_s2_h: int32
            b.write_all(&row.spellcaster_d_p_s2_h.to_le_bytes())?;

            // ranged_d_p_s: int32
            b.write_all(&row.ranged_d_p_s.to_le_bytes())?;

            // wand_d_p_s: int32
            b.write_all(&row.wand_d_p_s.to_le_bytes())?;

            // spell_power: int32
            b.write_all(&row.spell_power.to_le_bytes())?;

            // primary_budget: int32
            b.write_all(&row.primary_budget.to_le_bytes())?;

            // tertiary_budget: int32
            b.write_all(&row.tertiary_budget.to_le_bytes())?;

            // cloth_cloak_armor: int32
            b.write_all(&row.cloth_cloak_armor.to_le_bytes())?;

            // cloth_chest_armor: int32
            b.write_all(&row.cloth_chest_armor.to_le_bytes())?;

            // leather_chest_armor: int32
            b.write_all(&row.leather_chest_armor.to_le_bytes())?;

            // mail_chest_armor: int32
            b.write_all(&row.mail_chest_armor.to_le_bytes())?;

            // plate_chest_armor: int32
            b.write_all(&row.plate_chest_armor.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for ScalingStatValues {
    type PrimaryKey = ScalingStatValuesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ScalingStatValuesKey {
    pub id: i32
}

impl ScalingStatValuesKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for ScalingStatValuesKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for ScalingStatValuesKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for ScalingStatValuesKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl From<u8> for ScalingStatValuesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for ScalingStatValuesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl TryFrom<u32> for ScalingStatValuesKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScalingStatValuesRow {
    pub id: ScalingStatValuesKey,
    pub charlevel: i32,
    pub shoulder_budget: i32,
    pub trinket_budget: i32,
    pub weapon_budget1_h: i32,
    pub ranged_budget: i32,
    pub cloth_shoulder_armor: i32,
    pub leather_shoulder_armor: i32,
    pub mail_shoulder_armor: i32,
    pub plate_shoulder_armor: i32,
    pub weapon_d_p_s1_h: i32,
    pub weapon_d_p_s2_h: i32,
    pub spellcaster_d_p_s1_h: i32,
    pub spellcaster_d_p_s2_h: i32,
    pub ranged_d_p_s: i32,
    pub wand_d_p_s: i32,
    pub spell_power: i32,
    pub primary_budget: i32,
    pub tertiary_budget: i32,
    pub cloth_cloak_armor: i32,
    pub cloth_chest_armor: i32,
    pub leather_chest_armor: i32,
    pub mail_chest_armor: i32,
    pub plate_chest_armor: i32,
}

