//! Parse rules for TOML documents.
//!
//! Newlines are trivia, so the rules check [`Parser::has_preceding_line_break`]
//! wherever TOML requires a newline (between members and table headers) or
//! forbids one (inside a member or a table header).

use crate::lexer::TomlLexContext;
use crate::parser::TomlParser;
mod parse_error;

use crate::syntax::parse_error::{
    equals_on_next_line, expected_equals_and_value, expected_header_close_before_line_end,
    expected_key, expected_key_before_line_end, expected_member, expected_member_or_table_header,
    expected_newline, expected_value, expected_value_before_line_end, multiline_string_key,
    token_on_next_line,
};
use biome_parser::diagnostic::expected_token;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{
    ParseRecovery, ParseRecoveryTokenSet, RecoveryError, RecoveryResult,
};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_toml_syntax::TomlSyntaxKind::{self, *};
use biome_toml_syntax::{T, TextRange};

const KEY_FRAGMENT_START: TokenSet<TomlSyntaxKind> = token_set![
    TOML_BARE_KEY_LITERAL,
    TOML_BASIC_STRING_LITERAL,
    TOML_LITERAL_STRING_LITERAL,
    TOML_ML_BASIC_STRING_LITERAL,
    TOML_ML_LITERAL_STRING_LITERAL,
];

/// The tokens that start a key, including a `.` that follows a missing fragment.
const KEY_START: TokenSet<TomlSyntaxKind> = KEY_FRAGMENT_START.union(token_set![T![.]]);

const TABLE_HEADER_START: TokenSet<TomlSyntaxKind> = token_set![T!['['], T!["[["]];

/// The tokens that start a member or a table header.
const LINE_ITEM_START: TokenSet<TomlSyntaxKind> = KEY_START.union(TABLE_HEADER_START);

/// The closing brackets of a table header and of an array of tables header.
const TABLE_HEADER_END: TokenSet<TomlSyntaxKind> = token_set![T![']'], T!["]]"]];

/// The delimiters that close an array or an inline table.
const CLOSING_DELIMITERS: TokenSet<TomlSyntaxKind> = token_set![T![']'], T!['}']];

/// The tokens where recovery inside an array or inline table stops: a
/// separator, or a delimiter that closes this value or an enclosing one.
const LIST_RECOVERY_SET: TokenSet<TomlSyntaxKind> = CLOSING_DELIMITERS.union(token_set![T![,]]);

const QUOTED_KEY_LITERALS: TokenSet<TomlSyntaxKind> =
    token_set![TOML_BASIC_STRING_LITERAL, TOML_LITERAL_STRING_LITERAL];

const MULTILINE_STRING_LITERALS: TokenSet<TomlSyntaxKind> =
    token_set![TOML_ML_BASIC_STRING_LITERAL, TOML_ML_LITERAL_STRING_LITERAL];

const BOOLEAN_LITERALS: TokenSet<TomlSyntaxKind> = token_set![T![true], T![false]];

const STRING_LITERALS: TokenSet<TomlSyntaxKind> = token_set![
    TOML_BASIC_STRING_LITERAL,
    TOML_LITERAL_STRING_LITERAL,
    TOML_ML_BASIC_STRING_LITERAL,
    TOML_ML_LITERAL_STRING_LITERAL,
];

const DATE_TIME_LITERALS: TokenSet<TomlSyntaxKind> =
    token_set![TOML_DATE_LITERAL, TOML_TIME_LITERAL];

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
            _ if p.at_ts(KEY_START) => parse_member(p, TomlLexContext::Key),
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
        parsed_element.or_recover(p, &LineItemParseRecovery, expected_member_or_table_header)
    }
}

/// The members that follow a table header.
#[derive(Default)]
struct TableMemberList;

impl ParseNodeList for TableMemberList {
    type Kind = TomlSyntaxKind;
    type Parser<'source> = TomlParser<'source>;
    const LIST_KIND: Self::Kind = TOML_MEMBER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        prepare_line_item(p, true);

