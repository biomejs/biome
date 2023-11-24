use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::selector::parse_selector_function_close_token;
use crate::syntax::{is_at_identifier, parse_css_string, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

const PSEUDO_CLASS_FUNCTION_VALUE_LIST_SET: TokenSet<CssSyntaxKind> = token_set![LANG_KW];

#[inline]
pub(crate) fn is_at_pseudo_class_function_value_list(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_VALUE_LIST_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_value_list(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_value_list(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    CssPseudoValueList.parse_list(p);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST))
}

struct CssPseudoValueList;

impl CssPseudoValueList {
    const RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{'], T![')']];
}

impl ParseSeparatedList for CssPseudoValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const LIST_KIND: CssSyntaxKind = CSS_PSEUDO_VALUE_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_pseudo_value(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        p.at(T![')'])
    }

    fn recover(&mut self, p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, Self::RECOVERY_SET),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> CssSyntaxKind {
        T![,]
    }
}

#[inline]
fn is_at_pseudo_value(p: &mut CssParser) -> bool {
    is_at_identifier(p) || p.at(CSS_STRING_LITERAL)
}

#[inline]
fn parse_pseudo_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_value(p) {
        return Absent;
    }

    if p.at(CSS_STRING_LITERAL) {
        parse_css_string(p)
    } else {
        parse_regular_identifier(p)
    }
}
