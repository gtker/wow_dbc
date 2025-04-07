use std::collections::HashMap;
#[cfg(any(feature = "tbc", feature = "wrath"))]
use crate::tys::ExtendedLocalizedString;

use std::io::Read;

pub fn read_u8_le(b: &mut &[u8]) -> Result<u8, std::io::Error> {
    let mut buf = [0_u8; 1];
    b.read_exact(&mut buf)?;

    Ok(buf[0])
}

pub fn read_u32_le(b: &mut &[u8]) -> Result<u32, std::io::Error> {
    let mut buf = [0_u8; 4];
    b.read_exact(&mut buf)?;

    Ok(u32::from_le_bytes(buf))
}

pub fn read_i8_le(b: &mut &[u8]) -> Result<i8, std::io::Error> {
    let mut buf = [0_u8; 1];
    b.read_exact(&mut buf)?;

    Ok(i8::from_le_bytes(buf))
}

pub fn read_i32_le(b: &mut &[u8]) -> Result<i32, std::io::Error> {
    let mut buf = [0_u8; 4];
    b.read_exact(&mut buf)?;

    Ok(i32::from_le_bytes(buf))
}

pub fn read_f32_le(b: &mut &[u8]) -> Result<f32, std::io::Error> {
    let mut buf = [0_u8; 4];
    b.read_exact(&mut buf)?;

    Ok(f32::from_le_bytes(buf))
}

pub fn get_string_as_vec(b: &mut &[u8], string_block: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let start = read_u32_le(b)? as usize;

    if start == 0 {
        return Ok(Vec::new());
    }

    let end = {
        let mut i = 0;
        while string_block[start + i] != 0 {
            i += 1;
        }
        start + i
    };

    Ok(string_block[start..end].to_vec())
}

#[cfg(feature = "vanilla")]
pub fn read_localized_string(
    chunk: &mut &[u8],
    string_block: &[u8],
) -> Result<crate::tys::LocalizedString, crate::DbcError> {
    let en_gb = get_string_as_vec(chunk, string_block)?;
    let en_gb = String::from_utf8(en_gb)?;

    let ko_kr = get_string_as_vec(chunk, string_block)?;
    let ko_kr = String::from_utf8(ko_kr)?;

    let fr_fr = get_string_as_vec(chunk, string_block)?;
    let fr_fr = String::from_utf8(fr_fr)?;

    let de_de = get_string_as_vec(chunk, string_block)?;
    let de_de = String::from_utf8(de_de)?;

    let en_cn = get_string_as_vec(chunk, string_block)?;
    let en_cn = String::from_utf8(en_cn)?;

    let en_tw = get_string_as_vec(chunk, string_block)?;
    let en_tw = String::from_utf8(en_tw)?;

    let es_es = get_string_as_vec(chunk, string_block)?;
    let es_es = String::from_utf8(es_es)?;

    let es_mx = get_string_as_vec(chunk, string_block)?;
    let es_mx = String::from_utf8(es_mx)?;

    let flags = read_u32_le(chunk)?;

    Ok(crate::tys::LocalizedString::new(
        en_gb, ko_kr, fr_fr, de_de, en_cn, en_tw, es_es, es_mx, flags,
    ))
}

#[cfg(any(feature = "tbc", feature = "wrath"))]
pub fn read_extended_localized_string(
    chunk: &mut &[u8],
    string_block: &[u8],
) -> Result<ExtendedLocalizedString, crate::DbcError> {
    let en_gb = get_string_as_vec(chunk, string_block)?;
    let en_gb = String::from_utf8(en_gb)?;

    let ko_kr = get_string_as_vec(chunk, string_block)?;
    let ko_kr = String::from_utf8(ko_kr)?;

    let fr_fr = get_string_as_vec(chunk, string_block)?;
    let fr_fr = String::from_utf8(fr_fr)?;

    let de_de = get_string_as_vec(chunk, string_block)?;
    let de_de = String::from_utf8(de_de)?;

    let en_cn = get_string_as_vec(chunk, string_block)?;
    let en_cn = String::from_utf8(en_cn)?;

    let en_tw = get_string_as_vec(chunk, string_block)?;
    let en_tw = String::from_utf8(en_tw)?;

    let es_es = get_string_as_vec(chunk, string_block)?;
    let es_es = String::from_utf8(es_es)?;

    let es_mx = get_string_as_vec(chunk, string_block)?;
    let es_mx = String::from_utf8(es_mx)?;

    let ru_ru = get_string_as_vec(chunk, string_block)?;
    let ru_ru = String::from_utf8(ru_ru)?;

    let ja_jp = get_string_as_vec(chunk, string_block)?;
    let ja_jp = String::from_utf8(ja_jp)?;

    let pt_pt = get_string_as_vec(chunk, string_block)?;
    let pt_pt = String::from_utf8(pt_pt)?;

    let it_it = get_string_as_vec(chunk, string_block)?;
    let it_it = String::from_utf8(it_it)?;

    let unknown_12 = get_string_as_vec(chunk, string_block)?;
    let unknown_12 = String::from_utf8(unknown_12)?;

    let unknown_13 = get_string_as_vec(chunk, string_block)?;
    let unknown_13 = String::from_utf8(unknown_13)?;

    let unknown_14 = get_string_as_vec(chunk, string_block)?;
    let unknown_14 = String::from_utf8(unknown_14)?;

    let unknown_15 = get_string_as_vec(chunk, string_block)?;
    let unknown_15 = String::from_utf8(unknown_15)?;

    let flags = read_u32_le(chunk)?;

    Ok(ExtendedLocalizedString::new(
        en_gb, ko_kr, fr_fr, de_de, en_cn, en_tw, es_es, es_mx, ru_ru, ja_jp, pt_pt, it_it,
        unknown_12, unknown_13, unknown_14, unknown_15, flags,
    ))
}

