use crate::parser::CssParser;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax;

use super::parse_scss_expression_at_rule;

/// Parses the SCSS `@warn` at-rule.
///
/// # Example
///
/// ```scss
/// @warn "deprecated API";
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/warn/
#[inline]
pub(crate) fn parse_scss_warn_at_rule(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_expression_at_rule(p, T![warn], CssSyntaxKind::SCSS_WARN_AT_RULE)
}
