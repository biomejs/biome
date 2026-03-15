use crate::parser::CssParser;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax;

use super::parse_scss_expression_at_rule;

/// Parses the SCSS `@error` at-rule.
///
/// # Example
///
/// ```scss
/// @error "Invalid state";
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/error/
#[inline]
pub(crate) fn parse_scss_error_at_rule(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_expression_at_rule(p, T![error], CssSyntaxKind::SCSS_ERROR_AT_RULE)
}
