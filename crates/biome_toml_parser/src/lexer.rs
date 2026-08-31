use biome_parser::{
    diagnostic::ParseDiagnostic,
    lexer::{LexContext, Lexer, LexerCheckpoint, TokenFlags},
};
use biome_toml_syntax::{T, TextSize, TomlSyntaxKind, TomlSyntaxKind::*};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum TomlLexContext {
    #[default]
    Key,
    Value,
    ArrayValue,
}

impl LexContext for TomlLexContext {
    fn is_regular(&self) -> bool {
        matches!(self, Self::Key)
    }
}

pub(crate) struct TomlLexer<'source> {
    source: &'source str,
    position: usize,
    current_kind: TomlSyntaxKind,
    current_start: TextSize,
    diagnostics: Vec<ParseDiagnostic>,
    current_flags: TokenFlags,
    after_newline: bool,
    unicode_bom_length: usize,
}

impl<'source> TomlLexer<'source> {
    pub fn from_str(source: &'source str) -> Self {
        Self {
            source,
            position: 0,
            current_kind: EOF,
            current_start: TextSize::default(),
            diagnostics: Vec::new(),
            current_flags: TokenFlags::empty(),
            after_newline: false,
            unicode_bom_length: 0,
        }
    }

    fn consume_token(&mut self, context: TomlLexContext) -> TomlSyntaxKind {
        let Some(current) = self.current_byte() else {
            return EOF;
        };

        match current {
            b' ' | b'\t' | b'\n' => self.consume_newline_or_whitespaces(),
            b'\r' if self.peek_byte() == Some(b'\n') => self.consume_newline_or_whitespaces(),
            b'\r' => self.consume_bare_carriage_return(),
            b'#' => self.consume_comment(),
            b'"' | b'\'' => self.consume_string(current, context != TomlLexContext::Key),
            b'[' => self.consume_byte(T!['[']),
            b']' => self.consume_byte(T![']']),
            b'{' => self.consume_byte(T!['{']),
            b'}' => self.consume_byte(T!['}']),
            b',' => self.consume_byte(T![,]),
            b'=' if context == TomlLexContext::Key => self.consume_byte(T![=]),
            b'.' if context == TomlLexContext::Key => self.consume_byte(T![.]),
            byte if context == TomlLexContext::Key && is_bare_key_byte(byte) => {
                self.consume_bare_key()
            }
            byte if context == TomlLexContext::ArrayValue
                && self.after_newline
                && is_bare_key_byte(byte)
                && self.at_key_value() =>
            {
                self.consume_bare_key()
            }
            _ if matches!(context, TomlLexContext::Value | TomlLexContext::ArrayValue) => {
                self.consume_bare_value()
            }
            _ if self.position == 0 => {
                if let Some((kind, length)) = self.consume_potential_bom(UNICODE_BOM) {
                    self.unicode_bom_length = length;
                    kind
                } else {
                    self.consume_unexpected_character()
                }
            }
            _ => self.consume_unexpected_character(),
        }
    }

    fn consume_byte(&mut self, kind: TomlSyntaxKind) -> TomlSyntaxKind {
        self.advance(1);
        kind
    }

    fn consume_comment(&mut self) -> TomlSyntaxKind {
        while let Some(byte) = self.current_byte() {
            if matches!(byte, b'\r' | b'\n') {
                break;
            }
            if is_forbidden_control(byte) {
                let start = self.text_position();
                self.diagnostics.push(ParseDiagnostic::new(
                    "Control characters are not allowed in TOML comments",
                    start..start + TextSize::from(1),
                ));
            }
            self.advance_byte_or_char(byte);
        }
        COMMENT
    }

    fn consume_bare_key(&mut self) -> TomlSyntaxKind {
        while self.current_byte().is_some_and(is_bare_key_byte) {
            self.advance(1);
        }
        TOML_BARE_KEY
    }

    pub(crate) fn current_starts_key_value(&self) -> bool {
        self.at_key_value_from(u32::from(self.current_start) as usize)
    }

