//! Parse rules for TOML documents.
//!
//! Newlines are trivia, so the rules check [`Parser::has_preceding_line_break`]
//! wherever TOML requires a newline (between key/value pairs and table headers)
//! or forbids one (inside a key/value pair or a table header).

use crate::lexer::TomlLexContext;
use crate::parser::TomlParser;
mod parse_error;

use crate::syntax::parse_error::{
    equals_on_next_line, expected_equals_and_value, expected_key, expected_key_value,
    expected_key_value_or_table_header, expected_newline, expected_value,
    expected_value_on_same_line, multiline_string_key, nesting_too_deep, token_on_next_line,
};
use biome_parser::diagnostic::expected_token;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{RecoveryError, RecoveryResult};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_toml_syntax::TomlSyntaxKind::{self, *};
use biome_toml_syntax::{T, TextRange};

const KEY_PART_START: TokenSet<TomlSyntaxKind> = token_set![
    TOML_BARE_KEY_LITERAL,
    TOML_BASIC_STRING_LITERAL,
    TOML_LITERAL_STRING_LITERAL,
    TOML_ML_BASIC_STRING_LITERAL,
    TOML_ML_LITERAL_STRING_LITERAL,
];

const TABLE_HEADER_START: TokenSet<TomlSyntaxKind> = token_set![T!['['], T!["[["]];

const STRING_LITERALS: TokenSet<TomlSyntaxKind> = token_set![
    TOML_BASIC_STRING_LITERAL,
    TOML_LITERAL_STRING_LITERAL,
    TOML_ML_BASIC_STRING_LITERAL,
    TOML_ML_LITERAL_STRING_LITERAL,
];

const DATE_TIME_LITERALS: TokenSet<TomlSyntaxKind> = token_set![
    TOML_OFFSET_DATE_TIME_LITERAL,
    TOML_LOCAL_DATE_TIME_LITERAL,
    TOML_LOCAL_DATE_LITERAL,
    TOML_LOCAL_TIME_LITERAL,
];

const VALUE_START: TokenSet<TomlSyntaxKind> =
    STRING_LITERALS.union(DATE_TIME_LITERALS).union(token_set![
        TOML_INTEGER_LITERAL,
        TOML_FLOAT_LITERAL,
        T![true],
        T![false],
        T!['['],
        T!['{'],
    ]);

pub(crate) fn parse_root(p: &mut TomlParser) -> CompletedMarker {
    let m = p.start();

    p.eat(UNICODE_BOM);
    RootItemList::default().parse_list(p);
    p.expect(EOF);

    m.complete(p, TOML_ROOT)
}

/// Key/value pairs before the first table header, then the tables.
#[derive(Default)]
struct RootItemList {
    has_items: bool,
}

impl ParseNodeList for RootItemList {
    type Kind = TomlSyntaxKind;
    type Parser<'source> = TomlParser<'source>;
    const LIST_KIND: Self::Kind = TOML_ROOT_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        prepare_line_item(p, self.has_items);
        self.has_items = true;

        match p.cur() {
            T!["[["] => parse_array_table(p),
            T!['['] => parse_table(p),
            _ if p.at_ts(KEY_PART_START) => parse_key_value(p, TomlLexContext::Key),
            _ => Absent,
        }
    }

    fn is_at_list_end(&self, _p: &mut Self::Parser<'_>) -> bool {
        false
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        recover_line_item(p, parsed_element)
    }
}

/// The key/value pairs that follow a table header.
#[derive(Default)]
struct TableEntryList;

impl ParseNodeList for TableEntryList {
    type Kind = TomlSyntaxKind;
    type Parser<'source> = TomlParser<'source>;
    const LIST_KIND: Self::Kind = TOML_KEY_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        prepare_line_item(p, true);

        if p.at_ts(KEY_PART_START) {
            parse_key_value(p, TomlLexContext::Key)
        } else {
            Absent
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(TABLE_HEADER_START)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        recover_line_item(p, parsed_element)
    }
}

/// Prepares the parser for a key/value pair or table header.
///
/// Items start a line, so the current token is lexed again as a key if a
/// previous error left it lexed as a value. `requires_line_break` is `false`
/// for the first item of the document.
fn prepare_line_item(p: &mut TomlParser, requires_line_break: bool) {
    // A `[` lexed as a value can be the first half of a `[[` header.
    if p.at(T!['[']) || (!p.at_ts(KEY_PART_START.union(TABLE_HEADER_START)) && !p.at(EOF)) {
        p.re_lex(TomlLexContext::Key);
    }

    if requires_line_break && !p.has_preceding_line_break() {
        let range = p.cur_range();
        p.error(expected_newline(p, range));
    }
}

