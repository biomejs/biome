use crate::parser::CssParser;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax;

use super::parse_scss_expression_at_rule;

/// Parses the SCSS `@debug` at-rule.
///
/// # Example
///
/// ```scss
/// @debug $width * 2;
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/debug/
#[inline]
pub(crate) fn parse_scss_debug_at_rule(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_expression_at_rule(p, T![debug], CssSyntaxKind::SCSS_DEBUG_AT_RULE)
}
