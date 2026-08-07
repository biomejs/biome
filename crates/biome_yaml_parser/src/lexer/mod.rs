use std::collections::VecDeque;

use biome_parser::{
    TokenSet,
    diagnostic::ParseDiagnostic,
    lexer::{Lexer, LexerCheckpoint, LexerWithCheckpoint},
    token_set,
};
use biome_rowan::{TextLen, TextRange, TextSize};
use biome_unicode_table::{Dispatch::WHS, lookup_byte};
use biome_yaml_syntax::{T, YamlSyntaxKind, YamlSyntaxKind::*};
mod tests;

pub(crate) struct YamlLexer<'src> {
    /// Source text
    source: &'src str,

    /// Where the lexer is in the source
    current_coordinate: TextCoordinate,

    /// Diagnostics emitted during the parsing phase
    diagnostics: Vec<ParseDiagnostic>,

    /// Hierarchy of block scopes covering the lexer's current coordinate
    scopes: Vec<BlockScope>,

    /// Cache of tokens to be emitted to the parser
    tokens: VecDeque<LexToken>,

    /// Whether the current token boundary may start a document prefix.
    bom_allowed: bool,
}

impl<'src> YamlLexer<'src> {
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            diagnostics: Vec::new(),
            scopes: Default::default(),
            current_coordinate: Default::default(),
            tokens: VecDeque::from([LexToken::default()]),
            bom_allowed: true,
        }
    }

    /// The kind of the first buffered token that is neither a property nor
    /// trivia, lexing further ahead as needed. The tokens stay buffered, so
    /// consuming them later is unaffected
    pub(crate) fn kind_after_properties(&mut self) -> YamlSyntaxKind {
        /// The tokens the lookahead skips over: the properties themselves
        /// and the trivia between them
        const SKIPPED: TokenSet<YamlSyntaxKind> = token_set![
            ANCHOR_PROPERTY_LITERAL,
            TAG_PROPERTY_LITERAL,
            WHITESPACE,
            NEWLINE,
            COMMENT
        ];

        let mut index = 0;
        loop {
            while self.tokens.len() <= index {
                let before = self.tokens.len();
                self.consume_tokens();
                if self.tokens.len() == before {
                    return EOF;
                }
            }
            match self.tokens.get(index).map(|token| token.kind) {
                Some(kind) if SKIPPED.contains(kind) => index += 1,
                Some(kind) => return kind,
                None => return EOF,
            }
        }
    }

    /// Consume tokens until the lexer found a disambiguated checkpoint.
    /// This usually means that the lexer has determined whether the lexed tokens belong to a block
    /// map entry
    /// ```yaml
    /// - [a, b, c]: ...
    /// ```
    /// or just a normal yaml value
    /// ```yaml
    /// - [a, b, c]
    /// ```
    fn consume_tokens(&mut self) {
        let Some(current) = self.current_byte() else {
            let tokens = self.close_all_scopes();
            self.tokens.extend(tokens);
            self.tokens
                .push_back(LexToken::pseudo(EOF, self.current_coordinate));
            return;
        };

        let start = self.text_position();

        let bom_start = self.current_coordinate;
        let bom_kind = if self.position() == 0 {
            self.consume_potential_bom(UNICODE_BOM)
                .map(|(kind, _)| kind)
        } else if self.is_at_document_prefix_bom() {
            self.advance('\u{feff}'.len_utf8());
            Some(UNICODE_BOM)
        } else {
            None
        };
        if let Some(kind) = bom_kind {
            self.bom_allowed = false;
            self.current_coordinate.column = bom_start.column;
            self.tokens
                .push_back(LexToken::new(kind, bom_start, self.current_coordinate));
            return;
        }

        if !is_space(current) && !is_break(current) && current != b'#' {
            self.bom_allowed = false;
        }

        if self.is_at_bom() {
            let start = self.current_coordinate;
            self.consume_misplaced_bom();
            self.tokens
                .push_back(LexToken::new(ERROR_TOKEN, start, self.current_coordinate));
            return;
        }

        if !self.current_char_is_yaml_printable() {
            let token = self.consume_unexpected_token();
            self.tokens.push_back(token);
            return;
        }

        let tokens = match current {
            c if is_break(c) => self.evaluate_block_scope(),
            c if is_space(c) => self.consume_whitespace_token().into(),
            b'#' => self.consume_comment().into(),
            b'%' if self.is_at_directive() => self.consume_directive().into(),
            b'-' if self.is_at_directive_end() => self.consume_directive_end(),
            b'.' if self.is_at_doc_end() => self.consume_doc_end(),
            b'!' | b'&' => self.consume_block_properties(),
            current if maybe_at_mapping_start(current, self.peek_byte()) => self
                .consume_potential_mapping_start(current, VecDeque::new(), self.current_coordinate),
            // '?', '-' can be a valid plain token start
            b'?' => self.consume_explicit_mapping_key(current),
            b'-' => self.consume_sequence_entry(),
            b'|' | b'>' => self.consume_block_scalar(current),
            _ => self.consume_unexpected_token().into(),
        };
        self.tokens.extend(tokens);

        debug_assert!(self.text_position() > start, "Lexer did not advance");
    }

    fn consume_sequence_entry(&mut self) -> VecDeque<LexToken> {
        self.assert_byte(b'-');
        let indicator = self.consume_byte_as_token(T![-]);

        if self
            .scopes
            .last()
            .is_none_or(|scope| scope.indent_with_dash(indicator.start))
        {
            let mut tokens = VecDeque::new();
            tokens.push_back(indicator);
            tokens.push_front(LexToken::pseudo(SEQUENCE_START, indicator.start));
            self.scopes
                .push(BlockScope::new_sequence_scope(indicator.start));
            tokens
        } else {
            indicator.into()
        }
    }

    /// Consume an explicit mapping key, indicated by '?'.
    /// '?' signifies an explicit mapping key, which opens a block mapping entry
    /// at the current indentation.
    fn consume_explicit_mapping_key(&mut self, current: u8) -> VecDeque<LexToken> {
        debug_assert!(matches!(current, b'?' | b':'));
        let indicator = if current == b'?' {
            self.consume_byte_as_token(T![?])
        } else {
            self.consume_byte_as_token(T![:])
        };
        if self
            .scopes
            .last()
            .is_none_or(|scope| scope.indent(indicator.start))
        {
            let mut tokens = VecDeque::new();
            tokens.push_front(LexToken::pseudo(MAPPING_START, indicator.start));
            tokens.push_back(indicator);
            self.scopes
                .push(BlockScope::new_mapping_scope(indicator.start));
            tokens
        } else {
            indicator.into()
        }
    }

    /// Consume and disambiguate a YAML value to determine whether it opens a block
    /// mapping entry, with or without properties, or just a standalone flow value.
    fn consume_potential_mapping_start(
        &mut self,
        current: u8,
        properties: VecDeque<LexToken>,
        start_coordinate: TextCoordinate,
    ) -> VecDeque<LexToken> {
        debug_assert!(maybe_at_mapping_start(current, self.peek_byte()));

        // When the properties sit on lines of their own above the key, they
        // belong to the mapping rather than the key, and the mapping's
        // indentation is set by the key's column, not theirs:
        //
        // ```yaml
        // key: &anchor
        //   a: 1
        // ```
        let key_coordinate = self.current_coordinate;
        let same_line = start_coordinate.offset - start_coordinate.column
            == key_coordinate.offset - key_coordinate.column;
        let scope_coordinate = if same_line {
            start_coordinate
        } else {
            key_coordinate
        };

        let mut tokens = properties;
        let mut potential_mapping_keys = self.consume_potential_mapping_key(current);
        let key_end = self.current_coordinate;
        tokens.append(&mut potential_mapping_keys);

        // Consume any trailing trivia remaining before closing the mapping/flow, as we must not
        // have trailing trivia followed MAPPING_END/FLOW_END token
        let mut trivia = self.consume_trivia(true);
        tokens.append(&mut trivia);

        if self
            .scopes
            .last()
            .is_none_or(|scope| scope.indent(scope_coordinate))
        {
            if self.is_at_mapping_indicator() {
                self.report_multiline_implicit_key(key_coordinate, key_end);
                let indicator = self.consume_byte_as_token(T![:]);
                tokens.push_front(LexToken::pseudo(MAPPING_START, start_coordinate));
                tokens.push_back(indicator);
                self.scopes
                    .push(BlockScope::new_mapping_scope(scope_coordinate));
            } else {
                // Just a normal flow value
                tokens.push_front(LexToken::pseudo(FLOW_START, start_coordinate));
                tokens.push_back(LexToken::pseudo(FLOW_END, self.current_coordinate));
            }
        } else if self.is_at_mapping_indicator() {
            self.report_multiline_implicit_key(key_coordinate, key_end);
            // At a valid mapping key, lex the `:` so that the lexer wouldn't confuse it with a
            // standalone `:` token, which indicate the start of an empty mapping key
            let indicator = self.consume_byte_as_token(T![:]);
            tokens.push_back(indicator);
        }
        tokens
    }

    fn consume_block_scalar(&mut self, current: u8) -> VecDeque<LexToken> {
        debug_assert!(matches!(current, b'|' | b'>'));

        let mut tokens = VecDeque::new();

        let style_token = if current == b'|' {
            self.consume_byte_as_token(T![|])
        } else {
            self.consume_byte_as_token(T![>])
        };
        tokens.push_back(style_token);

        let (mut headers, explicit_indent) = self.consume_block_header_tokens();
        tokens.append(&mut headers);

        let required_indent = explicit_indent.map(|indent| {
            self.scopes
                .last()
                .map_or(indent, |scope| scope.border() + indent)
        });
        tokens.push_back(self.lex_block_content(required_indent));

        tokens
    }

    /// Lex block scalar header indicators, returning tokens and indent size
    fn consume_block_header_tokens(&mut self) -> (VecDeque<LexToken>, Option<usize>) {
        let mut tokens = VecDeque::new();
        let mut explicit_indent = None;

        while let Some(current) = self.current_byte() {
            match current {
                b'0'..=b'9' => {
                    if (b'1'..=b'9').contains(&current)
                        && self.peek_byte().is_none_or(|byte| !byte.is_ascii_digit())
                    {
                        explicit_indent = Some(usize::from(current - b'0'));
                    }
                    tokens.push_back(self.consume_indentation_indicator(current));
                }
                b'-' => {
                    tokens.push_back(self.consume_byte_as_token(T![-]));
                }
                b'+' => {
                    tokens.push_back(self.consume_byte_as_token(T![+]));
                }
                _ => break,
            }
        }

        let mut trivia = self.consume_trailing_trivia();
        tokens.append(&mut trivia);

        (tokens, explicit_indent)
    }

    fn consume_indentation_indicator(&mut self, first_digit: u8) -> LexToken {
        debug_assert!(first_digit.is_ascii_digit());
        let start_coordinate = self.current_coordinate;
        let start_pos = self.text_position();
        self.advance(1);

        let has_more_digits = self.current_byte().is_some_and(|c| c.is_ascii_digit());
        let starts_with_zero = first_digit == b'0';

        let kind = if starts_with_zero || has_more_digits {
            // Consume any remaining digits for better diagnostic
            while self.current_byte().is_some_and(|c| c.is_ascii_digit()) {
                self.advance(1);
            }

            let err = ParseDiagnostic::new(
                "Indentation indicator must be between '1' and '9'",
                start_pos..self.text_position(),
            );
            self.diagnostics.push(err);
            ERROR_TOKEN
        } else {
            INDENTATION_INDICATOR
        };
        LexToken::new(kind, start_coordinate, self.current_coordinate)
    }

    /// Lex the content of a block scalar.
    /// An explicit indentation indicator sets the minimum content indentation. Content remains a
    /// single token when a line is under-indented so the diagnostic does not fragment the scalar.
    /// Start with the newline followed the header, to handle cases where the block content is
    /// empty
    fn lex_block_content(&mut self, required_indent: Option<usize>) -> LexToken {
        debug_assert!(self.current_byte().is_none_or(is_break));
        let start = self.current_coordinate;

        while let Some(current) = self.current_byte() {
            if !self.current_char_is_yaml_printable() {
                self.consume_invalid_character();
                continue;
            }

            if is_break(current) {
                let might_be_token_end = self.current_coordinate;
                if !self.is_scalar_continuation(required_indent) {
                    return LexToken::new(BLOCK_CONTENT_LITERAL, start, might_be_token_end);
                }
            } else {
                self.advance_char_unchecked();
            }
        }

        LexToken::new(BLOCK_CONTENT_LITERAL, start, self.current_coordinate)
    }

    fn evaluate_block_scope(&mut self) -> VecDeque<LexToken> {
        debug_assert!(self.current_byte().is_some_and(is_break));
        let start = self.current_coordinate;
        let mut trivia = self.consume_trivia(false);
        let mut scope_end_tokens = self.close_breached_scopes(start);
        scope_end_tokens.append(&mut trivia);
        scope_end_tokens
    }

    /// Close all violated scopes, and emit closing tokens right after the last non trivia token
    fn close_breached_scopes(
        &mut self,
        scope_end_coordinate: TextCoordinate,
    ) -> VecDeque<LexToken> {
        let mut scope_end_tokens = VecDeque::new();
        while let Some(scope) = self.scopes.pop() {
            if scope.contains(
                self.current_coordinate,
                self.current_byte().is_some_and(|c| c == b'-'),
            ) {
                self.scopes.push(scope);
                break;
            } else {
                scope_end_tokens.push_back(LexToken::pseudo(
                    scope.close_token_kind(),
                    scope_end_coordinate,
                ));
            }
        }
        scope_end_tokens
    }

    fn close_all_scopes(&mut self) -> VecDeque<LexToken> {
        let mut tokens = VecDeque::new();
        while let Some(scope) = self.scopes.pop() {
            tokens.push_back(LexToken::pseudo(
                scope.close_token_kind(),
                self.current_coordinate,
            ));
        }
        tokens
    }

    /// Consume a YAML flow value that can be used inside an implicit mapping key
    /// https://yaml.org/spec/1.2.2/#rule-ns-s-block-map-implicit-key
    fn consume_potential_mapping_key(&mut self, current: u8) -> VecDeque<LexToken> {
        if is_flow_collection_indicator(current) {
            self.consume_flow_collection()
        } else if current == b'*' {
            self.consume_alias_node().into()
        } else if current == b'"' {
            self.consume_double_quoted_literal().into()
        } else if current == b'\'' {
            self.consume_single_quoted_literal().into()
        } else if is_start_of_plain(current, self.peek_byte(), false) {
            self.consume_plain_literal(current, false).into()
        } else {
            VecDeque::new()
        }
    }

    /// A yaml collection is a JSON-like data structure
    fn consume_flow_collection(&mut self) -> VecDeque<LexToken> {
        let mut current_depth: usize = 0;
        let mut already_warned_insufficient_indent = false;
        let mut collection_tokens = VecDeque::new();

        // https://yaml.org/spec/1.2.2/#rule-c-ns-flow-map-json-key-entry
        // Usually a ':' character has to be follow by a blank character to be lexed as a standalone
        // T![:] token
        // However, for JSON-compatibility, the spec allows ':' followed a JSON-like key to be lexed
        // as is, T![:], instead of a potential start of a plain token
        // According to the spec:
        // `{"a"    :b}`
        // Should be lexed as `{"a": b}`, instead of `{"a" (missing colon) :b}`
        let mut just_lexed_json_key = false;
        while let Some(current) = self.current_byte() {
            if !self.current_char_is_yaml_printable() {
                collection_tokens.push_back(self.consume_unexpected_token());
                continue;
            }

            if is_break(current) {
                let start = self.current_coordinate;
                let mut trivia = self.consume_trivia(false);
                if self.breach_parent_scope() {
                    if current_depth == 0 {
                        self.current_coordinate = start;
                        break;
                    }
                    // Per YAML 1.2.2 §6.5, flow collection content must be
                    // indented past the surrounding block scope's indent.
                    if !already_warned_insufficient_indent {
                        self.diagnostics.push(ParseDiagnostic::new(
                            "Insufficient indentation in flow collection",
                            self.text_position()..self.text_position(),
                        ));
                        already_warned_insufficient_indent = true;
                    }
                }

                collection_tokens.append(&mut trivia);
                continue;
            }
            let token = match (current, self.peek_byte()) {
                (c, _) if is_space(c) => self.consume_whitespace_token(),
                (b'#', _) => self.consume_comment(),
                (b':', _) if just_lexed_json_key => {
                    just_lexed_json_key = false;
                    self.consume_byte_as_token(T![:])
                }
                (b'\'', _) => {
                    just_lexed_json_key = true;
                    self.consume_single_quoted_literal()
                }
                (b'"', _) => {
                    just_lexed_json_key = true;
                    self.consume_double_quoted_literal()
                }
                (b'[', _) => {
                    current_depth += 1;
                    self.consume_byte_as_token(T!['['])
                }
                (b']', _) => {
                    just_lexed_json_key = true;
                    current_depth = current_depth.saturating_sub(1);
                    self.consume_byte_as_token(T![']'])
                }
                (b'{', _) => {
                    current_depth += 1;
                    self.consume_byte_as_token(T!['{'])
                }
                (b'}', _) => {
                    just_lexed_json_key = true;
                    current_depth = current_depth.saturating_sub(1);
                    self.consume_byte_as_token(T!['}'])
                }
                (b',', _) => self.consume_byte_as_token(T![,]),
                (b'*', _) => self.consume_alias_node(),
                (b'&', _) => self.consume_anchor_property(),
                (b'!', _) => self.consume_tag_property(),
                (current, peek) if is_start_of_plain(current, peek, true) => {
                    self.consume_plain_literal(current, true)
                }
                // ':', '?', '-' can be a valid plain token start, so it must be placed after plain
                (b':', _) => self.consume_byte_as_token(T![:]),
                (b'?', _) => self.consume_byte_as_token(T![?]),
                (b'-', _) => self.consume_byte_as_token(T![-]),
                _ => self.consume_unexpected_token(),
            };
            collection_tokens.push_back(token);
            if self.breach_parent_scope() && current_depth == 0 {
                break;
            }
            if current_depth == 0 {
                break;
            }
        }
        collection_tokens
    }

    // https://yaml.org/spec/1.2.2/#rule-ns-plain
    // TODO: parse multiline plain scalar at current indentation level
    fn consume_plain_literal(&mut self, current: u8, in_flow_collection: bool) -> LexToken {
        debug_assert!(is_start_of_plain(
            current,
            self.peek_byte(),
            in_flow_collection
        ));
        let start = self.current_coordinate;
        while let Some(c) = self.current_byte() {
            if self.is_at_bom() {
                self.consume_misplaced_bom();
                continue;
            }
            if !self.current_char_is_yaml_printable() {
                self.consume_invalid_character();
                continue;
            }

            // https://yaml.org/spec/1.2.2/#rule-ns-plain-char
            if is_plain_safe(c, in_flow_collection) && c != b':' && c != b'#' {
                self.advance_char_unchecked();
            }
            // A `#` right after a non-blank character doesn't start a
            // comment and stays part of the scalar
            else if c == b'#' && self.prev_byte().is_some_and(is_non_blank_char) {
                self.advance(1); // '#'
            } else if c == b':'
                && self
                    .peek_byte()
                    .is_some_and(|c| is_plain_safe(c, in_flow_collection))
            {
                // Don't advance past the next plain safe char since there's no guarantee that next
                // char is compliant. For example, in `[::]`, the second colon shouldn't be part
                // of the plain literal even though it's "plain safe"
                self.advance(1); // ':'
            } else if is_space(c) {
                let might_be_token_end = self.current_coordinate;
                self.consume_whitespaces();
                // A `#` preceded by a blank starts a comment, which ends the
                // scalar. The blanks belong to the comment's leading trivia
                if self.current_byte() == Some(b'#') {
                    self.current_coordinate = might_be_token_end;
                    return LexToken::new(PLAIN_LITERAL, start, might_be_token_end);
                }
            } else if is_break(c) {
                let might_be_token_end = self.current_coordinate;
                // A line whose content starts with `#` is a comment, which can
                // never be part of a plain scalar
                if !self.is_scalar_continuation(None) || self.current_byte() == Some(b'#') {
                    self.current_coordinate = might_be_token_end;
                    return LexToken::new(PLAIN_LITERAL, start, might_be_token_end);
                }
            } else {
                break;
            }
        }
        LexToken::new(PLAIN_LITERAL, start, self.current_coordinate)
    }

    // https://yaml.org/spec/1.2.2/#731-double-quoted-style
    fn consume_double_quoted_literal(&mut self) -> LexToken {
        self.assert_byte(b'"');
        let start = self.current_coordinate;
        self.advance(1);

        let token_end = loop {
            match self.current_byte() {
                Some(b'\\') => {
                    self.consume_double_quoted_escape();
                }
                Some(b'"') => {
                    self.advance(1);
                    break self.current_coordinate;
                }
                Some(c) if is_space(c) => self.consume_whitespaces(),
                Some(c) if is_break(c) => {
                    let might_be_token_end = self.current_coordinate;
                    if !self.is_scalar_continuation(None) {
                        break might_be_token_end;
                    }
                }
                Some(_) if !self.current_char_is_yaml_printable() => {
                    self.consume_invalid_character();
                }
                Some(_) => self.advance_char_unchecked(),
                None => {
                    let err = ParseDiagnostic::new(
                        "Missing closing `\"` quote",
                        self.text_position()..self.text_position(),
                    );
                    self.diagnostics.push(err);
                    break self.current_coordinate;
                }
            }
        };
        LexToken::new(DOUBLE_QUOTED_LITERAL, start, token_end)
    }

    // https://yaml.org/spec/1.2.2/#732-single-quoted-style
    fn consume_single_quoted_literal(&mut self) -> LexToken {
        self.assert_byte(b'\'');
        let start = self.current_coordinate;
        self.advance(1);

        let token_end = loop {
            match self.current_byte() {
                Some(b'\'') => {
                    if matches!(self.peek_byte(), Some(b'\'')) {
                        self.advance(2)
                    } else {
                        self.advance(1);
                        break self.current_coordinate;
                    }
                }
                Some(current) if is_space(current) => {
                    self.consume_whitespaces();
                }
                Some(current) if is_break(current) => {
                    let might_be_token_end = self.current_coordinate;
                    if !self.is_scalar_continuation(None) {
                        break might_be_token_end;
                    }
                }
                Some(_) if !self.current_char_is_yaml_printable() => {
                    self.consume_invalid_character();
                }
                Some(_) => self.advance_char_unchecked(),
                None => {
                    let err = ParseDiagnostic::new(
                        "Missing closing `'` quote",
                        self.text_position()..self.text_position(),
                    );
                    self.diagnostics.push(err);
                    break self.current_coordinate;
                }
            }
        };
        LexToken::new(SINGLE_QUOTED_LITERAL, start, token_end)
    }

    fn is_at_directive(&self) -> bool {
        self.current_coordinate.column == 0 && self.current_byte().is_some_and(|c| c == b'%')
    }

    fn consume_directive(&mut self) -> LexToken {
        self.assert_byte(b'%');
        let start = self.current_coordinate;
        while let Some(current) = self.current_byte() {
            if is_break(current) || self.is_at_directive_trailing_trivia() {
                break;
            }
            if self.current_char_is_yaml_printable() {
                self.advance_char_unchecked();
            } else {
                self.consume_invalid_character();
            }
        }

        LexToken::new(DIRECTIVE_LITERAL, start, self.current_coordinate)
    }

    fn is_at_directive_trailing_trivia(&self) -> bool {
        match self.current_byte() {
            Some(b'#') => self.prev_byte().is_none_or(is_blank),
            Some(current) if is_space(current) => {
                let mut offset = 0;
                while self.byte_at(offset).is_some_and(is_space) {
                    offset += 1;
                }

                self.byte_at(offset)
                    .is_none_or(|c| c == b'#' || is_break(c))
            }
            _ => false,
        }
    }

    fn is_at_directive_end(&self) -> bool {
        let is_dash = |c: u8| c == b'-';
        // A DOC_START token can be evaluated as a plain token if it's not placed at the start of
        // line or followed by a space, a break, or EOF.
        self.current_coordinate.column == 0
            && self.current_byte().is_some_and(is_dash)
            && self.peek_byte().is_some_and(is_dash)
            && self.byte_at(2).is_some_and(is_dash)
            && self.byte_at(3).is_none_or(|b| is_space(b) || is_break(b))
    }

    fn consume_directive_end(&mut self) -> VecDeque<LexToken> {
        self.assert_byte(b'-');
        debug_assert_eq!(self.byte_at(1), Some(b'-'));
        debug_assert_eq!(self.byte_at(2), Some(b'-'));
        let start = self.current_coordinate;
        let mut tokens = self.close_all_scopes();
        self.advance(3);
        tokens.push_back(LexToken::new(DIRECTIVE_END, start, self.current_coordinate));

        tokens
    }

    fn is_at_doc_end(&self) -> bool {
        let is_dot = |c: u8| c == b'.';
        // A DOC_END token can be evaluated as a plain token if it's not placed at the start of
        // line
        self.current_coordinate.column == 0
            && self.current_byte().is_some_and(is_dot)
            && self.peek_byte().is_some_and(is_dot)
            && self.byte_at(2).is_some_and(is_dot)
    }

    fn consume_doc_end(&mut self) -> VecDeque<LexToken> {
        self.assert_byte(b'.');
        debug_assert_eq!(self.byte_at(1), Some(b'.'));
        debug_assert_eq!(self.byte_at(2), Some(b'.'));
        let start = self.current_coordinate;
        let mut tokens = self.close_all_scopes();
        self.advance(3);
        tokens.push_back(LexToken::new(DOC_END, start, self.current_coordinate));
        let mut trivia = self.consume_trailing_trivia();
        tokens.append(&mut trivia);
        self.bom_allowed = true;

        tokens
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind.
    #[inline]
    fn consume_byte_as_token(&mut self, tok: YamlSyntaxKind) -> LexToken {
        let start = self.current_coordinate;
        self.advance(1);
        LexToken::new(tok, start, self.current_coordinate)
    }

    fn consume_trivia(&mut self, trailing: bool) -> VecDeque<LexToken> {
        let mut trivia = VecDeque::new();
        while let Some(current) = self.current_byte() {
            if is_space(current) {
                trivia.push_back(self.consume_whitespace_token());
            } else if is_break(current) {
                if trailing {
                    break;
                }
                trivia.push_back(self.consume_newline_token());
            } else if current == b'#' {
                trivia.push_back(self.consume_comment());
            } else {
                break;
            }
        }
        trivia
    }

    fn is_scalar_continuation(&mut self, required_indent: Option<usize>) -> bool {
        debug_assert!(self.current_byte().is_some_and(is_break));
        let start = self.current_coordinate;
        let diagnostics_len = self.diagnostics.len();
        let mut trivia = VecDeque::new();
        while let Some(current) = self.current_byte() {
            if is_space(current) {
                trivia
                    .push_back(self.consume_scalar_continuation_whitespace_token(required_indent));
            } else if is_break(current) {
                trivia.push_back(self.consume_newline_token());
            } else {
                break;
            }
        }
        if self.is_at_document_prefix_bom() {
            self.current_coordinate = start;
            self.diagnostics.truncate(diagnostics_len);
            return false;
        }
        // A document marker at the start of a line always ends the current
        // document, so it can never be part of a multiline scalar
        // https://yaml.org/spec/1.2.2/#rule-c-forbidden
        if self.breach_parent_scope() || self.is_at_directive_end() || self.is_at_doc_end() {
            self.current_coordinate = start;
            self.diagnostics.truncate(diagnostics_len);
            false
        } else {
            if let Some(required_indent) = required_indent
                && self
                    .current_byte()
                    .is_some_and(|byte| !is_break(byte) && byte != b'#')
                && self.current_coordinate.column < required_indent
            {
                let range = trivia
                    .back()
                    .filter(|token| token.kind == WHITESPACE)
                    .map_or_else(
                        || {
                            TextRange::at(
                                self.text_position(),
                                self.current_char_unchecked().text_len(),
                            )
                        },
                        LexToken::text_range,
                    );
                self.diagnostics.push(ParseDiagnostic::new(
                    format!(
                        "Block scalar content must be indented by at least {required_indent} spaces."
                    ),
                    range,
                ));
            }
            true
        }
    }

    fn consume_whitespace_token(&mut self) -> LexToken {
        self.consume_whitespace_token_with_tab_policy(false, None)
    }

    fn consume_scalar_continuation_whitespace_token(
        &mut self,
        required_indent: Option<usize>,
    ) -> LexToken {
        self.consume_whitespace_token_with_tab_policy(true, required_indent)
    }

    fn consume_whitespace_token_with_tab_policy(
        &mut self,
        allow_tab_after_space: bool,
        required_indent: Option<usize>,
    ) -> LexToken {
        debug_assert!(self.current_byte().is_some_and(is_space));
        let start = self.current_coordinate;
        self.consume_whitespaces();

        if start.column == 0
            && let Some(relative_offset) = self
                .source
                .get(start.offset..self.current_coordinate.offset)
                .and_then(|text| text.bytes().position(|byte| byte == b'\t'))
            && (!allow_tab_after_space
                || required_indent.map_or(relative_offset == 0, |indent| relative_offset < indent))
            && let Ok(offset) = TextSize::try_from(start.offset + relative_offset)
        {
            self.diagnostics.push(ParseDiagnostic::new(
                "Tabs are not allowed for indentation in YAML.",
                offset..offset + TextSize::from(1),
            ));
        }

        LexToken::new(WHITESPACE, start, self.current_coordinate)
    }

    fn consume_newline_token(&mut self) -> LexToken {
        debug_assert!(self.current_byte().is_some_and(is_break));
        let start = self.current_coordinate;
        self.consume_newline();
        LexToken::new(NEWLINE, start, self.current_coordinate)
    }

    fn consume_comment(&mut self) -> LexToken {
        self.assert_byte(b'#');
        let start = self.current_coordinate;
        while let Some(c) = self.current_byte() {
            if is_break(c) {
                break;
            }
            if self.current_char_is_yaml_printable() {
                self.advance_char_unchecked();
            } else {
                self.consume_invalid_character();
            }
        }
        LexToken::new(COMMENT, start, self.current_coordinate)
    }

    fn consume_block_properties(&mut self) -> VecDeque<LexToken> {
        debug_assert!(matches!(self.current_byte(), Some(b'!' | b'&')));

        let start_coordinate = self.current_coordinate;
        let mut start_column = self.current_coordinate.column;
        let mut properties = VecDeque::new();

        // Lex all properties until we find a non-property
        while let Some(current) = self.current_byte() {
            match current {
                b'&' => {
                    start_column = start_column.min(self.current_coordinate.column);
                    properties.push_back(self.consume_anchor_property());
                }
                b'!' => {
                    start_column = start_column.min(self.current_coordinate.column);
                    properties.push_back(self.consume_tag_property());
                }
                c if is_space(c) => properties.push_back(self.consume_whitespace_token()),
                b'#' => properties.push_back(self.consume_comment()),
                c if is_break(c) => {
                    // Check if we would breach parent scope before consuming trivia
                    let start = self.current_coordinate;
                    let mut trivia = self.consume_trivia(false);
                    if self
                        .scopes
                        .last()
                        .is_some_and(|scope| !scope.indent_with_dash(self.current_coordinate))
                    {
                        // Restore position and break
                        self.current_coordinate = start;
                        break;
                    } else {
                        properties.append(&mut trivia);
                    }
                }
                _ => break,
            }
        }

        let Some(current) = self.current_byte() else {
            // EOF after properties, wrap in flow markers as properties for empty plain node
            properties.push_front(LexToken::pseudo(FLOW_START, start_coordinate));
            properties.push_back(LexToken::pseudo(FLOW_END, self.current_coordinate));
            return properties;
        };

        // Property list terminated by a newline that breaches the enclosing block, which means an
        // empty plain node.
        // We only need to check for is_break here, as the lexer only stops at a line break if
        // consuming the line break would breach the parent scope
        if is_break(current) {
            properties.push_front(LexToken::pseudo(FLOW_START, start_coordinate));
            properties.push_back(LexToken::pseudo(FLOW_END, self.current_coordinate));
            return properties;
        }

        if maybe_at_mapping_start(current, self.peek_byte()) {
            if self.current_coordinate.column >= start_column {
                // properties of flow collection/scalar that could be a mapping key
                return self.consume_potential_mapping_start(current, properties, start_coordinate);
            }

            // The value can be on the line below its properties:
            //
            // ```yaml
            // key: &anchor
            //   value
            // ```
            //
            // Read the value to check whether `:` follows it. If so, the value
            // is actually the key of a nested mapping. Move back so it can be
            // read again as a key, and remove any errors found during the
            // first read so they are not reported twice.
            let saved_coordinate = self.current_coordinate;
            let saved_diagnostics = self.diagnostics.len();
            let mut value_tokens = self.consume_potential_mapping_key(current);
            let mut trivia = self.consume_trivia(true);
            if self.is_at_mapping_indicator() {
                self.current_coordinate = saved_coordinate;
                self.diagnostics.truncate(saved_diagnostics);
                return properties;
            }
            let mut tokens = properties;
            tokens.push_front(LexToken::pseudo(FLOW_START, start_coordinate));
            tokens.append(&mut value_tokens);
            tokens.append(&mut trivia);
            tokens.push_back(LexToken::pseudo(FLOW_END, self.current_coordinate));
            return tokens;
        }
        properties
    }

    fn consume_alias_node(&mut self) -> LexToken {
        self.assert_byte(b'*');
        let start = self.current_coordinate;
        self.advance(1);

        while let Some(c) = self.current_byte() {
            // An alias name is made up of `ns-anchor-char`s (rule [103],
            // section 7.1 of the spec), which includes `:`, so `*a:` is an
            // alias named `a:` rather than an alias used as a mapping key
            if is_anchor_char(c) {
                if self.current_char_is_yaml_printable() {
                    self.advance_char_unchecked();
                } else {
                    self.consume_invalid_character();
                }
            } else {
                break;
            }
        }

        LexToken::new(ALIAS_LITERAL, start, self.current_coordinate)
    }

    fn consume_anchor_property(&mut self) -> LexToken {
        self.assert_byte(b'&');
        let start = self.current_coordinate;
        self.advance(1);

        while let Some(c) = self.current_byte() {
            if is_anchor_char(c) {
                if self.current_char_is_yaml_printable() {
                    self.advance_char_unchecked();
                } else {
                    self.consume_invalid_character();
                }
            } else {
                break;
            }
        }

        LexToken::new(ANCHOR_PROPERTY_LITERAL, start, self.current_coordinate)
    }

    /// Consumes a tag property (verbatim `!<uri>`, secondary `!!tag`, named `!handle!tag`, primary `!tag`, or non-specific `!`).
    /// We might benefit from separating these different constructs in the grammar itself in the
    /// future.
    fn consume_tag_property(&mut self) -> LexToken {
        self.assert_byte(b'!');
        let start = self.current_coordinate;
        self.advance(1);

        match self.current_byte() {
            // verbatim: !<uri>
            Some(b'<') => {
                self.advance(1);
                while let Some(c) = self.current_byte() {
                    if c == b'>' {
                        self.advance(1);
                        break;
                    }
                    if !is_non_blank_char(c) {
                        break;
                    }
                    if self.current_char_is_yaml_printable() {
                        self.advance_char_unchecked();
                    } else {
                        self.consume_invalid_character();
                    }
                }
            }
            // secondary handle: !!body
            Some(b'!') => {
                self.advance(1);
                while let Some(c) = self.current_byte() {
                    if is_tag_char(c) {
                        if self.current_char_is_yaml_printable() {
                            self.advance_char_unchecked();
                        } else {
                            self.consume_invalid_character();
                        }
                    } else {
                        break;
                    }
                }
            }
            // primary: !body, or named handle: !handle!body
            Some(c) if is_tag_char(c) => {
                // named handle: !word! prefix, then body
                if is_word_char(c) {
                    let mut offset = 0;
                    while let Some(b) = self.byte_at(offset) {
                        if is_word_char(b) {
                            offset += 1;
                        } else {
                            break;
                        }
                    }
                    if self.byte_at(offset) == Some(b'!') {
                        self.advance(offset + 1);
                    }
                }
                while let Some(c) = self.current_byte() {
                    if is_tag_char(c) {
                        if self.current_char_is_yaml_printable() {
                            self.advance_char_unchecked();
                        } else {
                            self.consume_invalid_character();
                        }
                    } else {
                        break;
                    }
                }
            }
            // non-specific: bare !
            _ => {}
        }

        LexToken::new(TAG_PROPERTY_LITERAL, start, self.current_coordinate)
    }

    fn consume_unexpected_token(&mut self) -> LexToken {
        self.assert_current_char_boundary();
        let start = self.current_coordinate;

        self.consume_unexpected_character();
        LexToken::new(ERROR_TOKEN, start, self.current_coordinate)
    }

    /// Some constructs, like block header or document end (`...`), don't allow any trailing tokens
    /// except for trivia.
    /// This function is responsible for consuming the trailing trivia and any unexpected tokens
    fn consume_trailing_trivia(&mut self) -> VecDeque<LexToken> {
        self.assert_current_char_boundary();

        let mut tokens = self.consume_trivia(true);

        if self.current_byte().is_none_or(is_break) {
            return tokens;
        }

        let start = self.current_coordinate;
        while let Some(c) = self.current_byte() {
            if is_break(c) {
                break;
            }
            if self.current_char_is_yaml_printable() {
                self.advance_char_unchecked();
            } else {
                self.consume_invalid_character();
            }
        }
        tokens.push_back(LexToken::new(ERROR_TOKEN, start, self.current_coordinate));
        tokens
    }

    fn consume_unexpected_character(&mut self) {
        self.assert_current_char_boundary();

        let char = self.current_char_unchecked();
        let err = ParseDiagnostic::new(
            format!("Unexpected character `{char}`"),
            self.text_position()..self.text_position() + char.text_len(),
        );
        self.diagnostics.push(err);
        self.advance(char.len_utf8());
    }

    fn consume_invalid_character(&mut self) {
        let character = self.current_char_unchecked();
        let start = self.text_position();
        self.advance(character.len_utf8());
        self.diagnostics.push(ParseDiagnostic::new(
            "Character is not allowed in YAML.",
            start..self.text_position(),
        ));
    }

    fn consume_misplaced_bom(&mut self) {
        debug_assert!(self.is_at_bom());
        let start = self.current_coordinate;
        self.advance('\u{feff}'.len_utf8());
        self.diagnostics.push(ParseDiagnostic::new(
            "A byte order mark is only allowed at the start of a document.",
            TextRange::new(start.into(), self.current_coordinate.into()),
        ));
    }

    fn report_multiline_implicit_key(&mut self, start: TextCoordinate, end: TextCoordinate) {
        let Some(text) = self.source.get(start.offset..end.offset) else {
            return;
        };
        if text.bytes().any(is_break) {
            self.diagnostics.push(ParseDiagnostic::new(
                "An implicit mapping key must fit on a single line.",
                TextRange::new(start.into(), end.into()),
            ));
        }
    }

    fn current_char_is_yaml_printable(&self) -> bool {
        is_yaml_printable(self.current_char_unchecked())
    }

    fn is_at_bom(&self) -> bool {
        self.current_byte() == Some(0xef) && self.current_char_unchecked() == '\u{feff}'
    }

    fn is_at_document_prefix_bom(&self) -> bool {
        self.is_at_bom()
            && (self.bom_allowed
                || (self.current_coordinate.column == 0
                    && (self.byte_at(3) == Some(b'%')
                        || (self.byte_at(3) == Some(b'-')
                            && self.byte_at(4) == Some(b'-')
                            && self.byte_at(5) == Some(b'-')
                            && self.byte_at(6).is_none_or(is_blank)))))
    }

    fn consume_double_quoted_escape(&mut self) {
        debug_assert_eq!(self.current_byte(), Some(b'\\'));
        let start = self.text_position();
        self.advance(1);

        match self.current_byte() {
            Some(
                b'0' | b'a' | b'b' | b't' | b'n' | b'v' | b'f' | b'r' | b'e' | b' ' | b'"' | b'/'
                | b'\\' | b'N' | b'_' | b'L' | b'P',
            ) => self.advance(1),
            Some(b'x') => {
                self.advance(1);
                self.consume_hex_escape_digits(start, 2);
            }
            Some(b'u') => {
                self.advance(1);
                self.consume_hex_escape_digits(start, 4);
            }
            Some(b'U') => {
                self.advance(1);
                self.consume_hex_escape_digits(start, 8);
            }
            Some(current) if is_break(current) => {}
            Some(_) => {
                self.advance_char_unchecked();
                self.diagnostics.push(ParseDiagnostic::new(
                    "Unknown escape sequence in double-quoted scalar.",
                    start..self.text_position(),
                ));
            }
            None => {
                self.diagnostics.push(ParseDiagnostic::new(
                    "Incomplete escape sequence in double-quoted scalar.",
                    start..self.text_position(),
                ));
            }
        }
    }

    fn consume_hex_escape_digits(&mut self, escape_start: TextSize, count: usize) {
        let digits_start = self.text_position();
        let mut consumed = 0;
        let mut value = 0u32;
        while consumed < count {
            let Some(byte) = self.current_byte().filter(u8::is_ascii_hexdigit) else {
                break;
            };
            let digit = u32::from(byte & 0x0f) + u32::from(byte.is_ascii_alphabetic()) * 9;
            value = value * 16 + digit;
            self.advance(1);
            consumed += 1;
        }

        if consumed != count {
            self.diagnostics.push(ParseDiagnostic::new(
                format!("Expected {count} hexadecimal digits in escape sequence."),
                escape_start..self.text_position().max(digits_start),
            ));
        } else if char::from_u32(value).is_none() {
            self.diagnostics.push(ParseDiagnostic::new(
                "Escape sequence does not encode a valid Unicode scalar value.",
                escape_start..self.text_position(),
            ));
        }
    }

    fn is_at_mapping_indicator(&self) -> bool {
        self.current_byte().is_some_and(|c| c == b':') && self.peek_byte().is_none_or(is_blank)
    }

    fn breach_parent_scope(&self) -> bool {
        self.scopes
            .last()
            .is_some_and(|scope| !scope.indent(self.current_coordinate))
    }

    fn current_token(&self) -> LexToken {
        self.tokens
            .front()
            .copied()
            // shouldn't brick the server just because a user open a malformed YAML file or there's
            // a bug in the lexer
            .unwrap_or(LexToken::pseudo(EOF, self.current_coordinate))
    }
}

