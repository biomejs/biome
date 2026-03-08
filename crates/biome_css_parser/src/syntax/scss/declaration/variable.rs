use super::super::{
    is_at_scss_identifier, is_at_scss_namespaced_identifier, parse_scss_identifier,
    parse_scss_namespaced_identifier,
};
use super::variable_modifier::parse_scss_variable_modifiers;
use crate::parser::CssParser;
use crate::syntax::scss::{expected_scss_expression, parse_scss_expression_until};
use biome_css_syntax::CssSyntaxKind::{EOF, SCSS_DECLARATION};
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, token_set};

/// Detects a SCSS variable declaration (including module-qualified variables).
///
/// Example:
/// ```scss
/// $primary: #c00;
/// ```
///
/// Docs: https://sass-lang.com/documentation/variables
#[inline]
pub(crate) fn is_at_scss_declaration(p: &mut CssParser) -> bool {
    if is_at_scss_identifier(p) {
        p.nth_at(2, T![:])
    } else if is_at_scss_namespaced_identifier(p) {
        p.nth_at(4, T![:])
    } else {
        false
    }
}

/// Parses a SCSS variable declaration, including trailing variable modifiers.
///
/// Examples:
/// ```scss
/// $primary: #c00;
/// $spacing: 1rem;
/// ```
///
/// Specification: https://sass-lang.com/documentation/variables
#[inline]
pub(crate) fn parse_scss_declaration(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_declaration(p) {
        return Absent;
    }

    let m = p.start();

    parse_scss_declaration_name(p).ok();
    p.bump(T![:]);

    parse_scss_expression_until(p, token_set![T![!], T![;], T!['}']])
        .or_add_diagnostic(p, expected_scss_expression);
    parse_scss_variable_modifiers(p);

    if !p.at(T!['}']) && !p.at(EOF) {
        if p.nth_at(1, T!['}']) {
            // Allow a trailing `;` before `}` but don't require it.
            p.eat(T![;]);
        } else {
            p.expect(T![;]);
        }
    }

    Present(m.complete(p, SCSS_DECLARATION))
}

#[inline]
fn parse_scss_declaration_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_namespaced_identifier(p) {
        parse_scss_namespaced_identifier(p)
    } else {
        parse_scss_identifier(p)
    }
}
