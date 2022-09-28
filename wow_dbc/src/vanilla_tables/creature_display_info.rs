use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::SizeClass;
use crate::vanilla_tables::creature_display_info_extra::*;
use crate::vanilla_tables::creature_model_data::*;
use crate::vanilla_tables::creature_sound_data::*;
use crate::vanilla_tables::npc_sounds::*;
use crate::vanilla_tables::unit_blood::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CreatureDisplayInfo {
    pub rows: Vec<CreatureDisplayInfoRow>,
}

impl DbcTable for CreatureDisplayInfo {
    type Row = CreatureDisplayInfoRow;

    fn filename() -> &'static str { "CreatureDisplayInfo.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 48 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 48,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 12 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 12,
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

            // id: primary_key (CreatureDisplayInfo) uint32
            let id = CreatureDisplayInfoKey::new(crate::util::read_u32_le(chunk)?);

            // model: foreign_key (CreatureModelData) uint32
            let model = CreatureModelDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // sound: foreign_key (CreatureSoundData) uint32
            let sound = CreatureSoundDataKey::new(crate::util::read_u32_le(chunk)?.into());

            // extended_display_info: foreign_key (CreatureDisplayInfoExtra) uint32
            let extended_display_info = CreatureDisplayInfoExtraKey::new(crate::util::read_u32_le(chunk)?.into());

            // creature_model_scale: float
            let creature_model_scale = crate::util::read_f32_le(chunk)?;

            // creature_model_alpha: int32
            let creature_model_alpha = crate::util::read_i32_le(chunk)?;

            // texture_variation: string_ref[3]
            let texture_variation = {
                let mut arr = Vec::with_capacity(3);
                for _ in 0..3 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // size: SizeClass
            let size = SizeClass::try_from(crate::util::read_i32_le(chunk)?)?;

            // blood: foreign_key (UnitBlood) uint32
            let blood = UnitBloodKey::new(crate::util::read_u32_le(chunk)?.into());

            // npc_sound: foreign_key (NPCSounds) uint32
            let npc_sound = NPCSoundsKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(CreatureDisplayInfoRow {
                id,
                model,
                sound,
                extended_display_info,
                creature_model_scale,
                creature_model_alpha,
                texture_variation,
                size,
                blood,
                npc_sound,
            });
        }

        Ok(CreatureDisplayInfo { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 12,
            record_size: 48,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (CreatureDisplayInfo) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model: foreign_key (CreatureModelData) uint32
            b.write_all(&(row.model.id as u32).to_le_bytes())?;

            // sound: foreign_key (CreatureSoundData) uint32
            b.write_all(&(row.sound.id as u32).to_le_bytes())?;

            // extended_display_info: foreign_key (CreatureDisplayInfoExtra) uint32
            b.write_all(&(row.extended_display_info.id as u32).to_le_bytes())?;

            // creature_model_scale: float
            b.write_all(&row.creature_model_scale.to_le_bytes())?;

            // creature_model_alpha: int32
            b.write_all(&row.creature_model_alpha.to_le_bytes())?;

            // texture_variation: string_ref[3]
            for i in &row.texture_variation {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // size: SizeClass
            b.write_all(&(row.size.as_int() as i32).to_le_bytes())?;

            // blood: foreign_key (UnitBlood) uint32
            b.write_all(&(row.blood.id as u32).to_le_bytes())?;

            // npc_sound: foreign_key (NPCSounds) uint32
            b.write_all(&(row.npc_sound.id as u32).to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for CreatureDisplayInfo {
    type PrimaryKey = CreatureDisplayInfoKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl CreatureDisplayInfo {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            for s in &row.texture_variation {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            for s in &row.texture_variation {
                if !s.is_empty() { sum += s.len() + 1; };
            }

        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct CreatureDisplayInfoKey {
    pub id: u32
}

impl CreatureDisplayInfoKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatureDisplayInfoRow {
    pub id: CreatureDisplayInfoKey,
    pub model: CreatureModelDataKey,
    pub sound: CreatureSoundDataKey,
    pub extended_display_info: CreatureDisplayInfoExtraKey,
    pub creature_model_scale: f32,
    pub creature_model_alpha: i32,
    pub texture_variation: [String; 3],
    pub size: SizeClass,
    pub blood: UnitBloodKey,
    pub npc_sound: NPCSoundsKey,
}

