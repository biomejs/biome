//! The `carve` pass: strings, comments, template literals, regexes and JSX.
//!
//! It reads the source in three modes. In JavaScript, it visits only the bytes that may open
//! something: quotes, backticks, slashes, braces inside template substitutions and JSX
//! expressions, and `<` where JSX is allowed. In a JSX tag and among JSX children, it reads every
//! byte, as tags are short and children are mostly text.
//!
//! Whether a `/` starts a regex and a `<` starts a JSX element depends on whether an operand or
//! an operator comes next, which [Pass::is_operand_at] decides from the tokens before it.

use super::scan::{KEYWORD_KINDS, is_operand_keyword, keyword, utf8_len};
use super::{JSX_ATTRIBUTE_VALUE, JSX_CHILD, Pass, REGULAR, TEMPLATE, kind};
use crate::lexer::{JsLexContext, JsReLexContext, TextRange, TextSize};
use biome_js_syntax::JsSyntaxKind::*;
use biome_js_syntax::T;
use biome_parser::lexer::{Lexer, TokenFlags};

/// What `carve` reads at some point of the source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    JavaScript,
    /// A JSX tag, after its `<`.
    Tag {
        closing: bool,
    },
    /// The children of a JSX element. With `first_token`, the parser asks for the next token in
    /// [JsLexContext::JsxChild], which it does after a `>` or a `}`; after text, it asks in
    /// [JsLexContext::Regular].
    Children {
        first_token: bool,
    },
}

/// An enclosing construct of the current position.
#[derive(Clone, Copy, Debug)]
enum Frame {
    /// The JavaScript of a `${` … `}` in a template literal, with the number of `{` open in it.
    Substitution { braces: u32 },
    /// The JavaScript of a `{` … `}` in a JSX tag or among JSX children, with the number of `{`
    /// open in it.
    JsxExpression { braces: u32, among_children: bool },
    /// A JSX element, with where the parser continues after its last `>`.
    Element { returns_to: Mode },
}

/// How many tokens [Pass::is_operand_at] walks back to match a bracket before it guesses.
const MATCH_LIMIT: u32 = 4096;

impl Pass<'_, '_> {
    /// Finds the literals, comments, regexes and template and JSX tokens.
    pub(super) fn carve(&mut self) {
        let mut frames = Vec::new();
        let mut mode = Mode::JavaScript;
        let mut offset = 0;
        // The next `<` at or after `offset`, which only JSX needs.
        let mut next_angle = if self.options.jsx {
            memchr::memchr(b'<', self.source).unwrap_or(self.source.len())
        } else {
            self.source.len()
        };
        while offset < self.source.len() {
            (mode, offset) = match mode {
                Mode::JavaScript => self.carve_javascript(offset, &mut frames, &mut next_angle),
                Mode::Tag { closing } => self.carve_tag(offset, closing, &mut frames),
                Mode::Children { first_token } => {
                    self.carve_children(offset, first_token, &mut frames)
                }
            };
        }
    }

