use crate::parser::CssParser;
use crate::syntax::scss::{
    SCSS_IDENT_CONTINUATION_SET, SCSS_STATEMENT_START_SET, SCSS_VARIABLE_MODIFIER_LIST_END_SET,
    expected_scss_variable_modifier,
};
use biome_css_syntax::CssSyntaxKind::{
    CSS_BOGUS, SCSS_VARIABLE_MODIFIER, SCSS_VARIABLE_MODIFIER_LIST,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, TokenSet, token_set};
use biome_rowan::TextRange;

const SCSS_VARIABLE_MODIFIER_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![default], T![global]];

/// Parses trailing SCSS variable modifiers (`!default`, `!global`) after a
/// variable value.
///
/// Example:
/// ```scss
/// $x: 1 !default !global;
/// ```
///
/// Docs: https://sass-lang.com/documentation/variables
#[inline]
pub(crate) fn parse_scss_variable_modifiers(p: &mut CssParser) {
    ScssVariableModifierList.parse_list(p);
}

#[inline]
pub(crate) fn is_at_scss_variable_modifier(p: &mut CssParser) -> bool {
    p.at(T![!]) && !p.nth_at(1, T![important])
}

/// Parses `!default` or `!global` after a variable value and emits a targeted
/// error for any other `!` token to avoid silently accepting invalid modifiers.
///
/// Example:
/// ```scss
/// $x: 1 !global;
/// ```
///
/// Docs: https://sass-lang.com/documentation/variables
#[inline]
fn parse_scss_variable_modifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_variable_modifier(p) {
        return Absent;
    }

    let bang_range = p.cur_range();
    let m = p.start();
    p.bump(T![!]);

    if p.at_ts(SCSS_VARIABLE_MODIFIER_TOKEN_SET) {
        if p.at(T![default]) {
            p.bump(T![default]);
        } else {
            p.bump(T![global]);
        }
    } else {
        let range = TextRange::new(bang_range.start(), p.cur_range().end());
        p.error(expected_scss_variable_modifier(p, range));
        if !p.at_ts(SCSS_VARIABLE_MODIFIER_LIST_END_SET) {
            p.bump_any();
        }
    }

    Present(m.complete(p, SCSS_VARIABLE_MODIFIER))
}

/// Collects trailing variable modifiers so they don't get folded into the value
/// list.
///
/// Example:
/// ```scss
/// $x: 1 !default !global;
/// ```
///
/// Docs: https://sass-lang.com/documentation/variables
struct ScssVariableModifierList;

impl ParseNodeList for ScssVariableModifierList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_VARIABLE_MODIFIER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_scss_variable_modifier(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(SCSS_VARIABLE_MODIFIER_LIST_END_SET) || is_at_scss_statement_boundary(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        match parsed_element {
            Absent => Absent.or_recover(p, &ScssVariableModifierListParseRecovery, |p, range| {
                p.err_builder("Unexpected value or character.", range)
                    .with_hint("Expected a variable modifier or the end of the declaration.")
            }),
            Present(m) => Ok(m),
        }
    }
}

struct ScssVariableModifierListParseRecovery;

impl ParseRecovery for ScssVariableModifierListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![!])
            || p.at_ts(SCSS_VARIABLE_MODIFIER_LIST_END_SET)
            || p.has_preceding_line_break()
            || is_at_scss_statement_boundary(p)
    }
}

#[inline]
fn is_at_scss_statement_boundary(p: &mut CssParser) -> bool {
    p.at_ts(SCSS_STATEMENT_START_SET) || is_at_identifier_started_scss_statement_boundary(p)
}

#[inline]
fn is_at_identifier_started_scss_statement_boundary(p: &mut CssParser) -> bool {
    p.at(T![ident])
        && (p.nth_at(1, T![:])
            || p.nth_at_ts(1, SCSS_IDENT_CONTINUATION_SET)
            || (p.nth_at(1, T![*]) && p.nth_at(2, T![:]))
            || (p.nth_at(1, T![-]) && p.nth_at(2, T![*]) && p.nth_at(3, T![:]))
            || p.nth_at(1, T!['{']))
}
