use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::scss::at_rule::if_at_rule::{is_at_scss_if_at_rule, parse_scss_if_at_rule};
use biome_css_syntax::CssSyntaxKind::{CSS_BOGUS_AT_RULE, SCSS_ELSE_CLAUSE};
use biome_css_syntax::T;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

/// Parses an SCSS `@else` clause that follows an `@if` block.
///
/// # Example
///
/// ```scss
/// @else {
///   color: red;
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/control/if/
#[inline]
pub(super) fn parse_scss_else_clause(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_else_clause(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![@]);
    p.bump(T![else]);
    parse_scss_else_body(p).or_add_diagnostic(p, expected_scss_else_body);

    Present(m.complete(p, SCSS_ELSE_CLAUSE))
}

#[inline]
fn is_at_scss_else_clause(p: &mut CssParser) -> bool {
    p.at(T![@]) && p.nth_at(1, T![else])
}

/// Parses an orphan SCSS `@else` as a bogus at-rule.
///
/// # Example
///
/// ```scss
/// @else {
///   color: red;
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/control/if/
#[inline]
pub(crate) fn parse_bogus_scss_else_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_else_at_rule(p) {
        return Absent;
    }

    let m = p.start();
    let range = p.cur_range();

    p.bump(T![else]);
    p.error(
        p.err_builder("Unexpected `@else` without a preceding `@if`.", range)
            .with_hint("Move this `@else` after an `@if` block."),
    );
    parse_scss_else_body(p).or_add_diagnostic(p, expected_scss_else_body);

    Present(m.complete(p, CSS_BOGUS_AT_RULE))
}

#[inline]
fn is_at_scss_else_at_rule(p: &mut CssParser) -> bool {
    p.at(T![else])
}

#[inline]
fn parse_scss_else_body(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_if_at_rule(p) {
        parse_scss_if_at_rule(p)
    } else if p.at(T!['{']) {
        Present(parse_declaration_or_rule_list_block(p))
    } else {
        Absent
    }
}

#[inline]
fn expected_scss_else_body(p: &CssParser, range: biome_rowan::TextRange) -> ParseDiagnostic {
    p.err_builder("Expected `if` or a block after `@else`.", range)
        .with_hint("Add `if <condition>` or `{ ... }` after `@else`.")
}