    /// Reads JavaScript from `offset` until a JSX tag or children start. Returns the mode and the
    /// offset to continue at.
    fn carve_javascript(
        &mut self,
        mut offset: usize,
        frames: &mut Vec<Frame>,
        next_angle: &mut usize,
    ) -> (Mode, usize) {
        let len = self.source.len();
        loop {
            let counts_braces = matches!(
                frames.last(),
                Some(Frame::Substitution { .. } | Frame::JsxExpression { .. })
            );
            let mut next = if counts_braces {
                self.next_bit(offset, |block| {
                    block.special | block.non_ascii | block.brace
                })
            } else {
                self.next_bit(offset, |block| block.special | block.non_ascii)
            };
            if self.options.jsx {
                if *next_angle < offset {
                    *next_angle = memchr::memchr(b'<', &self.source[offset..])
                        .map_or(len, |index| offset + index);
                }
                next = next.min(*next_angle);
            }
            if next >= len {
                return (Mode::JavaScript, len);
            }
            offset = match self.source[next] {
                quote @ (b'"' | b'\'') => self.carve_string(next, quote),
                b'`' => self.carve_template(next + 1, frames),
                b'{' => {
                    if let Some(
                        Frame::Substitution { braces } | Frame::JsxExpression { braces, .. },
                    ) = frames.last_mut()
                    {
                        *braces += 1;
                    }
                    next + 1
                }
                b'}' => match frames.last_mut() {
                    Some(Frame::Substitution { braces: 0 }) => {
                        frames.pop();
                        self.carve_template(next + 1, frames)
                    }
                    Some(Frame::JsxExpression {
                        braces: 0,
                        among_children,
                    }) => {
                        let among_children = *among_children;
                        frames.pop();
                        let mode = if among_children {
                            Mode::Children { first_token: true }
                        } else {
                            Mode::Tag { closing: false }
                        };
                        return (mode, next + 1);
                    }
                    Some(Frame::Substitution { braces } | Frame::JsxExpression { braces, .. }) => {
                        *braces -= 1;
                        next + 1
                    }
                    _ => next + 1,
                },
                b'/' => self.carve_slash(next),
                b'<' => {
                    if self.starts_jsx_element(next) {
                        frames.push(Frame::Element {
                            returns_to: Mode::JavaScript,
                        });
                        return (Mode::Tag { closing: false }, next + 1);
                    }
                    next + 1
                }
                b'#' if next == 0 || next == self.bom_length => {
                    self.delegate(next, JsLexContext::Regular)
                }
                b'#' => next + 1,
                // `\`, non-ASCII chars and control chars.
                _ => self.delegate_containing(next),
            };
        }
    }

