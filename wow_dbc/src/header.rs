use crate::InvalidHeaderError;

pub const HEADER_SIZE: usize = 4 * 5;
const HEADER_MAGIC: u32 = 0x43424457;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DbcHeader {
    pub record_count: u32,
    pub field_count: u32,
    pub record_size: u32,
    pub string_block_size: u32,
}

#[cfg(test)]
pub(crate) const fn parse_header_panic(header: &[u8]) -> DbcHeader {
    let magic = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
    if magic != HEADER_MAGIC {
        panic!("Header magic does not match");
    }

    let record_count = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
    let field_count = u32::from_le_bytes([header[8], header[9], header[10], header[11]]);
    let record_size = u32::from_le_bytes([header[12], header[13], header[14], header[15]]);
    let string_block_size = u32::from_le_bytes([header[16], header[17], header[18], header[19]]);

    DbcHeader {
        record_count,
        field_count,
        record_size,
        string_block_size,
    }
}

pub const fn parse_header(header: &[u8; HEADER_SIZE]) -> Result<DbcHeader, InvalidHeaderError> {
    let magic = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
    if magic != HEADER_MAGIC {
        return Err(InvalidHeaderError::MagicValue { actual: magic });
    }

    let record_count = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
    let field_count = u32::from_le_bytes([header[8], header[9], header[10], header[11]]);
    let record_size = u32::from_le_bytes([header[12], header[13], header[14], header[15]]);
    let string_block_size = u32::from_le_bytes([header[16], header[17], header[18], header[19]]);

    Ok(DbcHeader {
        record_count,
        field_count,
        record_size,
        string_block_size,
    })
}

impl DbcHeader {
    pub fn write_header(&self) -> [u8; HEADER_SIZE] {
        let mut header = [0_u8; HEADER_SIZE];

        let fields = [
            HEADER_MAGIC.to_le_bytes(),
            self.record_count.to_le_bytes(),
            self.field_count.to_le_bytes(),
            self.record_size.to_le_bytes(),
            self.string_block_size.to_le_bytes(),
        ];
        let mut index = 0;

        for field in fields {
            header[index] = field[0];
            header[index + 1] = field[1];
            header[index + 2] = field[2];
            header[index + 3] = field[3];
            index += 4;
        }

        header
    }
}
