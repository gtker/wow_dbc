use crate::{DbcError, ExtendedLocalizedString, LocalizedString};
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

pub fn read_localized_string(
    chunk: &mut &[u8],
    string_block: &[u8],
) -> Result<LocalizedString, DbcError> {
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

    Ok(LocalizedString::new(
        en_gb, ko_kr, fr_fr, de_de, en_cn, en_tw, es_es, es_mx, flags,
    ))
}

pub fn read_extended_localized_string(
    chunk: &mut &[u8],
    string_block: &[u8],
) -> Result<ExtendedLocalizedString, DbcError> {
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
