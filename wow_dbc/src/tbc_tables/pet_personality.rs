use crate::header::{HEADER_SIZE, DbcHeader, parse_header};
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::ExtendedLocalizedString;

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
        let header = parse_header(&header)?;

        if header.record_size != 108 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 108,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 27 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 27,
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

            // id: primary_key (PetPersonality) int32
            let id = PetPersonalityKey::new(crate::util::read_i32_le(chunk)?);

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // happiness_threshold: int32[3]
            let happiness_threshold = crate::util::read_array_i32::<3>(chunk)?;

            // happiness_damage: float[3]
            let happiness_damage = crate::util::read_array_f32::<3>(chunk)?;

            // damage_modifier: float[3]
            let damage_modifier = crate::util::read_array_f32::<3>(chunk)?;


            rows.push(PetPersonalityRow {
                id,
                name_lang,
                happiness_threshold,
                happiness_damage,
                damage_modifier,
            });
        }

        Ok(PetPersonality { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 27,
            record_size: 108,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (PetPersonality) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // happiness_threshold: int32[3]
            for i in row.happiness_threshold {
                b.write_all(&i.to_le_bytes())?;
            }


            // happiness_damage: float[3]
            for i in row.happiness_damage {
                b.write_all(&i.to_le_bytes())?;
            }


            // damage_modifier: float[3]
            for i in row.damage_modifier {
                b.write_all(&i.to_le_bytes())?;
            }


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
pub struct PetPersonalityKey {
    pub id: i32
}

impl PetPersonalityKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for PetPersonalityKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for PetPersonalityKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for PetPersonalityKey {
    fn from(v: i32) -> Self {
        Self::new(v)
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PetPersonalityRow {
    pub id: PetPersonalityKey,
    pub name_lang: ExtendedLocalizedString,
    pub happiness_threshold: [i32; 3],
    pub happiness_damage: [f32; 3],
    pub damage_modifier: [f32; 3],
}

