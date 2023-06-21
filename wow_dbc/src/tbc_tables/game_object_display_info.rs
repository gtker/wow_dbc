use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GameObjectDisplayInfo {
    pub rows: Vec<GameObjectDisplayInfoRow>,
}

impl DbcTable for GameObjectDisplayInfo {
    type Row = GameObjectDisplayInfoRow;

    fn filename() -> &'static str { "GameObjectDisplayInfo.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 72 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 72,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 18 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 18,
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

            // id: primary_key (GameObjectDisplayInfo) int32
            let id = GameObjectDisplayInfoKey::new(crate::util::read_i32_le(chunk)?);

            // model_name: string_ref
            let model_name = {
                let s = crate::util::get_string_as_vec(chunk, &string_block)?;
                String::from_utf8(s)?
            };

            // sound: int32[10]
            let sound = crate::util::read_array_i32::<10>(chunk)?;

            // geo_box_min: float[3]
            let geo_box_min = crate::util::read_array_f32::<3>(chunk)?;

            // geo_box_max: float[3]
            let geo_box_max = crate::util::read_array_f32::<3>(chunk)?;


            rows.push(GameObjectDisplayInfoRow {
                id,
                model_name,
                sound,
                geo_box_min,
                geo_box_max,
            });
        }

        Ok(GameObjectDisplayInfo { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 18,
            record_size: 72,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (GameObjectDisplayInfo) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // model_name: string_ref
            if !row.model_name.is_empty() {
                b.write_all(&(string_index as u32).to_le_bytes())?;
                string_index += row.model_name.len() + 1;
            }
            else {
                b.write_all(&(0_u32).to_le_bytes())?;
            }

            // sound: int32[10]
            for i in row.sound {
                b.write_all(&i.to_le_bytes())?;
            }


            // geo_box_min: float[3]
            for i in row.geo_box_min {
                b.write_all(&i.to_le_bytes())?;
            }


            // geo_box_max: float[3]
            for i in row.geo_box_max {
                b.write_all(&i.to_le_bytes())?;
            }


        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for GameObjectDisplayInfo {
    type PrimaryKey = GameObjectDisplayInfoKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl GameObjectDisplayInfo {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            if !row.model_name.is_empty() { b.write_all(row.model_name.as_bytes())?; b.write_all(&[0])?; };
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            if !row.model_name.is_empty() { sum += row.model_name.len() + 1; };
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstGameObjectDisplayInfo<const S: usize> {
    pub rows: [ConstGameObjectDisplayInfoRow; S],
}

impl<const S: usize> ConstGameObjectDisplayInfo<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 72 {
            panic!("invalid record size, expected 72")
        }

        if header.field_count != 18 {
            panic!("invalid field count, expected 18")
        }

        let string_block = (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = 20;
        let mut rows = [
            ConstGameObjectDisplayInfoRow {
                id: GameObjectDisplayInfoKey::new(0),
                model_name: "",
                sound: [0; 10],
                geo_box_min: [0.0; 3],
                geo_box_max: [0.0; 3],
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (GameObjectDisplayInfo) int32
            let id = GameObjectDisplayInfoKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // model_name: string_ref
            let model_name = crate::util::get_string_from_block(b_offset, b, string_block);
            b_offset += 4;

            // sound: int32[10]
            let sound = {
                let mut a = [0; 10];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // geo_box_min: float[3]
            let geo_box_min = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // geo_box_max: float[3]
            let geo_box_max = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            rows[i] = ConstGameObjectDisplayInfoRow {
                id,
                model_name,
                sound,
                geo_box_min,
                geo_box_max,
            };
            i += 1;
        }

        Self { rows }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct GameObjectDisplayInfoKey {
    pub id: i32
}

impl GameObjectDisplayInfoKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for GameObjectDisplayInfoKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for GameObjectDisplayInfoKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for GameObjectDisplayInfoKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for GameObjectDisplayInfoKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for GameObjectDisplayInfoKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GameObjectDisplayInfoRow {
    pub id: GameObjectDisplayInfoKey,
    pub model_name: String,
    pub sound: [i32; 10],
    pub geo_box_min: [f32; 3],
    pub geo_box_max: [f32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstGameObjectDisplayInfoRow {
    pub id: GameObjectDisplayInfoKey,
    pub model_name: &'static str,
    pub sound: [i32; 10],
    pub geo_box_min: [f32; 3],
    pub geo_box_max: [f32; 3],
}

