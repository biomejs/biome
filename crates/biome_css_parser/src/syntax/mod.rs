mod at_rule;
mod blocks;
mod css_dimension;
mod parse_error;
mod selector;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::at_rule::{at_at_rule, parse_at_rule};
use crate::syntax::blocks::parse_or_recover_declaration_list_block;
use crate::syntax::css_dimension::{is_at_any_dimension, parse_any_dimension};
use crate::syntax::parse_error::expected_any_rule;
use crate::syntax::parse_error::expected_expression;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::selector::CssSelectorList;
use crate::syntax::selector::{is_at_selector, CssSubSelectorList};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

use self::parse_error::{expected_component_value, expected_declaration_item, expected_number};

const RULE_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![
    T![#],
    T![.],
    T![*],
    T![ident],
    T![:],
    T![::],
    T!['{'],
    T![@]
];
const SELECTOR_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{'], T!['}'],];
const BODY_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    SELECTOR_LIST_RECOVERY_SET.union(RULE_RECOVERY_SET);
const BINARY_OPERATION_TOKEN: TokenSet<CssSyntaxKind> = token_set![T![+], T![-], T![*], T![/]];

pub(crate) fn parse_root(p: &mut CssParser) {
    let m = p.start();
    p.eat(UNICODE_BOM);

    CssRuleList::new(EOF).parse_list(p);

    m.complete(p, CSS_ROOT);
}

struct CssRuleList {
    end_kind: CssSyntaxKind,
}

impl CssRuleList {
    fn new(end_kind: CssSyntaxKind) -> Self {
        Self { end_kind }
    }
}

// TODO: better recovery set. may be we need to pass function instead of token set
const CSS_RULE_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    token_set!(T![@], T![&], T![|], T![*], T![ident]).union(CssSubSelectorList::START_SET);
impl ParseNodeList for CssRuleList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_RULE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if at_at_rule(p) {
            parse_at_rule(p)
        } else if is_at_rule(p) {
            parse_rule(p)
        } else {
            Absent
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(self.end_kind)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS_RULE, CSS_RULE_LIST_RECOVERY_SET),
            expected_any_rule,
        )
    }
}

#[inline]
pub(crate) fn is_at_rule(p: &mut CssParser) -> bool {
    is_at_selector(p)
}

#[inline]
pub(crate) fn parse_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    CssSelectorList::default().parse_list(p);

    let kind = if parse_or_recover_declaration_list_block(p).is_ok() {
        CSS_RULE
    } else {
        CSS_BOGUS_RULE
    };

    Present(m.complete(p, kind))
}

pub(crate) struct CssDeclarationList;

impl ParseSeparatedList for CssDeclarationList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_DECLARATION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_declaration(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, token_set!(T!['}'])),
            expected_declaration_item,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![;]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

struct ListOfComponentValues;
impl ParseNodeList for ListOfComponentValues {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_any_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        !is_at_any_value(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS_COMPONENT_VALUE, token_set!(T!['}'], T![;])),
            expected_component_value,
        )
    }
}
#[inline]
pub(crate) fn parse_declaration(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }
    let m = p.start();
    parse_regular_identifier(p).ok();

    p.expect(T![:]);

    ListOfComponentValues.parse_list(p);

    parse_declaration_important(p).ok();
    Present(m.complete(p, CSS_DECLARATION))
}

#[inline]
pub(crate) fn parse_declaration_with_semicolon(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();

    parse_declaration(p).ok();
    p.expect(T![;]);

    Present(m.complete(p, CSS_DECLARATION_WITH_SEMICOLON))
}

#[inline]
fn is_at_declaration_important(p: &mut CssParser) -> bool {
    p.at(T![!]) && p.nth_at(1, T![important])
}

#[inline]
fn parse_declaration_important(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_declaration_important(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![!]);
    p.bump(T![important]);
    Present(m.complete(p, CSS_DECLARATION_IMPORTANT))
}

#[inline]
pub(crate) fn is_at_any_value(p: &mut CssParser) -> bool {
    is_at_any_function(p)
        || is_at_identifier(p)
        || p.at(CSS_STRING_LITERAL)
        || is_at_any_dimension(p)
        || p.at(CSS_NUMBER_LITERAL)
        || is_at_dashed_identifier(p)
        || is_at_ratio(p)
        || is_at_color(p)
}

