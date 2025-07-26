use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::{expected_identifier, expected_non_css_wide_keyword_identifier};
use crate::syntax::selector::eat_or_recover_selector_function_close_token;
use crate::syntax::{is_at_identifier, parse_custom_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, TokenSet, token_set};

const PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER_LIST_SET: TokenSet<CssSyntaxKind> =
    token_set![T![active_view_transition_type]];

#[inline]
pub(crate) fn is_at_pseudo_class_function_custom_identifier_list(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER_LIST_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_custom_identifier_list(
    p: &mut CssParser,
) -> ParsedSyntax {
    if !is_at_pseudo_class_function_custom_identifier_list(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).ok();
    p.bump(T!['(']);

    let list = CssCustomIdentifierList.parse_list(p);
    let list_range = list.range(p);

    if list_range.is_empty() {
        let diagnostic = expected_identifier(p, list_range);
        p.error(diagnostic);
    }

    let kind = if eat_or_recover_selector_function_close_token(
        p,
        list,
        expected_non_css_wide_keyword_identifier,
    ) && !list_range.is_empty()
    {
        CSS_PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER_LIST
    } else {
        CSS_BOGUS_PSEUDO_CLASS
    };

    Present(m.complete(p, kind))
}

struct CssCustomIdentifierList;

impl ParseNodeList for CssCustomIdentifierList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_CUSTOM_IDENTIFIER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_custom_identifier(p, CssLexContext::Regular)
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
            &CssCustomIdentifierListParseRecovery,
            expected_non_css_wide_keyword_identifier,
        )
    }
}

struct CssCustomIdentifierListParseRecovery;

impl ParseRecovery for CssCustomIdentifierListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_CUSTOM_IDENTIFIER;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')']) || is_at_identifier(p)
    }
}
