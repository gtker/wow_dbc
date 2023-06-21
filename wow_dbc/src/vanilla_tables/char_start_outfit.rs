use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::Gender;
use crate::vanilla_tables::chr_classes::*;
use crate::vanilla_tables::chr_races::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharStartOutfit {
    pub rows: Vec<CharStartOutfitRow>,
}

impl DbcTable for CharStartOutfit {
    type Row = CharStartOutfitRow;

    fn filename() -> &'static str { "CharStartOutfit.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 152 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 152,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 41 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 41,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (CharStartOutfit) uint32
            let id = CharStartOutfitKey::new(crate::util::read_u32_le(chunk)?);

            // race: foreign_key (ChrRaces) uint8
            let race = ChrRacesKey::new(crate::util::read_u8_le(chunk)?.into());

            // class: foreign_key (ChrClasses) uint8
            let class = ChrClassesKey::new(crate::util::read_u8_le(chunk)?.into());

            // gender: Gender
            let gender = Gender::try_from(crate::util::read_i8_le(chunk)?)?;

            // outfit_id: int8
            let outfit_id = crate::util::read_i8_le(chunk)?;

            // item_id: int32[12]
            let item_id = crate::util::read_array_i32::<12>(chunk)?;

            // display_id: int32[12]
            let display_id = crate::util::read_array_i32::<12>(chunk)?;

            // inv_slot_id: int32[12]
            let inv_slot_id = crate::util::read_array_i32::<12>(chunk)?;


            rows.push(CharStartOutfitRow {
                id,
                race,
                class,
                gender,
                outfit_id,
                item_id,
                display_id,
                inv_slot_id,
            });
        }

        Ok(CharStartOutfit { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 41,
            record_size: 152,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (CharStartOutfit) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // race: foreign_key (ChrRaces) uint8
            b.write_all(&(row.race.id as u8).to_le_bytes())?;

            // class: foreign_key (ChrClasses) uint8
            b.write_all(&(row.class.id as u8).to_le_bytes())?;

            // gender: Gender
            b.write_all(&(row.gender.as_int() as i8).to_le_bytes())?;

            // outfit_id: int8
            b.write_all(&row.outfit_id.to_le_bytes())?;

            // item_id: int32[12]
            for i in row.item_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // display_id: int32[12]
            for i in row.display_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // inv_slot_id: int32[12]
            for i in row.inv_slot_id {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for CharStartOutfit {
    type PrimaryKey = CharStartOutfitKey;
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
pub struct ConstCharStartOutfit<const S: usize> {
    pub rows: [CharStartOutfitRow; S],
}

impl<const S: usize> ConstCharStartOutfit<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 152 {
            panic!("invalid record size, expected 152")
        }

        if header.field_count != 41 {
            panic!("invalid field count, expected 41")
        }

        let mut b_offset = 20;
        let mut rows = [
            CharStartOutfitRow {
                id: CharStartOutfitKey::new(0),
                race: ChrRacesKey::new(0),
                class: ChrClassesKey::new(0),
                gender: Gender::Male,
                outfit_id: 0,
                item_id: [0; 12],
                display_id: [0; 12],
                inv_slot_id: [0; 12],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (CharStartOutfit) uint32
            let id = CharStartOutfitKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // race: foreign_key (ChrRaces) uint8
            let race = ChrRacesKey::new(u8::from_le_bytes([b[b_offset + 0]])as u32);
            b_offset += 1;

            // class: foreign_key (ChrClasses) uint8
            let class = ChrClassesKey::new(u8::from_le_bytes([b[b_offset + 0]])as u32);
            b_offset += 1;

            // gender: Gender
            let gender = match Gender::from_value(i8::from_le_bytes([b[b_offset + 0]]) as i8) {
                Some(e) => e,
                None => panic!(),
            };
            b_offset += 1;

            // outfit_id: int8
            let outfit_id = i8::from_le_bytes([b[b_offset + 0]]);
            b_offset += 1;

            // item_id: int32[12]
            let item_id = {
                let mut a = [0; 12];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // display_id: int32[12]
            let display_id = {
                let mut a = [0; 12];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // inv_slot_id: int32[12]
            let inv_slot_id = {
                let mut a = [0; 12];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = CharStartOutfitRow {
                id,
                race,
                class,
                gender,
                outfit_id,
                item_id,
                display_id,
                inv_slot_id,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> CharStartOutfit {
        CharStartOutfit {
            rows: self.rows.iter().map(|s| CharStartOutfitRow {
                id: s.id,
                race: s.race,
                class: s.class,
                gender: s.gender,
                outfit_id: s.outfit_id,
                item_id: s.item_id,
                display_id: s.display_id,
                inv_slot_id: s.inv_slot_id,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct CharStartOutfitKey {
    pub id: u32
}

impl CharStartOutfitKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

impl From<u8> for CharStartOutfitKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for CharStartOutfitKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

impl From<u32> for CharStartOutfitKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharStartOutfitRow {
    pub id: CharStartOutfitKey,
    pub race: ChrRacesKey,
    pub class: ChrClassesKey,
    pub gender: Gender,
    pub outfit_id: i8,
    pub item_id: [i32; 12],
    pub display_id: [i32; 12],
    pub inv_slot_id: [i32; 12],
}