impl<'src> Lexer<'src> for YamlLexer<'src> {
    const NEWLINE: Self::Kind = YamlSyntaxKind::NEWLINE;
    const WHITESPACE: Self::Kind = YamlSyntaxKind::WHITESPACE;

    type Kind = YamlSyntaxKind;
    type LexContext = ();
    type ReLexContext = ();

    fn source(&self) -> &'src str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_token().kind
    }

    fn current_range(&self) -> TextRange {
        self.current_token().text_range()
    }

    #[inline]
    fn advance(&mut self, n: usize) {
        self.current_coordinate = self.current_coordinate.advance(n);
    }

    #[inline]
    fn advance_char_unchecked(&mut self) {
        let c = self.current_char_unchecked();
        self.advance(c.len_utf8());
    }

    #[inline]
    fn current_start(&self) -> TextSize {
        self.current_token().start_position()
    }

    fn next_token(&mut self, _context: Self::LexContext) -> Self::Kind {
        self.tokens.pop_front();
        if self.tokens.is_empty() {
            self.consume_tokens();
        }
        self.current_token().kind
    }

    fn has_preceding_line_break(&self) -> bool {
        false
    }

    fn has_unicode_escape(&self) -> bool {
        false
    }

    fn rewind(&mut self, checkpoint: LexerCheckpoint<Self::Kind>) {
        let mut replay = Self::from_str(self.source);
        while !replay.matches_checkpoint(&checkpoint) {
            if replay.next_token(()) == EOF && !replay.matches_checkpoint(&checkpoint) {
                return;
            }
        }
        replay
            .diagnostics
            .truncate(checkpoint.diagnostics_pos as usize);
        *self = replay;
    }

    fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn position(&self) -> usize {
        self.current_coordinate.offset
    }

    /// Consumes all whitespace until a non-whitespace or a newline is found.
    ///
    /// ## Safety
    /// Must be called at a valid UTF8 char boundary
    fn consume_whitespaces(&mut self) {
        self.assert_current_char_boundary();

        while let Some(c) = self.current_byte() {
            let dispatch = lookup_byte(c);
            if !matches!(dispatch, WHS) {
                break;
            }

            if is_space(c) {
                self.advance(1);
            } else if is_break(c) {
                break;
            } else {
                let start = self.text_position();
                self.advance(1);

                self.push_diagnostic(
                    ParseDiagnostic::new(
                        "The YAML standard allows only two types of whitespace characters: tabs and spaces",
                        start..self.text_position(),
                    )
                        .with_hint("Use a regular whitespace character instead. For more detail, please check https://yaml.org/spec/1.2.2/#55-white-space-characters"),
                )
            }
        }
    }

    /// Consume just one newline/line break.
    ///
    /// ## Safety
    /// Must be called at a valid UTF8 char boundary
    fn consume_newline(&mut self) -> bool {
        self.assert_current_char_boundary();

        match self.current_byte() {
            Some(b'\n') => {
                self.advance(1);
                self.current_coordinate = self.current_coordinate.enter_new_line();
                true
            }
            Some(b'\r') => {
                if self.peek_byte() == Some(b'\n') {
                    self.advance(2)
                } else {
                    self.advance(1)
                }
                self.current_coordinate = self.current_coordinate.enter_new_line();
                true
            }

            _ => false,
        }
    }
}

