use crate::parser::CssParser;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::property::parse_generic_component_value;
use crate::syntax::scss::{SCSS_UNARY_OPERATOR_TOKEN_SET, parse_scss_expression};
use crate::syntax::value::parse_error::expected_expression;
use crate::syntax::{
    ValueParsingContext, is_at_any_value_with_context, parse_any_value_with_context,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, TokenSet, token_set};

#[inline]
pub(super) fn is_at_any_expression_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> bool {
    is_at_unary_operator(p)
        || is_at_parenthesized(p)
        || is_at_any_value_with_context(p, context)
        || is_at_comma_separated_value(p)
}

#[inline]
pub(super) fn parse_any_expression_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if !is_at_any_expression_with_context(p, context) {
        return Absent;
    }

    if context.is_scss_parsing_allowed()
        && (is_at_parenthesized(p)
            || is_at_any_value_with_context(p, context)
            || p.at_ts(SCSS_UNARY_OPERATOR_TOKEN_SET))
    {
        return parse_scss_expression(p);
    }

    let param = parse_unary_expression_operand_with_context(p, context);

    if is_at_binary_operator(p) {
        let binary_expression = param.precede(p);

        p.bump_ts(BINARY_OPERATION_TOKEN);
        parse_any_expression_with_context(p, context).or_add_diagnostic(p, expected_expression);

        Present(binary_expression.complete(p, CSS_BINARY_EXPRESSION))
    } else {
        param
    }
}

const BINARY_OPERATION_TOKEN: TokenSet<CssSyntaxKind> = token_set![T![+], T![-], T![*], T![/]];
const UNARY_OPERATION_TOKEN: TokenSet<CssSyntaxKind> = token_set![T![+], T![-], T![*]];
const COMPONENT_VALUE_EXPRESSION_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    token_set!(T![')'], T![;], T![,]).union(BINARY_OPERATION_TOKEN);

#[inline]
pub(crate) fn is_at_binary_operator(p: &mut CssParser) -> bool {
    p.at_ts(BINARY_OPERATION_TOKEN)
}

#[inline]
pub(crate) fn is_at_unary_operator(p: &mut CssParser) -> bool {
    p.at_ts(UNARY_OPERATION_TOKEN)
}

#[inline]
fn parse_unary_expression_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if !is_at_unary_operator(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(UNARY_OPERATION_TOKEN);
    parse_unary_expression_operand_with_context(p, context)
        .or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, CSS_UNARY_EXPRESSION))
}

#[inline]
fn parse_unary_expression_operand_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if is_at_unary_operator(p) {
        parse_unary_expression_with_context(p, context)
    } else if is_at_parenthesized(p) {
        parse_parenthesized_expression_with_context(p, context)
    } else if is_at_comma_separated_value(p) {
        parse_comma_separated_value(p)
    } else {
        parse_list_of_component_values_expression_with_context(p, context)
    }
}

#[inline]
pub(crate) fn is_at_parenthesized(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
fn parse_parenthesized_expression_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if !is_at_parenthesized(p) {
        return Absent;
    }

    let m = p.start();
    p.expect(T!['(']);
    parse_any_expression_with_context(p, context).ok();
    p.expect(T![')']);
    Present(m.complete(p, CSS_PARENTHESIZED_EXPRESSION))
}

#[inline]
fn parse_list_of_component_values_expression_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if !is_at_any_value_with_context(p, context) {
        return Absent;
    }

    let m = p.start();
    ComponentValueExpressionList::new(context).parse_list(p);
    Present(m.complete(p, CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION))
}

#[derive(Debug, Copy, Clone)]
struct ComponentValueExpressionList {
    context: ValueParsingContext,
}

impl ComponentValueExpressionList {
    #[inline]
    fn new(context: ValueParsingContext) -> Self {
        Self { context }
    }
}

impl ParseNodeList for ComponentValueExpressionList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_any_value_with_context(p, self.context)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![,]) || p.at(T![')']) || p.at_ts(BINARY_OPERATION_TOKEN)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, COMPONENT_VALUE_EXPRESSION_RECOVERY_SET),
            expected_component_value,
        )
    }
}

#[inline]
fn is_at_comma_separated_value(p: &mut CssParser) -> bool {
    p.at(T!['{'])
}

#[inline]
fn parse_comma_separated_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_comma_separated_value(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['{']);
    CommaSeparatedValueValueList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, CSS_COMMA_SEPARATED_VALUE))
}

struct CommaSeparatedValueValueList;

impl ParseNodeList for CommaSeparatedValueValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_GENERIC_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_generic_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_PROPERTY_VALUE, token_set![T!['}']])
                .enable_recovery_on_line_break(),
            expected_component_value,
        )
    }
}