        if p.at_ts(KEY_START) {
            parse_member(p, TomlLexContext::Key)
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
        parsed_element.or_recover(p, &LineItemParseRecovery, expected_member_or_table_header)
    }
}

/// Prepares the parser for a member or table header.
///
/// Items start a line, so the current token is lexed again as a key if a
/// previous error left it lexed as a value. `requires_line_break` is `false`
/// for the first item of the document.
fn prepare_line_item(p: &mut TomlParser, requires_line_break: bool) {
    // A `[` lexed as a value can be the first half of a `[[` header.
    if p.at(T!['[']) || (!p.at_ts(KEY_FRAGMENT_START.union(TABLE_HEADER_START)) && !p.at(EOF)) {
        p.re_lex(TomlLexContext::Key);
    }

    if requires_line_break && !p.has_preceding_line_break() {
        let range = p.cur_range();
        p.error(expected_newline(p, range));
    }
}

/// Skips an invalid line, and the following lines that can't start a
/// member or a table header, into a bogus member.
struct LineItemParseRecovery;

impl ParseRecovery for LineItemParseRecovery {
    type Kind = TomlSyntaxKind;
    type Parser<'source> = TomlParser<'source>;
    const RECOVERED_KIND: Self::Kind = TOML_BOGUS_MEMBER;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.has_preceding_line_break() && p.at_ts(LINE_ITEM_START)
    }
}

// [a.b]
// c = 1
fn parse_table(p: &mut TomlParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();
    parse_table_header(p, T!['['], T![']'], TOML_TABLE_HEADER).ok();
    TableMemberList.parse_list(p);

    Present(m.complete(p, TOML_TABLE))
}

// [[a.b]]
// c = 1
fn parse_array_table(p: &mut TomlParser) -> ParsedSyntax {
    if !p.at(T!["[["]) {
        return Absent;
    }

    let m = p.start();
    parse_table_header(p, T!["[["], T!["]]"], TOML_ARRAY_TABLE_HEADER).ok();
    TableMemberList.parse_list(p);

    Present(m.complete(p, TOML_ARRAY_TABLE))
}

/// Parses a table header that starts with `open` and ends with `close` on the
/// same line, and completes it as `kind`.
///
/// A header without a key or without its closing bracket on the same line
/// becomes a bogus header, so the members on the following lines remain
/// members of the table.
fn parse_table_header(
    p: &mut TomlParser,
    open: TomlSyntaxKind,
    close: TomlSyntaxKind,
    kind: TomlSyntaxKind,
) -> ParsedSyntax {
    if !p.at(open) {
        return Absent;
    }

    let m = p.start();
    let open_range = p.cur_range();
    p.bump(open);

    if is_at_line_end(p) {
        p.error(expected_key_before_line_end(p, open_range));
        return Present(m.complete(p, TOML_BOGUS_TABLE_HEADER));
    }

    let key = parse_key(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(TOML_BOGUS_KEY, TABLE_HEADER_END)
            .enable_recovery_on_line_break(),
        expected_key,
    );

    // Tokens between the key and the closing bracket, as in `[a b]`, make the
    // key a bogus key.
    let has_key = match key {
        Ok(key) if !p.at_ts(TABLE_HEADER_END) && !is_at_line_end(p) => {
            p.error(expected_token(close));
            let m = key.precede(p);
            ParseRecoveryTokenSet::new(TOML_BOGUS_KEY_FRAGMENT, TABLE_HEADER_END)
                .enable_recovery_on_line_break()
                .recover(p)
                .ok();
            m.complete(p, TOML_BOGUS_KEY);
            true
        }
        key => key.is_ok(),
    };

    if p.at(close) && !p.has_preceding_line_break() {
        p.bump(close);
        let kind = if has_key {
            kind
        } else {
            TOML_BOGUS_TABLE_HEADER
        };
        return Present(m.complete(p, kind));
    }

    if p.at_ts(TABLE_HEADER_END) && !p.has_preceding_line_break() {
        // The closing bracket of the other kind of header, as in `[[a]`.
        p.error(expected_token(close));
        p.bump_ts(TABLE_HEADER_END);
        return Present(m.complete(p, TOML_BOGUS_TABLE_HEADER));
    }

    if has_key {
        if p.at(close) {
            let range = p.cur_range();
            p.error(token_on_next_line(p, range));
        } else if !is_after_trailing_dot(p) {
            let range =
                TextRange::new(open_range.start(), p.last_end().unwrap_or(open_range.end()));
            p.error(expected_header_close_before_line_end(p, close, range));
        }
    }

    Present(m.complete(p, TOML_BOGUS_TABLE_HEADER))
}