/// Wraps the rest of the line into a bogus item when no item could be parsed.
fn recover_line_item(p: &mut TomlParser, parsed_element: ParsedSyntax) -> RecoveryResult {
    if let Present(marker) = parsed_element {
        return Ok(marker);
    }

    if p.at(EOF) {
        return Err(RecoveryError::Eof);
    }

    // The lexer has already reported invalid characters.
    if !p.at(ERROR_TOKEN) {
        let range = p.cur_range();
        p.error(expected_key_value_or_table_header(p, range));
    }

    let m = p.start();
    p.bump_any();
    while !p.at(EOF) && !p.has_preceding_line_break() {
        p.bump_any();
    }
    Ok(m.complete(p, TOML_BOGUS))
}

// [a.b]
fn parse_table(p: &mut TomlParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['[']);
    let key_range = parse_header_key(p);
    p.start_table(key_range);
    expect_on_same_line(p, T![']'], TomlLexContext::Key);
    TableEntryList.parse_list(p);

    Present(m.complete(p, TOML_TABLE))
}

// [[a.b]]
fn parse_array_table(p: &mut TomlParser) -> ParsedSyntax {
    if !p.at(T!["[["]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!["[["]);
    let key_range = parse_header_key(p);
    p.start_array_table(key_range);
    expect_on_same_line(p, T!["]]"], TomlLexContext::Key);
    TableEntryList.parse_list(p);

    Present(m.complete(p, TOML_ARRAY_TABLE))
}

/// Parses the key of a table header and returns its range, or the range of
/// the current token if the key is missing.
fn parse_header_key(p: &mut TomlParser) -> TextRange {
    if !p.has_preceding_line_break()
        && let Present(key) = parse_key(p)
    {
        return key.range(p);
    }

    p.start_key();
    p.invalidate_key();
    let range = p.cur_range();
    p.error(expected_key(p, range));
    range
}

/// Bumps `kind` if it's on the same line as the previous token and lexes the
/// following token in `context`.
fn expect_on_same_line(p: &mut TomlParser, kind: TomlSyntaxKind, context: TomlLexContext) -> bool {
    if !p.at(kind) {
        p.error(expected_token(kind));
        false
    } else if p.has_preceding_line_break() {
        let range = p.cur_range();
        p.error(token_on_next_line(p, range));
        false
    } else {
        p.bump_with_context(kind, context);
        true
    }
}

/// Parses a key/value pair. `after` is the context for the token that follows
/// the value.
fn parse_key_value(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at_ts(KEY_PART_START) {
        return Absent;
    }

    let m = p.start();
    let key_range = parse_key(p).ok().map(|key| key.range(p));

    if p.at(T![=]) && !p.has_preceding_line_break() {
        p.define_key_value();
        p.bump_with_context(T![=], TomlLexContext::Value);
    } else {
        report_missing_equals(p, key_range);
        return Present(m.complete(p, TOML_KEY_VALUE));
    }

    if p.has_preceding_line_break() || p.at(EOF) {
        // The value is missing and the current token belongs to the next line.
        let range = p.cur_range();
        p.error(expected_value_on_same_line(p, range));
        if !p.at(EOF) {
            p.re_lex(TomlLexContext::Key);
        }
    } else {
        parse_value(p, after).or_add_diagnostic(p, expected_value);
    }

    Present(m.complete(p, TOML_KEY_VALUE))
}

/// Reports a missing `=` after a key. Tokens that follow the key on the same
/// line become the bogus value of the pair, so they aren't reported again.
fn report_missing_equals(p: &mut TomlParser, key_range: Option<TextRange>) {
    if p.at(T![=]) {
        let range = p.cur_range();
        p.error(equals_on_next_line(p, range));
    } else if p.has_preceding_line_break() || p.at(EOF) {
        let range = key_range.unwrap_or_else(|| p.cur_range());
        p.error(expected_equals_and_value(p, range));
    } else {
        p.error(expected_token(T![=]));
        let m = p.start();
        while !p.at(EOF) && !p.at_ts(token_set![T![,], T!['}']]) && !p.has_preceding_line_break() {
            p.bump_any();
        }
        m.complete(p, TOML_BOGUS_VALUE);
    }
}

