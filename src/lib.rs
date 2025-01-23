//! Branchless encoding of codepoints into UTF-8.
//!
//! Thanks to Nathan for the question and Lorenz for the key insight.
//! Any remaining mistakes are mine.
//!
//! See https://github.com/skeeto/branchless-utf8 for decoding.

/// Encode a Unicode codepoint into UTF-8.
///
/// Returns an array of bytes and a count of how many are valid (1-4).
pub const fn branchless_utf8(codepoint: char) -> ([u8; 4], usize) {
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

/// Return the number of bytes required to encode the provided codepoint.
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

type Table = [[u8; 4]; 5];

// Byte prefix for a continuation byte.
const CONTINUE: u8 = 0b1000_0000;
const PREFIX: Table = [
    [0u8; 4],
    [0, 0, 0, 0],
    [0b1100_0000, CONTINUE, 0, 0],
    [0b1110_0000, CONTINUE, CONTINUE, 0],
    [0b1111_0000, CONTINUE, CONTINUE, CONTINUE],
];

// We must arrange that the most-significant bytes are always in byte 0.
const SHIFT: Table = [
    [0u8; 4],
    [0, 0, 0, 0],
    [6, 0, 0, 0],
    [12, 6, 0, 0],
    [18, 12, 6, 0],
];

const MASK: Table = [
    [0u8; 4],
    [0x7f, 0, 0, 0],
    [0x1f, 0x3f, 0, 0],
    [0x0f, 0x3f, 0x3f, 0],
    [0x07, 0x3f, 0x3f, 0x3f],
];

#[cfg(test)]
mod tests {
    use crate::{branchless_utf8, utf8_bytes_for_codepoint};

    #[test]
    fn length() {
        for i in 0..0xFF_FFFF {
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
        for i in 0..0xFF_FFFF {
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
