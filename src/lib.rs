//! Branchless encoding of codepoints into UTF-8.
//!
//! Thanks to Nathan for the question and Lorenz for the key insight.
//! Any remaining mistakes are mine.
//!
//! See https://github.com/skeeto/branchless-utf8 for decoding.

/// Encode a UTF-8 codepoint.
/// Returns a buffer and the number of valid bytes in the buffer.
///
/// To add this codepoint to a string, append all four bytes in order,
/// and record that (usize) bytes were added to the string.
///
/// Returns a length of zero for invalid codepoints (surrogates and out-of-bounds values).
pub fn branchless_utf8(codepoint: u32) -> ([u8; 4], usize) {
    let len = utf8_bytes_for_codepoint(codepoint);
    let buf = [
        PREFIX[len][0] | ((codepoint >> SHIFT[len][0]) & MASK[len][0] as u32) as u8,
        PREFIX[len][1] | ((codepoint >> SHIFT[len][1]) & MASK[len][1] as u32) as u8,
        PREFIX[len][2] | ((codepoint >> SHIFT[len][2]) & MASK[len][2] as u32) as u8,
        PREFIX[len][3] | ((codepoint >> SHIFT[len][3]) & MASK[len][3] as u32) as u8,
    ];

    (buf, len)
}

const fn utf8_bytes_for_codepoint(codepoint: u32) -> usize {
    let mut len = 1;
    // In Rust, true casts to 1 and false to 0, so we can "just" sum lengths.
    len += (codepoint > 0x7f) as usize;
    len += (codepoint > 0x7ff) as usize;
    len += (codepoint > 0xffff) as usize;

    // Handle surrogates via bit-twiddling.
    // Rust guarantees true == 1 and false == 0:
    let surrogate_bit = ((codepoint >= 0xD800) && (codepoint <= 0xDFFF)) as usize;
    // Extend that one bit into three, and use its inverse as a mask for length
    let surrogate_mask = surrogate_bit << 2 | surrogate_bit << 1 | surrogate_bit;

    // Handle exceeded values via bit-twiddling.
    // Unfortunately, these don't align precisely with a leading-zero boundary;
    // the largest codepoint is U+10FFFF.
    let exceeded_bit = (codepoint > 0x10_FFFF) as usize;
    let exceeded_mask = exceeded_bit << 2 | exceeded_bit << 1 | exceeded_bit;

    len & !surrogate_mask & !exceeded_mask
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
            let got = utf8_bytes_for_codepoint(i);
            let want = char::from_u32(i)
                .map(|c| c.to_string().as_bytes().len())
                .unwrap_or(0);
            assert_eq!(want, got, "{i:x}: {want} != {got}")
        }
    }

    #[test]
    fn branchless_same() {
        for i in 0..0xFF_FFFF {
            let (bytes, len) = branchless_utf8(i);
            let got = &bytes[..len];
            let want = char::from_u32(i)
                .map(|c| c.to_string().as_bytes().to_owned())
                .unwrap_or(Vec::new());
            assert_eq!(&want, got);
        }
    }
}