    pub(crate) fn current_starts_unambiguous_table_header(&self) -> bool {
        let bytes = self.source.as_bytes();
        let mut position = u32::from(self.current_start) as usize;
        if bytes.get(position) != Some(&b'[') {
            return false;
        }
        position += 1;

        let array_table = bytes.get(position) == Some(&b'[');
        if array_table {
            position += 1;
        }

        let key_start = position;
        let Some((key_end, dotted)) = scan_key(bytes, position) else {
            return false;
        };
        position = skip_horizontal_whitespace(bytes, key_end);
        if bytes.get(position) != Some(&b']') {
            return false;
        }
        position += 1;
        if array_table {
            if bytes.get(position) != Some(&b']') {
                return false;
            }
            position += 1;
        }

        position = skip_horizontal_whitespace(bytes, position);
        if !matches!(bytes.get(position), None | Some(b'\r' | b'\n' | b'#')) {
            return false;
        }

        let key = self.source[key_start..key_end].trim_matches([' ', '\t']);
        let quoted = matches!(key.as_bytes().first(), Some(b'\'' | b'"'));
        (dotted || !quoted) && bare_value_kind(key).is_none()
    }

    fn at_key_value(&self) -> bool {
        self.at_key_value_from(self.position)
    }

    fn at_key_value_from(&self, start: usize) -> bool {
        let bytes = self.source.as_bytes();
        scan_key(bytes, start).is_some_and(|(position, _)| {
            bytes.get(skip_horizontal_whitespace(bytes, position)) == Some(&b'=')
        })
    }

    fn consume_bare_value(&mut self) -> TomlSyntaxKind {
        let start = self.position;
        while let Some(byte) = self.current_byte() {
            if byte == b' '
                && self.position - start == 10
                && is_local_date(&self.source[start..self.position])
                && self.peek_byte().is_some_and(|byte| byte.is_ascii_digit())
            {
                self.advance(1);
                continue;
            }
            if matches!(
                byte,
                b' ' | b'\t' | b'\r' | b'\n' | b'#' | b',' | b']' | b'}'
            ) {
                break;
            }
            self.advance_byte_or_char(byte);
        }

        if start == self.position {
            return self.consume_unexpected_character();
        }

        let value = &self.source[start..self.position];
        if let Some(kind) = bare_value_kind(value) {
            kind
        } else {
            self.diagnostics.push(ParseDiagnostic::new(
                "Invalid TOML value",
                TextSize::from(start as u32)..self.text_position(),
            ));
            ERROR_TOKEN
        }
    }

    fn consume_string(&mut self, quote: u8, allow_multiline: bool) -> TomlSyntaxKind {
        let start = self.text_position();
        let kind = if quote == b'"' {
            TOML_BASIC_STRING
        } else {
            TOML_LITERAL_STRING
        };
        let multiline = self.byte_at(1) == Some(quote) && self.byte_at(2) == Some(quote);

        if multiline {
            self.advance(3);
            if !allow_multiline {
                self.diagnostics.push(ParseDiagnostic::new(
                    "Multiline strings cannot be used as TOML keys",
                    start..self.text_position(),
                ));
            }
        } else {
            self.advance(1);
        }

        while let Some(byte) = self.current_byte() {
            if byte == quote {
                if !multiline {
                    self.advance(1);
                    return kind;
                }

                let quote_count = self.count_consecutive(quote);
                if quote_count >= 3 {
                    let consumed = quote_count.min(5);
                    self.advance(consumed);
                    if quote_count > 5 {
                        self.diagnostics.push(ParseDiagnostic::new(
                            "Too many quotes at the end of a multiline string",
                            self.text_position() - TextSize::from(consumed as u32)
                                ..self.text_position(),
                        ));
                    }
                    return kind;
                }

                self.advance(quote_count);
                continue;
            }

            if byte == b'\\' && quote == b'"' {
                self.consume_escape(multiline);
                continue;
            }

            if byte == b'\n' || (byte == b'\r' && self.peek_byte() == Some(b'\n')) {
                if multiline {
                    self.consume_newline();
                    continue;
                }
                self.diagnostics.push(ParseDiagnostic::new(
                    "Single-line strings cannot contain line breaks",
                    start..self.text_position(),
                ));
                return kind;
            }

            if is_forbidden_control(byte) {
                let position = self.text_position();
                self.diagnostics.push(ParseDiagnostic::new(
                    "Control characters are not allowed in TOML strings",
                    position..position + TextSize::from(1),
                ));
            }
            self.advance_byte_or_char(byte);
        }

        self.diagnostics.push(
            ParseDiagnostic::new("Unterminated TOML string", start..self.text_position())
                .with_detail(
                    self.text_position()..self.text_position(),
                    "The file ends here",
                ),
        );
        kind
    }

