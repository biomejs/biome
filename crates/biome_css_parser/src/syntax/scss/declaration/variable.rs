use super::super::{is_at_scss_identifier, parse_scss_identifier};
use crate::parser::CssParser;
use crate::syntax::parse_error::expected_scss_expression;
use crate::syntax::scss::parse_scss_expression_until;
use crate::syntax::{is_at_identifier, is_nth_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::{
    EOF, SCSS_DECLARATION, SCSS_NAMESPACED_IDENTIFIER, SCSS_VARIABLE_MODIFIER,
    SCSS_VARIABLE_MODIFIER_LIST,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::diagnostic::expected_token_any;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{RecoveryError, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, TokenSet, token_set};

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

/// Parses a SCSS variable declaration, including trailing `!default`/`!global`.
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
    p.expect(T![:]);

    parse_scss_expression_until(p, token_set![T![!], T![;], T!['}']])
        .or_add_diagnostic(p, expected_scss_expression);
    ScssVariableModifierList.parse_list(p);

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
fn is_at_scss_namespaced_identifier(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
        && p.nth_at(1, T![.])
        && p.nth_at(2, T![$])
        && is_nth_at_identifier(p, 3)
}

#[inline]
fn parse_scss_namespaced_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_namespaced_identifier(p) {
        return Absent;
    }

    let m = p.start();
    parse_regular_identifier(p).ok();
    p.expect(T![.]);
    parse_scss_identifier(p).ok();
    Present(m.complete(p, SCSS_NAMESPACED_IDENTIFIER))
}

#[inline]
fn parse_scss_declaration_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_namespaced_identifier(p) {
        parse_scss_namespaced_identifier(p)
    } else {
        parse_scss_identifier(p)
    }
}

#[inline]
fn is_at_scss_variable_modifier(p: &mut CssParser) -> bool {
    p.at(T![!])
}

const SCSS_VARIABLE_MODIFIER_SET: TokenSet<CssSyntaxKind> = token_set!(T![default], T![global]);

#[inline]
fn parse_scss_variable_modifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_variable_modifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![!]);

    if p.at_ts(SCSS_VARIABLE_MODIFIER_SET) {
        p.bump_ts(SCSS_VARIABLE_MODIFIER_SET);
    } else {
        p.error(expected_token_any(&[T![default], T![global]]));
        if is_at_identifier(p) {
            p.bump_any();
        }
    }

    Present(m.complete(p, SCSS_VARIABLE_MODIFIER))
}

struct ScssVariableModifierList;

impl ParseNodeList for ScssVariableModifierList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_VARIABLE_MODIFIER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_scss_variable_modifier(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        !is_at_scss_variable_modifier(p)
    }

    fn recover(
        &mut self,
        _p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        match parsed_element {
            Absent => Err(RecoveryError::AlreadyRecovered),
            Present(m) => Ok(m),
        }
    }
}
