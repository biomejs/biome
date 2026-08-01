use crate::data::{
    PropertySyntax, PropertySyntaxComponent, PropertySyntaxComponentName, PropertySyntaxDiagnostic,
    PropertySyntaxErrorKind, PropertySyntaxMultiplier, PropertySyntaxParseDiagnostic,
    PropertySyntaxResult, PropertySyntaxType, RESERVED_CUSTOM_IDENTIFIERS,
};
use biome_css_syntax::{is_css_newline_byte, is_css_whitespace_byte};
use biome_rowan::{TextRange, TextSize};
use biome_unicode_table::{
    Dispatch::{DIG, IDT, MIN, ZER},
    is_css_non_ascii, lookup_byte,
};

/// Parses the decoded value of an `@property` `syntax` descriptor.
///
/// `range` must be the absolute source range occupied by `value`. Returned
/// diagnostics and syntax components use absolute source ranges within it.
///
/// The grammar follows the CSS Properties and Values API
/// [syntax-string parsing algorithms](https://drafts.css-houdini.org/css-properties-values-api-1/#parsing-syntax).
pub fn encode(value: &str, range: TextRange) -> PropertySyntaxResult {
    match Encoder::new(value, range).encode() {
        Ok(value) => PropertySyntaxResult::Value(value),
        Err(diagnostic) => PropertySyntaxResult::Error(PropertySyntaxDiagnostic::Parse(diagnostic)),
    }
}

struct Encoder<'source> {
    source: &'source str,
    bytes: &'source [u8],
    position: usize,
    source_range: TextRange,
}

