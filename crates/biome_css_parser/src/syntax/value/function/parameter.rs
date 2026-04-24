use crate::parser::CssParser;
use crate::syntax::ValueParsingContext;
use crate::syntax::parse_error::expected_declaration_item;
use crate::syntax::scss::parse_scss_expression_in_args_until;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::Absent;
use biome_parser::{Parser, TokenSet, token_set};

use super::expression::{is_at_any_expression_with_context, parse_any_expression_with_context};

const PARAMETER_RECOVERY_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set!(T![,], T![')'], T![;], T!['}']);

#[derive(Debug, Copy, Clone)]
struct ParameterListParseRecovery {
    context: ValueParsingContext,
}

impl ParseRecovery for ParameterListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_PARAMETER;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(PARAMETER_RECOVERY_TOKEN_SET) || is_at_parameter_with_context(p, self.context)
    }
}

pub(crate) struct ParameterList {
    context: ValueParsingContext,
}

impl ParameterList {
    #[inline]
    pub(crate) fn new(context: ValueParsingContext) -> Self {
        Self { context }
    }
}

impl ParseSeparatedList for ParameterList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_PARAMETER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_parameter_with_context(p, self.context)
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
            &ParameterListParseRecovery {
                context: self.context,
            },
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
fn is_at_parameter_with_context(p: &mut CssParser, context: ValueParsingContext) -> bool {
    is_at_any_expression_with_context(p, context)
}

#[inline]
fn parse_parameter_with_context(p: &mut CssParser, context: ValueParsingContext) -> ParsedSyntax {
    if !is_at_parameter_with_context(p, context) {
        return Absent;
    }

    if context.is_full_scss_parsing_allowed() {
        parse_scss_expression_in_args_until(p, PARAMETER_RECOVERY_TOKEN_SET)
    } else {
        parse_any_expression_with_context(p, context)
    }
}
