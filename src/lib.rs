//! "Fast" encoding of codepoints into UTF-8.
//!
//! # Features
//!
//! * `looped`: runs with a single while loop.
//! * `tabled`: runs with a table-driven implementation.
//!   Overrides `looped`.

#[cfg(feature = "tabled")]
mod tables;

/// Encode a Unicode `char` into UTF-8.
///
/// Returns an array of UTF-8 bytes and a count of how many
/// are valid (1-4).  Invalid bytes are
/// implementation-dependent.
pub const fn branchless_utf8(codepoint: char) -> ([u8; 4], usize) {
    #[cfg(feature = "tabled")] {
        use tables::*;

        let len = utf8_bytes_for_codepoint(codepoint);
        let codepoint = codepoint as u32;
        let buf = [
            PREFIX[len][0] | ((codepoint >> SHIFT[len][0]) & MASK[len][0] as u32) as u8,
            PREFIX[len][1] | ((codepoint >> SHIFT[len][1]) & MASK[len][1] as u32) as u8,
            PREFIX[len][2] | ((codepoint >> SHIFT[len][2]) & MASK[len][2] as u32) as u8,
            PREFIX[len][3] | ((codepoint >> SHIFT[len][3]) & MASK[len][3] as u32) as u8,
        ];
        (buf, len)
    }

    #[cfg(not(feature = "tabled"))] {
        let (word, len) = utf8_word(codepoint);
        (word.to_be_bytes(), len as usize)
    }
}


/// Encode a Unicode `char` into UTF-8.
///
/// Returns a count of valid bytes (1-4), and a 32-bit word
/// representing the bytes left-justified. Invalid byte
/// positions are implementation-dependent.
pub const fn utf8_word(codepoint: char) -> (u32, u32) {
    let codepoint = codepoint as u32;
    let p2 = (codepoint > 0x7f) as u32;
    let p3 = (codepoint > 0x7ff) as u32;
    let p4 = (codepoint > 0xffff) as u32;
    let pbits = p2 + p3 + p4;
    let plen = p2 + pbits;
    let len = 1 + pbits;

    let mut result = 0;

    #[cfg(not(feature = "looped"))] {
        macro_rules! iter {
            ($i:literal, $mask:literal) => {{
                let shift = 6 * (5 - len) - 2 * $i;
                (codepoint << shift) & ($mask >> (8 * $i))
            }};
        }
        result |= iter!(0, 0xff000000);
        result |= iter!(1, 0x3f000000);
        result |= iter!(2, 0x3f000000);
        result |= iter!(3, 0x3f000000);
    }
        
    #[cfg(feature = "looped")] {
        let mut i = 0;
        let mut mask = 0xff000000;
        while i < len {
            let shift = 6 * (5 - len) - 2 * i;
            result |= (codepoint << shift) & (mask >> (8 * i));
            mask = 0x3f000000;
            i += 1;
        }
    }

    result |= 0x00808080;
    let prefix = ((1 << (plen + 1)) - 2) << (31 - plen);
    result |= prefix;

    (result, len)
}

/// Return the number of bytes required to encode the provided `char`.
pub const fn utf8_bytes_for_codepoint(codepoint: char) -> usize {
    // The `char` type provides a proof that the value is in-bounds and not a surrogate.
    // We can "lower" here while relying on the proof from the type signature.
    let codepoint = codepoint as u32;

    let mut len = 1;
    // In Rust, true casts to 1 and false to 0, so we can "just" sum lengths.
    len += (codepoint > 0x7f) as usize;
    len += (codepoint > 0x7ff) as usize;
    len += (codepoint > 0xffff) as usize;

    len
}

#[cfg(test)]
mod tests {
    use crate::{branchless_utf8, utf8_bytes_for_codepoint};

    #[test]
    fn length() {
        for i in 0..=0xFF_FFFF {
            let c = match char::from_u32(i) {
                Some(c) => c,
                None => continue,
            };
            let got = utf8_bytes_for_codepoint(c);
            let want = c.to_string().as_bytes().len();
            assert_eq!(want, got, "{i:x}: {want} != {got}")
        }
    }

    #[test]
    fn branchless_same() {
        for i in 0..=0xFF_FFFF {
            let c = match char::from_u32(i) {
                Some(c) => c,
                None => continue,
            };
            let (bytes, len) = branchless_utf8(c);
            let got = &bytes[..len];
            let want = c.to_string().as_bytes().to_owned();
            assert_eq!(want, got, "{i:x}: {want:?} != {got:?}");
        }
    }
}
