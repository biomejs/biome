use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::scss::{
    expected_scss_expression, parse_scss_expression_until, parse_scss_identifier,
};
use biome_css_syntax::CssSyntaxKind::{self, CSS_BOGUS, SCSS_EACH_AT_RULE, SCSS_EACH_BINDING_LIST};
use biome_css_syntax::T;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{Parser, TokenSet, token_set};

const SCSS_EACH_ITERABLE_END_SET: TokenSet<CssSyntaxKind> = token_set![T!['{']];
const SCSS_EACH_BINDING_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T![,], T![in], T!['{']];

/// Parses the SCSS `@each` at-rule.
///
/// # Example
///
/// ```scss
/// @each $item in $list {
///   color: $item;
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/control/each/
#[inline]
pub(crate) fn parse_scss_each_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_each_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![each]);

    ScssEachBindingList.parse_list(p);

    p.expect(T![in]);

    parse_scss_expression_until(p, SCSS_EACH_ITERABLE_END_SET)
        .or_add_diagnostic(p, expected_scss_expression);
    parse_declaration_or_rule_list_block(p);

    Present(m.complete(p, SCSS_EACH_AT_RULE))
}

#[inline]
fn is_at_scss_each_at_rule(p: &mut CssParser) -> bool {
    p.at(T![each])
}

#[inline]
fn expected_scss_each_binding(p: &CssParser, range: biome_rowan::TextRange) -> ParseDiagnostic {
    p.err_builder("Expected a variable binding after `@each`.", range)
        .with_hint("Add a variable like `$item` before `in`.")
}

struct ScssEachBindingListParseRecovery;

impl ParseRecovery for ScssEachBindingListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(SCSS_EACH_BINDING_RECOVERY_SET)
    }
}

struct ScssEachBindingList;

impl ParseSeparatedList for ScssEachBindingList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_EACH_BINDING_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_scss_identifier(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![in]) || p.at(T!['{'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ScssEachBindingListParseRecovery,
            expected_scss_each_binding,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        false
    }

    fn allow_empty(&self) -> bool {
        false
    }
}
