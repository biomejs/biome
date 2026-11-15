//! Reads the variables a binding pattern written in the template declares,
//! where the host parser keeps the pattern as plain text: the value of
//! `{:then value}`, the error of `{:catch error}`, and the item of an
//! `{#each}` the host parser could not read itself.
//!
//! The patterns follow JavaScript destructuring: `value`, `{ a, b: c = 1 }`,
//! `[first, ...rest]`. Default values are skipped, not read.

use biome_rowan::{TextRange, TextSize};

/// Returns the name and the range of each variable `text` declares, where
/// `start` is the position of the first byte of `text` in the host document.
pub(crate) fn pattern_bindings(text: &str, start: TextSize) -> Vec<(String, TextRange)> {
    let mut reader = Reader {
        text,
        bytes: text.as_bytes(),
        position: 0,
        start,
        bindings: Vec::new(),
    };
    reader.pattern();
    reader.bindings
}

struct Reader<'a> {
    text: &'a str,
    bytes: &'a [u8],
    position: usize,
    start: TextSize,
    bindings: Vec<(String, TextRange)>,
}

impl Reader<'_> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.position).copied()
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|byte| byte.is_ascii_whitespace()) {
            self.position += 1;
        }
    }

    fn eat(&mut self, byte: u8) -> bool {
        self.skip_whitespace();
        if self.peek() == Some(byte) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn eat_spread(&mut self) -> bool {
        self.skip_whitespace();
        if self.text[self.position..].starts_with("...") {
            self.position += 3;
            true
        } else {
            false
        }
    }

    /// Reads an identifier, and returns its range in `text`.
    fn identifier(&mut self) -> Option<(usize, usize)> {
        self.skip_whitespace();
        let from = self.position;
        for (offset, char) in self.text[from..].char_indices() {
            let is_part = char == '_' || char == '$' || char.is_alphanumeric();
            if !is_part || (offset == 0 && char.is_ascii_digit()) {
                break;
            }
            self.position = from + offset + char.len_utf8();
        }
        (self.position > from).then_some((from, self.position))
    }

    fn bind(&mut self, (from, to): (usize, usize)) {
        let range = TextRange::new(
            self.start + TextSize::from(from as u32),
            self.start + TextSize::from(to as u32),
        );
        self.bindings.push((self.text[from..to].to_string(), range));
    }

    /// Reads a pattern and the default value that may follow it.
    fn pattern(&mut self) {
        if self.eat(b'{') {
            self.object();
        } else if self.eat(b'[') {
            self.array();
        } else if let Some(name) = self.identifier() {
            self.bind(name);
        } else {
            // Not a pattern: stop reading.
            self.position = self.bytes.len();
            return;
        }
        if self.eat(b'=') {
            self.skip_expression();
        }
    }

    fn object(&mut self) {
        loop {
            if self.eat(b'}') || self.peek().is_none() {
                return;
            }
            if self.eat_spread() {
                self.pattern();
            } else if self.eat(b'[') {
                // A computed key: `[key]: value`.
                self.skip_balanced(b']');
                if self.eat(b':') {
                    self.pattern();
                }
            } else if matches!(self.peek(), Some(b'"' | b'\'')) {
                self.skip_string();
                if self.eat(b':') {
                    self.pattern();
                }
            } else if let Some(key) = self.identifier() {
                if self.eat(b':') {
                    self.pattern();
                } else {
                    // A shorthand property: the key names the variable.
                    self.bind(key);
                    if self.eat(b'=') {
                        self.skip_expression();
                    }
                }
            } else {
                self.position = self.bytes.len();
                return;
            }
            self.eat(b',');
        }
    }

    fn array(&mut self) {
        loop {
            if self.eat(b']') || self.peek().is_none() {
                return;
            }
            if self.eat(b',') {
                // A hole.
                continue;
            }
            self.eat_spread();
            self.pattern();
            self.eat(b',');
        }
    }

    /// Skips a default value, up to the `,`, `}` or `]` that ends it.
    fn skip_expression(&mut self) {
        let mut depth = 0usize;
        while let Some(byte) = self.peek() {
            match byte {
                b'(' | b'[' | b'{' => depth += 1,
                b')' | b']' | b'}' if depth == 0 => return,
                b')' | b']' | b'}' => depth -= 1,
                b',' if depth == 0 => return,
                b'"' | b'\'' | b'`' => {
                    self.skip_string();
                    continue;
                }
                _ => {}
            }
            self.position += 1;
        }
    }

    /// Skips the content of a bracket up to its closing `close`, which is
    /// consumed.
    fn skip_balanced(&mut self, close: u8) {
        self.skip_expression();
        self.eat(close);
    }

    fn skip_string(&mut self) {
        let Some(quote) = self.peek() else {
            return;
        };
        self.position += 1;
        while let Some(byte) = self.peek() {
            self.position += 1;
            match byte {
                b'\\' => self.position += 1,
                _ if byte == quote => return,
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::pattern_bindings;
    use biome_rowan::TextSize;

    fn names(text: &str) -> Vec<String> {
        pattern_bindings(text, TextSize::from(0))
            .into_iter()
            .map(|(name, range)| {
                assert_eq!(&text[range], name);
                name
            })
            .collect()
    }

    #[test]
    fn reads_an_identifier() {
        assert_eq!(names(" value "), ["value"]);
    }

    #[test]
    fn reads_object_patterns() {
        assert_eq!(
            names("{ a, b: c, d = { e: 1 }, 'f': g, [h]: i, ...rest }"),
            ["a", "c", "d", "g", "i", "rest"]
        );
    }

    #[test]
    fn reads_array_patterns() {
        assert_eq!(
            names("[first, , [second = f(1, 2)], ...rest]"),
            ["first", "second", "rest"]
        );
    }

    #[test]
    fn reads_nothing_from_an_expression() {
        assert!(names("1 + 2").is_empty());
    }
}
