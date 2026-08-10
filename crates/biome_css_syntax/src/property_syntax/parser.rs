use super::data::{
    PropertySyntax, PropertySyntaxComponent, PropertySyntaxComponentName, PropertySyntaxErrorKind,
    PropertySyntaxMultiplier, PropertySyntaxParseDiagnostic, PropertySyntaxResult,
    PropertySyntaxType, RESERVED_CUSTOM_IDENTIFIERS,
};
use crate::{
    CssString,
    css_escape::{DecodedCharacter, DecodedCursor, is_css_newline, is_css_whitespace},
};
use biome_rowan::{AstNode, TextRange, TextSize};
use biome_unicode_table::{
    Dispatch::{DIG, IDT, MIN, ZER},
    lookup_byte,
};

/// Decodes and parses the value of a CSS string used as an `@property`
/// `syntax` descriptor.
///
/// The grammar follows the CSS Properties and Values API
/// [syntax-string parsing algorithms](https://drafts.css-houdini.org/css-properties-values-api-1/#parsing-syntax).
pub fn encode(string: &CssString) -> PropertySyntaxResult {
    let range = string.range();
    let Ok(token) = string.value_token() else {
        return invalid_css_string(range);
    };
    let Ok(value) = string.inner_string_text() else {
        return invalid_css_string(range);
    };
    let source_start = token.text_trimmed_range().start() + TextSize::from(1);

    match Encoder::new(DecodedCursor::new_css_string(value.text(), source_start)).encode() {
        Ok(value) => PropertySyntaxResult::Value(value),
        Err(diagnostic) => PropertySyntaxResult::Error(diagnostic),
    }
}

#[cfg(test)]
pub(crate) fn encode_decoded(value: &str, range: TextRange) -> PropertySyntaxResult {
    match Encoder::new(DecodedCursor::new(value, range.start())).encode() {
        Ok(value) => PropertySyntaxResult::Value(value),
        Err(diagnostic) => PropertySyntaxResult::Error(diagnostic),
    }
}

fn invalid_css_string(range: TextRange) -> PropertySyntaxResult {
    PropertySyntaxResult::Error(PropertySyntaxParseDiagnostic::new(
        PropertySyntaxErrorKind::ExpectedString,
        range,
    ))
}

struct Encoder<'source> {
    cursor: DecodedCursor<'source>,
    current: Option<DecodedCharacter>,
    source_end: usize,
}

impl<'source> Encoder<'source> {
    fn new(mut cursor: DecodedCursor<'source>) -> Self {
        let source_end = cursor.source.len();
        let current = cursor.next();
        Self {
            cursor,
            current,
            source_end,
        }
    }

