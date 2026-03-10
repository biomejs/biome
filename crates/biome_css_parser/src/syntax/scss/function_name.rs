use crate::parser::CssParser;
use crate::syntax::parse_regular_identifier;
use biome_parser::prelude::ParsedSyntax;

use super::{is_at_scss_qualified_name, parse_scss_qualified_name};

/// Parses a function name that may be module-qualified, preserving module
/// scoping.
///
/// Example:
/// ```scss
/// color.adjust($c, $lightness: 10%);
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/use
#[inline]
pub(crate) fn parse_scss_function_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_qualified_name(p) {
        parse_scss_qualified_name(p)
    } else {
        parse_regular_identifier(p)
    }
}