/// Returns `true` if the last token is a `.` that ends a key, which the key
/// fragments have already reported as a missing fragment.
fn is_after_trailing_dot(p: &TomlParser) -> bool {
    p.last() == Some(T![.])
}

/// Returns `true` if the current token is on the next line or the end of the
/// file, so a member or table header can't continue.
fn is_at_line_end(p: &TomlParser) -> bool {
    p.at(EOF) || p.has_preceding_line_break()
}

/// Parses a member. `after` is the context for the token that follows
/// the value.
fn parse_member(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at_ts(KEY_START) {
        return Absent;
    }

    let m = p.start();
    let key_range = parse_key(p).ok().map(|key| key.range(p));

    if is_at_line_end(p) {
        if p.at(T![=]) {
            let range = p.cur_range();
            p.error(equals_on_next_line(p, range));
        } else if !is_after_trailing_dot(p) {
            let range = key_range.unwrap_or_else(|| p.cur_range());
            p.error(expected_equals_and_value(p, range));
        }
        return Present(m.complete(p, TOML_MEMBER));
    }

    let eq_range = p.cur_range();
    expect_with_context(p, T![=], TomlLexContext::Value);

    if is_at_line_end(p) {
        // The value is missing and the current token belongs to the next line.
        p.error(expected_value_before_line_end(p, eq_range));
        if !p.at(EOF) {
            p.re_lex(TomlLexContext::Key);
        }
    } else {
        parse_value(p, after)
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(TOML_BOGUS_VALUE, LIST_RECOVERY_SET)
                    .enable_recovery_on_line_break(),
                expected_value,
            )
            .ok();
    }

    Present(m.complete(p, TOML_MEMBER))
}

// a."b".'c'
//
// A key with a missing fragment, such as `a.`, `.a`, or `a..b`, becomes a
// bogus key.
fn parse_key(p: &mut TomlParser) -> ParsedSyntax {
    if !p.at_ts(KEY_START) {
        return Absent;
    }

    let m = p.start();
    let mut fragments = KeyFragmentList::default();
    fragments.parse_list(p);
    let kind = if fragments.has_missing_fragments {
        TOML_BOGUS_KEY
    } else {
        TOML_KEY
    };
    Present(m.complete(p, kind))
}

/// The dot-separated fragments of a key, which must all be on one line.
#[derive(Default)]
struct KeyFragmentList {
    /// `true` after a fragment or `.`, when the rest of the key must be on the
    /// same line.
    has_fragments: bool,
    /// `true` if a fragment is missing.
    has_missing_fragments: bool,
    /// The range of the last `.`.
    last_dot_range: Option<TextRange>,
}

impl KeyFragmentList {
    /// Returns the range of the `.` that ends the line if the current token is
    /// on the next line, where the fragment after the `.` is missing.
    fn dot_at_line_end(&self, p: &TomlParser) -> Option<TextRange> {
        if self.has_fragments && is_at_line_end(p) {
            self.last_dot_range
        } else {
            None
        }
    }
}

