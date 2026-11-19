use std::borrow::Cow;

use biome_rowan::{Text, TokenText};

/// Returns the value of an identifier after decoding its Unicode escape sequences.
///
/// This decodes `\uXXXX` and `\u{...}` escapes without applying Unicode
/// normalization. If an escape is malformed or represents an invalid Unicode
/// scalar value, the original identifier is returned unchanged.
pub fn unescape_js_identifier(identifier: &str) -> Cow<'_, str> {
    let Some(first_escape) = identifier.find('\\') else {
        return Cow::Borrowed(identifier);
    };

    let mut result = String::with_capacity(identifier.len());
    let mut copied_until = 0;
    let mut escape_start = first_escape;

    loop {
        result.push_str(&identifier[copied_until..escape_start]);

        let Some((decoded, escape_end)) = decode_unicode_escape(identifier, escape_start) else {
            return Cow::Borrowed(identifier);
        };
        result.push(decoded);
        copied_until = escape_end;

        let Some(next_escape) = identifier[escape_end..].find('\\') else {
            result.push_str(&identifier[escape_end..]);
            return Cow::Owned(result);
        };
        escape_start = escape_end + next_escape;
    }
}

fn decode_unicode_escape(identifier: &str, start: usize) -> Option<(char, usize)> {
    let bytes = identifier.as_bytes();
    if bytes.get(start..start + 2)? != b"\\u" {
        return None;
    }

    if bytes.get(start + 2) == Some(&b'{') {
        let mut value = 0u32;
        let mut index = start + 3;
        let digits_start = index;

        while let Some(&byte) = bytes.get(index) {
            if byte == b'}' {
                if index == digits_start {
                    return None;
                }
                return char::from_u32(value).map(|decoded| (decoded, index + 1));
            }

            value = value.checked_mul(16)?.checked_add(hex_value(byte)?)?;
            if value > 0x10_ffff {
                return None;
            }
            index += 1;
        }
        None
    } else {
        let end = start + 6;
        let digits = bytes.get(start + 2..end)?;
        let mut value = 0u32;
        for &digit in digits {
            value = value * 16 + hex_value(digit)?;
        }
        char::from_u32(value).map(|decoded| (decoded, end))
    }
}

fn hex_value(byte: u8) -> Option<u32> {
    match byte {
        b'0'..=b'9' => Some(u32::from(byte - b'0')),
        b'a'..=b'f' => Some(u32::from(byte - b'a' + 10)),
        b'A'..=b'F' => Some(u32::from(byte - b'A' + 10)),
        _ => None,
    }
}

