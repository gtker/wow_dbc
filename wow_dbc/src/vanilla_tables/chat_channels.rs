use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::LocalizedString;
use crate::vanilla_tables::faction_group::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChatChannels {
    pub rows: Vec<ChatChannelsRow>,
}

impl DbcTable for ChatChannels {
    type Row = ChatChannelsRow;

    fn filename() -> &'static str { "ChatChannels.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 84 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 84,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 21 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 21,
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

            // id: primary_key (ChatChannels) uint32
            let id = ChatChannelsKey::new(crate::util::read_u32_le(chunk)?);

            // flags: ChannelFlags
            let flags = ChannelFlags::new(crate::util::read_i32_le(chunk)?);

            // faction_group: foreign_key (FactionGroup) uint32
            let faction_group = FactionGroupKey::new(crate::util::read_u32_le(chunk)?.into());

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // shortcut: string_ref_loc
            let shortcut = crate::util::read_localized_string(chunk, &string_block)?;


            rows.push(ChatChannelsRow {
                id,
                flags,
                faction_group,
                name,
                shortcut,
            });
        }

        Ok(ChatChannels { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 21,
            record_size: 84,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (ChatChannels) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // flags: ChannelFlags
            b.write_all(&(row.flags.as_int() as i32).to_le_bytes())?;

            // faction_group: foreign_key (FactionGroup) uint32
            b.write_all(&(row.faction_group.id as u32).to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // shortcut: string_ref_loc
            b.write_all(&row.shortcut.string_indices_as_array(&mut string_index))?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for ChatChannels {
    type PrimaryKey = ChatChannelsKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl ChatChannels {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name.string_block_as_array(b)?;
            row.shortcut.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
            sum += row.shortcut.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct ChatChannelsKey {
    pub id: u32
}

impl ChatChannelsKey {
    pub const fn new(id: u32) -> Self {
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

impl From<u32> for ChatChannelsKey {
    fn from(v: u32) -> Self {
        Self::new(v)
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct ChannelFlags {
    value: i32,
}

impl ChannelFlags {
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    pub const fn as_int(&self) -> i32 {
        self.value
    }

    pub const fn none(&self) -> bool {
        self.value == 0
    }

    pub const fn initial(&self) -> bool {
        (self.value & 1) != 0
    }

    pub const fn zone_dependency(&self) -> bool {
        (self.value & 2) != 0
    }

    pub const fn global(&self) -> bool {
        (self.value & 4) != 0
    }

    pub const fn trade(&self) -> bool {
        (self.value & 8) != 0
    }

    pub const fn city_only(&self) -> bool {
        (self.value & 16) != 0
    }

    pub const fn city_only2(&self) -> bool {
        (self.value & 32) != 0
    }

    pub const fn defence(&self) -> bool {
        (self.value & 65536) != 0
    }

    pub const fn unselected(&self) -> bool {
        (self.value & 262144) != 0
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChatChannelsRow {
    pub id: ChatChannelsKey,
    pub flags: ChannelFlags,
    pub faction_group: FactionGroupKey,
    pub name: LocalizedString,
    pub shortcut: LocalizedString,
}

