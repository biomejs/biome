use std::borrow::Cow;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Quote {
    Double,
    Single,
}

impl Quote {
    pub fn as_char(&self) -> char {
        match self {
            Self::Double => '"',
            Self::Single => '\'',
        }
    }

    pub fn as_byte(&self) -> u8 {
        self.as_char() as u8
    }

    /// Given the current quote, it returns the other one
    pub fn other(&self) -> Self {
        match self {
            Self::Double => Self::Single,
            Self::Single => Self::Double,
        }
    }
}

/// This function is responsible of:
///
/// - escaping `preferred_quote`
/// - unescape alternate quotes of `preferred_quote` if `quotes_will_change`
/// - normalise the new lines by replacing `\r\n` with `\n`.
///
/// The function allocates a new string only if at least one change is performed.
///
/// In the following example `"` is escaped and the newline is normalized.
///
/// ```
/// use biome_formatter::token::string::{normalize_string, Quote};
/// assert_eq!(
///     normalize_string(" \"He\\llo\\tworld\" \\' \\' \r\n ", Quote::Double, true),
///     " \\\"He\\llo\\tworld\\\" ' ' \n ",
/// );
/// ```
pub fn normalize_string(
    raw_content: &str,
    preferred_quote: Quote,
    quotes_will_change: bool,
) -> Cow<'_, str> {
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
                    if escaped == b'\r' {
                        // If we encounter the sequence "\r\n", then skip '\r'
                        if let Some((next_byte_index, b'\n')) = bytes.next() {
                            reduced_string.push_str(&raw_content[copy_start..escaped_index]);
                            copy_start = next_byte_index;
                        }
                    } else if quotes_will_change && escaped == alternate_quote {
                        // Unescape alternate quotes if quotes are changing
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
        assert_eq!(normalize_string("a\r\nb", Quote::Double, true), "a\nb");
        assert_eq!(normalize_string("a\\\r\nb", Quote::Double, true), "a\\\nb");
    }

    #[test]
    fn normalize_escapes() {
        assert_eq!(normalize_string("\\", Quote::Double, true), "\\");
        assert_eq!(normalize_string("\\t", Quote::Double, true), "\\t");
        assert_eq!(
            normalize_string("\\\u{2028}", Quote::Double, true),
            "\\\u{2028}"
        );
        assert_eq!(
            normalize_string("\\\u{2029}", Quote::Double, true),
            "\\\u{2029}"
        );

        assert_eq!(normalize_string(r"a\a", Quote::Double, true), r"a\a");
        assert_eq!(normalize_string(r"👍\👍", Quote::Single, true), r"👍\👍");
        assert_eq!(
            normalize_string("\\\u{2027}", Quote::Double, true),
            "\\\u{2027}"
        );
        assert_eq!(
            normalize_string("\\\u{2030}", Quote::Double, true),
            "\\\u{2030}"
        );
    }

    #[test]
    fn normalize_quotes() {
        assert_eq!(normalize_string("\"", Quote::Double, true), "\\\"");
        assert_eq!(normalize_string(r"\'", Quote::Double, true), r"'");

        assert_eq!(normalize_string(r"\'", Quote::Double, false), r"\'");
        assert_eq!(normalize_string("\"", Quote::Single, false), "\"");
        assert_eq!(normalize_string("\\'", Quote::Single, false), "\\'");
        assert_eq!(normalize_string("\\\"", Quote::Single, false), "\\\"");
    }
}
