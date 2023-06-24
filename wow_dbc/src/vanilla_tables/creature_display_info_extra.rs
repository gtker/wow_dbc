use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::chr_races::ChrRacesKey;
use std::io::Write;
use wow_world_base::vanilla::Gender;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreatureDisplayInfoExtra {
    pub rows: Vec<CreatureDisplayInfoExtraRow>,
}

impl DbcTable for CreatureDisplayInfoExtra {
    type Row = CreatureDisplayInfoExtraRow;

    const FILENAME: &'static str = "CreatureDisplayInfoExtra.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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

            // id: primary_key (CreatureDisplayInfoExtra) uint32
            let id = CreatureDisplayInfoExtraKey::new(crate::util::read_u32_le(chunk)?);

            // display_race: foreign_key (ChrRaces) uint32
            let display_race = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // gender: Gender
            let gender = crate::util::read_i32_le(chunk)?.try_into()?;

            // skin: int32
            let skin = crate::util::read_i32_le(chunk)?;

            // face: int32
            let face = crate::util::read_i32_le(chunk)?;

            // hair_style: int32
            let hair_style = crate::util::read_i32_le(chunk)?;

            // hair_colour: int32
            let hair_colour = crate::util::read_i32_le(chunk)?;

            // facial_hair: int32
            let facial_hair = crate::util::read_i32_le(chunk)?;

            // npc_item_display: uint32[9]
            let npc_item_display = crate::util::read_array_u32::<9>(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // bake_name: string_ref
            let bake_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };


            rows.push(CreatureDisplayInfoExtraRow {
                id,
                display_race,
                gender,
                skin,
                face,
                hair_style,
                hair_colour,
                facial_hair,
                npc_item_display,
                flags,
                bake_name,
            });
        }

        Ok(CreatureDisplayInfoExtra { rows, })
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
            // id: primary_key (CreatureDisplayInfoExtra) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // display_race: foreign_key (ChrRaces) uint32
            b.write_all(&(row.display_race.id as u32).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i32).to_le_bytes())?;

            // skin: int32
            b.write_all(&row.skin.to_le_bytes())?;

            // face: int32
            b.write_all(&row.face.to_le_bytes())?;

            // hair_style: int32
            b.write_all(&row.hair_style.to_le_bytes())?;

            // hair_colour: int32
            b.write_all(&row.hair_colour.to_le_bytes())?;

            // facial_hair: int32
            b.write_all(&row.facial_hair.to_le_bytes())?;

            // npc_item_display: uint32[9]
            for i in row.npc_item_display {
                b.write_all(&i.to_le_bytes())?;
            }


            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // bake_name: string_ref
            if !row.bake_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.bake_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for CreatureDisplayInfoExtra {
    type PrimaryKey = CreatureDisplayInfoExtraKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl CreatureDisplayInfoExtra {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.bake_name.is_empty() { b.write_all(row.bake_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.bake_name.is_empty() { sum += row.bake_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CreatureDisplayInfoExtraKey {
    pub id: u32
}

impl CreatureDisplayInfoExtraKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for CreatureDisplayInfoExtraKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for CreatureDisplayInfoExtraKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<u32> for CreatureDisplayInfoExtraKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreatureDisplayInfoExtraRow {
    pub id: CreatureDisplayInfoExtraKey,
    pub display_race: ChrRacesKey,
    pub gender: Gender,
    pub skin: i32,
    pub face: i32,
    pub hair_style: i32,
    pub hair_colour: i32,
    pub facial_hair: i32,
    pub npc_item_display: [u32; 9],
    pub flags: i32,
    pub bake_name: String,
}