    fn consume_escape(&mut self, multiline: bool) {
        let start = self.text_position();
        self.advance(1);

        if multiline {
            let mut offset = 0;
            while matches!(self.byte_at(offset), Some(b' ' | b'\t')) {
                offset += 1;
            }
            let at_newline = self.byte_at(offset) == Some(b'\n')
                || (self.byte_at(offset) == Some(b'\r') && self.byte_at(offset + 1) == Some(b'\n'));
            if at_newline {
                self.advance(offset);
                self.consume_newline();
                loop {
                    match self.current_byte() {
                        Some(b' ' | b'\t') => self.advance(1),
                        Some(b'\n') => {
                            self.consume_newline();
                        }
                        Some(b'\r') if self.peek_byte() == Some(b'\n') => {
                            self.consume_newline();
                        }
                        _ => break,
                    }
                }
                return;
            }
        }

        if !multiline
            && (self.current_byte() == Some(b'\n')
                || (self.current_byte() == Some(b'\r') && self.peek_byte() == Some(b'\n')))
        {
            self.diagnostics.push(ParseDiagnostic::new(
                "Invalid escape sequence in TOML basic string",
                start..self.text_position(),
            ));
            return;
        }

        match self.current_byte() {
            Some(b'b' | b'e' | b't' | b'n' | b'f' | b'r' | b'"' | b'\\') => self.advance(1),
            Some(b'x') => self.consume_unicode_escape(start, 2),
            Some(b'u') => self.consume_unicode_escape(start, 4),
            Some(b'U') => self.consume_unicode_escape(start, 8),
            Some(byte) => {
                self.advance_byte_or_char(byte);
                self.diagnostics.push(ParseDiagnostic::new(
                    "Invalid escape sequence in TOML basic string",
                    start..self.text_position(),
                ));
            }
            None => self.diagnostics.push(ParseDiagnostic::new(
                "Expected an escape sequence after the backslash",
                start..self.text_position(),
            )),
        }
    }

    fn consume_unicode_escape(&mut self, start: TextSize, digits: usize) {
        self.advance(1);
        let mut value = 0u32;
        let mut consumed = 0;
        while consumed < digits {
            let Some(byte) = self.current_byte().filter(u8::is_ascii_hexdigit) else {
                break;
            };
            let digit = u32::from(byte & 0x0f) + u32::from(byte.is_ascii_alphabetic()) * 9;
            value = value * 16 + digit;
            self.advance(1);
            consumed += 1;
        }

        if consumed != digits {
            self.diagnostics.push(ParseDiagnostic::new(
                format!("Unicode escapes must contain {digits} hexadecimal digits"),
                start..self.text_position(),
            ));
        } else if char::from_u32(value).is_none() {
            self.diagnostics.push(ParseDiagnostic::new(
                "Unicode escape does not encode a valid Unicode scalar value",
                start..self.text_position(),
            ));
        }
    }

    fn count_consecutive(&self, expected: u8) -> usize {
        let mut count = 0;
        while self.byte_at(count) == Some(expected) {
            count += 1;
        }
        count
    }

    fn consume_unexpected_character(&mut self) -> TomlSyntaxKind {
        let character = self.current_char_unchecked();
        let start = self.text_position();
        self.advance(character.len_utf8());
        self.diagnostics.push(ParseDiagnostic::new(
            format!("Unexpected character `{character}`"),
            start..self.text_position(),
        ));
        ERROR_TOKEN
    }

    fn consume_bare_carriage_return(&mut self) -> TomlSyntaxKind {
        let start = self.text_position();
        self.advance(1);
        self.diagnostics.push(ParseDiagnostic::new(
            "Carriage returns must be followed by a line feed",
            start..self.text_position(),
        ));
        ERROR_TOKEN
    }
}