    fn encode(mut self) -> Result<PropertySyntax, PropertySyntaxParseDiagnostic> {
        self.consume_whitespace();
        let content_start = self.position();
        let content_end = self.trimmed_end();

        if content_start == content_end {
            return Err(self.error(PropertySyntaxErrorKind::Empty, 0, self.source_end));
        }

        if self.current_character() == '*'
            && self
                .current
                .is_some_and(|current| current.source_end == content_end)
        {
            return Ok(PropertySyntax::Universal {
                range: self.range(content_start, content_end),
            });
        }

        if self.current_character() == '*' {
            return Err(self.error(
                PropertySyntaxErrorKind::InvalidUniversalSyntax,
                content_start,
                content_end,
            ));
        }

        let mut components = Vec::new();
        loop {
            components.push(self.consume_component(content_start, content_end)?);
            let component_end = self.position();
            self.consume_whitespace_until(content_end);
            if self.at_end(content_end) {
                break;
            }
            let current = self.current_character();
            if current != '|' {
                let position = self.position();
                let kind = if position > component_end && matches!(current, '+' | '#') {
                    PropertySyntaxErrorKind::UnexpectedWhitespace
                } else {
                    PropertySyntaxErrorKind::ExpectedPipe
                };
                let (start, end) = if kind == PropertySyntaxErrorKind::UnexpectedWhitespace {
                    (component_end, position)
                } else {
                    (position, content_end)
                };
                return Err(self.error(kind, start, end));
            }
            let pipe = self.bump();
            if self.at_end(content_end) {
                return Err(self.error(
                    PropertySyntaxErrorKind::ExpectedComponent,
                    pipe.source_start,
                    pipe.source_end,
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
        if self.at_end(content_end) {
            let position = self.position();
            return Err(self.error(
                PropertySyntaxErrorKind::ExpectedComponent,
                position,
                position,
            ));
        }
        if self.current_character() == '*' {
            return Err(self.error(
                PropertySyntaxErrorKind::InvalidUniversalSyntax,
                content_start,
                content_end,
            ));
        }

        let start = self.position();
        let name = if self.current_character() == '<' {
            PropertySyntaxComponentName::Type(self.consume_type(content_end)?)
        } else if self.would_start_identifier(content_end) {
            let identifier = self.consume_identifier(content_end)?;
            PropertySyntaxComponentName::CustomIdentifier(identifier.into_boxed_str())
        } else {
            let current = self.current();
            return Err(self.error(
                PropertySyntaxErrorKind::ExpectedComponent,
                current.source_start,
                current.source_end,
            ));
        };

        let pre_multiplied = matches!(
            name,
            PropertySyntaxComponentName::Type(PropertySyntaxType::TransformList)
        );
        if pre_multiplied
            && !self.at_end(content_end)
            && matches!(self.current_character(), '+' | '#')
        {
            let current = self.current();
            return Err(self.error(
                PropertySyntaxErrorKind::MultiplierAfterTransformList,
                current.source_start,
                current.source_end,
            ));
        }
        let multiplier = if pre_multiplied || self.at_end(content_end) {
            PropertySyntaxMultiplier::None
        } else {
            match self.current_character() {
                '+' => {
                    self.bump();
                    PropertySyntaxMultiplier::SpaceSeparated
                }
                '#' => {
                    self.bump();
                    PropertySyntaxMultiplier::CommaSeparated
                }
                _ => PropertySyntaxMultiplier::None,
            }
        };

        Ok(PropertySyntaxComponent {
            name,
            multiplier,
            range: self.range(start, self.position()),
        })
    }

    fn consume_type(
        &mut self,
        content_end: usize,
    ) -> Result<PropertySyntaxType, PropertySyntaxParseDiagnostic> {
        let start = self.position();
        self.bump();
        let mut candidates = [true; PropertySyntaxType::ALL.len()];
        let mut name_len = 0;

        while !self.at_end(content_end) {
            let current = self.current();
            let character = current.value;
            if character == '>' {
                let Some(ty) = candidates
                    .iter()
                    .copied()
                    .zip(PropertySyntaxType::ALL)
                    .find_map(|(candidate, syntax_type)| {
                        (candidate && syntax_type.name().len() == name_len).then_some(syntax_type)
                    })
                else {
                    return Err(self.error(
                        PropertySyntaxErrorKind::ExpectedTypeName,
                        start,
                        current.source_end,
                    ));
                };
                self.bump();
                return Ok(ty);
            }
            if !character.is_ascii()
                || !matches!(lookup_byte(character as u8), IDT | MIN | DIG | ZER)
            {
                let kind = if is_css_whitespace(character) {
                    PropertySyntaxErrorKind::UnexpectedWhitespace
                } else {
                    PropertySyntaxErrorKind::ExpectedTypeName
                };
                return Err(self.error(kind, current.source_start, current.source_end));
            }
            for (candidate, syntax_type) in candidates.iter_mut().zip(PropertySyntaxType::ALL) {
                *candidate &=
                    syntax_type.name().as_bytes().get(name_len) == Some(&(character as u8));
            }
            name_len += 1;
            self.bump();
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
        let start = self.position();
        let mut identifier = String::new();
        while !self.at_end(content_end) {
            let character = self.current_character();
            if character.is_ascii() && matches!(lookup_byte(character as u8), IDT | MIN | DIG | ZER)
            {
                identifier.push(character);
                self.bump();
            } else if character == '\0' {
                identifier.push('\u{fffd}');
                self.bump();
            } else if character == '\\' && self.starts_valid_escape(content_end) {
                identifier.push(self.consume_escaped_code_point(content_end));
            } else if is_css_identifier_non_ascii(character) {
                identifier.push(character);
                self.bump();
            } else {
                break;
            }
        }

        if is_invalid_custom_identifier(&identifier) {
            return Err(self.error(
                PropertySyntaxErrorKind::InvalidCustomIdentifier,
                start,
                self.position(),
            ));
        }

        Ok(identifier)
    }

    /// Implements CSS Syntax's
    /// [ident sequence lookahead](https://drafts.csswg.org/css-syntax-3/#would-start-an-identifier).
    fn would_start_identifier(&self, content_end: usize) -> bool {
        let first = self.current_character();
        if first == '\0' {
            return true;
        }
        if (first.is_ascii() && lookup_byte(first as u8) == IDT)
            || is_css_identifier_non_ascii(first)
        {
            return true;
        }
        if first == '\\' {
            return self.starts_valid_escape(content_end);
        }
        if first != '-' {
            return false;
        }

        let Some(second) = self
            .nth(1)
            .filter(|second| second.source_start < content_end)
        else {
            return false;
        };
        second.value == '-'
            || second.value == '\0'
            || (second.value.is_ascii() && lookup_byte(second.value as u8) == IDT)
            || is_css_identifier_non_ascii(second.value)
            || (second.value == '\\'
                && self.nth(2).is_some_and(|third| {
                    third.source_start < content_end && !is_css_newline(third.value)
                }))
    }

    fn starts_valid_escape(&self, content_end: usize) -> bool {
        self.nth(1)
            .is_some_and(|next| next.source_start < content_end && !is_css_newline(next.value))
    }

    /// Implements CSS Syntax's
    /// [escaped code point algorithm](https://drafts.csswg.org/css-syntax-3/#consume-escaped-code-point).
    fn consume_escaped_code_point(&mut self, content_end: usize) -> char {
        self.bump();
        let first = self.current_character();
        if !first.is_ascii_hexdigit() {
            let character = self.bump().value;
            return char::from_u32(preprocess_code_point(character as u32)).unwrap_or('\u{fffd}');
        }

        let mut value = 0_u32;
        let mut digits = 0;
        while !self.at_end(content_end) && digits < 6 {
            let Some(digit) = self.current_character().to_digit(16) else {
                break;
            };
            value = value * 16 + digit;
            self.bump();
            digits += 1;
        }
        if !self.at_end(content_end) && is_css_whitespace(self.current_character()) {
            if self.current_character() == '\r'
                && self
                    .nth(1)
                    .is_some_and(|next| next.source_start < content_end && next.value == '\n')
            {
                self.bump();
                self.bump();
            } else {
                self.bump();
            }
        }

        char::from_u32(preprocess_code_point(value)).unwrap_or('\u{fffd}')
    }

    fn consume_whitespace(&mut self) {
        while self.current.is_some() && is_css_whitespace(self.current_character()) {
            self.bump();
        }
    }

    fn consume_whitespace_until(&mut self, end: usize) {
        while !self.at_end(end) && is_css_whitespace(self.current_character()) {
            self.bump();
        }
    }

    fn trimmed_end(&self) -> usize {
        let mut current = self.current;
        let mut cursor = self.cursor.clone();
        let mut end = self.position();
        while let Some(character) = current {
            if !is_css_whitespace(character.value) {
                end = character.source_end;
            }
            current = cursor.next();
        }
        end
    }

    fn at_end(&self, end: usize) -> bool {
        self.current
            .is_none_or(|current| current.source_start >= end)
    }

    fn position(&self) -> usize {
        self.current
            .map_or(self.source_end, |current| current.source_start)
    }

    fn current(&self) -> DecodedCharacter {
        debug_assert!(self.current.is_some());
        // SAFETY: Parser operations call this only after checking the content
        // boundary or otherwise establishing that the cursor is not at EOF.
        self.current.expect("the cursor should not be at EOF")
    }

    fn current_character(&self) -> char {
        self.current().value
    }

    fn bump(&mut self) -> DecodedCharacter {
        let current = self.current();
        self.current = self.cursor.next();
        current
    }

    fn nth(&self, index: usize) -> Option<DecodedCharacter> {
        if index == 0 {
            return self.current;
        }
        self.cursor.clone().nth(index - 1)
    }

    fn range(&self, start: usize, end: usize) -> TextRange {
        self.cursor.source_range(start, end)
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

fn is_css_identifier_non_ascii(character: char) -> bool {
    character >= '\u{80}'
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
        match encode_decoded(
            value,
            TextRange::new(10.into(), (10 + value.len() as u32).into()),
        ) {
            PropertySyntaxResult::Value(value) => value,
            result => panic!("expected value, got {result:?}"),
        }
    }

    fn encode_error(value: &str) -> PropertySyntaxParseDiagnostic {
        match encode_decoded(
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
        let PropertySyntax::Components(components) = encode_value("éclair | 色 | \u{80}") else {
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
        assert_eq!(
            components[2].name,
            PropertySyntaxComponentName::CustomIdentifier("\u{80}".into())
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
                    encode_decoded(
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
                    encode_decoded(
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
    fn css_string_cursor_decodes_source_spans() {
        let mut cursor = DecodedCursor::new_css_string(r"\3c color\3e ", TextSize::from(10));

        assert_eq!(
            cursor.next().map(|character| (
                character.value,
                character.source_start,
                character.source_end
            )),
            Some(('<', 0, 4))
        );
        assert_eq!(
            cursor.next().map(|character| (
                character.value,
                character.source_start,
                character.source_end
            )),
            Some(('c', 4, 5))
        );
        assert_eq!(cursor.last().map(|character| character.value), Some('>'));
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
            let diagnostic = match encode_decoded(
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
