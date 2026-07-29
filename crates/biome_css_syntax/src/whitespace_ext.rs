/// Returns whether `byte` is a CSS newline byte.
///
/// CSS treats line feed, carriage return, and form feed as newlines.
pub const fn is_css_newline_byte(byte: u8) -> bool {
    matches!(byte, b'\n' | b'\r' | 0x0c)
}

/// Returns whether `byte` is horizontal CSS whitespace.
///
/// This excludes CSS newline bytes and accepts only space and tab.
pub const fn is_css_horizontal_whitespace_byte(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t')
}

/// Returns whether `byte` is CSS whitespace.
pub const fn is_css_whitespace_byte(byte: u8) -> bool {
    is_css_horizontal_whitespace_byte(byte) || is_css_newline_byte(byte)
}

#[cfg(test)]
mod tests {
    use super::{is_css_horizontal_whitespace_byte, is_css_newline_byte, is_css_whitespace_byte};

    #[test]
    fn classifies_css_newlines() {
        for byte in [b'\n', b'\r', 0x0c] {
            assert!(is_css_newline_byte(byte));
            assert!(is_css_whitespace_byte(byte));
            assert!(!is_css_horizontal_whitespace_byte(byte));
        }
    }

    #[test]
    fn classifies_horizontal_css_whitespace() {
        for byte in [b' ', b'\t'] {
            assert!(is_css_horizontal_whitespace_byte(byte));
            assert!(is_css_whitespace_byte(byte));
            assert!(!is_css_newline_byte(byte));
        }
    }

    #[test]
    fn rejects_other_bytes() {
        for byte in [b'a', b'\x0b', b'\0'] {
            assert!(!is_css_newline_byte(byte));
            assert!(!is_css_horizontal_whitespace_byte(byte));
            assert!(!is_css_whitespace_byte(byte));
        }
    }
}