impl<'source> Lexer<'source> for TomlLexer<'source> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;

    type Kind = TomlSyntaxKind;
    type LexContext = TomlLexContext;
    type ReLexContext = ();

    fn source(&self) -> &'source str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn current_start(&self) -> TextSize {
        self.current_start
    }

    fn next_token(&mut self, context: Self::LexContext) -> Self::Kind {
        self.current_start = self.text_position();
        self.current_flags = TokenFlags::empty();
        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_newline);

        let kind = if self.is_eof() {
            EOF
        } else {
            self.consume_token(context)
        };
        self.current_kind = kind;

        if kind == NEWLINE {
            self.after_newline = true;
        } else if !matches!(kind, WHITESPACE | COMMENT) {
            self.after_newline = false;
        }

        kind
    }

    fn has_preceding_line_break(&self) -> bool {
        self.current_flags
            .contains(TokenFlags::PRECEDING_LINE_BREAK)
    }

    fn has_unicode_escape(&self) -> bool {
        false
    }

    fn rewind(&mut self, checkpoint: LexerCheckpoint<Self::Kind>) {
        self.position = u32::from(checkpoint.position) as usize;
        self.current_start = checkpoint.current_start;
        self.current_kind = checkpoint.current_kind;
        self.current_flags = checkpoint.current_flags;
        self.after_newline = checkpoint.after_line_break;
        self.unicode_bom_length = checkpoint.unicode_bom_length;
        self.diagnostics
            .truncate(checkpoint.diagnostics_pos as usize);
    }

    fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    fn position(&self) -> usize {
        self.position
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn advance_char_unchecked(&mut self) {
        self.position += self.current_char_unchecked().len_utf8();
    }

    fn advance(&mut self, amount: usize) {
        self.position += amount;
    }
}

fn is_bare_key_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-')
}

fn scan_key(bytes: &[u8], start: usize) -> Option<(usize, bool)> {
    let mut position = start;
    let mut dotted = false;

    loop {
        position = skip_horizontal_whitespace(bytes, position);
        position = scan_key_segment(bytes, position)?;

        let next = skip_horizontal_whitespace(bytes, position);
        if bytes.get(next) == Some(&b'.') {
            dotted = true;
            position = next + 1;
        } else {
            return Some((position, dotted));
        }
    }
}

fn scan_key_segment(bytes: &[u8], mut position: usize) -> Option<usize> {
    match bytes.get(position).copied()? {
        byte if is_bare_key_byte(byte) => {
            position += 1;
            while bytes
                .get(position)
                .is_some_and(|byte| is_bare_key_byte(*byte))
            {
                position += 1;
            }
            Some(position)
        }
        b'\'' => {
            let length = bytes[position + 1..]
                .iter()
                .position(|byte| *byte == b'\'' || matches!(*byte, b'\r' | b'\n'))?;
            (bytes[position + 1 + length] == b'\'').then_some(position + length + 2)
        }
        b'"' => {
            position += 1;
            loop {
                match bytes.get(position).copied() {
                    Some(b'"') => return Some(position + 1),
                    Some(b'\\') => {
                        position += 1;
                        if bytes.get(position).is_none() || matches!(bytes[position], b'\r' | b'\n')
                        {
                            return None;
                        }
                        position += 1;
                    }
                    Some(b'\r' | b'\n') | None => return None,
                    Some(_) => position += 1,
                }
            }
        }
        _ => None,
    }
}

fn skip_horizontal_whitespace(bytes: &[u8], mut position: usize) -> usize {
    while bytes
        .get(position)
        .is_some_and(|byte| matches!(byte, b' ' | b'\t'))
    {
        position += 1;
    }
    position
}

fn bare_value_kind(value: &str) -> Option<TomlSyntaxKind> {
    if matches!(value, "true" | "false") {
        Some(TOML_BOOLEAN)
    } else if is_offset_date_time(value) {
        Some(TOML_OFFSET_DATE_TIME)
    } else if is_local_date_time(value) {
        Some(TOML_LOCAL_DATE_TIME)
    } else if is_local_date(value) {
        Some(TOML_LOCAL_DATE)
    } else if is_local_time(value) {
        Some(TOML_LOCAL_TIME)
    } else if is_float(value) {
        Some(TOML_FLOAT)
    } else if is_integer(value) {
        Some(TOML_INTEGER)
    } else {
        None
    }
}

fn is_forbidden_control(byte: u8) -> bool {
    (byte <= 0x1f && byte != b'\t') || byte == 0x7f
}

fn strip_sign(value: &str) -> &str {
    value.strip_prefix(['+', '-']).unwrap_or(value)
}

fn is_integer(value: &str) -> bool {
    let (unsigned, maximum, has_sign) = if let Some(unsigned) = value.strip_prefix('-') {
        (unsigned, i64::MAX as u64 + 1, true)
    } else if let Some(unsigned) = value.strip_prefix('+') {
        (unsigned, i64::MAX as u64, true)
    } else {
        (value, i64::MAX as u64, false)
    };
    let (digits, radix) = if let Some(digits) = unsigned.strip_prefix("0x") {
        (digits, 16)
    } else if let Some(digits) = unsigned.strip_prefix("0o") {
        (digits, 8)
    } else if let Some(digits) = unsigned.strip_prefix("0b") {
        (digits, 2)
    } else {
        if !valid_digits(unsigned, 10) {
            return false;
        }
        let digit_count = unsigned.bytes().filter(u8::is_ascii_digit).count();
        return (digit_count == 1 || !unsigned.starts_with('0'))
            && integer_magnitude_fits(unsigned, 10, maximum);
    };

    !has_sign && valid_digits(digits, radix) && integer_magnitude_fits(digits, radix, maximum)
}

