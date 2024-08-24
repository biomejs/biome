use std::borrow::Cow;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Quote {
    Double,
    Single,
}

impl Quote {
    pub fn as_char(&self) -> char {
        match self {
            Quote::Double => '"',
            Quote::Single => '\'',
        }
    }

    pub fn as_byte(&self) -> u8 {
        self.as_char() as u8
    }

    /// Given the current quote, it returns the other one
    pub fn other(&self) -> Self {
        match self {
            Quote::Double => Quote::Single,
            Quote::Single => Quote::Double,
        }
    }
}

/// This function is responsible of:
///
/// - escape `preferred_quote`
/// - unescape alternate quotes of `preferred_quote`
/// - remove unneed escapes (if `is_escape_preserved` is `true`)
/// - normalise the new lines by replacing `\r\n` with `\n`.
///
/// The function allocates a new string only if at least one change is performed.
///
/// In the followinf example `"` is escaped, `\'` and `\l` are unescaped, and the newline is normalized.
///
/// ```
/// use biome_formatter::token::string::{normalize_string, Quote};
/// assert_eq!(
///     normalize_string(" \"He\\llo\\tworld\" \\' \\' \r\n ", Quote::Double, false),
///     " \\\"Hello\\tworld\\\" ' ' \n ",
/// );
/// ```
pub fn normalize_string(
    raw_content: &str,
    preferred_quote: Quote,
    is_escape_preserved: bool,
) -> Cow<str> {
    let alternate_quote = preferred_quote.other().as_byte();
    let preferred_quote = preferred_quote.as_byte();
    let mut reduced_string = String::new();
    let mut copy_start = 0;
    let mut bytes = raw_content.bytes().enumerate();
    while let Some((byte_index, byte)) = bytes.next() {
        match byte {
            // If the next character is escaped
            b'\\' => {
                if let Some((escaped_index, escaped)) = bytes.next() {
                    // If we encounter an alternate quote that is escaped, we have to
                    // remove the escape from it.
                    // This is done because of how the enclosed strings can change.
                    // Check `computed_preferred_quote` for more details.
                    let should_unescape = match escaped {
                        // The next character is another backslash, or
                        // a character that should be kept in the next iteration
                        b'^'
                        | b'\n'
                        | b'0'..=b'7'
                        | b'\\'
                        | b'b'
                        | b'f'
                        | b'n'
                        | b'r'
                        | b't'
                        | b'u'
                        | b'v'
                        | b'x' => false,
                        b'\r' => {
                            // If we encounter the sequence "\r\n", then skip '\r'
                            if let Some((next_byte_index, b'\n')) = bytes.next() {
                                reduced_string.push_str(&raw_content[copy_start..escaped_index]);
                                copy_start = next_byte_index;
                            }
                            false
                        }
                        0xE2 => {
                            // Prserve escaping of Unicode characters U+2028 and U+2029
                            !(matches!(bytes.next(), Some((_, 0x80)))
                                && matches!(bytes.next(), Some((_, 0xA8 | 0xA9))))
                        }
                        _ => {
                            // these, usually characters that can have their
                            // escape removed: "\a" => "a"
                            // So we ignore the current slash and we continue
                            // to the next iteration
                            //
                            // We always unescape alternate quots regardless of `is_escape_preserved`.
                            escaped == alternate_quote
                                || (escaped != preferred_quote && !is_escape_preserved)
                        }
                    };
                    if should_unescape {
                        reduced_string.push_str(&raw_content[copy_start..byte_index]);
                        copy_start = escaped_index;
                    }
                }
            }
            // If we encounter the sequence "\r\n", then skip '\r'
            b'\r' => {
                if let Some((next_byte_index, b'\n')) = bytes.next() {
                    reduced_string.push_str(&raw_content[copy_start..byte_index]);
                    copy_start = next_byte_index;
                }
            }
            _ => {
                // If we encounter a preferred quote and it's not escaped, we have to replace it with
                // an escaped version.
                // This is done because of how the enclosed strings can change.
                // Check `computed_preferred_quote` for more details.
                if byte == preferred_quote {
                    reduced_string.push_str(&raw_content[copy_start..byte_index]);
                    reduced_string.push('\\');
                    copy_start = byte_index;
                }
            }
        }
    }
    if copy_start == 0 && reduced_string.is_empty() {
        Cow::Borrowed(raw_content)
    } else {
        // Copy the remaining characters
        reduced_string.push_str(&raw_content[copy_start..]);
        Cow::Owned(reduced_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_newline() {
        assert_eq!(normalize_string("a\nb", Quote::Double, true), "a\nb");
        assert_eq!(normalize_string("a\r\nb", Quote::Double, false), "a\nb");
        assert_eq!(normalize_string("a\\\r\nb", Quote::Double, false), "a\\\nb");
    }

    #[test]
    fn normalize_escapes() {
        assert_eq!(normalize_string("\\", Quote::Double, false), "\\");
        assert_eq!(normalize_string("\\t", Quote::Double, false), "\\t");
        assert_eq!(
            normalize_string("\\\u{2028}", Quote::Double, false),
            "\\\u{2028}"
        );
        assert_eq!(
            normalize_string("\\\u{2029}", Quote::Double, false),
            "\\\u{2029}"
        );

        assert_eq!(normalize_string("a\\a", Quote::Double, false), "aa");
        assert_eq!(normalize_string("👍\\👍", Quote::Single, false), "👍👍");
        assert_eq!(
            normalize_string("\\\u{2027}", Quote::Double, false),
            "\u{2027}"
        );
        assert_eq!(
            normalize_string("\\\u{2030}", Quote::Double, false),
            "\u{2030}"
        );

        assert_eq!(normalize_string("a\\a", Quote::Double, true), "a\\a");
        assert_eq!(normalize_string("👍\\👍", Quote::Single, true), "👍\\👍");
    }

    #[test]
    fn normalize_quotes() {
        assert_eq!(normalize_string("\"", Quote::Double, false), "\\\"");
        assert_eq!(normalize_string("\'", Quote::Double, false), "'");
        assert_eq!(normalize_string("\\'", Quote::Double, false), "'");

        assert_eq!(normalize_string("\"", Quote::Single, false), "\"");
        assert_eq!(normalize_string("\\'", Quote::Single, false), "\\'");
        assert_eq!(normalize_string("\\\"", Quote::Single, false), "\"");
    }
}
