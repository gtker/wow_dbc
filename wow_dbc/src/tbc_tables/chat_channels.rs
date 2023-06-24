use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChatChannels {
    pub rows: Vec<ChatChannelsRow>,
}

impl DbcTable for ChatChannels {
    type Row = ChatChannelsRow;

    const FILENAME: &'static str = "ChatChannels.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != 148 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 148,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 37 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 37,
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

            // id: primary_key (ChatChannels) int32
            let id = ChatChannelsKey::new(crate::util::read_i32_le(chunk)?);

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // faction_group: int32
            let faction_group = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // shortcut_lang: string_ref_loc (Extended)
            let shortcut_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;


            rows.push(ChatChannelsRow {
                id,
                flags,
                faction_group,
                name_lang,
                shortcut_lang,
            });
        }

        Ok(ChatChannels { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 37,
            record_size: 148,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ChatChannels) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // faction_group: int32
            b.write_all(&row.faction_group.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // shortcut_lang: string_ref_loc (Extended)
            b.write_all(&row.shortcut_lang.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ChatChannels {
    type PrimaryKey = ChatChannelsKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

impl ChatChannels {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
            row.shortcut_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
            sum += row.shortcut_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ChatChannelsKey {
    pub id: i32
}

impl ChatChannelsKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for ChatChannelsKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for ChatChannelsKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for ChatChannelsKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for ChatChannelsKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for ChatChannelsKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for ChatChannelsKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for ChatChannelsKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for ChatChannelsKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for ChatChannelsKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for ChatChannelsKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChatChannelsRow {
    pub id: ChatChannelsKey,
    pub flags: i32,
    pub faction_group: i32,
    pub name_lang: ExtendedLocalizedString,
    pub shortcut_lang: ExtendedLocalizedString,
}

