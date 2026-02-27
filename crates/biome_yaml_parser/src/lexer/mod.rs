use std::collections::LinkedList;

use biome_parser::{
    diagnostic::ParseDiagnostic,
    lexer::{Lexer, LexerCheckpoint},
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
    tokens: LinkedList<LexToken>,
}

impl<'src> YamlLexer<'src> {
    pub fn from_str(source: &'src str) -> Self {
        Self {
            source,
            diagnostics: Vec::new(),
            scopes: Default::default(),
            current_coordinate: Default::default(),
            tokens: LexToken::default().into(),
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
            let mut tokens = self.close_all_scopes();
            self.tokens.append(&mut tokens);
            self.tokens
                .push_back(LexToken::pseudo(EOF, self.current_coordinate));
            return;
        };

        let start = self.text_position();

        let mut tokens = match current {
            c if is_break(c) => self.evaluate_block_scope(),
            c if is_space(c) => self.consume_whitespace_token().into(),
            b'#' => self.consume_comment().into(),
            b'.' if self.is_at_doc_end() => self.consume_doc_end(),
            current if maybe_at_mapping_start(current, self.peek_byte()) => {
                self.consume_potential_mapping_start(current)
            }
            // ':', '?', '-' can be a valid plain token start
            b'?' | b':' => self.consume_mapping_key(current),
            b'-' => self.consume_sequence_entry(),
            b'|' | b'>' => self.consume_block_scalar(current),
            _ => self.consume_unexpected_token().into(),
        };
        self.tokens.append(&mut tokens);

        debug_assert!(self.text_position() > start, "Lexer did not advance");
    }

    fn consume_sequence_entry(&mut self) -> LinkedList<LexToken> {
        self.assert_byte(b'-');
        let indicator = self.consume_byte_as_token(T![-]);

        if self
            .scopes
            .last()
            .is_none_or(|scope| scope.indent_with_dash(indicator.start))
        {
            let mut tokens = LinkedList::new();
            tokens.push_back(indicator);
            tokens.push_front(LexToken::pseudo(SEQUENCE_START, indicator.start));
            self.scopes
                .push(BlockScope::new_sequence_scope(indicator.start));
            tokens
        } else {
            indicator.into()
        }
    }