impl<'src> LexerWithCheckpoint<'src> for YamlLexer<'src> {
    fn checkpoint(&self) -> LexerCheckpoint<Self::Kind> {
        LexerCheckpoint {
            position: self.current_range().end(),
            current_start: self.current_start(),
            current_kind: self.current(),
            current_flags: self.current_flags(),
            after_line_break: false,
            after_whitespace: false,
            // YAML tracks BOM eligibility separately. This field carries the queued-token count
            // needed to distinguish adjacent zero-width scope-closing tokens during replay.
            unicode_bom_length: self.tokens.len(),
            diagnostics_pos: self.diagnostics.len() as u32,
        }
    }
}

impl YamlLexer<'_> {
    fn matches_checkpoint(&self, checkpoint: &LexerCheckpoint<YamlSyntaxKind>) -> bool {
        self.current_range().end() == checkpoint.position
            && self.current_start() == checkpoint.current_start
            && self.current() == checkpoint.current_kind
            && self.tokens.len() == checkpoint.unicode_bom_length
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LexToken {
    start: TextCoordinate,
    end: TextCoordinate,
    kind: YamlSyntaxKind,
}

impl Default for LexToken {
    fn default() -> Self {
        Self {
            kind: TOMBSTONE,
            start: TextCoordinate::default(),
            end: TextCoordinate::default(),
        }
    }
}

impl LexToken {
    fn new(kind: YamlSyntaxKind, start: TextCoordinate, end: TextCoordinate) -> Self {
        Self { kind, start, end }
    }

    fn pseudo(kind: YamlSyntaxKind, start: TextCoordinate) -> Self {
        Self {
            kind,
            start,
            end: start,
        }
    }

    fn text_range(&self) -> TextRange {
        TextRange::new(self.start.into(), self.end.into())
    }

    fn start_position(&self) -> TextSize {
        self.start.into()
    }
}

impl From<LexToken> for VecDeque<LexToken> {
    fn from(value: LexToken) -> Self {
        let mut s = Self::new();
        s.push_back(value);
        s
    }
}

/// Scope of one Yaml collection. Stores the leftmost border of the scope. Any tokens to the right
/// of this border will belong to that scope.
/// https://yaml.org/spec/1.2.2/#82-block-collection-styles
#[derive(Debug)]
enum BlockScope {
    Sequence(usize),
    Map(usize),
}

impl BlockScope {
    fn new_mapping_scope(coordinate: TextCoordinate) -> Self {
        Self::Map(coordinate.column)
    }

    fn new_sequence_scope(coordinate: TextCoordinate) -> Self {
        Self::Sequence(coordinate.column)
    }

    fn border(&self) -> usize {
        match self {
            Self::Sequence(border) | Self::Map(border) => *border,
        }
    }

    /// Whether the supplied coordinate strictly belongs to this scope, i.e. it doesn't share the
    /// scope's border.
    /// Used to check whether the supplied coordinate is the start of a new
    fn indent(&self, coordinate: TextCoordinate) -> bool {
        match self {
            Self::Sequence(border) => coordinate.column > *border,
            Self::Map(border) => coordinate.column > *border,
        }
    }

    /// Check for indentation. This version is used when dealing with '-', as it can be considered
    /// an indent character, except for when used inside a block sequence
    /// For example, in this case, '-' is part of the indent
    /// ```yaml
    /// a:
    /// - b
    /// ```
    /// However, in this case, the second dash is not
    /// ```yaml
    /// - x
    /// - b
    /// ```
    fn indent_with_dash(&self, coordinate: TextCoordinate) -> bool {
        match self {
            Self::Sequence(border) => coordinate.column > *border,
            Self::Map(border) => coordinate.column >= *border,
        }
    }

    fn contains(&self, coordinate: TextCoordinate, is_sequence_entry: bool) -> bool {
        match self {
            Self::Sequence(border) => {
                // Since a sequence entry can start on the same column as a map entry, we have to check
                // whether the current entry is a map or a sequence entry.
                // If it's a map entry and starts on the same column as the current sequence scope, it
                // belongs to the parent scope instead.
                coordinate.column > *border || (is_sequence_entry && coordinate.column == *border)
            }
            Self::Map(border) => coordinate.column >= *border,
        }
    }

    fn close_token_kind(&self) -> YamlSyntaxKind {
        match self {
            Self::Sequence(_) => SEQUENCE_END,
            Self::Map(_) => MAPPING_END,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct TextCoordinate {
    /// The byte position in the source text.
    offset: usize,
    /// The number of bytes since the last newline.
    column: usize,
}

impl From<TextCoordinate> for TextSize {
    fn from(value: TextCoordinate) -> Self {
        Self::from(value.offset as u32)
    }
}

impl TextCoordinate {
    #[inline]
    fn advance(&self, n: usize) -> Self {
        Self {
            offset: self.offset + n,
            column: self.column + n,
        }
    }

    #[inline]
    fn enter_new_line(&self) -> Self {
        Self {
            offset: self.offset,
            column: 0,
        }
    }
}

// https://yaml.org/spec/1.2.2/#rule-ns-l-block-map-implicit-entry
#[inline]
fn maybe_at_mapping_start(current: u8, peek: Option<u8>) -> bool {
    is_flow_collection_indicator(current)
        || is_start_of_plain(current, peek, false)
        || current == b'"'
        || current == b'\''
        || current == b'*'
        // empty key
        || (current == b':' && peek.is_none_or(is_blank))
}

// https://yaml.org/spec/1.2.2/#rule-ns-plain-first
#[inline]
fn is_start_of_plain(current: u8, peek: Option<u8>, in_flow_collection: bool) -> bool {
    (is_non_blank_char(current) && !is_indicator(current))
        || ((current == b'?' || current == b':' || current == b'-')
            && peek.is_some_and(|c| is_plain_safe(c, in_flow_collection)))
}

// https://yaml.org/spec/1.2.2/#rule-ns-plain-safe
#[inline]
fn is_plain_safe(c: u8, in_flow_collection: bool) -> bool {
    if in_flow_collection {
        is_non_blank_char(c) && !is_flow_collection_indicator(c)
    } else {
        is_non_blank_char(c)
    }
}

// https://yaml.org/spec/1.2.2/#rule-ns-char
#[inline]
fn is_non_blank_char(c: u8) -> bool {
    c >= 0x80 || c.is_ascii_graphic()
}

#[inline]
fn is_yaml_printable(c: char) -> bool {
    matches!(
        c,
        '\u{0009}' | '\u{000A}' | '\u{000D}' | '\u{0020}'..='\u{007E}' | '\u{0085}'
            | '\u{00A0}'..='\u{D7FF}' | '\u{E000}'..='\u{FFFD}' | '\u{10000}'..='\u{10FFFF}'
    )
}

#[inline]
fn is_blank(c: u8) -> bool {
    is_space(c) || is_break(c)
}

// https://yaml.org/spec/1.2.2/#rule-s-white
#[inline]
fn is_space(c: u8) -> bool {
    c == b' ' || c == b'\t'
}

// https://yaml.org/spec/1.2.2/#rule-b-char
#[inline]
fn is_break(c: u8) -> bool {
    c == b'\n' || c == b'\r'
}

// https://yaml.org/spec/1.2.2/#rule-c-indicator
#[inline]
fn is_indicator(c: u8) -> bool {
    c == b'-'
        || c == b'?'
        || c == b':'
        || c == b'#'
        || c == b'&'
        || c == b'*'
        || c == b'!'
        || c == b'|'
        || c == b'>'
        || c == b'\''
        || c == b'"'
        || c == b'%'
        || c == b'@'
        || c == b'`'
        || is_flow_collection_indicator(c)
}

// https://yaml.org/spec/1.2.2/#rule-c-flow-indicator
#[inline]
fn is_flow_collection_indicator(c: u8) -> bool {
    c == b',' || c == b'[' || c == b']' || c == b'{' || c == b'}'
}

// https://yaml.org/spec/1.2.2/#rule-ns-anchor-char
#[inline]
fn is_anchor_char(c: u8) -> bool {
    is_non_blank_char(c) && !is_flow_collection_indicator(c)
}

// https://yaml.org/spec/1.2.2/#rule-ns-word-char
#[inline]
fn is_word_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'-'
}

// https://yaml.org/spec/1.2.2/#rule-ns-tag-char
#[inline]
fn is_tag_char(c: u8) -> bool {
    is_non_blank_char(c) && c != b'!' && !is_flow_collection_indicator(c)
}
