use crate::{
    DbcTable, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::chr_races::ChrRacesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharHairGeosets {
    pub rows: Vec<CharHairGeosetsRow>,
}

impl DbcTable for CharHairGeosets {
    type Row = CharHairGeosetsRow;

    fn filename() -> &'static str { "CharHairGeosets.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 24 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 24,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 6 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 6,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (CharHairGeosets) int32
            let id = CharHairGeosetsKey::new(crate::util::read_i32_le(chunk)?);

            // race_id: foreign_key (ChrRaces) int32
            let race_id = ChrRacesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sex_id: int32
            let sex_id = crate::util::read_i32_le(chunk)?;

            // variation_id: int32
            let variation_id = crate::util::read_i32_le(chunk)?;

            // geoset_id: int32
            let geoset_id = crate::util::read_i32_le(chunk)?;

            // showscalp: int32
            let showscalp = crate::util::read_i32_le(chunk)?;


            rows.push(CharHairGeosetsRow {
                id,
                race_id,
                sex_id,
                variation_id,
                geoset_id,
                showscalp,
            });
        }

        Ok(CharHairGeosets { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 6,
            record_size: 24,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (CharHairGeosets) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // race_id: foreign_key (ChrRaces) int32
            b.write_all(&(row.race_id.id as i32).to_le_bytes())?;

            // sex_id: int32
            b.write_all(&row.sex_id.to_le_bytes())?;

            // variation_id: int32
            b.write_all(&row.variation_id.to_le_bytes())?;

            // geoset_id: int32
            b.write_all(&row.geoset_id.to_le_bytes())?;

            // showscalp: int32
            b.write_all(&row.showscalp.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for CharHairGeosets {
    type PrimaryKey = CharHairGeosetsKey;
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
pub struct CharHairGeosetsKey {
    pub id: i32
}

impl CharHairGeosetsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for CharHairGeosetsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for CharHairGeosetsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for CharHairGeosetsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for CharHairGeosetsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for CharHairGeosetsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharHairGeosetsRow {
    pub id: CharHairGeosetsKey,
    pub race_id: ChrRacesKey,
    pub sex_id: i32,
    pub variation_id: i32,
    pub geoset_id: i32,
    pub showscalp: i32,
}