impl ParseSeparatedList for KeyFragmentList {
    type Kind = TomlSyntaxKind;
    type Parser<'source> = TomlParser<'source>;
    const LIST_KIND: Self::Kind = TOML_KEY_FRAGMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        // A fragment on the line after a `.` belongs to the next line item.
        if self.has_fragments && is_at_line_end(p) {
            return Absent;
        }
        let fragment = parse_key_fragment(p);
        self.has_fragments |= fragment.is_present();
        fragment
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        !p.at(T![.]) || (self.has_fragments && p.has_preceding_line_break())
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        let dot_at_line_end = self.dot_at_line_end(p);
        let recovered = parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                TOML_BOGUS_KEY_FRAGMENT,
                token_set![T![.], T![=], T![,], T!['}']].union(TABLE_HEADER_END),
            )
            .enable_recovery_on_line_break(),
            |p, range| match dot_at_line_end {
                Some(dot_range) => expected_key_before_line_end(p, dot_range),
                None => expected_key(p, range),
            },
        );
        self.has_missing_fragments |= recovered.is_err();
        recovered
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![.]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        self.has_fragments = true;
        self.last_dot_range = Some(p.cur_range());
        p.expect(T![.])
    }

    fn allow_empty(&self) -> bool {
        false
    }

    fn diagnose_missing_element(&mut self, p: &mut Self::Parser<'_>) {
        self.has_missing_fragments = true;
        if let Some(range) = self.dot_at_line_end(p) {
            p.error(expected_key_before_line_end(p, range));
        } else {
            let range = p.cur_range();
            p.error(expected_key(p, range));
        }
    }
}

fn parse_key_fragment(p: &mut TomlParser) -> ParsedSyntax {
    match p.cur() {
        TOML_BARE_KEY_LITERAL => parse_bare_key(p),
        TOML_BASIC_STRING_LITERAL | TOML_LITERAL_STRING_LITERAL => parse_quoted_key(p),
        TOML_ML_BASIC_STRING_LITERAL | TOML_ML_LITERAL_STRING_LITERAL => {
            parse_multiline_string_key(p)
        }
        _ => Absent,
    }
}

