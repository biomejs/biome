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

    /// Hiarchy of block scopes covering the lexer's current coordinate
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
            while let Some(scope) = self.scopes.pop() {
                self.tokens.push_back(LexToken::pseudo(
                    scope.close_token_kind(),
                    self.current_coordinate,
                ));
            }
            self.tokens
                .push_back(LexToken::pseudo(EOF, self.current_coordinate));
            return;
        };

        let start = self.text_position();

        let mut result = match (current, self.peek_byte()) {
            _ if is_start_of_trivia(current) => {
                BlockScopedTokens::trivia(self.consume_trivia(current))
            }
            (current, peek) if maybe_at_mapping_start(current, peek) => {
                self.consume_potential_mapping_start(current)
            }
            // ':', '?', '-' can be a valid plain token start
            (b'?', _) => self.consume_explicit_mapping_key(),
            (b'-', _) => self.consume_sequence_entry(),
            (b':', _) => self.consume_empty_mapping_key(),
            _ => self.consume_unexpected_token().into(),
        };
        if let Some(scope) = result.scope {
            self.scopes.push(scope)
        }
        self.tokens.append(&mut result.tokens);

        while let Some(scope) = self.scopes.pop() {
            if scope.contains(
                self.current_coordinate,
                self.current_byte().is_some_and(|c| c == b'-'),
            ) {
                self.scopes.push(scope);
                break;
            } else {
                self.tokens.push_back(LexToken::pseudo(
                    scope.close_token_kind(),
                    self.current_coordinate,
                ));
            }
        }

        self.tokens.append(&mut result.trailing_trivia);

        debug_assert!(self.text_position() > start, "Lexer did not advance");
    }

    fn consume_sequence_entry(&mut self) -> BlockScopedTokens {
        self.assert_byte(b'-');
        let token = self.consume_byte(T![-]);
        BlockScopedTokens::sequence_entry(token, self.scopes.last())
    }

    fn consume_explicit_mapping_key(&mut self) -> BlockScopedTokens {
        self.assert_byte(b'?');
        let token = self.consume_byte(T![?]);
        BlockScopedTokens::empty_mapping_key(token, self.scopes.last())
    }

    /// Mapping key can be empty. For example, `: abc` is a valid yaml mapping, which is equivalent
    /// to `{null: abc}`
    fn consume_empty_mapping_key(&mut self) -> BlockScopedTokens {
        let indicator = self.consume_byte(T![:]);
        BlockScopedTokens::empty_mapping_key(indicator, self.scopes.last())
    }

    /// Consume and disambiguate a YAML value to determine whether it's a YAML block map or just a
    /// YAML flow value
    fn consume_potential_mapping_start(&mut self, current: u8) -> BlockScopedTokens {
        debug_assert!(maybe_at_mapping_start(current, self.peek_byte()));

        let start = self.current_coordinate;
        match self.consume_potential_mapping_key(current) {
            FlowScopedTokens::Closed(tokens, trailing_trivia) => {
                BlockScopedTokens::flow_value(tokens, start, trailing_trivia)
            }
            FlowScopedTokens::Continuable(tokens) => {
                if self.is_at_mapping_indicator() {
                    let indicator = self.consume_byte(T![:]);
                    BlockScopedTokens::filled_mapping_key(tokens, indicator, self.scopes.last())
                } else {
                    BlockScopedTokens::flow_value(tokens, start, LinkedList::new())
                }
            }
        }
    }

    /// Consume a YAML flow value that can be used inside an implicit mapping key
    /// https://yaml.org/spec/1.2.2/#rule-ns-s-block-map-implicit-key
    fn consume_potential_mapping_key(&mut self, current: u8) -> FlowScopedTokens {
        if is_flow_collection_indicator(current) {
            self.consume_flow_collection()
        } else if current == b'"' {
            self.consume_double_quoted_literal()
        } else if current == b'\'' {
            self.consume_single_quoted_literal()
        } else {
            self.consume_plain_literal(current, false)
        }
    }

    /// A yaml collection is a JSON-like data structure
    fn consume_flow_collection(&mut self) -> FlowScopedTokens {
        let mut current_depth: usize = 0;
        let mut collection_tokens = LinkedList::new();

        // https://yaml.org/spec/1.2.2/#rule-c-ns-flow-map-json-key-entry
        // Usually a ':' character has to be follow by a blank character to be lexed as a standalone
        // T![:] token
        // However, for JSON-compability, the spec allows ':' followed a JSON-like key to be lexed
        // as is, T![:], instead of a potential start of a plain token
        // According to the spec:
        // `{"a"    :b}`
        // Should be lexed as `{"a": b}`, instead of `{"a" (missing colon) :b}`
        let mut just_lexed_json_key = false;
        while let Some(current) = self.current_byte() {
            let tokens = match (current, self.peek_byte()) {
                _ if is_start_of_trivia(current) => {
                    let trivia = self.consume_trivia(current);
                    if self.no_longer_flow() {
                        FlowScopedTokens::Closed(LinkedList::new(), trivia)
                    } else {
                        FlowScopedTokens::Continuable(trivia)
                    }
                }
                (b':', _) if just_lexed_json_key => {
                    just_lexed_json_key = false;
                    self.consume_byte(T![:]).into()
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
                    self.consume_byte(T!['[']).into()
                }
                (b']', _) => {
                    just_lexed_json_key = true;
                    current_depth = current_depth.saturating_sub(1);
                    self.consume_byte(T![']']).into()
                }
                (b'{', _) => {
                    current_depth += 1;
                    self.consume_byte(T!['{']).into()
                }
                (b'}', _) => {
                    just_lexed_json_key = true;
                    current_depth = current_depth.saturating_sub(1);
                    self.consume_byte(T!['}']).into()
                }
                (b',', _) => self.consume_byte(T![,]).into(),
                (current, peek) if is_start_of_plain(current, peek, true) => {
                    self.consume_plain_literal(current, true)
                }
                // ':', '?', '-' can be a valid plain token start
                (b':', _) => self.consume_byte(T![:]).into(),
                (b'?', _) => self.consume_byte(T![?]).into(),
                (b'-', _) => self.consume_byte(T![-]).into(),
                _ => self.consume_unexpected_token().into(),
            };
            match tokens {
                FlowScopedTokens::Continuable(mut tokens) => collection_tokens.append(&mut tokens),
                FlowScopedTokens::Closed(mut tokens, trailing_trivia) => {
                    collection_tokens.append(&mut tokens);
                    return FlowScopedTokens::Closed(collection_tokens, trailing_trivia);
                }
            }
            if current_depth == 0 {
                break;
            }
        }
        FlowScopedTokens::Continuable(collection_tokens)
    }

    // https://yaml.org/spec/1.2.2/#rule-ns-plain
    // TODO: parse multiline plain scalar at current indentation level
    fn consume_plain_literal(&mut self, current: u8, in_flow_collection: bool) -> FlowScopedTokens {
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
                // char is conformant. For example, in `[::]`, the second colon shouldn't be part
                // of the plain literal even though it's "plain safe"
                self.advance(1); // ':'
            } else if is_blank(c) {
                let blank = self.consume_blank();
                // Handle trailing trivia
                if self.no_longer_flow() {
                    let plain = LexToken::new(PLAIN_LITERAL, start, blank.start);
                    return FlowScopedTokens::Closed(plain.into(), blank.into());
                }
            } else {
                break;
            }
        }
        LexToken::new(PLAIN_LITERAL, start, self.current_coordinate).into()
    }

    // https://yaml.org/spec/1.2.2/#731-double-quoted-style
    fn consume_double_quoted_literal(&mut self) -> FlowScopedTokens {
        self.assert_byte(b'"');
        let start = self.current_coordinate;
        self.advance(1);

        loop {
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
                    break;
                }
                Some(current) if is_blank(current) => {
                    let blank = self.consume_blank();
                    if self.no_longer_flow() {
                        let quoted = LexToken::new(DOUBLE_QUOTED_LITERAL, start, blank.start);
                        return FlowScopedTokens::Closed(quoted.into(), blank.into());
                    }
                }
                Some(_) => self.advance(1),
                None => {
                    let err = ParseDiagnostic::new(
                        "Missing closing `\"` quote",
                        self.text_position()..self.text_position(),
                    );
                    self.diagnostics.push(err);
                    break;
                }
            }
        }
        LexToken::new(DOUBLE_QUOTED_LITERAL, start, self.current_coordinate).into()
    }

    // https://yaml.org/spec/1.2.2/#732-single-quoted-style
    fn consume_single_quoted_literal(&mut self) -> FlowScopedTokens {
        self.assert_byte(b'\'');
        let start = self.current_coordinate;
        self.advance(1);

        loop {
            match self.current_byte() {
                Some(b'\'') => {
                    if matches!(self.peek_byte(), Some(b'\'')) {
                        self.advance(2)
                    } else {
                        self.advance(1);
                        break;
                    }
                }
                Some(current) if is_blank(current) => {
                    let blank = self.consume_blank();
                    if self.no_longer_flow() {
                        let quoted = LexToken::new(SINGLE_QUOTED_LITERAL, start, blank.start);
                        return FlowScopedTokens::Closed(quoted.into(), blank.into());
                    }
                }
                Some(_) => self.advance(1),
                None => {
                    let err = ParseDiagnostic::new(
                        "Missing closing `'` quote",
                        self.text_position()..self.text_position(),
                    );
                    self.diagnostics.push(err);
                    break;
                }
            }
        }
        LexToken::new(SINGLE_QUOTED_LITERAL, start, self.current_coordinate).into()
    }

    /// Bumps the current byte and creates a lexed token of the passed in kind.
    #[inline]
    fn consume_byte(&mut self, tok: YamlSyntaxKind) -> LexToken {
        let start = self.current_coordinate;
        self.advance(1);
        LexToken::new(tok, start, self.current_coordinate)
    }

    /// Consume all trivia tokens. The lexer is guaranteed to land on a non trivia token after this,
    /// which it can then use to check if it's still within the current scope or has broken out
    fn consume_trivia(&mut self, current: u8) -> LinkedList<LexToken> {
        debug_assert!(current == b'#' || is_blank(current));
        let mut tokens = LinkedList::new();
        let mut start = self.current_coordinate;
        while let Some(current) = self.current_byte() {
            if is_space(current) {
                self.consume_whitespaces();
                tokens.push_back(LexToken::new(WHITESPACE, start, self.current_coordinate));
                start = self.current_coordinate
            } else if is_break(current) {
                self.consume_newline();
                self.current_coordinate = self.current_coordinate.enter_new_line();
                tokens.push_back(LexToken::new(NEWLINE, start, self.current_coordinate));
                start = self.current_coordinate
            } else if current == b'#' {
                let token = self.consume_comment();
                tokens.push_back(token);
                start = self.current_coordinate
            } else {
                break;
            }
        }
        tokens
    }

    /// Consume all whitespace characters and newlines into a single token
    fn consume_blank(&mut self) -> LexToken {
        let mut is_newline = false;
        let start = self.current_coordinate;
        while let Some(current) = self.current_byte() {
            if is_space(current) {
                self.consume_whitespaces();
            } else if is_break(current) {
                self.consume_newline();
                self.current_coordinate = self.current_coordinate.enter_new_line();
                is_newline = true;
            } else {
                break;
            }
        }
        let kind = if is_newline { NEWLINE } else { WHITESPACE };
        LexToken::new(kind, start, self.current_coordinate)
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

        let char = self.current_char_unchecked();
        let err = ParseDiagnostic::new(
            format!("unexpected character `{char}`"),
            self.text_position()..self.text_position() + char.text_len(),
        );
        self.diagnostics.push(err);
        self.advance(char.len_utf8());
        LexToken::new(ERROR_TOKEN, start, self.current_coordinate)
    }

    fn is_at_mapping_indicator(&self) -> bool {
        self.current_byte().is_some_and(|c| c == b':') && self.peek_byte().is_none_or(is_blank)
    }

    // Flow node must be indented by at least one more space than the parent's scope
    // https://yaml.org/spec/1.2.2/#rule-s-l+flow-in-block
    fn no_longer_flow(&self) -> bool {
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
    /// Must be called at a valid UT8 char boundary
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

    fn contains(&self, coordinate: TextCoordinate, is_dash: bool) -> bool {
        match self {
            Self::Sequence(border) => {
                // Since a sequence entry can start on the same column as a map entry, we have to check
                // whether the current entry is a map or a sequence entry.
                // If it's a map entry and starts on the same column as the current sequence scope, it
                // belongs to the parent scope instead.
                coordinate.column > *border || (is_dash && coordinate.column == *border)
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

struct BlockScopedTokens {
    scope: Option<BlockScope>,
    tokens: LinkedList<LexToken>,
    trailing_trivia: LinkedList<LexToken>,
}

impl BlockScopedTokens {
    fn empty_mapping_key(indicator: LexToken, parent_scope: Option<&BlockScope>) -> Self {
        if parent_scope.is_none_or(|scope| scope.indent(indicator.start)) {
            let mut tokens = LinkedList::new();
            tokens.push_front(LexToken::pseudo(MAPPING_START, indicator.start));
            tokens.push_back(indicator);
            Self {
                scope: Some(BlockScope::new_mapping_scope(indicator.start)),
                tokens,
                trailing_trivia: LinkedList::default(),
            }
        } else {
            Self {
                scope: None,
                tokens: indicator.into(),
                trailing_trivia: LinkedList::default(),
            }
        }
    }

    fn filled_mapping_key(
        mut tokens: LinkedList<LexToken>,
        indicator: LexToken,
        parent_scope: Option<&BlockScope>,
    ) -> Self {
        let start = tokens.front().map_or(indicator.start, |token| token.start);
        tokens.push_back(indicator);

        if parent_scope.is_none_or(|scope| scope.indent(start)) {
            tokens.push_front(LexToken::pseudo(MAPPING_START, start));
            Self {
                scope: Some(BlockScope::new_mapping_scope(start)),
                tokens,
                trailing_trivia: LinkedList::new(),
            }
        } else {
            Self {
                scope: None,
                tokens,
                trailing_trivia: LinkedList::new(),
            }
        }
    }

    fn sequence_entry(indicator: LexToken, parent_scope: Option<&BlockScope>) -> Self {
        debug_assert_eq!(indicator.kind, T![-]);
        if parent_scope.is_none_or(|scope| scope.indent_with_dash(indicator.start)) {
            let mut tokens = LinkedList::new();
            tokens.push_back(indicator);
            tokens.push_front(LexToken::pseudo(SEQUENCE_START, indicator.start));
            Self {
                scope: Some(BlockScope::new_sequence_scope(indicator.start)),
                tokens,
                trailing_trivia: LinkedList::new(),
            }
        } else {
            Self {
                scope: None,
                tokens: indicator.into(),
                trailing_trivia: LinkedList::new(),
            }
        }
    }

    fn flow_value(
        mut tokens: LinkedList<LexToken>,
        start: TextCoordinate,
        trailing_trivia: LinkedList<LexToken>,
    ) -> Self {
        let start = tokens.front().map_or(start, |token| token.start);
        let end = tokens.back().map_or(start, |token| token.end);
        tokens.push_front(LexToken::pseudo(FLOW_START, start));
        tokens.push_back(LexToken::pseudo(FLOW_END, end));
        Self {
            scope: None,
            tokens,
            trailing_trivia,
        }
    }

    fn trivia(trailing_trivia: LinkedList<LexToken>) -> Self {
        Self {
            scope: None,
            tokens: LinkedList::new(),
            trailing_trivia,
        }
    }
}

impl From<LexToken> for BlockScopedTokens {
    fn from(token: LexToken) -> Self {
        BlockScopedTokens {
            scope: None,
            tokens: token.into(),
            trailing_trivia: LinkedList::default(),
        }
    }
}

impl From<LinkedList<LexToken>> for BlockScopedTokens {
    fn from(tokens: LinkedList<LexToken>) -> Self {
        BlockScopedTokens {
            scope: None,
            tokens,
            trailing_trivia: LinkedList::default(),
        }
    }
}

enum FlowScopedTokens {
    Continuable(LinkedList<LexToken>),
    Closed(LinkedList<LexToken>, LinkedList<LexToken>),
}

impl From<LexToken> for FlowScopedTokens {
    fn from(value: LexToken) -> Self {
        Self::Continuable(value.into())
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct TextCoordinate {
    /// The byte position in the source text.
    offset: usize,
    /// The number of bytes since the last new line.
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
fn is_start_of_trivia(c: u8) -> bool {
    is_blank(c) || c == b'#'
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
