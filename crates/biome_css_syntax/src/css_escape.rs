use crate::{is_css_newline_byte, is_css_whitespace_byte};
use biome_rowan::{TextRange, TextSize};
use std::borrow::Cow;

/// Resolves CSS escape sequences in an identifier or dimension unit.
pub fn decode_css_identifier(value: &str) -> Cow<'_, str> {
    if !value.contains(['\\', '\0']) {
        return Cow::Borrowed(value);
    }

    Cow::Owned(
        DecodedCursor::new_css_string(value, TextSize::default())
            .map(|character| character.value)
            .collect(),
    )
}

fn decode_css_code_point(value: u32) -> char {
    if value == 0 || value > 0x0010_ffff || (0xd800..=0xdfff).contains(&value) {
        '\u{fffd}'
    } else {
        // SAFETY: The invalid scalar value ranges are rejected above.
        char::from_u32(value).expect("the escape should decode to a character")
    }
}

#[derive(Clone, Copy)]
pub(super) struct DecodedCharacter {
    pub(super) value: char,
    pub(super) source_start: usize,
    pub(super) source_end: usize,
}

#[derive(Clone)]
pub(super) struct DecodedCursor<'source> {
    pub(super) source: &'source str,
    source_start: TextSize,
    position: usize,
    decode_css_escapes: bool,
}

impl<'source> DecodedCursor<'source> {
    #[cfg(test)]
    pub(super) fn new(source: &'source str, source_start: TextSize) -> Self {
        Self {
            source,
            source_start,
            position: 0,
            decode_css_escapes: false,
        }
    }

    pub(super) fn new_css_string(source: &'source str, source_start: TextSize) -> Self {
        Self {
            source,
            source_start,
            position: 0,
            decode_css_escapes: true,
        }
    }

    pub(super) fn source_range(&self, start: usize, end: usize) -> TextRange {
        TextRange::new(
            self.source_start + TextSize::from(start as u32),
            self.source_start + TextSize::from(end as u32),
        )
    }
}

impl Iterator for DecodedCursor<'_> {
    type Item = DecodedCharacter;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let source_start = self.position;
            let character = self.source.get(self.position..)?.chars().next()?;
            self.position += character.len_utf8();

            if !self.decode_css_escapes || character != '\\' {
                return Some(DecodedCharacter {
                    value: if self.decode_css_escapes && character == '\0' {
                        '\u{fffd}'
                    } else {
                        character
                    },
                    source_start,
                    source_end: self.position,
                });
            }

            let Some(escaped) = self.source.get(self.position..)?.chars().next() else {
                return Some(DecodedCharacter {
                    value: '\u{fffd}',
                    source_start,
                    source_end: self.position,
                });
            };
            if is_css_newline(escaped) {
                self.position += escaped.len_utf8();
                if escaped == '\r'
                    && self
                        .source
                        .get(self.position..)
                        .is_some_and(|rest| rest.starts_with('\n'))
                {
                    self.position += 1;
                }
                continue;
            }

            let value = if escaped.is_ascii_hexdigit() {
                let mut code_point = 0_u32;
                let mut digits = 0;
                while digits < 6 {
                    let Some(character) = self.source.get(self.position..)?.chars().next() else {
                        break;
                    };
                    let Some(digit) = character.to_digit(16) else {
                        break;
                    };
                    code_point = code_point * 16 + digit;
                    self.position += character.len_utf8();
                    digits += 1;
                }
                if let Some(whitespace) = self.source.get(self.position..)?.chars().next()
                    && is_css_whitespace(whitespace)
                {
                    self.position += whitespace.len_utf8();
                    if whitespace == '\r'
                        && self
                            .source
                            .get(self.position..)
                            .is_some_and(|rest| rest.starts_with('\n'))
                    {
                        self.position += 1;
                    }
                }
                decode_css_code_point(code_point)
            } else {
                self.position += escaped.len_utf8();
                escaped
            };

            return Some(DecodedCharacter {
                value,
                source_start,
                source_end: self.position,
            });
        }
    }
}

pub(super) fn is_css_whitespace(character: char) -> bool {
    character.is_ascii() && is_css_whitespace_byte(character as u8)
}

pub(super) fn is_css_newline(character: char) -> bool {
    character.is_ascii() && is_css_newline_byte(character as u8)
}

#[cfg(test)]
mod tests {
    use super::decode_css_identifier;

    #[test]
    fn decodes_css_identifier_escapes() {
        assert_eq!(decode_css_identifier("plain"), "plain");
        assert_eq!(decode_css_identifier(r"f\6f o"), "foo");
        assert_eq!(decode_css_identifier(r"p\78"), "px");
        assert_eq!(decode_css_identifier("\\"), "\u{fffd}");
        assert_eq!(decode_css_identifier(r"\_"), "_");
        assert_eq!(decode_css_identifier("a\\\nb"), "ab");
        assert_eq!(decode_css_identifier("a\0b"), "a\u{fffd}b");
        assert_eq!(decode_css_identifier(r"\d800"), "\u{fffd}");
        assert_eq!(decode_css_identifier(r"\110000"), "\u{fffd}");
    }
}
