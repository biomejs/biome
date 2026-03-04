use super::super::{is_at_scss_identifier, parse_scss_identifier};
use crate::parser::CssParser;
use crate::syntax::parse_error::expected_scss_expression;
use crate::syntax::scss::parse_scss_expression_until;
use crate::syntax::{is_nth_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::{
    EOF, ERROR_TOKEN, SCSS_DECLARATION, SCSS_NAMESPACED_IDENTIFIER, SCSS_VARIABLE_MODIFIER,
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
    p.expect(T![:]);

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

const SCSS_VARIABLE_MODIFIER_LIST_END_SET: TokenSet<CssSyntaxKind> =
    token_set![T![;], T!['}'], EOF];
const SCSS_VARIABLE_MODIFIER_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![default], T![global], T![important]];

#[inline]
fn parse_scss_variable_modifiers(p: &mut CssParser) {
    ScssVariableModifierList::default().parse_list(p);
    recover_scss_variable_modifier_tail(p);
}

#[inline]
fn recover_scss_variable_modifier_tail(p: &mut CssParser) {
    loop {
        if p.at_ts(SCSS_VARIABLE_MODIFIER_LIST_END_SET) {
            return;
        }

        if p.at(T![!]) {
            parse_scss_variable_modifier(p).ok();
            continue;
        }

        if p.at(ERROR_TOKEN) {
            p.bump_any();
            continue;
        }

        // Recover malformed modifier separators only when they directly precede
        // another modifier (`bar !global`). Otherwise leave the token for
        // missing-semicolon recovery at declaration level.
        if p.nth_at(1, T![!]) {
            let range = p.cur_range();
            p.error(
                p.err_builder("Unexpected value or character.", range)
                    .with_hint("Expected a variable modifier or the end of the declaration."),
            );
            p.bump_any();
            continue;
        }

        return;
    }
}

#[inline]
fn parse_scss_variable_modifier(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![!]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![!]);

    if p.at_ts(SCSS_VARIABLE_MODIFIER_TOKEN_SET) {
        p.bump_ts(SCSS_VARIABLE_MODIFIER_TOKEN_SET);
    } else {
        p.error(expected_token_any(&[
            T![default],
            T![global],
            T![important],
        ]));
        if !p.at_ts(SCSS_VARIABLE_MODIFIER_LIST_END_SET) {
            p.bump_any();
        }
    }

    Present(m.complete(p, SCSS_VARIABLE_MODIFIER))
}

#[derive(Default)]
struct ScssVariableModifierList;

impl ParseNodeList for ScssVariableModifierList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_VARIABLE_MODIFIER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_scss_variable_modifier(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(SCSS_VARIABLE_MODIFIER_LIST_END_SET) || !p.at(T![!])
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
