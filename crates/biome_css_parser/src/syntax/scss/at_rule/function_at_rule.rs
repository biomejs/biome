use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::parse_regular_identifier;
use crate::syntax::scss::at_rule::parameter::parse_scss_parameter_list;
use biome_css_syntax::CssSyntaxKind::SCSS_FUNCTION_AT_RULE;
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

/// Parses the SCSS `@function` at-rule.
///
/// # Example
///
/// ```scss
/// @function double($value: 2) {
///   @return $value * 2;
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/function/
#[inline]
pub(crate) fn parse_scss_function_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_function_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![function]);
    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    // The parameter list is optional in the grammar, so a missing `(` is valid.
    parse_scss_parameter_list(p).ok();
    parse_declaration_or_rule_list_block(p);

    Present(m.complete(p, SCSS_FUNCTION_AT_RULE))
}

#[inline]
fn is_at_scss_function_at_rule(p: &mut CssParser) -> bool {
    p.at(T![function])
}