// a."b".'c'
fn parse_key(p: &mut TomlParser) -> ParsedSyntax {
    if !p.at_ts(KEY_PART_START) {
        return Absent;
    }

    let m = p.start();
    let parts = p.start();

    p.start_key();
    parse_key_part(p).ok();
    while p.at(T![.]) && !p.has_preceding_line_break() {
        p.bump(T![.]);

        if p.has_preceding_line_break() {
            p.invalidate_key();
            let range = p.cur_range();
            p.error(expected_key(p, range));
            break;
        }

        if parse_key_part(p).is_absent() {
            p.invalidate_key();
            let range = p.cur_range();
            p.error(expected_key(p, range));
        }
    }

    parts.complete(p, TOML_KEY_PART_LIST);
    Present(m.complete(p, TOML_KEY))
}

fn parse_key_part(p: &mut TomlParser) -> ParsedSyntax {
    let kind = match p.cur() {
        TOML_BARE_KEY_LITERAL => {
            p.push_key_part();
            TOML_BARE_KEY
        }
        TOML_BASIC_STRING_LITERAL | TOML_LITERAL_STRING_LITERAL => {
            p.push_key_part();
            TOML_QUOTED_KEY
        }
        TOML_ML_BASIC_STRING_LITERAL | TOML_ML_LITERAL_STRING_LITERAL => {
            p.invalidate_key();
            let range = p.cur_range();
            p.error(multiline_string_key(p, range));
            TOML_BOGUS
        }
        _ => return Absent,
    };

    let m = p.start();
    p.bump_any();
    Present(m.complete(p, kind))
}

/// Parses a value. `after` is the context for the token that follows it.
fn parse_value(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    let kind = match p.cur() {
        T!['['] => return parse_array(p, after),
        T!['{'] => return parse_inline_table(p, after),
        TOML_INTEGER_LITERAL => TOML_INTEGER,
        TOML_FLOAT_LITERAL => TOML_FLOAT,
        T![true] | T![false] => TOML_BOOLEAN,
        // The lexer has already reported the invalid value.
        ERROR_TOKEN => TOML_BOGUS_VALUE,
        kind if STRING_LITERALS.contains(kind) => TOML_STRING,
        kind if DATE_TIME_LITERALS.contains(kind) => TOML_DATE_TIME,
        _ => return Absent,
    };

    let m = p.start();
    p.bump_any_with_context(after);
    Present(m.complete(p, kind))
}

/// The maximum number of arrays and inline tables that can be nested in each
/// other. It bounds the recursion of the parser and of tree consumers.
pub(crate) const MAX_NESTING_DEPTH: usize = 128;

// [1, 2, 3]
fn parse_array(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    if p.nesting_depth() >= MAX_NESTING_DEPTH {
        return Present(skip_nested_value(p, after));
    }

    let m = p.start();
    p.bump_with_context(T!['['], TomlLexContext::Value);
    p.enter_nesting(T!['[']);
    ArrayElementList::default().parse_list(p);
    p.exit_nesting();
    expect_with_context(p, T![']'], after);

    Present(m.complete(p, TOML_ARRAY))
}

/// Wraps an array or inline table that exceeds [MAX_NESTING_DEPTH] into a
/// bogus value, without recursing into it.
fn skip_nested_value(p: &mut TomlParser, after: TomlLexContext) -> CompletedMarker {
    let range = p.cur_range();
    p.error(nesting_too_deep(p, range));

    let m = p.start();
    // The brackets that are open, innermost last.
    let mut open = Vec::new();
    let mut previous = None;
    loop {
        let kind = p.cur();
        if kind == EOF
            || previous
                .is_some_and(|previous| ends_skipped_value(p, previous, open.last().copied()))
        {
            break;
        }

        match kind {
            T!['['] | T!['{'] => open.push(kind),
            T![']'] if open.last() == Some(&T!['[']) => {
                open.pop();
            }
            T!['}'] if open.last() == Some(&T!['{']) => {
                open.pop();
            }
            _ => {}
        }

        // Lex the next token the way the regular parse rules would.
        let context = match open.last() {
            None => after,
            Some(_) if kind == T![=] => TomlLexContext::Value,
            Some(&T!['[']) => TomlLexContext::Value,
            Some(_) => TomlLexContext::Key,
        };
        p.bump_any_with_context(context);
        previous = Some(kind);

        if open.is_empty() {
            break;
        }
    }

    m.complete(p, TOML_BOGUS_VALUE)
}

