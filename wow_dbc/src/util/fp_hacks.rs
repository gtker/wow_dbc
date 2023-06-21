use std::num::FpCategory;
// STOLEN FROM STANDARD LIBRARY f32::from_bits in order to avoid having to use a nightly flag
// This is probably a bad idea, and it should be removed immediately after
// https://github.com/rust-lang/rust/issues/72447 is stabilized.

const fn classify_bits(b: u32) -> FpCategory {
    const EXP_MASK: u32 = 0x7f800000;
    const MAN_MASK: u32 = 0x007fffff;

    match (b & MAN_MASK, b & EXP_MASK) {
        (0, EXP_MASK) => FpCategory::Infinite,
        (_, EXP_MASK) => FpCategory::Nan,
        (0, 0) => FpCategory::Zero,
        (_, 0) => FpCategory::Subnormal,
        _ => FpCategory::Normal,
    }
}

// Changed signature to take array instead of u32
pub(crate) const fn ct_u32_to_f32(ct: [u8; 4]) -> f32 {
    let ct = u32::from_le_bytes(ct);

    match classify_bits(ct) {
        FpCategory::Subnormal => {
            panic!("const-eval error: cannot use f32::from_bits on a subnormal number")
        }
        FpCategory::Nan => {
            panic!("const-eval error: cannot use f32::from_bits on NaN")
        }
        FpCategory::Infinite | FpCategory::Normal | FpCategory::Zero => {
            // SAFETY: It's not a frumious number
            unsafe { std::mem::transmute::<u32, f32>(ct) }
        }
    }
}
