use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::selector::eat_or_recover_selector_function_close_token;
use crate::syntax::{is_at_identifier, parse_regular_identifier, parse_string};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{RecoveryError, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, TokenSet, token_set};

const PSEUDO_CLASS_FUNCTION_VALUE_LIST_SET: TokenSet<CssSyntaxKind> = token_set![T![lang]];

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

    parse_regular_identifier(p).ok();
    p.bump(T!['(']);

    let list = PseudoValueList.parse_list(p);
    let list_range = list.range(p);

    if list_range.is_empty() {
        let diagnostic = expected_identifier(p, list_range);
        p.error(diagnostic);
    }

    let kind = if eat_or_recover_selector_function_close_token(p, list, expected_identifier)
        && !list_range.is_empty()
    {
        CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST
    } else {
        CSS_BOGUS_PSEUDO_CLASS
    };

    Present(m.complete(p, kind))
}

struct PseudoValueList;

impl ParseSeparatedList for PseudoValueList {
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
        match parsed_element.or_add_diagnostic(p, expected_identifier) {
            Some(m) => Ok(m),
            // we don't need to recover here, because we have a better diagnostic message in a close token
            None => Err(RecoveryError::RecoveryDisabled),
        }
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
        parse_string(p)
    } else {
        parse_regular_identifier(p)
    }
}
