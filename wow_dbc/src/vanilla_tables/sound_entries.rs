use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundEntries {
    pub rows: Vec<SoundEntriesRow>,
}

impl DbcTable for SoundEntries {
    type Row = SoundEntriesRow;

    fn filename() -> &'static str { "SoundEntries.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 116 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 116,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 29 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 29,
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

            // id: primary_key (SoundEntries) uint32
            let id = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?);

            // sound_type: SoundType
            let sound_type = SoundType::try_from(crate::util::read_i32_le(chunk)?)?;

            // name: string_ref
            let name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // files: string_ref[10]
            let files = {
                let mut arr = Vec::with_capacity(10);
                for _ in 0..10 {
                    let i ={
                        let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                        String::from_utf8(s)?
                    };
                    arr.push(i);
                }

                arr.try_into().unwrap()
            };

            // frequency: uint32[10]
            let frequency = crate::util::read_array_u32::<10>(chunk)?;

            // directory_base: string_ref
            let directory_base = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // volume: float
            let volume = crate::util::read_f32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // min_distance: float
            let min_distance = crate::util::read_f32_le(chunk)?;

            // distance_cutoff: float
            let distance_cutoff = crate::util::read_f32_le(chunk)?;

            // sound_entries_advanced: int32
            let sound_entries_advanced = crate::util::read_i32_le(chunk)?;


            rows.push(SoundEntriesRow {
                id,
                sound_type,
                name,
                files,
                frequency,
                directory_base,
                volume,
                flags,
                min_distance,
                distance_cutoff,
                sound_entries_advanced,
            });
        }

        Ok(SoundEntries { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 29,
            record_size: 116,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (SoundEntries) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // sound_type: SoundType
            b.write_all(&(row.sound_type.as_int() as i32).to_le_bytes())?;

            // name: string_ref
            if !row.name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // files: string_ref[10]
            for i in &row.files {
                if !i.is_empty() {
                    b.write_all(&(string_index as u32).to_le_bytes())?;
                    string_index += i.len() + 1;
                }
                else {
                    b.write_all(&(0_u32).to_le_bytes())?;
                }
            }


            // frequency: uint32[10]
            for i in row.frequency {
                b.write_all(&i.to_le_bytes())?;
            }


            // directory_base: string_ref
            if !row.directory_base.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.directory_base.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // volume: float
            b.write_all(&row.volume.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // min_distance: float
            b.write_all(&row.min_distance.to_le_bytes())?;

            // distance_cutoff: float
            b.write_all(&row.distance_cutoff.to_le_bytes())?;

            // sound_entries_advanced: int32
            b.write_all(&row.sound_entries_advanced.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for SoundEntries {
    type PrimaryKey = SoundEntriesKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl SoundEntries {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.name.is_empty() { b.write_all(row.name.as_bytes())?; b.write_all(&[0])?; };
            for s in &row.files {
                if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };
            }

            if !row.directory_base.is_empty() { b.write_all(row.directory_base.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.name.is_empty() { sum += row.name.len() + 1; };
            for s in &row.files {
                if !s.is_empty() { sum += s.len() + 1; };
            }

            if !row.directory_base.is_empty() { sum += row.directory_base.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstSoundEntries<const S: usize> {
    pub rows: [ConstSoundEntriesRow; S],
}

impl<const S: usize> ConstSoundEntries<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 116 {
            panic!("invalid record size, expected 116")
        }

        if header.field_count != 29 {
            panic!("invalid field count, expected 29")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstSoundEntriesRow {
                id: SoundEntriesKey::new(0),
                sound_type: SoundType::Unused,
                name: "",
                files: [""; 10],
                frequency: [0; 10],
                directory_base: "",
                volume: 0.0,
                flags: 0,
                min_distance: 0.0,
                distance_cutoff: 0.0,
                sound_entries_advanced: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (SoundEntries) uint32
            let id = SoundEntriesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // sound_type: SoundType
            let sound_type = match SoundType::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 4;

            // name: string_ref
            let name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // files: string_ref[10]
            let files = {
                let mut a = [""; 10];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::get_string_from_block(b_offset, b, string_block);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // frequency: uint32[10]
            let frequency = {
                let mut a = [0; 10];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // directory_base: string_ref
            let directory_base = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // volume: float
            let volume = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // flags: int32
            let flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // min_distance: float
            let min_distance = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // distance_cutoff: float
            let distance_cutoff = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // sound_entries_advanced: int32
            let sound_entries_advanced = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstSoundEntriesRow {
                id,
                sound_type,
                name,
                files,
                frequency,
                directory_base,
                volume,
                flags,
                min_distance,
                distance_cutoff,
                sound_entries_advanced,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> SoundEntries {
        SoundEntries {
            rows: self.rows.iter().map(|s| SoundEntriesRow {
                id: s.id,
                sound_type: s.sound_type,
                name: s.name.to_string(),
                files: s.files.map(|a| a.to_string()),
                frequency: s.frequency,
                directory_base: s.directory_base.to_string(),
                volume: s.volume,
                flags: s.flags,
                min_distance: s.min_distance,
                distance_cutoff: s.distance_cutoff,
                sound_entries_advanced: s.sound_entries_advanced,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SoundEntriesKey {
    pub id: u32
}

impl SoundEntriesKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for SoundEntriesKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for SoundEntriesKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for SoundEntriesKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SoundType {
    Unused,
    Spells,
    Ui,
    Footsteps,
    WeaponImpact,
    WeaponMiss,
    PickUpPutDown,
    NpcCombat,
    Errors,
    Objects,
    Death,
    NpcGreetings,
    Test,
    ArmourFoley,
    Footsteps2,
    WaterCharacter,
    WaterLiquid,
    Tradeskills,
    Doodads,
    SpellFizzle,
    NpcLoops,
    ZoneMusic,
    Emotes,
    NarrationMusic,
    Narration,
    ZoneAmbience,
}

impl SoundType {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            0 => Self::Unused,
            1 => Self::Spells,
            2 => Self::Ui,
            3 => Self::Footsteps,
            4 => Self::WeaponImpact,
            6 => Self::WeaponMiss,
            9 => Self::PickUpPutDown,
            10 => Self::NpcCombat,
            12 => Self::Errors,
            14 => Self::Objects,
            16 => Self::Death,
            17 => Self::NpcGreetings,
            18 => Self::Test,
            19 => Self::ArmourFoley,
            20 => Self::Footsteps2,
            21 => Self::WaterCharacter,
            22 => Self::WaterLiquid,
            23 => Self::Tradeskills,
            25 => Self::Doodads,
            26 => Self::SpellFizzle,
            27 => Self::NpcLoops,
            28 => Self::ZoneMusic,
            29 => Self::Emotes,
            30 => Self::NarrationMusic,
            31 => Self::Narration,
            50 => Self::ZoneAmbience,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for SoundType {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("SoundType", value as i64))
    }

}

impl SoundType {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::Unused => 0,
            Self::Spells => 1,
            Self::Ui => 2,
            Self::Footsteps => 3,
            Self::WeaponImpact => 4,
            Self::WeaponMiss => 6,
            Self::PickUpPutDown => 9,
            Self::NpcCombat => 10,
            Self::Errors => 12,
            Self::Objects => 14,
            Self::Death => 16,
            Self::NpcGreetings => 17,
            Self::Test => 18,
            Self::ArmourFoley => 19,
            Self::Footsteps2 => 20,
            Self::WaterCharacter => 21,
            Self::WaterLiquid => 22,
            Self::Tradeskills => 23,
            Self::Doodads => 25,
            Self::SpellFizzle => 26,
            Self::NpcLoops => 27,
            Self::ZoneMusic => 28,
            Self::Emotes => 29,
            Self::NarrationMusic => 30,
            Self::Narration => 31,
            Self::ZoneAmbience => 50,
        }

    }

}

impl Default for SoundType {
    fn default() -> Self {
        Self::Unused
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SoundEntriesRow {
    pub id: SoundEntriesKey,
    pub sound_type: SoundType,
    pub name: String,
    pub files: [String; 10],
    pub frequency: [u32; 10],
    pub directory_base: String,
    pub volume: f32,
    pub flags: i32,
    pub min_distance: f32,
    pub distance_cutoff: f32,
    pub sound_entries_advanced: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstSoundEntriesRow {
    pub id: SoundEntriesKey,
    pub sound_type: SoundType,
    pub name: &'static str,
    pub files: [&'static str; 10],
    pub frequency: [u32; 10],
    pub directory_base: &'static str,
    pub volume: f32,
    pub flags: i32,
    pub min_distance: f32,
    pub distance_cutoff: f32,
    pub sound_entries_advanced: i32,
}

