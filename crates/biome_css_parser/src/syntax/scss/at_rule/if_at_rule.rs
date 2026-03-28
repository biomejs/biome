use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::scss::at_rule::else_clause::parse_scss_else_clause;
use crate::syntax::scss::{expected_scss_expression, parse_scss_expression_until};
use biome_css_syntax::CssSyntaxKind::{self, SCSS_IF_AT_RULE};
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{TokenSet, token_set};

const SCSS_IF_CONDITION_END_SET: TokenSet<CssSyntaxKind> = token_set![T!['{']];

/// Parses the SCSS `@if` at-rule with an optional chained `@else` clause.
///
/// # Example
///
/// ```scss
/// @if $theme == dark {
///   color: white;
/// } @else {
///   color: black;
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/control/if/
#[inline]
pub(crate) fn parse_scss_if_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_if_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![if]);
    parse_scss_expression_until(p, SCSS_IF_CONDITION_END_SET)
        .or_add_diagnostic(p, expected_scss_expression);
    parse_declaration_or_rule_list_block(p);
    // `@else` is optional after `@if`, so `Absent` is valid here.
    parse_scss_else_clause(p).ok();

    Present(m.complete(p, SCSS_IF_AT_RULE))
}

#[inline]
pub(super) fn is_at_scss_if_at_rule(p: &mut CssParser) -> bool {
    p.at(T![if])
}