// key
fn parse_bare_key(p: &mut TomlParser) -> ParsedSyntax {
    if !p.at(TOML_BARE_KEY_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump(TOML_BARE_KEY_LITERAL);
    Present(m.complete(p, TOML_BARE_KEY))
}

// "key" or 'key'
fn parse_quoted_key(p: &mut TomlParser) -> ParsedSyntax {
    if !p.at_ts(QUOTED_KEY_LITERALS) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(QUOTED_KEY_LITERALS);
    Present(m.complete(p, TOML_QUOTED_KEY))
}

/// Parses a multi-line string used as a key, which TOML doesn't allow, into a
/// bogus key fragment.
fn parse_multiline_string_key(p: &mut TomlParser) -> ParsedSyntax {
    if !p.at_ts(MULTILINE_STRING_LITERALS) {
        return Absent;
    }

    let range = p.cur_range();
    p.error(multiline_string_key(p, range));

    let m = p.start();
    p.bump_ts(MULTILINE_STRING_LITERALS);
    Present(m.complete(p, TOML_BOGUS_KEY_FRAGMENT))
}

/// Parses a value. `after` is the context for the token that follows it.
fn parse_value(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    match p.cur() {
        T!['['] => parse_array(p, after),
        T!['{'] => parse_inline_table(p, after),
        TOML_INTEGER_LITERAL => parse_integer(p, after),
        TOML_FLOAT_LITERAL => parse_float(p, after),
        T![true] | T![false] => parse_boolean(p, after),
        ERROR_TOKEN => parse_invalid_value(p, after),
        TOML_BASIC_STRING_LITERAL => parse_basic_string(p, after),
        TOML_LITERAL_STRING_LITERAL => parse_literal_string(p, after),
        TOML_ML_BASIC_STRING_LITERAL => parse_multiline_basic_string(p, after),
        TOML_ML_LITERAL_STRING_LITERAL => parse_multiline_literal_string(p, after),
        TOML_DATE_LITERAL => parse_date(p, after),
        TOML_TIME_LITERAL => parse_time(p, after),
        _ => Absent,
    }
}

// "a"
fn parse_basic_string(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(TOML_BASIC_STRING_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(TOML_BASIC_STRING_LITERAL, after);
    Present(m.complete(p, TOML_BASIC_STRING))
}

// 'a'
fn parse_literal_string(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(TOML_LITERAL_STRING_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(TOML_LITERAL_STRING_LITERAL, after);
    Present(m.complete(p, TOML_LITERAL_STRING))
}

// """a"""
fn parse_multiline_basic_string(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(TOML_ML_BASIC_STRING_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(TOML_ML_BASIC_STRING_LITERAL, after);
    Present(m.complete(p, TOML_MULTILINE_BASIC_STRING))
}

// '''a'''
fn parse_multiline_literal_string(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(TOML_ML_LITERAL_STRING_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(TOML_ML_LITERAL_STRING_LITERAL, after);
    Present(m.complete(p, TOML_MULTILINE_LITERAL_STRING))
}

fn parse_integer(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(TOML_INTEGER_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(TOML_INTEGER_LITERAL, after);
    Present(m.complete(p, TOML_INTEGER))
}

fn parse_float(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(TOML_FLOAT_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(TOML_FLOAT_LITERAL, after);
    Present(m.complete(p, TOML_FLOAT))
}

fn parse_boolean(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at_ts(BOOLEAN_LITERALS) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts_with_context(BOOLEAN_LITERALS, after);
    Present(m.complete(p, TOML_BOOLEAN))
}

// 1979-05-27 or 1979-05-27T07:32:00Z
fn parse_date(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(TOML_DATE_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(TOML_DATE_LITERAL, after);
    Present(m.complete(p, TOML_DATE))
}

// 07:32:00
fn parse_time(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(TOML_TIME_LITERAL) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(TOML_TIME_LITERAL, after);
    Present(m.complete(p, TOML_TIME))
}

/// Wraps a token that the lexer couldn't read as a value, and has already
/// reported, into a bogus value.
fn parse_invalid_value(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(ERROR_TOKEN) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(ERROR_TOKEN, after);
    Present(m.complete(p, TOML_BOGUS_VALUE))
}

// [1, 2, 3]
fn parse_array(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T!['['], TomlLexContext::Value);
    ArrayElementList.parse_list(p);
    expect_with_context(p, T![']'], after);

    Present(m.complete(p, TOML_ARRAY))
}

/// Returns `true` if the current value-like token on a new line is the start
/// of a member or a table header that follows an unterminated array.
///
/// The current token is lexed again as a key to scan for a member
/// (`key.fragments =`) or a table header (`[key]` or `[[key]]` followed by a line
/// break). The check only succeeds where valid TOML can't continue the array:
/// an element is always followed by `,`, `]`, or a comment, never by `=`, and
/// a valid element such as `[[2]]` that ends a line is followed by `,` or `]`.
fn is_at_line_item(p: &mut TomlParser) -> bool {
    let first = p.re_lex(TomlLexContext::Key);
    let is_line_item = match first {
        T!['['] | T!["[["] => is_at_table_header(p, first),
        _ if KEY_FRAGMENT_START.contains(first) => {
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
        if (n > 0 && p.has_nth_preceding_line_break(n)) || !p.nth_at_ts(n, KEY_FRAGMENT_START) {
            return None;
        }
        n += 1;
        if !p.nth_at(n, T![.]) || p.has_nth_preceding_line_break(n) {
            return Some(n);
        }
        n += 1;
    }
}

struct ArrayElementList;

impl ParseSeparatedList for ArrayElementList {
    type Kind = TomlSyntaxKind;
    type Parser<'source> = TomlParser<'source>;
    const LIST_KIND: Self::Kind = TOML_ARRAY_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_value(p, TomlLexContext::Value)
    }

    /// Ends the list at `]`, or at a `}` that leaves the array unterminated
    /// and closes an enclosing inline table.
    ///
    /// The array is also most likely unterminated if a token on a new line
    /// can't continue it: anything but a `,` after an element, or anything but
    /// a value after a `,`. That token starts the next table header or
    /// member.
    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        if p.at_ts(CLOSING_DELIMITERS) {
            return true;
        }
        if !p.has_preceding_line_break() || p.at(T![,]) {
            return false;
        }
        let expects_element = matches!(p.last(), Some(T!['['] | T![,]));
        !expects_element || !p.at_ts(VALUE_START) || is_at_line_item(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &ArrayElementListParseRecovery, expected_value)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
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

/// Skips an invalid array element into a bogus value, up to the next
/// separator, closing delimiter, or line break.
struct ArrayElementListParseRecovery;

impl ParseRecovery for ArrayElementListParseRecovery {
    type Kind = TomlSyntaxKind;
    type Parser<'source> = TomlParser<'source>;
    const RECOVERED_KIND: Self::Kind = TOML_BOGUS_VALUE;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(LIST_RECOVERY_SET) || p.has_preceding_line_break()
    }

    fn recover(&self, p: &mut Self::Parser<'_>) -> RecoveryResult {
        if p.at(EOF) {
            return Err(RecoveryError::Eof);
        }

        if self.is_at_recovered(p) {
            return Err(RecoveryError::AlreadyRecovered);
        }

        if p.is_speculative_parsing() {
            return Err(RecoveryError::RecoveryDisabled);
        }

        let m = p.start();

        // The default recovery lexes the skipped tokens as keys, which would
        // read the `]]` that closes two nested arrays as a single token.
        while !(self.is_at_recovered(p) || p.at(EOF)) {
            p.bump_any_with_context(TomlLexContext::Value);
        }

        Ok(m.complete(p, Self::RECOVERED_KIND))
    }
}

// { a = 1, b = 2 }
fn parse_inline_table(p: &mut TomlParser, after: TomlLexContext) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T!['{'], TomlLexContext::Key);
    InlineTableMemberList.parse_list(p);
    expect_with_context(p, T!['}'], after);

    Present(m.complete(p, TOML_INLINE_TABLE))
}

struct InlineTableMemberList;

impl ParseSeparatedList for InlineTableMemberList {
    type Kind = TomlSyntaxKind;
    type Parser<'source> = TomlParser<'source>;
    const LIST_KIND: Self::Kind = TOML_INLINE_TABLE_MEMBER_LIST;

    /// Parses a member, whose value is followed by a token lexed as a value, so
    /// the `]]` that closes two enclosing arrays is two tokens.
    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        // A member that doesn't follow `{` or `,` follows the value of the
        // previous member, as in `{ a = 1 b = 2 }`.
        if !matches!(p.last(), Some(T!['{'] | T![,])) {
            p.re_lex(TomlLexContext::Key);
        }
        parse_member(p, TomlLexContext::Value)
    }

    /// Ends the list at `}`, or at a `]` that leaves the inline table
    /// unterminated and closes an enclosing array.
    ///
    /// The inline table is also most likely unterminated if a token on a new
    /// line can't continue it: anything but a `,` after a member, or a table
    /// header.
    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        if p.at_ts(CLOSING_DELIMITERS) {
            return true;
        }
        if !p.has_preceding_line_break() || p.at(T![,]) {
            return false;
        }
        let expects_member = matches!(p.last(), Some(T!['{'] | T![,]));
        !expects_member || p.at_ts(TABLE_HEADER_START)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(TOML_BOGUS_MEMBER, LIST_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_member,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        expect_with_context(p, T![,], TomlLexContext::Key)
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }

    fn diagnose_missing_element(&mut self, p: &mut Self::Parser<'_>) {
        let range = p.cur_range();
        p.error(expected_member(p, range));
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
