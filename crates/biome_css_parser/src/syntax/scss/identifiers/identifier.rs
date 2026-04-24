use crate::parser::CssParser;
use crate::syntax::{is_nth_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::{SCSS_NAMESPACED_VARIABLE, SCSS_VARIABLE};
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

/// Checks for a Sass variable (`$name`).
///
/// Example:
/// ```scss
/// $color: red;
/// ```
///
/// Docs: https://sass-lang.com/documentation/variables
#[inline]
pub(crate) fn is_at_scss_variable(p: &mut CssParser) -> bool {
    p.at(T![$]) && is_nth_at_identifier(p, 1)
}

/// Detects module-qualified variables (`module.$name`) from the Sass module
/// system.
///
/// Example:
/// ```scss
/// math.$pi
/// ```
///
/// Docs: https://sass-lang.com/documentation/modules
#[inline]
pub(crate) fn is_at_scss_namespaced_variable(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
        && p.nth_at(1, T![.])
        && p.nth_at(2, T![$])
        && is_nth_at_identifier(p, 3)
}

/// Parses `$name` as an `SCSS_VARIABLE` node.
///
/// Example:
/// ```scss
/// $spacing: 1rem;
/// ```
///
/// Docs: https://sass-lang.com/documentation/variables
#[inline]
pub(crate) fn parse_scss_variable(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_variable(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![$]);
    parse_regular_identifier(p).ok();
    Present(m.complete(p, SCSS_VARIABLE))
}

/// Parses `module.$name` as a single AST node for module scoping.
///
/// Example:
/// ```scss
/// color.$red
/// ```
///
/// Docs: https://sass-lang.com/documentation/modules
#[inline]
pub(crate) fn parse_scss_namespaced_variable(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_namespaced_variable(p) {
        return Absent;
    }

    let m = p.start();
    parse_regular_identifier(p).ok();
    p.expect(T![.]);
    parse_scss_variable(p).ok();
    Present(m.complete(p, SCSS_NAMESPACED_VARIABLE))
}