#[inline]
pub(crate) fn parse_any_value(p: &mut CssParser) -> ParsedSyntax {
    if is_at_any_function(p) {
        parse_any_function(p)
    } else if is_at_dashed_identifier(p) {
        parse_dashed_identifier(p)
    } else if is_at_identifier(p) {
        parse_regular_identifier(p)
    } else if p.at(CSS_STRING_LITERAL) {
        parse_string(p)
    } else if is_at_any_dimension(p) {
        parse_any_dimension(p)
    } else if is_at_ratio(p) {
        parse_ratio(p)
    } else if p.at(CSS_NUMBER_LITERAL) {
        parse_regular_number(p)
    } else if is_at_color(p) {
        parse_color(p)
    } else {
        Absent
    }
}

#[inline]
pub(crate) fn is_at_color(p: &mut CssParser) -> bool {
    p.at(T![#])
}
#[inline]
pub(crate) fn parse_color(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_color(p) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T![#], CssLexContext::Color);
    p.expect(CSS_COLOR_LITERAL);
    Present(m.complete(p, CSS_COLOR))
}

#[inline]
pub(crate) fn is_at_any_function(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.nth_at(1, T!['('])
}

pub(crate) struct CssParameterList;

impl ParseSeparatedList for CssParameterList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_PARAMETER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_parameter(p)
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
            &ParseRecovery::new(CSS_BOGUS_PARAMETER, token_set!(T![,], T![')'])),
            expected_declaration_item,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}
#[inline]
pub(crate) fn is_at_parameter(p: &mut CssParser) -> bool {
    is_at_parenthesized(p) || is_at_any_value(p)
}
#[inline]
pub(crate) fn is_at_parenthesized(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
pub(crate) fn parse_parameter(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_parameter(p) {
        return Absent;
    }
    let param = p.start();
    parse_any_expression(p).ok();
    Present(param.complete(p, CSS_PARAMETER))
}
#[inline]
pub(crate) fn is_at_any_expression(p: &mut CssParser) -> bool {
    is_at_parenthesized(p) || is_at_any_value(p)
}
#[inline]
pub(crate) fn parse_any_expression(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_expression(p) {
        return Absent;
    }
    let param = if is_at_parenthesized(p) {
        parse_parenthesized_expression(p)
    } else {
        parse_list_of_component_values_expression(p)
    };
    if is_at_binary_operator(p) {
        let binary_expression = param.precede(p);
        bump_operator_token(p);
        parse_any_expression(p).or_add_diagnostic(p, expected_expression);
        return Present(binary_expression.complete(p, CSS_BINARY_EXPRESSION));
    }
    param
}

#[inline]
pub(crate) fn parse_list_of_component_values_expression(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_value(p) {
        return Absent;
    }
    let m = p.start();
    ListOfComponentValues.parse_list(p);
    Present(m.complete(p, CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION))
}

#[inline]
pub(crate) fn is_at_binary_operator(p: &mut CssParser) -> bool {
    p.at_ts(BINARY_OPERATION_TOKEN)
}

#[inline]
pub(crate) fn bump_operator_token(p: &mut CssParser) {
    p.bump_ts(BINARY_OPERATION_TOKEN);
}

#[inline]
pub(crate) fn parse_parenthesized_expression(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_parenthesized(p) {
        return Absent;
    }
    let m = p.start();
    p.expect(T!['(']);
    parse_any_expression(p).ok();
    p.expect(T![')']);
    Present(m.complete(p, CSS_PARENTHESIZED_EXPRESSION))
}

#[inline]
pub(crate) fn parse_any_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_function(p) {
        return Absent;
    }
    if is_at_url_function(p) {
        return parse_url_function(p);
    }
    parse_simple_function(p)
}

fn parse_simple_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_function(p) {
        return Absent;
    }
    let simple_fn = p.start();
    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.expect(T!['(']);
    CssParameterList.parse_list(p);
    p.expect(T![')']);
    Present(simple_fn.complete(p, CSS_SIMPLE_FUNCTION))
}

pub(crate) fn is_at_url_function(p: &mut CssParser) -> bool {
    p.at(T![url]) && p.nth_at(1, T!['('])
}

pub(crate) fn parse_url_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_url_function(p) {
        return Absent;
    }
    let url_fn = p.start();
    p.expect(T![url]);
    p.expect_with_context(T!['('], CssLexContext::UrlRawValue);
    parse_url_value(p).ok();
    p.expect(T![')']);
    Present(url_fn.complete(p, CSS_URL_FUNCTION))
}

#[inline]
pub(crate) fn is_at_ratio(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER_LITERAL) && p.nth_at(1, T![/])
}

