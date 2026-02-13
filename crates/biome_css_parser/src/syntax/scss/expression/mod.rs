use crate::parser::CssParser;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::property::parse_generic_component_value;
use biome_css_syntax::CssSyntaxKind::{CSS_BOGUS_PROPERTY_VALUE, EOF, SCSS_EXPRESSION};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, TokenSet, token_set};

const SCSS_BINARY_OPERATOR_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![+], T![-], T![*], T![/]];

pub(crate) const END_OF_SCSS_EXPRESSION_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![,], T![')'], T![;], T!['}']].union(SCSS_BINARY_OPERATOR_TOKEN_SET);

#[inline]
pub(crate) fn parse_scss_expression(p: &mut CssParser) -> ParsedSyntax {
    parse_scss_expression_until(p, END_OF_SCSS_EXPRESSION_TOKEN_SET)
}

#[inline]
fn parse_scss_expression_until(p: &mut CssParser, end_ts: TokenSet<CssSyntaxKind>) -> ParsedSyntax {
    parse_scss_expression_with_options(p, end_ts.union(SCSS_BINARY_OPERATOR_TOKEN_SET), false)
}

#[inline]
fn parse_scss_expression_with_options(
    p: &mut CssParser,
    end_ts: TokenSet<CssSyntaxKind>,
    allow_empty: bool,
) -> ParsedSyntax {
    if !allow_empty && (p.at_ts(end_ts) || p.at(EOF)) {
        return Absent;
    }

    let expression = ScssExpressionItemList::new(end_ts).parse_list(p);
    Present(expression)
}

struct ScssExpressionItemList {
    end_ts: TokenSet<CssSyntaxKind>,
}

impl ScssExpressionItemList {
    fn new(end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self { end_ts }
    }
}

impl ParseNodeList for ScssExpressionItemList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_EXPRESSION;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_generic_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(self.end_ts)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_PROPERTY_VALUE, self.end_ts)
                .enable_recovery_on_line_break(),
            expected_component_value,
        )
    }
}