    /// Consume a known mapping key, indicated by either '?' or ':'.
    /// '?' signifies an explicit mapping key, while ':' represents an empty one.
    /// For example, `: abc` is a valid yaml mapping, which is equivalent
    /// to `{null: abc}`
    fn consume_mapping_key(&mut self, current: u8) -> LinkedList<LexToken> {
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
            let mut tokens = LinkedList::new();
            tokens.push_front(LexToken::pseudo(MAPPING_START, indicator.start));
            tokens.push_back(indicator);
            self.scopes
                .push(BlockScope::new_mapping_scope(indicator.start));
            tokens
        } else {
            indicator.into()
        }
    }

    /// Consume and disambiguate a YAML value to determine whether it's a YAML block map or just a
    /// YAML flow value
    fn consume_potential_mapping_start(&mut self, current: u8) -> LinkedList<LexToken> {
        debug_assert!(maybe_at_mapping_start(current, self.peek_byte()));

        let start = self.current_coordinate;
        let mut tokens = self.consume_potential_mapping_key(current);
        if self.scopes.last().is_none_or(|scope| scope.indent(start)) {
            if self.is_at_mapping_indicator() {
                let indicator = self.consume_byte_as_token(T![:]);
                tokens.push_front(LexToken::pseudo(MAPPING_START, start));
                tokens.push_back(indicator);
                self.scopes.push(BlockScope::new_mapping_scope(start));
            } else {
                // Just a normal flow value
                tokens.push_front(LexToken::pseudo(FLOW_START, start));
                // Consume any trailing trivia remaining before closing the flow, as we must not
                // have trailing trivia followed FLOW_END token
                let mut trivia = self.consume_trivia(true);
                tokens.append(&mut trivia);
                tokens.push_back(LexToken::pseudo(FLOW_END, self.current_coordinate));
            }
        } else if self.is_at_mapping_indicator() {
            // At a valid mapping key, lex the `:` so that the lexer wouldn't confuse it with a
            // standalone `:` token, which indicate the start of an empty mapping key
            let indicator = self.consume_byte_as_token(T![:]);
            tokens.push_back(indicator);
        }
        tokens
    }

    fn consume_block_scalar(&mut self, current: u8) -> LinkedList<LexToken> {
        debug_assert!(matches!(current, b'|' | b'>'));

        let mut tokens = LinkedList::new();

        let style_token = if current == b'|' {
            self.consume_byte_as_token(T![|])
        } else {
            self.consume_byte_as_token(T![>])
        };
        tokens.push_back(style_token);

        let mut headers = self.consume_block_header_tokens();
        tokens.append(&mut headers);

        tokens.push_back(self.lex_block_content());

        tokens
    }

    /// Lex block scalar header indicators, returning tokens and indent size
    fn consume_block_header_tokens(&mut self) -> LinkedList<LexToken> {
        let mut tokens = LinkedList::new();

        while let Some(current) = self.current_byte() {
            match current {
                b'0'..=b'9' => {
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

        tokens
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
    /// We don't need to take into account the indentation size declared in the header, since it's
    /// only relevant when we need to extract/format the content of the scalar.
    /// By ignoring the declared indentation and just lex the block content like a plain
    /// literal, we can provide better diagnostic messages when the content is indented less
    /// than the declared value.
    /// A syntax analyzer rule can then be used to identify erroneous blocks.
    /// Start with the newline followed the header, to handle cases where the block content is
    /// empty
    fn lex_block_content(&mut self) -> LexToken {
        debug_assert!(self.current_byte().is_none_or(is_break));
        let start = self.current_coordinate;

        while let Some(current) = self.current_byte() {
            if is_break(current) {
                let might_be_token_end = self.current_coordinate;
                if !self.is_scalar_continuation() {
                    return LexToken::new(BLOCK_CONTENT_LITERAL, start, might_be_token_end);
                }
            } else {
                self.advance(1);
            }
        }

        LexToken::new(BLOCK_CONTENT_LITERAL, start, self.current_coordinate)
    }

    fn evaluate_block_scope(&mut self) -> LinkedList<LexToken> {
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
    ) -> LinkedList<LexToken> {
        let mut scope_end_tokens = LinkedList::new();
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

    fn close_all_scopes(&mut self) -> LinkedList<LexToken> {
        let tokens = LinkedList::new();
        while let Some(scope) = self.scopes.pop() {
            self.tokens.push_back(LexToken::pseudo(
                scope.close_token_kind(),
                self.current_coordinate,
            ));
        }
        tokens
    }

    /// Consume a YAML flow value that can be used inside an implicit mapping key
    /// https://yaml.org/spec/1.2.2/#rule-ns-s-block-map-implicit-key
    fn consume_potential_mapping_key(&mut self, current: u8) -> LinkedList<LexToken> {
        if is_flow_collection_indicator(current) {
            self.consume_flow_collection()
        } else if current == b'"' {
            self.consume_double_quoted_literal().into()
        } else if current == b'\'' {
            self.consume_single_quoted_literal().into()
        } else {
            self.consume_plain_literal(current, false).into()
        }
    }

    /// A yaml collection is a JSON-like data structure
    fn consume_flow_collection(&mut self) -> LinkedList<LexToken> {
        let mut current_depth: usize = 0;
        let mut collection_tokens = LinkedList::new();

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
            if is_break(current) {
                let start = self.current_coordinate;
                let mut trivia = self.consume_trivia(false);
                if self.breach_parent_scope() {
                    // The lexed trivia is actually significant and signals the end of current
                    // block scope
                    self.current_coordinate = start;
                    break;
                } else {
                    collection_tokens.append(&mut trivia);
                    continue;
                }
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
            if self.breach_parent_scope() {
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
            // https://yaml.org/spec/1.2.2/#rule-ns-plain-char
            if is_plain_safe(c, in_flow_collection) && c != b':' && c != b'#' {
                self.advance_char_unchecked();
            }
            // Technically this should have been (current == b'#' &&
            // is_non_blank_char(self.last_byte())), but this is simpler
            else if is_non_blank_char(c) && self.peek_byte().is_some_and(|c| c == b'#') {
                self.advance_char_unchecked();
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
                self.consume_whitespaces();
            } else if is_break(c) {
                let might_be_token_end = self.current_coordinate;
                if !self.is_scalar_continuation() {
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
                    if matches!(self.peek_byte(), Some(b'"')) {
                        self.advance(2)
                    } else {
                        self.advance(1)
                    }
                }
                Some(b'"') => {
                    self.advance(1);
                    break self.current_coordinate;
                }
                Some(c) if is_space(c) => self.consume_whitespaces(),
                Some(c) if is_break(c) => {
                    let might_be_token_end = self.current_coordinate;
                    if !self.is_scalar_continuation() {
                        break might_be_token_end;
                    }
                }
                Some(_) => self.advance(1),
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
                    if !self.is_scalar_continuation() {
                        break might_be_token_end;
                    }
                }
                Some(_) => self.advance(1),
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

    fn is_at_doc_end(&self) -> bool {
        let is_dot = |c: u8| c == b'.';
        // A DOC_END token can be evaluated as a plain token if it's not placed at the start of
        // line
        self.current_coordinate.column == 0
            && self.current_byte().is_some_and(is_dot)
            && self.peek_byte().is_some_and(is_dot)
            && self.byte_at(2).is_some_and(is_dot)
    }

    fn consume_doc_end(&mut self) -> LinkedList<LexToken> {
        self.assert_byte(b'.');
        debug_assert_eq!(self.byte_at(1), Some(b'.'));
        debug_assert_eq!(self.byte_at(2), Some(b'.'));
        let start = self.current_coordinate;
        let mut tokens = self.close_all_scopes();
        self.advance(3);
        tokens.push_back(LexToken::new(DOC_END, start, self.current_coordinate));
        let mut trivia = self.consume_trailing_trivia();
        tokens.append(&mut trivia);

        tokens
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind.
    #[inline]
    fn consume_byte_as_token(&mut self, tok: YamlSyntaxKind) -> LexToken {
        let start = self.current_coordinate;
        self.advance(1);
        LexToken::new(tok, start, self.current_coordinate)
    }

    fn consume_trivia(&mut self, trailing: bool) -> LinkedList<LexToken> {
        let mut trivia = LinkedList::new();
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

    fn is_scalar_continuation(&mut self) -> bool {
        debug_assert!(self.current_byte().is_some_and(is_break));
        let start = self.current_coordinate;
        let mut trivia = LinkedList::new();
        while let Some(current) = self.current_byte() {
            if is_space(current) {
                trivia.push_back(self.consume_whitespace_token());
            } else if is_break(current) {
                trivia.push_back(self.consume_newline_token());
            } else {
                break;
            }
        }
        if self.breach_parent_scope() {
            self.current_coordinate = start;
            false
        } else {
            true
        }
    }

    fn consume_whitespace_token(&mut self) -> LexToken {
        debug_assert!(self.current_byte().is_some_and(is_space));
        let start = self.current_coordinate;
        self.consume_whitespaces();
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
            self.advance(1);
        }
        LexToken::new(COMMENT, start, self.current_coordinate)
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
    fn consume_trailing_trivia(&mut self) -> LinkedList<LexToken> {
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
            self.advance_char_unchecked();
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

    fn rewind(&mut self, _: LexerCheckpoint<Self::Kind>) {
        unimplemented!()
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

impl From<LexToken> for LinkedList<LexToken> {
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

// https://yaml.org/spec/1.2.2/#rule-ns-s-block-map-implicit-key
#[inline]
fn maybe_at_mapping_start(current: u8, peek: Option<u8>) -> bool {
    is_flow_collection_indicator(current)
        || is_start_of_plain(current, peek, false)
        || current == b'"'
        || current == b'\''
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
    !is_blank(c)
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
