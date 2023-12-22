mod at_rule;
mod css_dimension;
mod parse_error;
mod selector;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::at_rule::{at_at_rule, parse_at_rule};
use crate::syntax::css_dimension::{is_at_any_dimension, parse_any_dimension};
use crate::syntax::parse_error::expected_any_at_rule;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::parse_error::{expected_block, expected_express};
use crate::syntax::selector::CssSelectorList;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, CompletedMarker, Parser, ParserProgress, TokenSet};
use biome_rowan::SyntaxKind;

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

pub(crate) fn parse_root(p: &mut CssParser) {
    let m = p.start();
    p.eat(UNICODE_BOM);

    parse_rule_list(p, EOF);

    m.complete(p, CSS_ROOT);
}

#[inline]
pub(crate) fn parse_rule_list(p: &mut CssParser, end_kind: CssSyntaxKind) {
    let mut progress = ParserProgress::default();

    let rules = p.start();
    while !p.at(end_kind) {
        progress.assert_progressing(p);

        if at_at_rule(p) {
            if let Ok(m) = parse_at_rule(p).or_recover(
                p,
                &ParseRecovery::new(CSS_BOGUS_AT_RULE, token_set!['}']),
                expected_any_at_rule,
            ) {
                if m.kind(p).is_bogus() {
                    p.eat(T!['}']);
                }
            }
        } else {
            parse_rule(p);
        }
    }

    rules.complete(p, CSS_RULE_LIST);
}

#[inline]
pub(crate) fn parse_rule(p: &mut CssParser) -> CompletedMarker {
    let m = p.start();

    CssSelectorList::default().parse_list(p);

    let kind = if parse_or_recover_declaration_list_block(p).is_ok() {
        CSS_RULE
    } else {
        CSS_BOGUS_RULE
    };

    m.complete(p, kind)
}

#[inline]
pub(crate) fn parse_or_recover_declaration_list_block(p: &mut CssParser) -> RecoveryResult {
    parse_declaration_list_block(p).or_recover(
        p,
        &ParseRecovery::new(CSS_BOGUS_BLOCK, BODY_RECOVERY_SET).enable_recovery_on_line_break(),
        expected_block,
    )
}

#[inline]
pub(crate) fn parse_declaration_list_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.expect(T!['{']);
    CssDeclarationList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, CSS_DECLARATION_LIST_BLOCK))
}

#[inline]
pub(crate) fn parse_or_recover_rule_list_block(p: &mut CssParser) -> RecoveryResult {
    parse_rule_list_block(p).or_recover(
        p,
        &ParseRecovery::new(CSS_BOGUS_BLOCK, BODY_RECOVERY_SET).enable_recovery_on_line_break(),
        expected_block,
    )
}
#[inline]
pub(crate) fn parse_rule_list_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.expect(T!['{']);
    parse_rule_list(p, T!['}']);
    p.expect(T!['}']);

    Present(m.complete(p, CSS_RULE_LIST_BLOCK))
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
        || is_at_custom_property(p)
        || is_at_ratio(p)
        || is_at_color(p)
}

