//! Escape sequences of TOML basic strings.

/// Decodes the escape sequences of the content of a basic string.
///
/// A backslash at the end of a line removes itself and all whitespace and
/// newlines that follow it. Invalid escape sequences are kept verbatim; the
/// parser reports them.
pub fn unescape_basic_string(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut rest = text;

    while let Some(index) = rest.find('\\') {
        result.push_str(&rest[..index]);
        let escape = &rest[index..];
        let mut chars = escape[1..].chars();

        let consumed = match chars.next() {
            Some('b') => push(&mut result, '\u{8}', 2),
            Some('t') => push(&mut result, '\t', 2),
            Some('n') => push(&mut result, '\n', 2),
            Some('f') => push(&mut result, '\u{c}', 2),
            Some('r') => push(&mut result, '\r', 2),
            Some('e') => push(&mut result, '\u{1b}', 2),
            Some('"') => push(&mut result, '"', 2),
            Some('\\') => push(&mut result, '\\', 2),
            Some('x') => push_code_point(&mut result, escape, 2),
            Some('u') => push_code_point(&mut result, escape, 4),
            Some('U') => push_code_point(&mut result, escape, 8),
            Some(' ' | '\t' | '\n' | '\r') => {
                let trimmed = escape[1..].trim_start_matches([' ', '\t', '\n', '\r']);
                let skipped = &escape[1..escape.len() - trimmed.len()];
                if skipped.contains('\n') {
                    escape.len() - trimmed.len()
                } else {
                    result.push('\\');
                    1
                }
            }
            _ => {
                result.push('\\');
                1
            }
        };

        rest = &escape[consumed..];
    }

    result.push_str(rest);
    result
}

fn push(result: &mut String, char: char, consumed: usize) -> usize {
    result.push(char);
    consumed
}

/// Decodes `\xHH`, `\uHHHH`, or `\UHHHHHHHH` at the start of `escape` and
/// returns the number of bytes consumed.
fn push_code_point(result: &mut String, escape: &str, digits: usize) -> usize {
    let decoded = escape
        .get(2..2 + digits)
        .filter(|hex| hex.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .and_then(|hex| u32::from_str_radix(hex, 16).ok())
        .and_then(char::from_u32);

    match decoded {
        Some(char) => push(result, char, 2 + digits),
        None => {
            result.push('\\');
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::unescape_basic_string;

    #[test]
    fn unescapes_simple_escapes() {
        assert_eq!(
            unescape_basic_string(r#"a\tb\n\"c\"\\\e\b\f\r"#),
            "a\tb\n\"c\"\\\u{1b}\u{8}\u{c}\r"
        );
    }

    #[test]
    fn unescapes_code_points() {
        assert_eq!(
            unescape_basic_string(r"\x41é\U0001F600"),
            "A\u{e9}\u{1F600}"
        );
    }

    #[test]
    fn trims_line_ending_backslash() {
        assert_eq!(unescape_basic_string("a \\  \n  \n\tb"), "a b");
        assert_eq!(unescape_basic_string("a\\\r\nb"), "ab");
    }

    #[test]
    fn keeps_invalid_escapes() {
        assert_eq!(unescape_basic_string(r"\q\u12"), r"\q\u12");
        assert_eq!(unescape_basic_string(r"\ x"), r"\ x");
        assert_eq!(unescape_basic_string(r"\uD800"), r"\uD800");
    }
}