impl<'source> Encoder<'source> {
    fn new(source: &'source str, range: TextRange) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            position: 0,
            source_range: range,
        }
    }

    fn encode(mut self) -> Result<PropertySyntax, PropertySyntaxParseDiagnostic> {
        self.consume_whitespace();
        let content_start = self.position;
        let content_end = self.trimmed_end();

        if content_start == content_end {
            return Err(self.error(PropertySyntaxErrorKind::Empty, 0, self.bytes.len()));
        }

        if self.byte_range(content_start, content_end) == [b'*'] {
            return Ok(PropertySyntax::Universal {
                range: self.range(content_start, content_end),
            });
        }

        if self.byte_at(content_start) == b'*' {
            return Err(self.error(
                PropertySyntaxErrorKind::InvalidUniversalSyntax,
                content_start,
                content_end,
            ));
        }

        let mut components = Vec::new();
        loop {
            components.push(self.consume_component(content_start, content_end)?);
            let component_end = self.position;
            self.consume_whitespace_until(content_end);
            if self.position == content_end {
                break;
            }
            let current = self.current_byte();
            if current != b'|' {
                let kind = if self.position > component_end && matches!(current, b'+' | b'#') {
                    PropertySyntaxErrorKind::UnexpectedWhitespace
                } else {
                    PropertySyntaxErrorKind::ExpectedPipe
                };
                let (start, end) = if kind == PropertySyntaxErrorKind::UnexpectedWhitespace {
                    (component_end, self.position)
                } else {
                    (self.position, content_end)
                };
                return Err(self.error(kind, start, end));
            }
            self.position += 1;
            if self.position == content_end {
                return Err(self.error(
                    PropertySyntaxErrorKind::ExpectedComponent,
                    self.position - 1,
                    self.position,
                ));
            }
        }

        Ok(PropertySyntax::Components(components.into_boxed_slice()))
    }

    fn consume_component(
        &mut self,
        content_start: usize,
        content_end: usize,
    ) -> Result<PropertySyntaxComponent, PropertySyntaxParseDiagnostic> {
        self.consume_whitespace_until(content_end);
        if self.position == content_end {
            return Err(self.error(
                PropertySyntaxErrorKind::ExpectedComponent,
                self.position,
                self.position,
            ));
        }
        if self.current_byte() == b'*' {
            return Err(self.error(
                PropertySyntaxErrorKind::InvalidUniversalSyntax,
                content_start,
                content_end,
            ));
        }

        let start = self.position;
        let name = if self.current_byte() == b'<' {
            PropertySyntaxComponentName::Type(self.consume_type(content_end)?)
        } else if self.would_start_identifier(content_end) {
            let identifier = self.consume_identifier(content_end)?;
            PropertySyntaxComponentName::CustomIdentifier(identifier.into_boxed_str())
        } else {
            return Err(self.error(
                PropertySyntaxErrorKind::ExpectedComponent,
                self.position,
                self.position + self.current_len(),
            ));
        };

        let pre_multiplied = matches!(
            name,
            PropertySyntaxComponentName::Type(PropertySyntaxType::TransformList)
        );
        if pre_multiplied
            && self.position < content_end
            && matches!(self.current_byte(), b'+' | b'#')
        {
            return Err(self.error(
                PropertySyntaxErrorKind::MultiplierAfterTransformList,
                self.position,
                self.position + 1,
            ));
        }
        let multiplier = if pre_multiplied || self.position == content_end {
            PropertySyntaxMultiplier::None
        } else {
            match self.current_byte() {
                b'+' => {
                    self.position += 1;
                    PropertySyntaxMultiplier::SpaceSeparated
                }
                b'#' => {
                    self.position += 1;
                    PropertySyntaxMultiplier::CommaSeparated
                }
                _ => PropertySyntaxMultiplier::None,
            }
        };

        Ok(PropertySyntaxComponent {
            name,
            multiplier,
            range: self.range(start, self.position),
        })
    }

    fn consume_type(
        &mut self,
        content_end: usize,
    ) -> Result<PropertySyntaxType, PropertySyntaxParseDiagnostic> {
        let start = self.position;
        self.position += 1;
        let name_start = self.position;

        while self.position < content_end {
            let byte = self.current_byte();
            if byte == b'>' {
                let Some(ty) =
                    PropertySyntaxType::from_name(self.byte_range(name_start, self.position))
                else {
                    return Err(self.error(
                        PropertySyntaxErrorKind::ExpectedTypeName,
                        start,
                        self.position + 1,
                    ));
                };
                self.position += 1;
                return Ok(ty);
            }
            if !matches!(lookup_byte(byte), IDT | MIN | DIG | ZER) {
                let kind = if is_css_whitespace_byte(byte) {
                    PropertySyntaxErrorKind::UnexpectedWhitespace
                } else {
                    PropertySyntaxErrorKind::ExpectedTypeName
                };
                return Err(self.error(kind, self.position, self.position + self.current_len()));
            }
            self.position += 1;
        }

        Err(self.error(
            PropertySyntaxErrorKind::ExpectedTypeName,
            start,
            content_end,
        ))
    }

    /// Implements CSS Syntax's
    /// [ident sequence algorithm](https://drafts.csswg.org/css-syntax-3/#consume-name).
    fn consume_identifier(
        &mut self,
        content_end: usize,
    ) -> Result<String, PropertySyntaxParseDiagnostic> {
        let start = self.position;
        let mut identifier = String::new();
        while self.position < content_end {
            let byte = self.current_byte();
            if matches!(lookup_byte(byte), IDT | MIN | DIG | ZER) {
                identifier.push(byte as char);
                self.position += 1;
            } else if byte == 0 {
                identifier.push('\u{fffd}');
                self.position += 1;
            } else if byte == b'\\' && self.starts_valid_escape(content_end) {
                identifier.push(self.consume_escaped_code_point(content_end));
            } else if byte >= 0x80 {
                let character = self.char_at(self.position);
                if !is_css_non_ascii(character) {
                    break;
                }
                identifier.push(character);
                self.position += character.len_utf8();
            } else {
                break;
            }
        }

        if is_invalid_custom_identifier(&identifier) {
            return Err(self.error(
                PropertySyntaxErrorKind::InvalidCustomIdentifier,
                start,
                self.position,
            ));
        }

        Ok(identifier)
    }

    /// Implements CSS Syntax's
    /// [ident sequence lookahead](https://drafts.csswg.org/css-syntax-3/#would-start-an-identifier).
    fn would_start_identifier(&self, content_end: usize) -> bool {
        let first = self.current_byte();
        if first == 0 {
            return true;
        }
        if lookup_byte(first) == IDT || first >= 0x80 {
            return first < 0x80 || is_css_non_ascii(self.char_at(self.position));
        }
        if first == b'\\' {
            return self.starts_valid_escape(content_end);
        }
        if first != b'-' || self.position + 1 >= content_end {
            return false;
        }

        let second = self.byte_at(self.position + 1);
        second == b'-'
            || second == 0
            || lookup_byte(second) == IDT
            || (second >= 0x80 && is_css_non_ascii(self.char_at(self.position + 1)))
            || (second == b'\\'
                && self.position + 2 < content_end
                && !is_css_newline_byte(self.byte_at(self.position + 2)))
    }

    fn starts_valid_escape(&self, content_end: usize) -> bool {
        self.position + 1 < content_end && !is_css_newline_byte(self.byte_at(self.position + 1))
    }

    /// Implements CSS Syntax's
    /// [escaped code point algorithm](https://drafts.csswg.org/css-syntax-3/#consume-escaped-code-point).
    fn consume_escaped_code_point(&mut self, content_end: usize) -> char {
        self.position += 1;
        let first = self.current_byte();
        if !first.is_ascii_hexdigit() {
            let character = self.char_at(self.position);
            self.position += character.len_utf8();
            return char::from_u32(preprocess_code_point(character as u32)).unwrap_or('\u{fffd}');
        }

        let mut value = 0_u32;
        let mut digits = 0;
        while self.position < content_end && digits < 6 {
            let Some(digit) = char::from(self.current_byte()).to_digit(16) else {
                break;
            };
            value = value * 16 + digit;
            self.position += 1;
            digits += 1;
        }
        if self.position < content_end && is_css_whitespace_byte(self.current_byte()) {
            if self.current_byte() == b'\r'
                && self.position + 1 < content_end
                && self.byte_at(self.position + 1) == b'\n'
            {
                self.position += 2;
            } else {
                self.position += 1;
            }
        }

        char::from_u32(preprocess_code_point(value)).unwrap_or('\u{fffd}')
    }

    fn consume_whitespace(&mut self) {
        while self.position < self.bytes.len() && is_css_whitespace_byte(self.current_byte()) {
            self.position += 1;
        }
    }

    fn consume_whitespace_until(&mut self, end: usize) {
        while self.position < end && is_css_whitespace_byte(self.current_byte()) {
            self.position += 1;
        }
    }

    fn trimmed_end(&self) -> usize {
        let mut end = self.bytes.len();
        while end > self.position && is_css_whitespace_byte(self.byte_at(end - 1)) {
            end -= 1;
        }
        end
    }

    fn current_len(&self) -> usize {
        if self.position == self.bytes.len() {
            0
        } else if self.current_byte() < 0x80 {
            1
        } else {
            self.char_at(self.position).len_utf8()
        }
    }

    /// Returns the byte at the current non-EOF parser position.
    fn current_byte(&self) -> u8 {
        self.byte_at(self.position)
    }

    /// Returns the byte at a non-EOF parser position.
    fn byte_at(&self, position: usize) -> u8 {
        debug_assert!(position < self.bytes.len());
        // SAFETY: Callers check the applicable input boundary before passing
        // a byte position.
        self.bytes[position]
    }

    /// Returns the bytes in a parser range.
    fn byte_range(&self, start: usize, end: usize) -> &[u8] {
        debug_assert!(start <= end);
        debug_assert!(end <= self.bytes.len());
        // SAFETY: Parser ranges are ordered byte offsets bounded by the input.
        &self.bytes[start..end]
    }

    /// Returns the character at a non-EOF UTF-8 byte boundary.
    fn char_at(&self, position: usize) -> char {
        debug_assert!(position < self.source.len());
        debug_assert!(self.source.is_char_boundary(position));
        // SAFETY: Callers provide non-EOF positions reached by advancing over
        // ASCII bytes or complete UTF-8 characters.
        self.source[position..]
            .chars()
            .next()
            .expect("the position should point to a character")
    }

    fn range(&self, start: usize, end: usize) -> TextRange {
        let range = TextRange::new(
            self.source_range.start() + TextSize::from(start as u32),
            self.source_range.start() + TextSize::from(end as u32),
        );
        debug_assert!(range.end() <= self.source_range.end());
        range
    }

    fn error(
        &self,
        kind: PropertySyntaxErrorKind,
        start: usize,
        end: usize,
    ) -> PropertySyntaxParseDiagnostic {
        PropertySyntaxParseDiagnostic::new(kind, self.range(start, end))
    }
}