fn is_float(value: &str) -> bool {
    if matches!(value, "inf" | "+inf" | "-inf" | "nan" | "+nan" | "-nan") {
        return true;
    }

    let unsigned = strip_sign(value);
    let mut exponent_parts = unsigned.split(['e', 'E']);
    let mantissa = exponent_parts.next().unwrap_or_default();
    let exponent = exponent_parts.next();
    if exponent_parts.next().is_some() {
        return false;
    }

    let mut fraction_parts = mantissa.split('.');
    let integer = fraction_parts.next().unwrap_or_default();
    let fraction = fraction_parts.next();
    if fraction_parts.next().is_some() || !valid_digits(integer, 10) {
        return false;
    }
    if integer.bytes().filter(u8::is_ascii_digit).count() > 1 && integer.starts_with('0') {
        return false;
    }
    if fraction.is_some_and(|fraction| !valid_digits(fraction, 10)) {
        return false;
    }
    if exponent.is_some_and(|exponent| !valid_digits(strip_sign(exponent), 10)) {
        return false;
    }

    fraction.is_some() || exponent.is_some()
}

fn valid_digits(value: &str, radix: u32) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.first().is_some_and(u8::is_ascii_hexdigit)
        && bytes.last().is_some_and(u8::is_ascii_hexdigit)
        && bytes.iter().enumerate().all(|(index, byte)| {
            if *byte == b'_' {
                index > 0
                    && index + 1 < bytes.len()
                    && bytes[index - 1].is_ascii_hexdigit()
                    && bytes[index + 1].is_ascii_hexdigit()
            } else {
                match radix {
                    2 => matches!(byte, b'0' | b'1'),
                    8 => matches!(byte, b'0'..=b'7'),
                    10 => byte.is_ascii_digit(),
                    16 => byte.is_ascii_hexdigit(),
                    _ => false,
                }
            }
        })
}

fn integer_magnitude_fits(digits: &str, radix: u32, maximum: u64) -> bool {
    let mut magnitude = 0u64;
    for byte in digits.bytes().filter(|byte| *byte != b'_') {
        let digit = match byte {
            b'0'..=b'9' => u64::from(byte - b'0'),
            b'a'..=b'f' => u64::from(byte - b'a' + 10),
            b'A'..=b'F' => u64::from(byte - b'A' + 10),
            _ => return false,
        };
        if digit >= u64::from(radix) {
            return false;
        }
        let Some(next) = magnitude
            .checked_mul(u64::from(radix))
            .and_then(|magnitude| magnitude.checked_add(digit))
        else {
            return false;
        };
        if next > maximum {
            return false;
        }
        magnitude = next;
    }
    true
}

fn is_local_date(value: &str) -> bool {
    if !value.is_ascii()
        || value.len() != 10
        || value.as_bytes()[4] != b'-'
        || value.as_bytes()[7] != b'-'
        || !value[..4].bytes().all(|byte| byte.is_ascii_digit())
        || !value[5..7].bytes().all(|byte| byte.is_ascii_digit())
        || !value[8..10].bytes().all(|byte| byte.is_ascii_digit())
    {
        return false;
    }

    let Ok(year) = value[..4].parse::<u16>() else {
        return false;
    };
    let Ok(month) = value[5..7].parse::<u8>() else {
        return false;
    };
    let Ok(day) = value[8..10].parse::<u8>() else {
        return false;
    };

    day > 0 && day <= days_in_month(year, month)
}

