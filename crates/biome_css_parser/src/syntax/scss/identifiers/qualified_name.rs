use crate::parser::CssParser;
use crate::syntax::{is_nth_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::SCSS_QUALIFIED_NAME;
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

use super::{is_at_scss_identifier, parse_scss_identifier};

/// Detects `module.member` or `module.$var` so module-qualified
/// values/functions are recognized before plain identifiers.
///
/// Example:
/// ```scss
/// math.$pi
/// ```
///
/// Docs: https://sass-lang.com/documentation/modules
#[inline]
pub(crate) fn is_at_scss_qualified_name(p: &mut CssParser) -> bool {
    is_nth_at_scss_qualified_name(p, 0)
}

/// Detects a qualified name starting `n` tokens ahead.
///
/// Example:
/// ```scss
/// math.pow(2, 3)
/// ```
///
/// Docs: https://sass-lang.com/documentation/modules
#[inline]
pub(crate) fn is_nth_at_scss_qualified_name(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_identifier(p, n)
        && p.nth_at(n + 1, T![.])
        && ((p.nth_at(n + 2, T![$]) && is_nth_at_identifier(p, n + 3))
            || is_nth_at_identifier(p, n + 2))
}

/// Parses a module-qualified name, preserving whether the member is a `$var` or
/// a plain identifier for later resolution.
///
/// Example:
/// ```scss
/// math.pow(2, 3)
/// ```
///
/// Docs: https://sass-lang.com/documentation/modules
#[inline]
pub(crate) fn parse_scss_qualified_name(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_qualified_name(p) {
        return Absent;
    }

    let m = p.start();
    parse_regular_identifier(p).ok();
    p.expect(T![.]);

    if is_at_scss_identifier(p) {
        parse_scss_identifier(p).ok();
    } else {
        parse_regular_identifier(p).ok();
    }

    Present(m.complete(p, SCSS_QUALIFIED_NAME))
}
