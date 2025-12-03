use crate::syntax::at_rule::parse_error::expected_function_parameter;
use crate::syntax::block::parse_declaration_or_at_rule_list_block;
use crate::syntax::is_at_dashed_identifier;
use crate::syntax::parse_error::{expected_component_value, expected_dashed_identifier};
use crate::syntax::property::parse_generic_component_value;
use crate::syntax::value::r#type::{
    is_at_syntax_single_component, is_at_type_function, parse_any_syntax_component,
    parse_type_function,
};
use crate::{parser::CssParser, syntax::parse_dashed_identifier};
use biome_css_syntax::{
    CssSyntaxKind::{self, *},
    T,
};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::token_set;
use biome_parser::{
    Parser,
    parsed_syntax::ParsedSyntax::{self, Present},
    prelude::ParsedSyntax::Absent,
};

#[inline]
pub(crate) fn is_at_function_at_rule(p: &mut CssParser) -> bool {
    p.at(T![function])
}

#[inline]
pub(crate) fn parse_function_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_function_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    parse_function_at_rule_declarator(p).ok();
    parse_declaration_or_at_rule_list_block(p);

    Present(m.complete(p, CSS_FUNCTION_AT_RULE))
}

#[inline]
pub(crate) fn parse_function_at_rule_declarator(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_function_at_rule(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![function]);

    // TODO: recover on type, default arg
    parse_dashed_identifier(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, token_set![T!['('], T!['{']])
                .enable_recovery_on_line_break(),
            expected_dashed_identifier,
        )
        .ok();

    p.expect(T!['(']);
    CssFunctionParameterList.parse_list(p);
    p.expect(T![')']);

    parse_returns_statement(p).ok();

    Present(m.complete(p, CSS_FUNCTION_AT_RULE_DECLARATOR))
}

#[inline]
fn parse_function_parameter(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_dashed_identifier(p) {
        return Absent;
    }

    let m = p.start();

    parse_dashed_identifier(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, token_set![T![,], T![')']])
                .enable_recovery_on_line_break(),
            expected_dashed_identifier,
        )
        .ok();

    // TODO: recover
    parse_any_type(p).ok();

    parse_function_parameter_default_value(p).ok();

    Present(m.complete(p, CSS_FUNCTION_PARAMETER))
}

#[inline]
fn parse_any_type(p: &mut CssParser) -> ParsedSyntax {
    if is_at_type_function(p) {
        return parse_type_function(p);
    }

    if is_at_syntax_single_component(p) {
        return parse_any_syntax_component(p);
    }

    Absent
}

#[inline]
fn is_at_function_parameter_default_value(p: &mut CssParser) -> bool {
    p.at(T![:])
}

#[inline]
fn parse_function_parameter_default_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_function_parameter_default_value(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![:]);
    CssFunctionParameterDefaultValueList.parse_list(p);

    Present(m.complete(p, CSS_FUNCTION_PARAMETER_DEFAULT_VALUE))
}

#[inline]
fn is_at_returns_statement(p: &mut CssParser) -> bool {
    p.at(T![returns])
}

#[inline]
fn parse_returns_statement(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_returns_statement(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![returns]);

    // TODO: recover
    parse_any_type(p).ok();

    Present(m.complete(p, CSS_RETURNS_STATEMENT))
}

struct CssFunctionParameterListParseRecovery;

impl ParseRecovery for CssFunctionParameterListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_FUNCTION_PARAMETER;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![,]) || p.at(T![')']) || p.at(T![returns]) || p.at(T!['{'])
    }
}

struct CssFunctionParameterList;

impl ParseSeparatedList for CssFunctionParameterList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_FUNCTION_PARAMETER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_function_parameter(p)
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
            &CssFunctionParameterListParseRecovery,
            expected_function_parameter,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_empty(&self) -> bool {
        true
    }
}

struct CssFunctionParameterDefaultValueList;

impl ParseSeparatedList for CssFunctionParameterDefaultValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_GENERIC_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_generic_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_PROPERTY_VALUE, token_set![T![,], T![')']])
                .enable_recovery_on_line_break(),
            expected_component_value,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    // TODO: double check this
    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}
