pub type Table = [[u8; 4]; 5];

// Byte prefix for a continuation byte.
pub const CONTINUE: u8 = 0b1000_0000;
pub const PREFIX: Table = [
    [0u8; 4],
    [0, 0, 0, 0],
    [0b1100_0000, CONTINUE, 0, 0],
    [0b1110_0000, CONTINUE, CONTINUE, 0],
    [0b1111_0000, CONTINUE, CONTINUE, CONTINUE],
];

// We must arrange that the most-significant bytes are always in byte 0.
pub const SHIFT: Table = [
    [0u8; 4],
    [0, 0, 0, 0],
    [6, 0, 0, 0],
    [12, 6, 0, 0],
    [18, 12, 6, 0],
];

pub const MASK: Table = [
    [0u8; 4],
    [0x7f, 0, 0, 0],
    [0x1f, 0x3f, 0, 0],
    [0x0f, 0x3f, 0x3f, 0],
    [0x07, 0x3f, 0x3f, 0x3f],
];
