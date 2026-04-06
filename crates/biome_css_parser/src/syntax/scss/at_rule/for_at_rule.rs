use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::scss::{
    expected_scss_expression, parse_scss_expression_until, parse_scss_identifier,
};
use biome_css_syntax::CssSyntaxKind::{self, SCSS_FOR_AT_RULE};
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{TokenSet, token_set};

const SCSS_FOR_LOWER_BOUND_END_SET: TokenSet<CssSyntaxKind> =
    token_set![T![to], T![through], T!['{']];
const SCSS_FOR_UPPER_BOUND_END_SET: TokenSet<CssSyntaxKind> = token_set![T!['{']];

/// Parses the SCSS `@for` at-rule.
///
/// # Example
///
/// ```scss
/// @for $i from 1 through 3 {
///   width: $i * 1px;
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/control/for/
#[inline]
pub(crate) fn parse_scss_for_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_for_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![for]);
    parse_scss_identifier(p).or_add_diagnostic(p, expected_scss_for_binding);
    p.expect(T![from]);
    parse_scss_expression_until(p, SCSS_FOR_LOWER_BOUND_END_SET)
        .or_add_diagnostic(p, expected_scss_expression);
    parse_scss_for_range_operator(p);
    parse_scss_expression_until(p, SCSS_FOR_UPPER_BOUND_END_SET)
        .or_add_diagnostic(p, expected_scss_expression);
    parse_declaration_or_rule_list_block(p);

    Present(m.complete(p, SCSS_FOR_AT_RULE))
}

#[inline]
fn is_at_scss_for_at_rule(p: &mut CssParser) -> bool {
    p.at(T![for])
}

#[inline]
fn parse_scss_for_range_operator(p: &mut CssParser) {
    if p.at(T![to]) || p.at(T![through]) {
        p.bump_any();
    } else {
        p.error(expected_scss_for_range_operator(p, p.cur_range()));
    }
}

#[inline]
fn expected_scss_for_binding(p: &CssParser, range: biome_rowan::TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a variable binding after `@for`.", range)
        .with_hint("Add a variable like `$i` before `from`.")
}

#[inline]
fn expected_scss_for_range_operator(
    p: &CssParser,
    range: biome_rowan::TextRange,
) -> ParseDiagnostic {
    p.err_builder("Expected `to` or `through` in the `@for` range.", range)
        .with_hint("Use `to` or `through` between the lower and upper bounds.")
}