#[inline]
pub(crate) fn parse_any_value(p: &mut CssParser) -> ParsedSyntax {
    if is_at_any_function(p) {
        parse_any_function(p)
    } else if is_at_custom_property(p) {
        parse_custom_property(p)
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
pub(crate) fn is_at_custom_property(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.cur_text().starts_with("--")
}

#[inline]
pub(crate) fn parse_custom_property(p: &mut CssParser) -> ParsedSyntax {
    if is_at_custom_property(p) {
        let m = p.start();
        parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
        return Present(m.complete(p, CSS_CUSTOM_PROPERTY));
    }
    Absent
}

#[inline]
pub(crate) fn is_at_any_function(p: &mut CssParser) -> bool {
    (p.at(T![ident]) || p.cur().is_keyword()) && p.nth_at(1, T!['('])
}

#[derive(Default)]
pub(crate) struct CssParameterList {}

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
}
#[inline]
pub(crate) fn is_parameter(p: &mut CssParser) -> bool {
    is_css_parenthesized(p) || is_at_any_value(p)
}
#[inline]
pub(crate) fn is_css_parenthesized(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
pub(crate) fn parse_parameter(p: &mut CssParser) -> ParsedSyntax {
    if !is_parameter(p) {
        return Absent;
    }
    let param = p.start();

    parse_any_express(p).ok();
    Present(param.complete(p, CSS_PARAMETER))
}
#[inline]
pub(crate) fn is_any_express(p: &mut CssParser) -> bool {
    is_css_parenthesized(p) || is_at_any_value(p)
}
#[inline]
pub(crate) fn parse_any_express(p: &mut CssParser) -> ParsedSyntax {
    if !is_any_express(p) {
        return Absent;
    }
    let param = if is_css_parenthesized(p) {
        parse_parenthesized_express(p)
    } else {
        parse_any_value(p)
    };
    if is_operator_token(p) {
        let css_binary_express = param.precede(p);
        parse_operator_token(p);
        parse_any_express(p).or_add_diagnostic(p, expected_express);
        return Present(css_binary_express.complete(p, CSS_BINARY_EXPRESS));
    }
    if is_at_any_value(p) {
        let component_value_list = param.precede(p);
        while is_at_any_value(p) {
            parse_any_value(p).ok();
        }
        let m = component_value_list
            .complete(p, CSS_COMPONENT_VALUE_LIST)
            .precede(p);
        return Present(m.complete(p, CSS_LIST_OF_COMPONENT_VALUES_EXPRESS));
    }
    param
}

#[inline]
pub(crate) fn is_operator_token(p: &mut CssParser) -> bool {
    p.at(T![+]) || p.at(T![-]) || p.at(T![*]) || p.at(T![/])
}

#[inline]
pub(crate) fn parse_operator_token(p: &mut CssParser) {
    p.bump_ts(token_set![T![+], T![-], T![*], T![/]]);
}

#[inline]
pub(crate) fn parse_parenthesized_express(p: &mut CssParser) -> ParsedSyntax {
    if !is_css_parenthesized(p) {
        return Absent;
    }
    let m = p.start();
    p.expect(T!['(']);
    parse_any_express(p).ok();
    p.expect(T![')']);
    Present(m.complete(p, CSS_PARENTHESIZED_EXPRESSION))
}

#[inline]
pub(crate) fn parse_any_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_function(p) {
        return Absent;
    }
    if is_url_function(p) {
        return parse_url_function(p);
    }
    parse_simple_function(p)
}

fn parse_simple_function(p: &mut CssParser<'_>) -> ParsedSyntax {
    if !is_at_any_function(p) {
        return Absent;
    }
    let simple_fn = p.start();
    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.expect(T!['(']);
    CssParameterList::default().parse_list(p);
    p.expect(T![')']);
    Present(simple_fn.complete(p, CSS_SIMPLE_FUNCTION))
}

pub(crate) fn is_url_function(p: &mut CssParser) -> bool {
    p.at(T![url]) && p.nth_at(1, T!['('])
}

pub(crate) fn parse_url_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_url_function(p) {
        return Absent;
    }
    let url_fn = p.start();
    p.expect(T![url]);
    p.expect_with_context(T!['('], CssLexContext::UrlRawValue);
    parse_url_value_raw(p).ok();
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

#[inline]
pub(crate) fn is_at_identifier(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_identifier(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, T![ident]) || p.nth(n).is_contextual_keyword()
}
#[inline]
pub(crate) fn parse_regular_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_identifier(p, CssLexContext::Regular)
}

pub(crate) fn is_at_url_value_raw(p: &mut CssParser) -> bool {
    p.at(CSS_URL_VALUE_RAW_LITERAL) || is_at_string(p)
}

#[inline]
pub(crate) fn parse_url_value_raw(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_url_value_raw(p) {
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
pub(crate) fn parse_identifier(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(T![ident], context);
    let identifier = m.complete(p, CSS_IDENTIFIER);

    Present(identifier)
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

fn is_at_string(p: &mut CssParser<'_>) -> bool {
    p.at(CSS_STRING_LITERAL)
}