fn is_invalid_custom_identifier(identifier: &str) -> bool {
    RESERVED_CUSTOM_IDENTIFIERS
        .iter()
        .any(|reserved| identifier.eq_ignore_ascii_case(reserved))
}

const fn preprocess_code_point(code_point: u32) -> u32 {
    if code_point == 0 || code_point > 0x0010_ffff || (code_point >= 0xd800 && code_point <= 0xdfff)
    {
        0xfffd
    } else {
        code_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_diagnostics::{DiagnosticExt, print_diagnostic_to_string};

    fn encode_value(value: &str) -> PropertySyntax {
        match encode(
            value,
            TextRange::new(10.into(), (10 + value.len() as u32).into()),
        ) {
            PropertySyntaxResult::Value(value) => value,
            result => panic!("expected value, got {result:?}"),
        }
    }

    fn encode_error(value: &str) -> PropertySyntaxDiagnostic {
        match encode(
            value,
            TextRange::new(10.into(), (10 + value.len() as u32).into()),
        ) {
            PropertySyntaxResult::Error(error) => error,
            result => panic!("expected error, got {result:?}"),
        }
    }

    #[test]
    fn encodes_universal_syntax() {
        assert_eq!(
            encode_value(" \t*\n"),
            PropertySyntax::Universal {
                range: TextRange::new(12.into(), 13.into())
            }
        );
    }

    #[test]
    fn encodes_supported_types() {
        let cases = [
            ("<angle>", PropertySyntaxType::Angle),
            ("<color>", PropertySyntaxType::Color),
            ("<custom-ident>", PropertySyntaxType::CustomIdent),
            ("<image>", PropertySyntaxType::Image),
            ("<integer>", PropertySyntaxType::Integer),
            ("<length>", PropertySyntaxType::Length),
            ("<length-percentage>", PropertySyntaxType::LengthPercentage),
            ("<number>", PropertySyntaxType::Number),
            ("<percentage>", PropertySyntaxType::Percentage),
            ("<resolution>", PropertySyntaxType::Resolution),
            ("<string>", PropertySyntaxType::String),
            ("<time>", PropertySyntaxType::Time),
            (
                "<transform-function>",
                PropertySyntaxType::TransformFunction,
            ),
            ("<transform-list>", PropertySyntaxType::TransformList),
            ("<url>", PropertySyntaxType::Url),
        ];

        for (source, expected) in cases {
            let PropertySyntax::Components(components) = encode_value(source) else {
                panic!("expected components for {source}");
            };
            assert_eq!(
                components[0].name,
                PropertySyntaxComponentName::Type(expected)
            );
        }
    }

    #[test]
    fn encodes_ordered_alternatives_and_multipliers() {
        let PropertySyntax::Components(components) = encode_value("foo | <color># | <length>+")
        else {
            panic!("expected components");
        };
        assert_eq!(components.len(), 3);
        assert_eq!(
            components[0].name,
            PropertySyntaxComponentName::CustomIdentifier("foo".into())
        );
        assert_eq!(components[0].multiplier, PropertySyntaxMultiplier::None);
        assert_eq!(
            components[1].multiplier,
            PropertySyntaxMultiplier::CommaSeparated
        );
        assert_eq!(
            components[2].multiplier,
            PropertySyntaxMultiplier::SpaceSeparated
        );
    }

    #[test]
    fn encodes_case_sensitive_and_escaped_identifiers() {
        let PropertySyntax::Components(components) = encode_value("Red | red | \\66 oo") else {
            panic!("expected components");
        };
        assert_eq!(
            components
                .iter()
                .map(|component| &component.name)
                .collect::<Vec<_>>(),
            [
                &PropertySyntaxComponentName::CustomIdentifier("Red".into()),
                &PropertySyntaxComponentName::CustomIdentifier("red".into()),
                &PropertySyntaxComponentName::CustomIdentifier("foo".into()),
            ]
        );
    }

    #[test]
    fn encodes_non_ascii_identifiers_using_byte_offsets() {
        let PropertySyntax::Components(components) = encode_value("éclair | 色") else {
            panic!("expected components");
        };
        assert_eq!(
            components[0].name,
            PropertySyntaxComponentName::CustomIdentifier("éclair".into())
        );
        assert_eq!(components[0].range, TextRange::new(10.into(), 17.into()));
        assert_eq!(
            components[1].name,
            PropertySyntaxComponentName::CustomIdentifier("色".into())
        );
    }

    #[test]
    fn accepts_web_platform_syntax_cases() {
        let valid = [
            "--foo",
            "--foo | <color>",
            "--foo+",
            "<length>|<percentage>|<length-percentage>",
            "<color> | <image> | <url> | <integer> | <angle>",
            "<time> | <resolution> | <transform-list> | <custom-ident>",
            "\t<color>\n|   foo",
            "big | bigger | BIGGER",
            "foo+|bar",
            "banana\t",
            "\nbanana\r\n",
            "ba\x0c\n|\tna\r|nya",
            "\\1F914",
            "hmm\\1F914",
            "\\1F914hmm",
            "\\1F914 hmm",
            "\\1F914\\1F914",
            "⌘",
            "☀",
        ];

        for source in valid {
            assert!(
                matches!(
                    encode(
                        source,
                        TextRange::new(0.into(), (source.len() as u32).into())
                    ),
                    PropertySyntaxResult::Value(_)
                ),
                "source: {source}"
            );
        }
    }

    #[test]
    fn rejects_invalid_syntax() {
        let cases = [
            ("", PropertySyntaxErrorKind::Empty),
            ("   ", PropertySyntaxErrorKind::Empty),
            ("* | auto", PropertySyntaxErrorKind::InvalidUniversalSyntax),
            ("<unknown>", PropertySyntaxErrorKind::ExpectedTypeName),
            ("< color>", PropertySyntaxErrorKind::UnexpectedWhitespace),
            ("<color >", PropertySyntaxErrorKind::UnexpectedWhitespace),
            ("<color", PropertySyntaxErrorKind::ExpectedTypeName),
            ("<color> #", PropertySyntaxErrorKind::UnexpectedWhitespace),
            (
                "<transform-list>+",
                PropertySyntaxErrorKind::MultiplierAfterTransformList,
            ),
            ("auto || none", PropertySyntaxErrorKind::ExpectedComponent),
            ("auto |", PropertySyntaxErrorKind::ExpectedComponent),
            ("| auto", PropertySyntaxErrorKind::ExpectedComponent),
            ("initial", PropertySyntaxErrorKind::InvalidCustomIdentifier),
            ("DEFAULT", PropertySyntaxErrorKind::InvalidCustomIdentifier),
        ];

        for (source, expected) in cases {
            assert_eq!(encode_error(source).kind(), expected, "source: {source}");
        }
    }

    #[test]
    fn rejects_web_platform_syntax_cases() {
        let invalid = [
            "banana,nya",
            "<\\6c ength>",
            "<banana>",
            "<Number>",
            "<LENGTH>",
            "<length>++",
            "<length>##",
            "<length>+#",
            "<length>#+",
            "<length> | *",
            "*|banana",
            "*+",
            "||",
            "foo bar",
            "foo foo foo",
            "foo § bar",
            "foo \\1F914 bar",
            "<length> <number>",
            "<length> <length> <length>",
            "<length>|initial",
            "<length>|INHERIT",
            "<percentage>|unsEt",
            "<color>|REVert",
            "<integer>|deFAUlt",
        ];

        for source in invalid {
            assert!(
                matches!(
                    encode(
                        source,
                        TextRange::new(0.into(), (source.len() as u32).into())
                    ),
                    PropertySyntaxResult::Error(_)
                ),
                "source: {source}"
            );
        }
    }

    #[test]
    fn diagnostics_use_absolute_byte_ranges() {
        let diagnostic = encode_error("é | ?");
        assert_eq!(diagnostic.range(), TextRange::new(15.into(), 16.into()));
    }

    #[test]
    fn preprocesses_null_code_points() {
        let PropertySyntax::Components(components) = encode_value("\0replacement") else {
            panic!("expected components");
        };
        assert_eq!(
            components[0].name,
            PropertySyntaxComponentName::CustomIdentifier("�replacement".into())
        );
    }

    #[test]
    fn parse_diagnostics() {
        let mut snapshot = String::new();
        for value in [
            "   ",
            "foo | | bar",
            "foo bar",
            "<unknown>",
            "initial",
            "* | auto",
            "<length> | *",
            "<transform-list>+",
            "<color> #",
        ] {
            let prefix = "@property --foo {\n  syntax: \"";
            let source = format!("{prefix}{value}\";\n}}\n");
            let start = prefix.len() as u32;
            let diagnostic = match encode(
                value,
                TextRange::new(start.into(), (start + value.len() as u32).into()),
            ) {
                PropertySyntaxResult::Error(diagnostic) => diagnostic,
                result => panic!("expected error, got {result:?}"),
            }
            .with_file_path("property.css")
            .with_file_source_code(source);

            snapshot.push_str(&print_diagnostic_to_string(&diagnostic));
            snapshot.push('\n');
        }

        let snapshot = snapshot
            .lines()
            .map(str::trim_end)
            .collect::<Vec<_>>()
            .join("\n");
        insta::assert_snapshot!(snapshot);
    }
}