/// Returns `text` with escape sequences processed.
///
/// If nothing is escaped, `text` is returned without any allocation. If at
/// least one character is escaped, then a string is allocated and holds the
/// unescaped string.
///
/// This function must be called on the `inner_string_text()` of a token,
/// meaning the outer quotes are already expected to be trimmed.
pub fn unescape_js_string(text: TokenText) -> Text {
    enum State {
        // Consume characters until an escape sequence is discovered.
        Normal,
        // `\u{...}`
        Codepoint(u32),
        // Start of an escape sequence (`\...`).
        Escaped,
        // `\uXXXX`
        Hex4Digits(u8, u32),
        // `\xXX`
        HexEscape(u8, u8),
        // `\0...`
        LegacyOctal,
        // Skips one character.
        Skip,
    }

    match text.find('\\') {
        Some(index) => {
            let mut state = State::Escaped;
            let mut string = text[..index].to_string();
            string.reserve(usize::from(text.len()) - string.len());

            let remainder = &text[(index + 1)..];
            let mut next_byte_index = 0;
            for c in remainder.chars() {
                next_byte_index += c.len_utf8();

                match state {
                    State::Codepoint(char) => {
                        let value = match c {
                            '}' => {
                                string.push(char.try_into().unwrap_or('\u{fffd}'));
                                state = State::Normal;
                                continue;
                            }
                            c if c.is_ascii_digit() => c as u32 - '0' as u32,
                            c if ('a'..='f').contains(&c) => c as u32 - 'a' as u32 + 10,
                            c if ('A'..='F').contains(&c) => c as u32 - 'A' as u32 + 10,
                            _ => {
                                continue;
                            }
                        };

                        state = State::Codepoint(16 * char + value);
                    }
                    State::Escaped => {
                        let escaped = match c {
                            'b' => '\u{0008}',
                            'f' => '\u{000c}',
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            'u' => {
                                if remainder
                                    .as_bytes()
                                    .get(next_byte_index)
                                    .is_some_and(|lookahead| *lookahead == b'{')
                                {
                                    state = State::Codepoint(0);
                                } else {
                                    state = State::Hex4Digits(0, 0);
                                }
                                continue;
                            }
                            'v' => '\u{000b}',
                            'x' => {
                                state = State::HexEscape(0, 0);
                                continue;
                            }
                            '0' => {
                                if remainder
                                    .as_bytes()
                                    .get(next_byte_index)
                                    .is_some_and(u8::is_ascii_digit)
                                {
                                    state = State::LegacyOctal;
                                    continue;
                                } else {
                                    '\0'
                                }
                            }
                            '\r' => {
                                if remainder.as_bytes().get(next_byte_index) == Some(&b'\n') {
                                    state = State::Skip;
                                } else {
                                    state = State::Normal;
                                }
                                continue;
                            }
                            '\n' | '\u{2028}' | '\u{2029}' => {
                                state = State::Normal;
                                continue;
                            }
                            c => c,
                        };
                        string.push(escaped);
                        state = State::Normal;
                    }
                    State::Hex4Digits(digit, char) => {
                        let value = match c {
                            c if c.is_ascii_digit() => c as u32 - '0' as u32,
                            c if ('a'..='f').contains(&c) => c as u32 - 'a' as u32 + 10,
                            c if ('A'..='F').contains(&c) => c as u32 - 'A' as u32 + 10,
                            _ => {
                                string.push(c);
                                state = State::Normal;
                                continue;
                            }
                        };

                        let char = 16 * char + value;
                        if digit == 3 {
                            string.push(char.try_into().unwrap_or('\u{fffd}'));
                            state = State::Normal;
                        } else {
                            state = State::Hex4Digits(digit + 1, char);
                        }
                    }
                    State::HexEscape(digit, char) => {
                        let value = match c {
                            c if c.is_ascii_digit() => c as u8 - b'0',
                            c if ('a'..='f').contains(&c) => c as u8 - b'a' + 10,
                            c if ('A'..='F').contains(&c) => c as u8 - b'A' + 10,
                            _ => {
                                string.push(c);
                                state = State::Normal;
                                continue;
                            }
                        };

                        if digit == 1 {
                            let char = (16 * char + value) as char;
                            string.push(char);
                            state = State::Normal;
                        } else {
                            state = State::HexEscape(1, value);
                        }
                    }
                    State::LegacyOctal => {
                        // legacy octals are not allowed in strict mode, and
                        // so far we only use this function for modules...
                        unimplemented!()
                    }
                    State::Normal if c == '\\' => state = State::Escaped,
                    State::Normal => string.push(c),
                    State::Skip => {
                        state = State::Normal;
                    }
                }
            }
            string.into()
        }
        None => text.into(),
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use biome_rowan::RawSyntaxKind;

    use super::*;

    #[test]
    fn test_unescape_js_string() {
        let test_cases: &[(&str, &str)] = &[
            ("Hello, \\u{1F600} world!\\n", "Hello, 😀 world!\n"),
            ("\\x41\\x42\\x43", "ABC"),
            ("\\u0041\\u0042\\u0043", "ABC"),
            ("\\u{1F600}\\u{1F601}", "😀😁"),
            //("\\0\\07\\77", "\0\u{07}\u{3F}"), TODO: legacy octals
            ("\\b\\f\\n\\r\\t\\v", "\x08\x0C\n\r\t\x0B"),
            ("\\\r\n\n", "\n"), // Line continuations
            ("Hello, 😀\\u{1F601}\\0", "Hello, 😀😁\0"),
        ];

        for (token_text, expected) in test_cases {
            let token = TokenText::new_raw(RawSyntaxKind(1), token_text);
            let actual = unescape_js_string(token);
            assert_eq!(actual.text(), *expected, "failed test case: {token_text}");
        }
    }

    #[test]
    fn test_unescape_js_identifier() {
        let test_cases: &[(&str, &str)] = &[
            ("identifier", "identifier"),
            ("\\u0065", "e"),
            ("\\u{65}", "e"),
            ("a\\u0062\\u{63}", "abc"),
            ("\\u{10400}", "𐐀"),
            ("a\\u200cb", "a\u{200c}b"),
            ("a\\u200db", "a\u{200d}b"),
        ];

        for (identifier, expected) in test_cases {
            assert_eq!(
                unescape_js_identifier(identifier),
                *expected,
                "failed test case: {identifier}"
            );
        }

        assert!(matches!(
            unescape_js_identifier("identifier"),
            Cow::Borrowed(_)
        ));
        assert!(matches!(unescape_js_identifier("\\u0065"), Cow::Owned(_)));
    }

    #[test]
    fn test_unescape_malformed_js_identifier() {
        let test_cases = [
            "\\x65",
            "\\u065",
            "\\u006g",
            "\\u{}",
            "\\u{65",
            "\\u{110000}",
            "\\ud800",
        ];

        for identifier in test_cases {
            assert!(matches!(
                unescape_js_identifier(identifier),
                Cow::Borrowed(text) if text == identifier
            ));
        }
    }
}
