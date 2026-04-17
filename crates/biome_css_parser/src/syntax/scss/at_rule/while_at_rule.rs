use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::scss::{expected_scss_expression, parse_scss_expression_until};
use biome_css_syntax::CssSyntaxKind::SCSS_WHILE_AT_RULE;
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{TokenSet, token_set};

const SCSS_WHILE_CONDITION_END_SET: TokenSet<biome_css_syntax::CssSyntaxKind> = token_set![T!['{']];

/// Parses the SCSS `@while` at-rule.
///
/// # Example
///
/// ```scss
/// @while $n > 1 {
///   $n: $n - 1;
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/control/while/
#[inline]
pub(crate) fn parse_scss_while_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_while_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![while]);
    parse_scss_expression_until(p, SCSS_WHILE_CONDITION_END_SET)
        .or_add_diagnostic(p, expected_scss_expression);

    parse_scss_while_block(p);

    Present(m.complete(p, SCSS_WHILE_AT_RULE))
}

#[inline]
fn is_at_scss_while_at_rule(p: &mut CssParser) -> bool {
    p.at(T![while])
}

#[inline]
fn parse_scss_while_block(p: &mut CssParser) {
    parse_declaration_or_rule_list_block(p);
}