/// Returns `true` if the current value-like token on a new line is the start
/// of a key/value pair or a table header that follows an unterminated array.
///
/// The current token is lexed again as a key to scan for a key/value pair
/// (`key.parts =`) or a table header (`[key]` or `[[key]]` followed by a line
/// break). The check only succeeds where valid TOML can't continue the array:
/// an element is always followed by `,`, `]`, or a comment, never by `=`, and
/// a valid element such as `[[2]]` that ends a line is followed by `,` or `]`.
fn is_at_line_item(p: &mut TomlParser) -> bool {
    let first = p.re_lex(TomlLexContext::Key);
    let is_line_item = match first {
        T!['['] | T!["[["] => is_at_table_header(p, first),
        _ if KEY_PART_START.contains(first) => {
            matches!(scan_key(p, 0), Some(n) if p.nth_at(n, T![=]) && !p.has_nth_preceding_line_break(n))
        }
        _ => false,
    };

    if !is_line_item {
        p.re_lex(TomlLexContext::Value);
    }
    is_line_item
}

/// Returns `true` if the `[` or `[[` at the current token opens a table
/// header that ends its line.
fn is_at_table_header(p: &mut TomlParser, open: TomlSyntaxKind) -> bool {
    let close = if open == T!["[["] { T!["]]"] } else { T![']'] };
    let Some(n) = scan_key(p, 1) else {
        return false;
    };
    if !p.nth_at(n, close) || p.has_nth_preceding_line_break(n) {
        return false;
    }

    let next = p.nth(n + 1);
    next == EOF || (p.has_nth_preceding_line_break(n + 1) && next != T![,] && next != T![']'])
}

/// Scans a dotted key on the current line that starts at the `n`th token and
/// returns the position of the token after it.
fn scan_key(p: &mut TomlParser, mut n: usize) -> Option<usize> {
    loop {
        if (n > 0 && p.has_nth_preceding_line_break(n)) || !p.nth_at_ts(n, KEY_PART_START) {
            return None;
        }
        n += 1;
        if !p.nth_at(n, T![.]) || p.has_nth_preceding_line_break(n) {
            return Some(n);
        }
        n += 1;
    }
}

/// Returns `true` if the current token can't continue a value that is skipped
/// by [skip_nested_value] and most likely starts the next line item.
///
/// `previous` is the last skipped token and `innermost` the innermost open
/// bracket. The rules mirror the line-break handling of the list parsers.
fn ends_skipped_value(
    p: &mut TomlParser,
    previous: TomlSyntaxKind,
    innermost: Option<TomlSyntaxKind>,
) -> bool {
    if !p.has_preceding_line_break() {
        return false;
    }

    let at_separator_or_close = p.at_ts(token_set![T![,], T![']'], T!['}']]);
    match previous {
        // A key and its `=` must be on the same line as the value.
        T![=] | T![.] | TOML_BARE_KEY_LITERAL => true,
        T!['['] | T![,] if innermost == Some(T!['[']) => {
            !at_separator_or_close && (!p.at_ts(VALUE_START) || is_at_line_item(p))
        }
        T!['{'] | T![,] => p.at_ts(TABLE_HEADER_START),
        // The previous token ended a value or a key.
        _ => !at_separator_or_close,
    }
}

#[derive(Default)]
struct ArrayElementList {
    /// `true` after an element, when a `,` or `]` should follow.
    expects_separator: bool,
}

