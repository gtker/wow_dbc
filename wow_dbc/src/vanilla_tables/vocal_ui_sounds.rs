use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::chr_races::*;
use crate::vanilla_tables::sound_entries::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VocalUISounds {
    pub rows: Vec<VocalUISoundsRow>,
}

impl DbcTable for VocalUISounds {
    type Row = VocalUISoundsRow;

    fn filename() -> &'static str { "VocalUISounds.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 28 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 28,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 7 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 7,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (VocalUISounds) uint32
            let id = VocalUISoundsKey::new(crate::util::read_u32_le(chunk)?);

            // vocal_ui_enum: int32
            let vocal_ui_enum = crate::util::read_i32_le(chunk)?;

            // race: foreign_key (ChrRaces) uint32
            let race = ChrRacesKey::new(crate::util::read_u32_le(chunk)?.into());

            // normal_male_sound: foreign_key (SoundEntries) uint32
            let normal_male_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // normal_female_sound: foreign_key (SoundEntries) uint32
            let normal_female_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // pissed_male_sound: foreign_key (SoundEntries) uint32
            let pissed_male_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // pissed_female_sound: foreign_key (SoundEntries) uint32
            let pissed_female_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(VocalUISoundsRow {
                id,
                vocal_ui_enum,
                race,
                normal_male_sound,
                normal_female_sound,
                pissed_male_sound,
                pissed_female_sound,
            });
        }

        Ok(VocalUISounds { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 7,
            record_size: 28,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (VocalUISounds) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // vocal_ui_enum: int32
            b.write_all(&row.vocal_ui_enum.to_le_bytes())?;

            // race: foreign_key (ChrRaces) uint32
            b.write_all(&(row.race.id as u32).to_le_bytes())?;

            // normal_male_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.normal_male_sound.id as u32).to_le_bytes())?;

            // normal_female_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.normal_female_sound.id as u32).to_le_bytes())?;

            // pissed_male_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.pissed_male_sound.id as u32).to_le_bytes())?;

            // pissed_female_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.pissed_female_sound.id as u32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for VocalUISounds {
    type PrimaryKey = VocalUISoundsKey;
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
pub struct ConstVocalUISounds<const S: usize> {
    pub rows: [VocalUISoundsRow; S],
}

impl<const S: usize> ConstVocalUISounds<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 28 {
            panic!("invalid record size, expected 28")
        }

        if header.field_count != 7 {
            panic!("invalid field count, expected 7")
        }

        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            VocalUISoundsRow {
                id: VocalUISoundsKey::new(0),
                vocal_ui_enum: 0,
                race: ChrRacesKey::new(0),
                normal_male_sound: SoundEntriesKey::new(0),
                normal_female_sound: SoundEntriesKey::new(0),
                pissed_male_sound: SoundEntriesKey::new(0),
                pissed_female_sound: SoundEntriesKey::new(0),
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (VocalUISounds) uint32
            let id = VocalUISoundsKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // vocal_ui_enum: int32
            let vocal_ui_enum = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // race: foreign_key (ChrRaces) uint32
            let race = ChrRacesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // normal_male_sound: foreign_key (SoundEntries) uint32
            let normal_male_sound = SoundEntriesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // normal_female_sound: foreign_key (SoundEntries) uint32
            let normal_female_sound = SoundEntriesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // pissed_male_sound: foreign_key (SoundEntries) uint32
            let pissed_male_sound = SoundEntriesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // pissed_female_sound: foreign_key (SoundEntries) uint32
            let pissed_female_sound = SoundEntriesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            rows[i] = VocalUISoundsRow {
                id,
                vocal_ui_enum,
                race,
                normal_male_sound,
                normal_female_sound,
                pissed_male_sound,
                pissed_female_sound,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> VocalUISounds {
        VocalUISounds {
            rows: self.rows.iter().map(|s| VocalUISoundsRow {
                id: s.id,
                vocal_ui_enum: s.vocal_ui_enum,
                race: s.race,
                normal_male_sound: s.normal_male_sound,
                normal_female_sound: s.normal_female_sound,
                pissed_male_sound: s.pissed_male_sound,
                pissed_female_sound: s.pissed_female_sound,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct VocalUISoundsKey {
    pub id: u32
}

impl VocalUISoundsKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for VocalUISoundsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for VocalUISoundsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for VocalUISoundsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VocalUISoundsRow {
    pub id: VocalUISoundsKey,
    pub vocal_ui_enum: i32,
    pub race: ChrRacesKey,
    pub normal_male_sound: SoundEntriesKey,
    pub normal_female_sound: SoundEntriesKey,
    pub pissed_male_sound: SoundEntriesKey,
    pub pissed_female_sound: SoundEntriesKey,
}