fn is_local_time(value: &str) -> bool {
    if !value.is_ascii() {
        return false;
    }
    let (time, fraction) = value
        .split_once('.')
        .map_or((value, None), |(time, fraction)| (time, Some(fraction)));
    match time.len() {
        5 => {
            fraction.is_none()
                && time.as_bytes()[2] == b':'
                && number_in_range(&time[..2], 0, 23)
                && number_in_range(&time[3..5], 0, 59)
        }
        8 => {
            time.as_bytes()[2] == b':'
                && time.as_bytes()[5] == b':'
                && number_in_range(&time[..2], 0, 23)
                && number_in_range(&time[3..5], 0, 59)
                && number_in_range(&time[6..8], 0, 60)
                && fraction.is_none_or(|fraction| {
                    !fraction.is_empty() && fraction.bytes().all(|byte| byte.is_ascii_digit())
                })
        }
        _ => false,
    }
}

fn is_local_date_time(value: &str) -> bool {
    value.is_ascii()
        && value.len() > 11
        && matches!(value.as_bytes()[10], b'T' | b't' | b' ')
        && is_local_date(&value[..10])
        && is_local_time(&value[11..])
}

fn is_offset_date_time(value: &str) -> bool {
    if !value.is_ascii()
        || value.len() <= 11
        || !matches!(value.as_bytes()[10], b'T' | b't' | b' ')
        || !is_local_date(&value[..10])
    {
        return false;
    }

    let time_and_offset = &value[11..];
    if let Some(time) = time_and_offset
        .strip_suffix('Z')
        .or_else(|| time_and_offset.strip_suffix('z'))
    {
        return is_offset_time(&value[..10], time, 0);
    }

    if time_and_offset.len() < 11 {
        return false;
    }
    let offset_start = time_and_offset.len() - 6;
    let offset = &time_and_offset[offset_start..];
    let Some(offset_minutes) = parse_offset_minutes(offset) else {
        return false;
    };
    is_offset_time(
        &value[..10],
        &time_and_offset[..offset_start],
        offset_minutes,
    )
}

fn parse_offset_minutes(offset: &str) -> Option<i16> {
    if offset.len() != 6
        || !matches!(offset.as_bytes()[0], b'+' | b'-')
        || offset.as_bytes()[3] != b':'
        || !number_in_range(&offset[1..3], 0, 23)
        || !number_in_range(&offset[4..6], 0, 59)
    {
        return None;
    }

    let hours = offset[1..3].parse::<i16>().ok()?;
    let minutes = offset[4..6].parse::<i16>().ok()?;
    let magnitude = hours * 60 + minutes;
    Some(if offset.starts_with('-') {
        -magnitude
    } else {
        magnitude
    })
}

fn is_offset_time(date: &str, time: &str, offset_minutes: i16) -> bool {
    if !is_local_time(time) {
        return false;
    }
    if time.as_bytes().get(6..8) != Some(b"60") {
        return true;
    }

    let Ok(year) = date[..4].parse::<u16>() else {
        return false;
    };
    let Ok(month) = date[5..7].parse::<u8>() else {
        return false;
    };
    let Ok(day) = date[8..10].parse::<u8>() else {
        return false;
    };
    let Ok(hour) = time[..2].parse::<i16>() else {
        return false;
    };
    let Ok(minute) = time[3..5].parse::<i16>() else {
        return false;
    };

    let utc_minutes = hour * 60 + minute - offset_minutes;
    if utc_minutes.rem_euclid(24 * 60) != 23 * 60 + 59 {
        return false;
    }
    let Some((_, utc_month, utc_day)) =
        shift_date(year, month, day, utc_minutes.div_euclid(24 * 60))
    else {
        return false;
    };
    matches!((utc_month, utc_day), (6, 30) | (12, 31))
}

fn shift_date(year: u16, month: u8, day: u8, days: i16) -> Option<(u16, u8, u8)> {
    match days {
        0 => Some((year, month, day)),
        -1 if day > 1 => Some((year, month, day - 1)),
        -1 if month > 1 => {
            let month = month - 1;
            Some((year, month, days_in_month(year, month)))
        }
        -1 => Some((year.checked_sub(1)?, 12, 31)),
        1 if day < days_in_month(year, month) => Some((year, month, day + 1)),
        1 if month < 12 => Some((year, month + 1, 1)),
        1 => Some((year.checked_add(1)?, 1, 1)),
        _ => None,
    }
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year.is_multiple_of(400) || (year.is_multiple_of(4) && !year.is_multiple_of(100)) => {
            29
        }
        2 => 28,
        _ => 0,
    }
}

fn number_in_range(value: &str, minimum: u8, maximum: u8) -> bool {
    value.bytes().all(|byte| byte.is_ascii_digit())
        && value
            .parse::<u8>()
            .is_ok_and(|value| (minimum..=maximum).contains(&value))
}
