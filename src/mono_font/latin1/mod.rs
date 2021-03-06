//! Latin1.
//!
//! TODO: docs

mod generated;

pub use generated::*;

fn char_offset(c: char) -> u32 {
    let c = c as u32;

    match c {
        0x20..=0x7F => c - 0x20,
        0xA0..=0xFF => c - 0xA0 + 0x60,
        _ => '?' as u32 - ' ' as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mono_font::tests::test_baseline;

    fn c(index: u32) -> char {
        core::char::from_u32(index).unwrap()
    }

    #[test]
    fn char_offset_valid() {
        assert_eq!(char_offset(c(0x20)), 0);
        assert_eq!(char_offset(c(0x40)), 2 * 16);
        assert_eq!(char_offset(c(0x7F)), 6 * 16 - 1);
        assert_eq!(char_offset(c(0xA0)), 6 * 16);
        assert_eq!(char_offset(c(0xD0)), 9 * 16);
        assert_eq!(char_offset(c(0xFF)), 12 * 16 - 1);
    }

    #[test]
    fn char_offset_fallback() {
        assert_eq!(char_offset(c(0x1F)), char_offset('?'));
        assert_eq!(char_offset(c(0x80)), char_offset('?'));
        assert_eq!(char_offset(c(0x9F)), char_offset('?'));
        assert_eq!(char_offset(c(0x100)), char_offset('?'));
    }

    #[test]
    fn baseline() {
        test_baseline(Font4x6);
        test_baseline(Font5x7);
        test_baseline(Font5x8);
        test_baseline(Font6x10);
        test_baseline(Font6x12);
        test_baseline(Font6x13Bold);
        test_baseline(Font6x13);
        test_baseline(Font6x13Italic);
        test_baseline(Font6x9);
        test_baseline(Font7x13Bold);
        test_baseline(Font7x13);
        test_baseline(Font7x13Italic);
        test_baseline(Font7x14Bold);
        test_baseline(Font7x14);
        test_baseline(Font8x13Bold);
        test_baseline(Font8x13);
        test_baseline(Font8x13Italic);
        test_baseline(Font9x15Bold);
        test_baseline(Font9x15);
        test_baseline(Font9x18Bold);
        test_baseline(Font9x18);
        test_baseline(Font10x20);
    }
}
