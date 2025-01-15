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
pub fn branchless_utf8(codepoint: u32) -> ([u8; 4], usize) {
    let len = utf8_bytes_for_codepoint(codepoint);
    let buf = [
        PREFIX[len][0] | (codepoint >> SHIFT[len][0] & MASK[len][0] as u32) as u8,
        PREFIX[len][1] | ((codepoint >> SHIFT[len][1]) & MASK[len][1] as u32) as u8,
        PREFIX[len][2] | ((codepoint >> SHIFT[len][2]) & MASK[len][2] as u32) as u8,
        PREFIX[len][3] | ((codepoint >> SHIFT[len][3]) & MASK[len][3] as u32) as u8,
    ];

    (buf, len)
}

const fn utf8_bytes_for_codepoint(codepoint: u32) -> usize {
    LEN[codepoint.leading_zeros() as usize] as usize
}

type Table = [[u8; 4]; 5];

/// Length, based on the number of leading zeros.
const LEN: [u8; 33] = [
    //0-11 leading zeros: not valid
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
    // 8-15 leading zeros: 4 bytes
    4, 4, 4, 4, 4, //
    //16-20 leading zeros: 3 bytes
    3, 3, 3, 3, 3, //
    // 21-24 leading zeros: 2 bytes
    2, 2, 2, 2, //
    // 25-32 leading zeros: 1 byte
    1, 1, 1, 1, 1, 1, 1, 1,
];

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
        for i in 0..0x10FFFF {
            let c = match char::from_u32(i) {
                None => continue,
                Some(c) => c,
            };
            let want = c.to_string().as_bytes().len();
            let got = utf8_bytes_for_codepoint(i);
            assert_eq!(want, got, "{i:x}: {want} != {got}")
        }
    }

    #[test]
    fn branchless_one_byte() {
        for i in 0..=0x7f {
            let (bytes, len) = branchless_utf8(i);
            assert_eq!(len, 1);
            let slice = &bytes[0..len];
            let s = std::str::from_utf8(slice).unwrap_or_else(|_| panic!("{i}"));
            assert_eq!(s.len(), 1, "{i}");
            assert_eq!(s.chars().next().unwrap() as u32, i);
        }
    }

    #[test]
    fn branchless_two_bytes() {
        for i in 0x80..=0x07ff {
            let (bytes, len) = branchless_utf8(i);
            let slice = &bytes[0..len];
            let s = std::str::from_utf8(slice).unwrap_or_else(|_| panic!("{i}"));
            assert_eq!(s.chars().next().unwrap() as u32, i);
        }
    }
    #[test]
    fn branchless_three_bytes() {
        for i in 0x800..=0xffff {
            // TODO: Test surrogate codepoints
            let c = match char::from_u32(i) {
                None => continue,
                Some(c) => c,
            };

            let (bytes, len) = branchless_utf8(i);
            let got = &bytes[0..len];

            let want = c.to_string().as_bytes().to_owned();
            assert_eq!(&want, got, "{i:x}");
        }
    }
    #[test]
    fn branchless_four_bytes() {
        for i in 0x01_0000..=0x10_ffff {
            let c = match char::from_u32(i) {
                None => continue,
                Some(c) => c,
            };

            let (bytes, len) = branchless_utf8(i);
            let got = &bytes[0..len];

            let want = c.to_string().as_bytes().to_owned();
            assert_eq!(&want, got, "{i:x}");
        }
    }
}