    /// Reads a JSX tag from after its `<` (or a `}` inside it) up to its `>`. Returns the mode
    /// and the offset to continue at.
    fn carve_tag(
        &mut self,
        mut offset: usize,
        closing: bool,
        frames: &mut Vec<Frame>,
    ) -> (Mode, usize) {
        let len = self.source.len();
        // Whether the last token is a `=` that starts an attribute value.
        let mut after_equals = false;
        // Whether the last token is a `/` that self-closes the element.
        let mut after_slash = false;
        // The number of `<` open in the type arguments of a TSX element, `<Generic<T>>`.
        let mut type_arguments = 0u32;
        while offset < len {
            let byte = self.source[offset];
            let (next, equals, slash) = match byte {
                b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C => {
                    (offset + 1, after_equals, after_slash)
                }
                b'/' if matches!(self.byte(offset + 1), b'/' | b'*') => {
                    (self.carve_comment(offset), after_equals, after_slash)
                }
                b'/' if self.byte(offset + 1) == b'=' => {
                    self.set_token(offset, offset + 2, T![/=] as u16 | REGULAR);
                    (offset + 2, false, false)
                }
                b'/' => (offset + 1, false, !closing),
                b'>' if type_arguments > 0 => {
                    // `=>` in a function type doesn't close type arguments.
                    if self.source[offset - 1] != b'=' {
                        type_arguments -= 1;
                    }
                    (offset + 1, false, false)
                }
                b'>' => {
                    if after_slash || closing {
                        return (self.close_element(offset, frames), offset + 1);
                    }
                    return (Mode::Children { first_token: true }, offset + 1);
                }
                quote @ (b'"' | b'\'') => {
                    let end = if after_equals {
                        self.carve_attribute_string(offset)
                    } else {
                        self.carve_string(offset, quote)
                    };
                    (end, false, false)
                }
                b'{' => {
                    frames.push(Frame::JsxExpression {
                        braces: 0,
                        among_children: false,
                    });
                    return (Mode::JavaScript, offset + 1);
                }
                b'<' if after_equals => {
                    frames.push(Frame::Element {
                        returns_to: Mode::Tag { closing: false },
                    });
                    return (Mode::Tag { closing: false }, offset + 1);
                }
                b'<' if self.options.typescript && !closing => {
                    type_arguments += 1;
                    (offset + 1, false, false)
                }
                b'=' => {
                    let standalone = !matches!(self.byte(offset + 1), b'=' | b'>');
                    (offset + 1, standalone, false)
                }
                b'\\' | 0x80.. => (self.delegate_containing(offset), false, false),
                0..0x20 | 0x7F => (self.delegate(offset, JsLexContext::Regular), false, false),
                byte if byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$' => {
                    let mut end = offset + 1;
                    while self.byte(end).is_ascii_alphanumeric()
                        || matches!(self.byte(end), b'_' | b'$')
                    {
                        end += 1;
                    }
                    (end, false, false)
                }
                _ => (offset + 1, false, false),
            };
            offset = next;
            after_equals = equals;
            after_slash = slash;
        }
        (Mode::JavaScript, len)
    }

    /// Returns where the parser continues after the last `>` of the innermost element.
    fn close_element(&mut self, angle: usize, frames: &mut Vec<Frame>) -> Mode {
        while let Some(frame) = frames.pop() {
            if let Frame::Element { returns_to } = frame {
                if returns_to == Mode::JavaScript {
                    self.jsx_ends.push(angle as u32);
                }
                return returns_to;
            }
        }
        Mode::JavaScript
    }

    /// Reads the children of a JSX element from `offset` up to the next tag or `{`. Returns the
    /// mode and the offset to continue at.
    fn carve_children(
        &mut self,
        offset: usize,
        mut first_token: bool,
        frames: &mut Vec<Frame>,
    ) -> (Mode, usize) {
        let len = self.source.len();
        let stop =
            memchr::memchr2(b'<', b'{', &self.source[offset..]).map_or(len, |index| offset + index);
        if stop > offset {
            // `JsLexer` reports a `>` or a `}` in text.
            if memchr::memchr2(b'>', b'}', &self.source[offset..stop]).is_some() {
                self.delegate(offset, JsLexContext::JsxChild { astro: false });
            } else {
                self.set_token(offset, stop, JSX_TEXT_LITERAL as u16 | JSX_CHILD);
            }
            first_token = false;
        }
        if stop >= len {
            return (Mode::JavaScript, len);
        }
        if self.source[stop] == b'{' {
            if first_token {
                self.set_token(stop, stop + 1, T!['{'] as u16 | JSX_CHILD | REGULAR);
            }
            frames.push(Frame::JsxExpression {
                braces: 0,
                among_children: true,
            });
            return (Mode::JavaScript, stop + 1);
        }
        if first_token {
            // In the regular context, `<<` and `<=` are one token.
            let regular = if matches!(self.byte(stop + 1), b'<' | b'=') {
                0
            } else {
                REGULAR
            };
            self.set_token(stop, stop + 1, T![<] as u16 | JSX_CHILD | regular);
        }
        if self.next_significant_byte(stop + 1) == b'/' {
            return (Mode::Tag { closing: true }, stop + 1);
        }
        frames.push(Frame::Element {
            returns_to: Mode::Children { first_token: true },
        });
        (Mode::Tag { closing: false }, stop + 1)
    }

    /// Returns the first byte at or after `offset` that isn't trivia, or zero at the end.
    fn next_significant_byte(&self, offset: usize) -> u8 {
        self.byte(self.skip_whitespace(offset))
    }

    /// Records the quoted value of a JSX attribute, which has no escapes. Returns its end.
    fn carve_attribute_string(&mut self, start: usize) -> usize {
        let quote = self.source[start];
        match memchr::memchr(quote, &self.source[start + 1..]) {
            Some(index) => {
                let end = start + index + 2;
                self.set_token(start, end, JSX_STRING_LITERAL as u16 | JSX_ATTRIBUTE_VALUE);
                end
            }
            None => self.delegate(start, JsLexContext::JsxAttributeValue { astro: false }),
        }
    }

    /// Whether the `<` at `offset` starts a JSX element, as the parser reads it.
    fn starts_jsx_element(&self, offset: usize) -> bool {
        // `parse_jsx_tag_expression`: `<>` or `<name`.
        let after = self.skip_whitespace(offset + 1);
        let first = self.byte(after);
        let starts_name =
            first.is_ascii_alphabetic() || matches!(first, b'_' | b'$' | b'\\') || first >= 0x80;
        if !(first == b'>' || starts_name) || !self.is_operand_at(offset) {
            return false;
        }
        !(self.options.typescript && starts_name && self.is_type_parameters(after))
    }

    /// Whether a `<` followed by a name at `name` starts type parameters in TSX: those of an
    /// arrow function, as `is_parenthesized_arrow_function_expression` decides, or those of a
    /// function type or call signature, which sit where the parser expects a type.
    fn is_type_parameters(&self, mut name: usize) -> bool {
        if self.word_at(name) == b"const" {
            let after = self.skip_whitespace(name + 5);
            if after > name + 5
                && self
                    .word_at(after)
                    .first()
                    .is_some_and(u8::is_ascii_alphabetic)
            {
                name = after;
            }
        }
        let after_name = self.skip_whitespace(name + self.word_at(name).len());
        match self.byte(after_name) {
            b',' | b'=' => true,
            b'>' => self.parameters_precede_return_type(after_name + 1),
            _ if self.word_at(after_name) == b"extends" => {
                let after_extends = self.skip_whitespace(after_name + 7);
                match self.byte(after_extends) {
                    // `<a extends=`, `<a extends/>` and `<a extends>` are elements.
                    b'=' | b'/' | b'>' => false,
                    // `<A extends B>` is either; the parser tries an arrow function first.
                    byte if byte.is_ascii_alphabetic() || matches!(byte, b'_' | b'$') => {
                        self.closes_type_parameters_before_parenthesis(after_extends)
                    }
                    _ => true,
                }
            }
            _ => false,
        }
    }

    /// Whether the type parameters around `offset` end with a `>` followed by `(`.
    fn closes_type_parameters_before_parenthesis(&self, offset: usize) -> bool {
        let mut depth = 1u32;
        let mut position = offset;
        while position < self.source.len() && position - offset < 1024 {
            match self.source[position] {
                b'<' => depth += 1,
                b'>' if self.source[position - 1] == b'=' => {}
                b'>' => {
                    depth -= 1;
                    if depth == 0 {
                        return self.byte(self.skip_whitespace(position + 1)) == b'(';
                    }
                }
                b'{' | b'}' | b';' => return false,
                _ => {}
            }
            position += 1;
        }
        false
    }

    /// Whether parameters start at `offset` and end with a `)` followed by `=>` or `:`, as in
    /// `<T>(x: T) => T` or `<T>(x: T): T`, rather than with JSX text such as `<b>(note)</b>`.
    fn parameters_precede_return_type(&self, offset: usize) -> bool {
        let open = self.skip_whitespace(offset);
        if self.byte(open) != b'(' {
            return false;
        }
        let mut depth = 0u32;
        let mut position = open;
        while position < self.source.len() && position - open < 1024 {
            match self.source[position] {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        let after = self.skip_whitespace(position + 1);
                        return match self.byte(after) {
                            b':' => true,
                            b'=' => self.byte(after + 1) == b'>',
                            _ => false,
                        };
                    }
                }
                b'{' | b'}' | b';' => return false,
                _ => {}
            }
            position += 1;
        }
        false
    }

    /// Returns the ASCII identifier bytes at `offset`.
    fn word_at(&self, offset: usize) -> &[u8] {
        let mut end = offset;
        while self.byte(end).is_ascii_alphanumeric() || matches!(self.byte(end), b'_' | b'$') {
            end += 1;
        }
        &self.source[offset.min(end)..end]
    }

    /// Returns the first offset at or after `offset` that isn't whitespace or a comment.
    fn skip_whitespace(&self, mut offset: usize) -> usize {
        loop {
            match (self.byte(offset), self.byte(offset + 1)) {
                (b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C, _) => offset += 1,
                (b'/', b'/') => {
                    offset = memchr::memchr2(b'\n', b'\r', &self.source[offset..])
                        .map_or(self.source.len(), |index| offset + index);
                }
                (b'/', b'*') => {
                    offset = memchr::memmem::find(&self.source[offset + 2..], b"*/")
                        .map_or(self.source.len(), |index| offset + 2 + index + 2);
                }
                _ => return offset,
            }
        }
    }

    fn carve_string(&mut self, start: usize, quote: u8) -> usize {
        let mut offset = start + 1;
        loop {
            offset = self.next_bit(offset, |block| block.special | block.line_break);
            if offset >= self.source.len() {
                return self.delegate(start, JsLexContext::Regular);
            }
            match self.source[offset] {
                byte if byte == quote => {
                    self.set_token(start, offset + 1, JS_STRING_LITERAL as u16 | REGULAR);
                    return offset + 1;
                }
                b'\\' => match self.skip_escape(offset) {
                    Some(end) => offset = end,
                    None => return self.delegate(start, JsLexContext::Regular),
                },
                b'\n' | b'\r' => return self.delegate(start, JsLexContext::Regular),
                _ => offset += 1,
            }
        }
    }

    /// Returns the end of the escape sequence at `offset`, or `None` if it may be invalid.
    fn skip_escape(&self, offset: usize) -> Option<usize> {
        let is_hex = |offset: usize| self.byte(offset).is_ascii_hexdigit();
        match self.byte(offset + 1) {
            _ if offset + 1 >= self.source.len() => None,
            b'u' if (2..6).all(|index| is_hex(offset + index)) => Some(offset + 6),
            b'x' if is_hex(offset + 2) && is_hex(offset + 3) => Some(offset + 4),
            b'u' | b'x' => None,
            b'\r' if self.byte(offset + 2) == b'\n' => Some(offset + 3),
            byte if byte >= 0x80 => Some(offset + 1 + utf8_len(byte)),
            _ => Some(offset + 2),
        }
    }

    /// Lexes the template elements from `offset` up to the end of the template or the start of
    /// a substitution. Returns the offset after them.
    fn carve_template(&mut self, mut offset: usize, frames: &mut Vec<Frame>) -> usize {
        let len = self.source.len();
        loop {
            if offset >= len {
                return len;
            }
            match self.source[offset] {
                b'`' => {
                    self.set_token(offset, offset + 1, BACKTICK as u16 | TEMPLATE);
                    return offset + 1;
                }
                b'$' if self.byte(offset + 1) == b'{' => {
                    self.set_token(offset, offset + 2, DOLLAR_CURLY as u16 | TEMPLATE);
                    frames.push(Frame::Substitution { braces: 0 });
                    return offset + 2;
                }
                _ => offset = self.carve_template_chunk(offset),
            }
        }
    }

    fn carve_template_chunk(&mut self, start: usize) -> usize {
        let len = self.source.len();
        let mut offset = start;
        loop {
            offset = match memchr::memchr3(b'`', b'$', b'\\', &self.source[offset..]) {
                Some(index) => offset + index,
                None => len,
            };
            match self.byte(offset) {
                _ if offset >= len => {
                    return self.delegate(start, JsLexContext::TemplateElement { tagged: false });
                }
                b'$' if self.byte(offset + 1) != b'{' => offset += 1,
                b'\\' => match self.skip_escape(offset) {
                    Some(end) => offset = end,
                    None => {
                        return self
                            .delegate(start, JsLexContext::TemplateElement { tagged: false });
                    }
                },
                _ => {
                    self.set_token(start, offset, TEMPLATE_CHUNK as u16 | TEMPLATE);
                    return offset;
                }
            }
        }
    }

    /// Records the comment starting with `//` or `/*` at `start`. Returns its end.
    fn carve_comment(&mut self, start: usize) -> usize {
        if self.byte(start + 1) == b'/' {
            let mut end = self.next_bit(start + 2, |block| block.line_break);
            if let Some(separator) = self.find_line_separator(start + 2, end) {
                end = separator;
            }
            self.set_token(start, end, COMMENT as u16 | REGULAR);
            return end;
        }
        let close = self
            .block_comment_end
            .get_or_insert_with(|| memchr::memmem::Finder::new(b"*/"))
            .find(&self.source[start + 2..]);
        let Some(close) = close else {
            return self.delegate(start, JsLexContext::Regular);
        };
        let close = start + 2 + close;
        let has_line_break = self.next_bit_before(start + 2, close, |block| block.line_break)
            < close
            || self.find_line_separator(start + 2, close).is_some();
        let kind = if has_line_break {
            MULTILINE_COMMENT
        } else {
            COMMENT
        };
        self.set_token(start, close + 2, kind as u16 | REGULAR);
        close + 2
    }

    fn carve_slash(&mut self, start: usize) -> usize {
        let next = self.byte(start + 1);
        if matches!(next, b'/' | b'*') {
            return self.carve_comment(start);
        }
        let slash = if next == b'=' { T![/=] } else { T![/] };
        let slash_len = if next == b'=' { 2 } else { 1 };
        if self.is_regex_start(start) {
            let range = TextRange::new(
                TextSize::from(start as u32),
                TextSize::from((start + slash_len) as u32),
            );
            let diagnostics = self.lexer.diagnostics_len();
            let regex =
                self.lexer
                    .re_lex_at(range, slash, TokenFlags::empty(), JsReLexContext::Regex);
            let end = self.lexer.position().min(self.source.len());
            let reported = self.lexer.take_diagnostics_after(diagnostics);
            if regex == JS_REGEX_LITERAL {
                // The regex's diagnostics, of its flags, are the same when the parser re-lexes it.
                let has_diagnostics = !reported.is_empty();
                self.diagnostics
                    .extend(
                        reported
                            .into_iter()
                            .map(|diagnostic| super::TokenDiagnostic {
                                start: start as u32,
                                diagnostic,
                                in_tagged_template: true,
                            }),
                    );
                let flags = if has_diagnostics {
                    super::DIAGNOSTICS
                } else {
                    0
                };
                self.set_token(start, end, JS_REGEX_LITERAL as u16 | REGULAR | flags);
                return end;
            }
            // An unterminated regex: leave the `/` to the parser, which re-lexes it.
        }
        if slash_len == 2 {
            self.set_token(start, start + 2, T![/=] as u16 | REGULAR);
        }
        start + slash_len
    }

    /// Whether the `/` at `offset` starts a regex.
    fn is_regex_start(&self, offset: usize) -> bool {
        // A `/` after `<` ends a JSX element, even where the `<` reads as an operator.
        if offset > 0 && self.source[offset - 1] == b'<' {
            return false;
        }
        self.is_operand_at(offset)
    }

    /// Returns the offset of the first U+2028 or U+2029 in `from..to`.
    fn find_line_separator(&self, from: usize, to: usize) -> Option<usize> {
        let mut offset = from;
        loop {
            offset = self.next_bit_before(offset, to, |block| block.non_ascii);
            if offset >= to {
                return None;
            }
            if self.source[offset..].starts_with(b"\xE2\x80\xA8")
                || self.source[offset..].starts_with(b"\xE2\x80\xA9")
            {
                return Some(offset);
            }
            offset += 1;
        }
    }

    /// Lexes the token that contains `offset` with `JsLexer`, and the tokens after it until one
    /// ends after `offset`. Returns the end of the last one.
    ///
    /// A `\` or non-ASCII char continues the identifier or whitespace before it, if any.
    fn delegate_containing(&mut self, offset: usize) -> usize {
        let continues_token = offset > 0
            && matches!(
                self.source[offset - 1],
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'$' | b'\t' | 0x0B | 0x0C | b' ' | 0x80..
            );
        let mut start = if continues_token {
            self.previous_start(offset)
                .unwrap_or(0)
                .max(self.carved_end)
        } else {
            offset
        };
        loop {
            let end = self.delegate(start, JsLexContext::Regular);
            if end > offset {
                return end;
            }
            start = end;
        }
    }

    /// Returns the start of the last token before `offset` that isn't trivia.
    fn previous_token(&self, offset: usize) -> Option<usize> {
        let mut position = offset;
        while let Some(start) = self.previous_start(position) {
            position = start;
            match self.source[start] {
                b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C => {}
                b'/' if self
                    .carved_kind(start)
                    .is_some_and(|raw| matches!(kind(raw), COMMENT | MULTILINE_COMMENT)) => {}
                _ if self
                    .carved_kind(start)
                    .is_some_and(|raw| matches!(kind(raw), WHITESPACE | NEWLINE)) => {}
                _ => return Some(start),
            }
        }
        None
    }

    /// Whether an operand, rather than an operator, starts at `offset`: whether a `/` there
    /// starts a regex and a `<` may start a JSX element. Decides from the tokens before it, as
    /// the parser's grammar would.
    pub(super) fn is_operand_at(&self, offset: usize) -> bool {
        self.is_operand_after(self.previous_token(offset))
    }

    /// Whether an operand follows the token starting at `previous`, or the start of the source.
    fn is_operand_after(&self, previous: Option<usize>) -> bool {
        let Some(start) = previous else {
            return true;
        };
        if let Some(raw) = self.carved_kind(start)
            && raw & (TEMPLATE | REGULAR) != 0
        {
            return match kind(raw) {
                DOLLAR_CURLY => true,
                // A template literal, a string, a regex, a number or an escaped identifier.
                BACKTICK | JS_STRING_LITERAL | JS_REGEX_LITERAL | JS_NUMBER_LITERAL | IDENT => {
                    false
                }
                kind if kind.is_keyword() => is_operand_keyword(kind),
                _ => true,
            };
        }
        match self.source[start] {
            b'0'..=b'9' | b']' => false,
            byte if byte.is_ascii_alphabetic() || matches!(byte, b'_' | b'$') => {
                if start > 0
                    && self.source[start - 1] == b'.'
                    && (start < 2 || self.source[start - 2] != b'.')
                {
                    return false;
                }
                let end = self.token_end(start);
                is_operand_keyword(kind(
                    KEYWORD_KINDS[keyword(self.source, start, end) as usize],
                ))
            }
            b')' => self.parenthesis_ends_statement_head(start),
            b'}' => !self.brace_closes_value(start),
            // Postfix `++` and `--` end an operand, prefix ones start one.
            byte @ (b'+' | b'-') if start > 0 && self.source[start - 1] == byte => {
                !self.ends_operand_before(start - 1)
            }
            // TypeScript's postfix `!` ends an operand.
            b'!' if self.options.typescript && self.byte(start + 1) != b'=' => {
                !self.ends_operand_before(start)
            }
            b'>' => {
                // The last `>` of a JSX element ends an operand.
                self.jsx_ends.binary_search(&(start as u32)).is_err()
            }
            _ => true,
        }
    }

    /// Whether only an expression, and not a statement, can follow the token starting at
    /// `previous`, so that a `function` or `class` there starts an expression rather than a
    /// declaration.
    fn only_expression_after(&self, previous: Option<usize>) -> bool {
        let Some(start) = previous else {
            return false;
        };
        if let Some(raw) = self.carved_kind(start)
            && kind(raw) == DOLLAR_CURLY
        {
            return true;
        }
        match self.source[start] {
            b'(' | b'[' | b',' | b'=' | b'?' | b':' | b'+' | b'-' | b'*' | b'%' | b'&' | b'|'
            | b'^' | b'!' | b'~' | b'<' | b'>' => true,
            byte if byte.is_ascii_alphabetic() => {
                let end = start + self.word_at(start).len();
                let keyword = kind(KEYWORD_KINDS[keyword(self.source, start, end) as usize]);
                // A statement follows `else` and `do`.
                is_operand_keyword(keyword) && !matches!(keyword, T![else] | T![do])
            }
            _ => false,
        }
    }

    /// Whether the byte right before `offset`, on the same line, ends an operand: a name, a
    /// literal or a closing bracket.
    fn ends_operand_before(&self, offset: usize) -> bool {
        offset > 0
            && matches!(
                self.source[offset - 1],
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'$' | b')' | b']' | b'"' | b'\''
                    | b'`'
            )
    }

    /// Returns the start of the bracket that the one at `close` closes, counting only
    /// `open` and `close` tokens.
    fn matching_open(&self, close: usize, open: u8) -> Option<usize> {
        let close_byte = self.source[close];
        let mut depth = 1u32;
        let mut steps = 0;
        let mut position = close;
        while let Some(start) = self.previous_start(position) {
            position = start;
            steps += 1;
            if steps > MATCH_LIMIT {
                return None;
            }
            let byte = self.source[start];
            let is_open = if byte == open {
                // A carved token starting with `(` is JSX text; `{` always opens a block.
                open == b'{' || self.blocks[start / 64].overridden & (1 << (start % 64)) == 0
            } else {
                open == b'{'
                    && byte == b'$'
                    && self
                        .carved_kind(start)
                        .is_some_and(|raw| kind(raw) == DOLLAR_CURLY)
            };
            if is_open {
                depth -= 1;
                if depth == 0 {
                    return Some(start);
                }
            } else if byte == close_byte
                && (close_byte == b'}'
                    || self.blocks[start / 64].overridden & (1 << (start % 64)) == 0)
            {
                depth += 1;
            }
        }
        None
    }

    /// Whether the `)` at `close` ends the head of an `if`, `for`, `while` or `with`, after which
    /// a statement follows.
    fn parenthesis_ends_statement_head(&self, close: usize) -> bool {
        let Some(open) = self.matching_open(close, b'(') else {
            return false;
        };
        let Some(before) = self.previous_token(open) else {
            return false;
        };
        match self.word_at(before) {
            b"if" | b"for" | b"while" | b"with" => true,
            b"await" => self
                .previous_token(before)
                .is_some_and(|start| self.word_at(start) == b"for"),
            _ => false,
        }
    }

    /// Whether the `}` at `close` closes a value, an object literal or the body of a function
    /// or class expression, rather than a block, after which a statement follows.
    fn brace_closes_value(&self, close: usize) -> bool {
        let Some(open) = self.matching_open(close, b'{') else {
            return false;
        };
        let Some(before) = self.previous_token(open) else {
            return false;
        };
        if let Some(raw) = self.carved_kind(before)
            && kind(raw) == DOLLAR_CURLY
        {
            return true;
        }
        match self.source[before] {
            b'(' | b'[' | b',' | b'=' | b'?' | b'+' | b'-' | b'*' | b'%' | b'&' | b'|' | b'^'
            | b'!' | b'~' | b'<' => true,
            // An arrow function's body.
            b'>' if before > 0 && self.source[before - 1] == b'=' => false,
            b'>' => true,
            b')' => {
                // A function expression's body is a value, unlike a declaration's.
                let Some(open_parenthesis) = self.matching_open(before, b'(') else {
                    return false;
                };
                let mut keyword = self.previous_token(open_parenthesis);
                if let Some(name) = keyword
                    && self.word_at(name) != b"function"
                    && self.source[name].is_ascii_alphabetic()
                {
                    keyword = self.previous_token(name);
                }
                if let Some(star) = keyword
                    && self.source[star] == b'*'
                {
                    keyword = self.previous_token(star);
                }
                match keyword {
                    Some(function) if self.word_at(function) == b"function" => {
                        self.only_expression_after(self.previous_token(function))
                    }
                    _ => false,
                }
            }
            byte if byte.is_ascii_alphabetic() || matches!(byte, b'_' | b'$') => {
                let word = self.word_at(before);
                if is_operand_keyword(kind(
                    KEYWORD_KINDS[keyword(self.source, before, before + word.len()) as usize],
                )) {
                    return true;
                }
                // A class expression's body is a value, unlike a declaration's.
                let mut position = before;
                for _ in 0..6 {
                    if self.word_at(position) == b"class" {
                        return self.only_expression_after(self.previous_token(position));
                    }
                    match self.previous_token(position) {
                        Some(previous) => position = previous,
                        None => return false,
                    }
                }
                false
            }
            _ => false,
        }
    }
}
