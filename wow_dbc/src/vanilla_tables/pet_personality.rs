use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PetPersonality {
    pub rows: Vec<PetPersonalityRow>,
}

impl DbcTable for PetPersonality {
    type Row = PetPersonalityRow;

    fn filename() -> &'static str { "PetPersonality.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 76 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 76,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 19 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 19,
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

            // id: primary_key (PetPersonality) uint32
            let id = PetPersonalityKey::new(crate::util::read_u32_le(chunk)?);

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // threshold_unhappy: int32
            let threshold_unhappy = crate::util::read_i32_le(chunk)?;

            // threshold_content: int32
            let threshold_content = crate::util::read_i32_le(chunk)?;

            // threshold_happy: int32
            let threshold_happy = crate::util::read_i32_le(chunk)?;

            // damage_unhappy: float
            let damage_unhappy = crate::util::read_f32_le(chunk)?;

            // damage_content: float
            let damage_content = crate::util::read_f32_le(chunk)?;

            // damage_happy: float
            let damage_happy = crate::util::read_f32_le(chunk)?;

            // modifier_unhappy: float
            let modifier_unhappy = crate::util::read_f32_le(chunk)?;

            // modifier_content: float
            let modifier_content = crate::util::read_f32_le(chunk)?;

            // modifier_happy: float
            let modifier_happy = crate::util::read_f32_le(chunk)?;


            rows.push(PetPersonalityRow {
                id,
                name,
                threshold_unhappy,
                threshold_content,
                threshold_happy,
                damage_unhappy,
                damage_content,
                damage_happy,
                modifier_unhappy,
                modifier_content,
                modifier_happy,
            });
        }

        Ok(PetPersonality { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 19,
            record_size: 76,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (PetPersonality) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // threshold_unhappy: int32
            b.write_all(&row.threshold_unhappy.to_le_bytes())?;

            // threshold_content: int32
            b.write_all(&row.threshold_content.to_le_bytes())?;

            // threshold_happy: int32
            b.write_all(&row.threshold_happy.to_le_bytes())?;

            // damage_unhappy: float
            b.write_all(&row.damage_unhappy.to_le_bytes())?;

            // damage_content: float
            b.write_all(&row.damage_content.to_le_bytes())?;

            // damage_happy: float
            b.write_all(&row.damage_happy.to_le_bytes())?;

            // modifier_unhappy: float
            b.write_all(&row.modifier_unhappy.to_le_bytes())?;

            // modifier_content: float
            b.write_all(&row.modifier_content.to_le_bytes())?;

            // modifier_happy: float
            b.write_all(&row.modifier_happy.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for PetPersonality {
    type PrimaryKey = PetPersonalityKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl PetPersonality {
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
pub struct ConstPetPersonality<const S: usize> {
    pub rows: [ConstPetPersonalityRow; S],
}

impl<const S: usize> ConstPetPersonality<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 76 {
            panic!("invalid record size, expected 76")
        }

        if header.field_count != 19 {
            panic!("invalid field count, expected 19")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstPetPersonalityRow {
                id: PetPersonalityKey::new(0),
                name: crate::ConstLocalizedString::empty(),
                threshold_unhappy: 0,
                threshold_content: 0,
                threshold_happy: 0,
                damage_unhappy: 0.0,
                damage_content: 0.0,
                damage_happy: 0.0,
                modifier_unhappy: 0.0,
                modifier_content: 0.0,
                modifier_happy: 0.0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (PetPersonality) uint32
            let id = PetPersonalityKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
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

            // threshold_unhappy: int32
            let threshold_unhappy = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // threshold_content: int32
            let threshold_content = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // threshold_happy: int32
            let threshold_happy = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // damage_unhappy: float
            let damage_unhappy = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // damage_content: float
            let damage_content = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // damage_happy: float
            let damage_happy = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // modifier_unhappy: float
            let modifier_unhappy = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // modifier_content: float
            let modifier_content = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // modifier_happy: float
            let modifier_happy = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstPetPersonalityRow {
                id,
                name,
                threshold_unhappy,
                threshold_content,
                threshold_happy,
                damage_unhappy,
                damage_content,
                damage_happy,
                modifier_unhappy,
                modifier_content,
                modifier_happy,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct PetPersonalityKey {
    pub id: u32
}

impl PetPersonalityKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for PetPersonalityKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for PetPersonalityKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for PetPersonalityKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PetPersonalityRow {
    pub id: PetPersonalityKey,
    pub name: LocalizedString,
    pub threshold_unhappy: i32,
    pub threshold_content: i32,
    pub threshold_happy: i32,
    pub damage_unhappy: f32,
    pub damage_content: f32,
    pub damage_happy: f32,
    pub modifier_unhappy: f32,
    pub modifier_content: f32,
    pub modifier_happy: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstPetPersonalityRow {
    pub id: PetPersonalityKey,
    pub name: ConstLocalizedString,
    pub threshold_unhappy: i32,
    pub threshold_content: i32,
    pub threshold_happy: i32,
    pub damage_unhappy: f32,
    pub damage_content: f32,
    pub damage_happy: f32,
    pub modifier_unhappy: f32,
    pub modifier_content: f32,
    pub modifier_happy: f32,
}

