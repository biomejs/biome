use crate::parser::CssParser;
use crate::syntax::scss::{
    expected_scss_expression, is_at_scss_identifier, parse_scss_expression_in_variable_value_until,
    parse_scss_identifier,
};
use crate::syntax::{is_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::{
    self, CSS_BOGUS, EOF, SCSS_MODULE_CONFIGURATION, SCSS_MODULE_CONFIGURATION_ITEM_LIST,
    SCSS_MODULE_CONFIGURATION_LIST, SCSS_MODULE_MEMBER_LIST, SCSS_VARIABLE_MODIFIER,
};
use biome_css_syntax::T;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{Parser, TokenSet, token_set};
use biome_rowan::TextRange;

const SCSS_MODULE_CONFIGURATION_VALUE_END_SET: TokenSet<CssSyntaxKind> = token_set![T![,], T![')']];
const SCSS_MODULE_CONFIGURATION_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T![,], T![')']];
const SCSS_MODULE_MEMBER_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    token_set![T![,], T![with], T![;], EOF];

/// Parses the SCSS module configuration list used by `with (...)` clauses.
///
/// # Example
///
/// ```scss
/// @use "theme" with ($spacing: 4px, $radius: 8px);
///                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/use/#configuration
#[inline]
pub(super) fn parse_scss_module_configuration_list(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_module_configuration_list(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    parse_scss_module_configuration_item_list(p);
    p.expect(T![')']);

    Present(m.complete(p, SCSS_MODULE_CONFIGURATION_LIST))
}

#[inline]
fn is_at_scss_module_configuration_list(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
fn parse_scss_module_configuration_item_list(p: &mut CssParser) {
    ScssModuleConfigurationItemList.parse_list(p);
}

/// Parses a single SCSS module configuration item inside `with (...)`.
///
/// # Example
///
/// ```scss
/// @use "theme" with ($spacing: 4px !default, $radius: 8px);
///                    ^^^^^^^^^^^^^^^^^^^^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/use/#configuration
#[inline]
fn parse_scss_module_configuration(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_identifier(p) {
        return Absent;
    }

    let m = p.start();

    // We only enter this branch after `is_at_scss_identifier`, so `Absent` is impossible here.
    parse_scss_identifier(p).ok();
    p.expect(T![:]);
    parse_scss_expression_in_variable_value_until(p, SCSS_MODULE_CONFIGURATION_VALUE_END_SET)
        .or_add_diagnostic(p, expected_scss_expression);
    // Optional by grammar.
    parse_scss_module_configuration_modifier(p).ok();

    Present(m.complete(p, SCSS_MODULE_CONFIGURATION))
}

/// Parses the optional `!default` modifier after a SCSS module configuration value.
///
/// # Example
///
/// ```scss
/// @use "theme" with ($primary: blue !default);
///                                  ^^^^^^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/forward/#configuring-modules
#[inline]
fn parse_scss_module_configuration_modifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_module_configuration_modifier_start(p) {
        return Absent;
    }

    let bang_range = p.cur_range();
    let m = p.start();
    p.bump(T![!]);

    if p.at(T![default]) {
        p.bump(T![default]);
    } else {
        let range = TextRange::new(bang_range.start(), p.cur_range().end());
        p.error(
            p.err_builder(
                "Expected `!default` after a module configuration value.",
                range,
            )
            .with_hint(
                "Only `!default` is allowed in `@use` and `@forward` configuration clauses.",
            ),
        );

        if !p.at_ts(SCSS_MODULE_CONFIGURATION_VALUE_END_SET) {
            p.bump_any();
        }
    }

    Present(m.complete(p, SCSS_VARIABLE_MODIFIER))
}

#[inline]
fn is_at_scss_module_configuration_modifier_start(p: &mut CssParser) -> bool {
    p.at(T![!])
}

/// Parses the SCSS module member list used by `show` and `hide` clauses.
///
/// # Example
///
/// ```scss
/// @forward "theme" show $color, mixin-name;
///                       ^^^^^^^^^^^^^^^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/forward/
#[inline]
pub(super) fn parse_scss_module_member_list(p: &mut CssParser) -> ParsedSyntax {
    Present(ScssModuleMemberList.parse_list(p))
}

#[inline]
fn parse_scss_module_member(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_identifier(p) {
        parse_scss_identifier(p)
    } else if is_at_identifier(p) {
        parse_regular_identifier(p)
    } else {
        Absent
    }
}

#[inline]
pub(super) fn expected_scss_module_configuration(
    p: &CssParser,
    range: TextRange,
) -> ParseDiagnostic {
    p.err_builder("Expected a module configuration.", range)
        .with_hint("Add a configuration like `$name: value` or remove the extra separator.")
}

#[inline]
pub(super) fn expected_scss_module_member(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a module member.", range)
        .with_hint("Add a member like `$name` or `mixin-name` here.")
}

struct ScssModuleMemberListRecovery;

impl ParseRecovery for ScssModuleMemberListRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(SCSS_MODULE_MEMBER_RECOVERY_SET)
    }
}

struct ScssModuleMemberList;

impl ParseSeparatedList for ScssModuleMemberList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_MODULE_MEMBER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_scss_module_member(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![with]) || p.at(T![;]) || p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ScssModuleMemberListRecovery,
            expected_scss_module_member,
        )
    }

    fn allow_empty(&self) -> bool {
        false
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        false
    }

    fn diagnose_missing_element(&mut self, p: &mut Self::Parser<'_>) {
        p.error(expected_scss_module_member(p, p.cur_range()));
    }
}

struct ScssModuleConfigurationItemListRecovery;

impl ParseRecovery for ScssModuleConfigurationItemListRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(SCSS_MODULE_CONFIGURATION_RECOVERY_SET)
    }
}

struct ScssModuleConfigurationItemList;

impl ParseSeparatedList for ScssModuleConfigurationItemList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_MODULE_CONFIGURATION_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_scss_module_configuration(p)
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
            &ScssModuleConfigurationItemListRecovery,
            expected_scss_module_configuration,
        )
    }

    fn allow_empty(&self) -> bool {
        false
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        false
    }
}
