use crate::parser::CssParser;
use crate::syntax::scss::{
    expected_scss_expression, is_at_scss_identifier, parse_scss_expression_until,
    parse_scss_identifier,
};
use biome_css_syntax::CssSyntaxKind::{
    self, CSS_BOGUS_PARAMETER, SCSS_PARAMETER, SCSS_PARAMETER_DEFAULT_VALUE,
    SCSS_PARAMETER_ITEM_LIST, SCSS_PARAMETER_LIST,
};
use biome_css_syntax::T;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{Parser, TokenSet, token_set};

const SCSS_PARAMETER_DEFAULT_VALUE_END_SET: TokenSet<CssSyntaxKind> =
    token_set![T![,], T![')'], T![...]];
const SCSS_PARAMETER_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T![,], T![')'], T!['{']];

/// Parses the SCSS parameter list used by `@mixin` and `@function`.
///
/// # Example
///
/// ```scss
/// @mixin button($radius: 4px, $args...) {}
///              ^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/mixin/
#[inline]
pub(super) fn parse_scss_parameter_list(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_parameter_list(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    ScssParameterItemList.parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, SCSS_PARAMETER_LIST))
}

#[inline]
fn is_at_scss_parameter_list(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
fn parse_scss_parameter(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_identifier(p) {
        return Absent;
    }

    let m = p.start();

    // We only enter this branch after `is_at_scss_identifier`, so `Absent` is impossible here.
    parse_scss_identifier(p).ok();
    // The default value is optional in the grammar, so `Absent` is expected when `:` is missing.
    parse_scss_parameter_default_value(p).ok();

    p.eat(T![...]);

    Present(m.complete(p, SCSS_PARAMETER))
}

/// Parses the optional default-value clause of an SCSS parameter.
///
/// # Example
///
/// ```scss
/// @mixin button($radius: 4px, $args...) {}
///                       ^^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/mixin/
#[inline]
fn parse_scss_parameter_default_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_parameter_default_value(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![:]);
    parse_scss_expression_until(p, SCSS_PARAMETER_DEFAULT_VALUE_END_SET)
        .or_add_diagnostic(p, expected_scss_expression);

    Present(m.complete(p, SCSS_PARAMETER_DEFAULT_VALUE))
}

#[inline]
fn is_at_scss_parameter_default_value(p: &mut CssParser) -> bool {
    p.at(T![:])
}

#[inline]
fn expected_scss_parameter(p: &CssParser, range: biome_rowan::TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a parameter.", range)
        .with_hint("Add a parameter like `$value` or remove the extra separator.")
}

struct ScssParameterItemListParseRecovery;

impl ParseRecovery for ScssParameterItemListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_PARAMETER;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(SCSS_PARAMETER_RECOVERY_SET)
    }
}

struct ScssParameterItemList;

impl ParseSeparatedList for ScssParameterItemList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_PARAMETER_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_scss_parameter(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ScssParameterItemListParseRecovery,
            expected_scss_parameter,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}
