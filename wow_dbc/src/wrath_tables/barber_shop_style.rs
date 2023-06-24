use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::wrath_tables::chr_races::ChrRacesKey;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BarberShopStyle {
    pub rows: Vec<BarberShopStyleRow>,
}

impl DbcTable for BarberShopStyle {
    type Row = BarberShopStyleRow;

    const FILENAME: &'static str = "BarberShopStyle.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 160 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 160,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 40 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 40,
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

            // id: primary_key (BarberShopStyle) int32
            let id = BarberShopStyleKey::new(crate::util::read_i32_le(chunk)?);

            // ty: int32
            let ty = crate::util::read_i32_le(chunk)?;

            // display_name_lang: string_ref_loc (Extended)
            let display_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // cost_modifier: float
            let cost_modifier = crate::util::read_f32_le(chunk)?;

            // race: foreign_key (ChrRaces) int32
            let race = ChrRacesKey::new(crate::util::read_i32_le(chunk)?.into());

            // sex: int32
            let sex = crate::util::read_i32_le(chunk)?;

            // data: int32
            let data = crate::util::read_i32_le(chunk)?;


            rows.push(BarberShopStyleRow {
                id,
                ty,
                display_name_lang,
                description_lang,
                cost_modifier,
                race,
                sex,
                data,
            });
        }

        Ok(BarberShopStyle { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 40,
            record_size: 160,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (BarberShopStyle) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // ty: int32
            b.write_all(&row.ty.to_le_bytes())?;

            // display_name_lang: string_ref_loc (Extended)
            b.write_all(&row.display_name_lang.string_indices_as_array(&mut string_index))?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_index))?;

            // cost_modifier: float
            b.write_all(&row.cost_modifier.to_le_bytes())?;

            // race: foreign_key (ChrRaces) int32
            b.write_all(&(row.race.id as i32).to_le_bytes())?;

            // sex: int32
            b.write_all(&row.sex.to_le_bytes())?;

            // data: int32
            b.write_all(&row.data.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for BarberShopStyle {
    type PrimaryKey = BarberShopStyleKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl BarberShopStyle {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.display_name_lang.string_block_as_array(b)?;
            row.description_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.display_name_lang.string_block_size();
            sum += row.description_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct BarberShopStyleKey {
    pub id: i32
}

impl BarberShopStyleKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<i8> for BarberShopStyleKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }

}

impl From<i16> for BarberShopStyleKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }

}

impl From<i32> for BarberShopStyleKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }

}

impl From<u8> for BarberShopStyleKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }

}

impl From<u16> for BarberShopStyleKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BarberShopStyleRow {
    pub id: BarberShopStyleKey,
    pub ty: i32,
    pub display_name_lang: ExtendedLocalizedString,
    pub description_lang: ExtendedLocalizedString,
    pub cost_modifier: f32,
    pub race: ChrRacesKey,
    pub sex: i32,
    pub data: i32,
}