#[inline]
pub(crate) fn parse_ratio(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_ratio(p) {
        return Absent;
    }
    let m = p.start();
    parse_regular_number(p).ok();
    p.eat(T![/]);
    parse_regular_number(p).or_add_diagnostic(p, expected_number);
    Present(m.complete(p, CSS_RATIO))
}

pub(crate) fn is_at_url_value(p: &mut CssParser) -> bool {
    p.at(CSS_URL_VALUE_RAW_LITERAL) || is_at_string(p)
}

#[inline]
pub(crate) fn parse_url_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_url_value(p) {
        return Absent;
    }

    if is_at_string(p) {
        return parse_string(p);
    }
    let m = p.start();
    p.expect(CSS_URL_VALUE_RAW_LITERAL);
    Present(m.complete(p, CSS_URL_VALUE_RAW))
}

#[inline]
pub(crate) fn is_at_css_wide_keyword(p: &mut CssParser) -> bool {
    p.cur().is_css_wide_keyword()
}

#[inline]
pub(crate) fn is_at_identifier(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_identifier(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, T![ident]) || p.nth(n).is_contextual_keyword()
}

/// Parse any identifier using the Regular lexing context.
#[inline]
pub(crate) fn parse_regular_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_identifier(p, CssLexContext::Regular)
}

/// Parse any identifier as a general CssIdentifier. Regular identifiers are
/// case-insensitive, often used for property names, values, etc.
#[inline]
pub(crate) fn parse_identifier(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(T![ident], context);
    let identifier = m.complete(p, CSS_IDENTIFIER);

    Present(identifier)
}

/// Custom identifiers are identifiers not defined by CSS itself. These _are_
/// case-sensitive, used for class names, ids, etc. Custom identifiers _may_
/// have the same value as an identifier defined by CSS (e.g, `color`, used as
/// a class name), however they _must not_ be any of the CSS-wide keywords.
///
/// Custom identifiers have the same syntax as general identifiers, so the
/// [is_at_identifier] function can be used to check for both while parsing.
///
/// Custom identifiers can also be used in places where the CSS grammar
/// specifies `<ident>` but also includes case-sensitivity, such as in
/// class and id selectors. In these cases, CSS wide keywords _are_ accepted,
/// and can be handled by calling `parse_custom_identifier_with_keywords` with
/// `allow_css_wide_keywords` as `true` to cast them as identifiers.
///
/// When recovering from a parse error here, use
/// [parse_error::expected_non_css_wide_keyword_identifier] to provide the user
/// with additional information about how the CSS-wide keywords are not allowed
/// as custom identifiers.
#[inline]
pub(crate) fn parse_custom_identifier(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    parse_custom_identifier_with_keywords(p, context, false)
}

/// See [parse_custom_identifier]. This function allows for overriding the
/// handling of CSS-wide keywords using the `allow_css_wide_keywords` parameter.
///
/// This function should only be needed in cases where the CSS specification
/// defines a token as `<ident>` _and also_ case-sensitive. Otherwise, either
/// `parse_identifer` or `parse_custom_identifier` should be sufficient.
#[inline]
pub(crate) fn parse_custom_identifier_with_keywords(
    p: &mut CssParser,
    context: CssLexContext,
    allow_css_wide_keywords: bool,
) -> ParsedSyntax {
    if !is_at_identifier(p) || (!allow_css_wide_keywords && is_at_css_wide_keyword(p)) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(T![ident], context);
    let identifier = m.complete(p, CSS_CUSTOM_IDENTIFIER);

    Present(identifier)
}

#[inline]
pub(crate) fn is_at_dashed_identifier(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.cur_text().starts_with("--")
}

/// Dashed identifiers are any identifiers that start with two dashes (`--`).
/// Case sensitive, these are guaranteed to never overlap with an identifier
/// defined by CSS.
#[inline]
pub(crate) fn parse_dashed_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_dashed_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![ident]);
    Present(m.complete(p, CSS_DASHED_IDENTIFIER))
}

#[inline]
pub(crate) fn parse_regular_number(p: &mut CssParser) -> ParsedSyntax {
    parse_number(p, CssLexContext::Regular)
}
#[inline]
pub(crate) fn parse_number(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !p.at(CSS_NUMBER_LITERAL) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(CSS_NUMBER_LITERAL, context);

    Present(m.complete(p, CSS_NUMBER))
}

#[inline]
pub(crate) fn parse_string(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_string(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(CSS_STRING_LITERAL);

    Present(m.complete(p, CSS_STRING))
}

fn is_at_string(p: &mut CssParser) -> bool {
    p.at(CSS_STRING_LITERAL)
}
