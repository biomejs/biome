use super::module_clauses::{
    expected_scss_module_configuration, parse_scss_module_configuration_list,
};
use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::parse_error::expected_string;
use crate::syntax::{parse_regular_identifier, parse_string};
use biome_css_syntax::CssSyntaxKind::{
    SCSS_USE_ALL_NAMESPACE, SCSS_USE_AS_CLAUSE, SCSS_USE_AT_RULE,
};
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

/// Parses the SCSS `@use` at-rule.
///
/// # Example
///
/// ```scss
/// @use "sass:math" as math;
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/use/
#[inline]
pub(crate) fn parse_scss_use_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_use_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![use]);
    parse_string(p).or_add_diagnostic(p, expected_string);
    // The `as` clause is optional in the grammar.
    parse_scss_use_as_clause(p).ok();
    // The `with` clause is optional in the grammar.
    parse_scss_with_clause(p).ok();
    p.expect(T![;]);

    Present(m.complete(p, SCSS_USE_AT_RULE))
}

#[inline]
fn is_at_scss_use_at_rule(p: &mut CssParser) -> bool {
    p.at(T![use])
}

/// Parses the optional SCSS `as` clause inside `@use`.
///
/// # Example
///
/// ```scss
/// @use "sass:math" as math;
///                   ^^^^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/use/#choosing-a-namespace
#[inline]
fn parse_scss_use_as_clause(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_use_as_clause(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![as]);
    parse_scss_use_namespace(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, SCSS_USE_AS_CLAUSE))
}

#[inline]
fn is_at_scss_use_as_clause(p: &mut CssParser) -> bool {
    p.at(T![as])
}

/// Parses the namespace value for an SCSS `@use ... as ...` clause.
///
/// # Example
///
/// ```scss
/// @use "sass:math" as math;
///                      ^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/use/#choosing-a-namespace
#[inline]
fn parse_scss_use_namespace(p: &mut CssParser) -> ParsedSyntax {
    if p.at(T![*]) {
        let m = p.start();
        p.bump(T![*]);
        Present(m.complete(p, SCSS_USE_ALL_NAMESPACE))
    } else {
        parse_regular_identifier(p)
    }
}

/// Parses the optional SCSS `with (...)` clause used by `@use` and `@forward`.
///
/// # Example
///
/// ```scss
/// @use "theme" with ($spacing: 4px, $radius: 8px);
///              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/use/#configuration
#[inline]
pub(super) fn parse_scss_with_clause(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_with_clause(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![with]);
    parse_scss_module_configuration_list(p)
        .or_add_diagnostic(p, expected_scss_module_configuration);

    Present(m.complete(p, biome_css_syntax::CssSyntaxKind::SCSS_WITH_CLAUSE))
}
#[inline]
pub(super) fn is_at_scss_with_clause(p: &mut CssParser) -> bool {
    p.at(T![with])
}