impl ParseSeparatedList for ArrayElementList {
    type Kind = TomlSyntaxKind;
    type Parser<'source> = TomlParser<'source>;
    const LIST_KIND: Self::Kind = TOML_ARRAY_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_value(p, TomlLexContext::Value)
    }

    /// Ends the list at `]`.
    ///
    /// The array is most likely unterminated if a token on a new line can't
    /// continue it: anything but a `,` after an element, or anything but a
    /// value after a `,`. That token starts the next table header or
    /// key/value pair.
    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        if p.at(T![']']) || (p.at(T!['}']) && p.is_enclosed_by(T!['{'])) {
            return true;
        }
        if !p.has_preceding_line_break() || p.at(T![,]) {
            return false;
        }
        self.expects_separator || !p.at_ts(VALUE_START) || is_at_line_item(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        self.expects_separator = true;
        recover_list_element(
            p,
            parsed_element,
            if p.is_enclosed_by(T!['{']) {
                token_set![T![,], T![']'], T!['}']]
            } else {
                token_set![T![,], T![']']]
            },
            TomlLexContext::Value,
            TOML_BOGUS_VALUE,
            expected_value,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        self.expects_separator = false;
        expect_with_context(p, T![,], TomlLexContext::Value)
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }

    fn diagnose_missing_element(&mut self, p: &mut Self::Parser<'_>) {
        let range = p.cur_range();
        p.error(expected_value(p, range));
    }
}

// { a = 1, b = 2 }
fn parse_inline_table(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    if p.nesting_depth() >= MAX_NESTING_DEPTH {
        return Present(skip_nested_value(p, after));
    }

    let m = p.start();
    let range = p.cur_range();
    p.bump_with_context(T!['{'], TomlLexContext::Key);
    p.enter_nesting(T!['{']);
    p.start_inline_table(range);
    InlineTableEntryList::default().parse_list(p);
    p.end_inline_table();
    p.exit_nesting();
    expect_with_context(p, T!['}'], after);

    Present(m.complete(p, TOML_INLINE_TABLE))
}

#[derive(Default)]
struct InlineTableEntryList {
    /// `true` after an entry, when a `,` or `}` should follow.
    expects_separator: bool,
}

impl ParseSeparatedList for InlineTableEntryList {
    type Kind = TomlSyntaxKind;
    type Parser<'source> = TomlParser<'source>;
    const LIST_KIND: Self::Kind = TOML_INLINE_TABLE_ENTRY_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_key_value(p, TomlLexContext::Key)
    }

    /// Ends the list at `}`.
    ///
    /// The inline table is most likely unterminated if a token on a new line
    /// can't continue it: anything but a `,` after an entry, or a table header.
    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        if p.at(T!['}']) || (p.at(T![']']) && p.is_enclosed_by(T!['['])) {
            return true;
        }
        if !p.has_preceding_line_break() || p.at(T![,]) {
            return false;
        }
        self.expects_separator || p.at_ts(TABLE_HEADER_START)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        self.expects_separator = true;
        recover_list_element(
            p,
            parsed_element,
            if p.is_enclosed_by(T!['[']) {
                token_set![T![,], T!['}'], T![']']]
            } else {
                token_set![T![,], T!['}']]
            },
            TomlLexContext::Key,
            TOML_BOGUS,
            expected_key_value,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        self.expects_separator = false;
        expect_with_context(p, T![,], TomlLexContext::Key)
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }

    fn diagnose_missing_element(&mut self, p: &mut Self::Parser<'_>) {
        let range = p.cur_range();
        p.error(expected_key_value(p, range));
    }
}

/// Bumps `kind` and lexes the following token in `context`, or reports that
/// `kind` is missing.
fn expect_with_context(p: &mut TomlParser, kind: TomlSyntaxKind, context: TomlLexContext) -> bool {
    if p.at(kind) {
        p.bump_with_context(kind, context);
        true
    } else {
        p.error(expected_token(kind));
        false
    }
}

/// Reports the current token with `expected` and wraps tokens into a
/// `bogus_kind` node until a token in `recovery_set`, a line break, or the end
/// of the file.
fn recover_list_element(
    p: &mut TomlParser,
    parsed_element: ParsedSyntax,
    recovery_set: TokenSet<TomlSyntaxKind>,
    context: TomlLexContext,
    bogus_kind: TomlSyntaxKind,
    expected: fn(&TomlParser, TextRange) -> ParseDiagnostic,
) -> RecoveryResult {
    if let Present(marker) = parsed_element {
        return Ok(marker);
    }

    if p.at(EOF) {
        return Err(RecoveryError::Eof);
    }

    if p.at_ts(recovery_set) {
        return Err(RecoveryError::AlreadyRecovered);
    }

    let range = p.cur_range();
    p.error(expected(p, range));

    let m = p.start();
    p.bump_any_with_context(context);
    while !p.at(EOF) && !p.at_ts(recovery_set) && !p.has_preceding_line_break() {
        p.bump_any_with_context(context);
    }
    Ok(m.complete(p, bogus_kind))
}
