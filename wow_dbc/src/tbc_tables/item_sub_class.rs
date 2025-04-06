use crate::{
    DbcTable, ExtendedLocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemSubClass {
    pub rows: Vec<ItemSubClassRow>,
}

impl DbcTable for ItemSubClass {
    type Row = ItemSubClassRow;

    const FILENAME: &'static str = "ItemSubClass.dbc";
    const FIELD_COUNT: usize = 44;
    const ROW_SIZE: usize = 176;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
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

            // class_id: int32
            let class_id = crate::util::read_i32_le(chunk)?;

            // sub_class_id: int32
            let sub_class_id = crate::util::read_i32_le(chunk)?;

            // prerequisite_proficiency: int32
            let prerequisite_proficiency = crate::util::read_i32_le(chunk)?;

            // postrequisite_proficiency: int32
            let postrequisite_proficiency = crate::util::read_i32_le(chunk)?;

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // display_flags: int32
            let display_flags = crate::util::read_i32_le(chunk)?;

            // weapon_parry_seq: int32
            let weapon_parry_seq = crate::util::read_i32_le(chunk)?;

            // weapon_ready_seq: int32
            let weapon_ready_seq = crate::util::read_i32_le(chunk)?;

            // weapon_attack_seq: int32
            let weapon_attack_seq = crate::util::read_i32_le(chunk)?;

            // weapon_swing_size: int32
            let weapon_swing_size = crate::util::read_i32_le(chunk)?;

            // display_name_lang: string_ref_loc (Extended)
            let display_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // verbose_name_lang: string_ref_loc (Extended)
            let verbose_name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(ItemSubClassRow {
                class_id,
                sub_class_id,
                prerequisite_proficiency,
                postrequisite_proficiency,
                flags,
                display_flags,
                weapon_parry_seq,
                weapon_ready_seq,
                weapon_attack_seq,
                weapon_swing_size,
                display_name_lang,
                verbose_name_lang,
            });
        }

        Ok(ItemSubClass { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // class_id: int32
            b.write_all(&row.class_id.to_le_bytes())?;

            // sub_class_id: int32
            b.write_all(&row.sub_class_id.to_le_bytes())?;

            // prerequisite_proficiency: int32
            b.write_all(&row.prerequisite_proficiency.to_le_bytes())?;

            // postrequisite_proficiency: int32
            b.write_all(&row.postrequisite_proficiency.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // display_flags: int32
            b.write_all(&row.display_flags.to_le_bytes())?;

            // weapon_parry_seq: int32
            b.write_all(&row.weapon_parry_seq.to_le_bytes())?;

            // weapon_ready_seq: int32
            b.write_all(&row.weapon_ready_seq.to_le_bytes())?;

            // weapon_attack_seq: int32
            b.write_all(&row.weapon_attack_seq.to_le_bytes())?;

            // weapon_swing_size: int32
            b.write_all(&row.weapon_swing_size.to_le_bytes())?;

            // display_name_lang: string_ref_loc (Extended)
            b.write_all(&row.display_name_lang.string_indices_as_array(&mut string_index))?;

            // verbose_name_lang: string_ref_loc (Extended)
            b.write_all(&row.verbose_name_lang.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl ItemSubClass {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.display_name_lang.string_block_as_array(b)?;
            row.verbose_name_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.display_name_lang.string_block_size();
            sum += row.verbose_name_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemSubClassRow {
    pub class_id: i32,
    pub sub_class_id: i32,
    pub prerequisite_proficiency: i32,
    pub postrequisite_proficiency: i32,
    pub flags: i32,
    pub display_flags: i32,
    pub weapon_parry_seq: i32,
    pub weapon_ready_seq: i32,
    pub weapon_attack_seq: i32,
    pub weapon_swing_size: i32,
    pub display_name_lang: ExtendedLocalizedString,
    pub verbose_name_lang: ExtendedLocalizedString,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn item_sub_class() {
        let mut file = File::open("../tbc-dbc/ItemSubClass.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = ItemSubClass::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = ItemSubClass::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