pub fn read_array_f32<const N: usize>(chunk: &mut &[u8]) -> Result<[f32; N], std::io::Error> {
    let mut arr = [f32::default(); N];
    for i in arr.iter_mut() {
        *i = read_f32_le(chunk)?;
    }

    Ok(arr)
}

pub fn read_array_u32<const N: usize>(chunk: &mut &[u8]) -> Result<[u32; N], std::io::Error> {
    let mut arr = [u32::default(); N];
    for i in arr.iter_mut() {
        *i = read_u32_le(chunk)?;
    }

    Ok(arr)
}

pub fn read_array_i32<const N: usize>(chunk: &mut &[u8]) -> Result<[i32; N], std::io::Error> {
    let mut arr = [i32::default(); N];
    for i in arr.iter_mut() {
        *i = read_i32_le(chunk)?;
    }

    Ok(arr)
}

/// This struct implements a string cache for writing the string block of a DBC file.
///
/// It maintains an in-memory buffer of the final string block and a mapping of strings to their offset in the buffer.
pub struct StringCache {
    /// Mapping of already written strings to their offsets in the buffer.
    offsets: HashMap<String, u32>,

    /// The buffer that contains the final string block.
    buffer: Vec<u8>,
}


impl StringCache {
    /// Creates a new `StringCache`.
    pub fn new() -> Self {
        let mut new = Self {
            offsets: HashMap::new(),
            buffer: Vec::new(),
        };

        // add an empty string, which makes empty strings reference 0
        new.buffer.push(0_u8);
        new.offsets.insert("".to_string(), 0);

        new
    }

    /// Adds a string to the cache and returns its offset in the buffer.
    ///
    /// If the string already exists in the cache, it returns the existing offset.
    pub fn add_string(&mut self, s: &str) -> u32 {

        // if offset already exists, return it
        if let Some(offset) = self.offsets.get(s) {
            return *offset;
        }

        // if the string does not exist, we add it to the cache
        let offset = self.buffer.len() as u32;
        self.buffer.extend_from_slice(s.as_bytes());

        // null-terminate the string
        self.buffer.push(0_u8);

        // and then we add an offset for the string and all of it's suffixes
        // this allows us to reuse suffixes of strings, like "abc" and "bc", which both end with "bc"
        // the original dbc files do this as well but not as thoroughly
        // there are instances where suffixes stand alone, like "23" after "123" is already in the cache
        // this means that bit perfect recreation is not possible
        for i in 0..s.len() {
            // we need to check if the resulting substring is a valid UTF-8 string, if not, we skip it
            // this happens if we are dealing with a multibyte character in a localized string

            if !s.is_char_boundary(i) {
                continue;
            }

            let suffix = &s[i..];
            self.offsets.insert(suffix.to_string(), offset + i as u32);
        }

        offset
    }

    /// Returns the size of the string block.
    pub fn size(&self) -> u32 {
        self.buffer.len() as u32
    }

    /// Returns the buffer containing the string block.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_cache() {
        let mut cache = StringCache::new();
        assert_eq!(cache.add_string("Hello"), 1);
        assert_eq!(cache.add_string("World"), 7);
        assert_eq!(cache.add_string("Hello"), 1);
        assert_eq!(cache.size(), 13);
    }

    #[test]
    fn test_empty_string_cache() {
        let mut cache = StringCache::new();
        assert_eq!(cache.size(), 1);
        assert_eq!(cache.buffer().len(), 1);
    }

    #[test]
    fn test_empty_string() {
        let mut cache = StringCache::new();
        assert_eq!(cache.add_string(""), 0);
        assert_eq!(cache.size(), 1);
        assert_eq!(cache.buffer().len(), 1);
        assert_eq!(cache.buffer()[0], 0);
    }

    #[test]
    fn test_suffix_match() {
        let mut cache = StringCache::new();

        // all of these strings have the same suffixes and will point to the first "abc"
        assert_eq!(cache.add_string("abc"), 1);
        assert_eq!(cache.add_string("bc"), 2);
        assert_eq!(cache.add_string("c"), 3);

        // shares the same prefix, but different suffix, not a match
        assert_eq!(cache.add_string("a"), 5);

        // while "123" would contain both "23" and "3", if they are added in reverse order, they are not a match
        assert_eq!(cache.add_string("3"), 7);
        assert_eq!(cache.add_string("23"), 9);
        assert_eq!(cache.add_string("123"), 12);

        assert_eq!(cache.size(), 16);
        assert_eq!(cache.buffer().len(), 16);
    }
}
