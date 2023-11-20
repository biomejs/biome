use crate::parser::CssParser;
use crate::syntax::parse_error::{expect_any_selector, expected_identifier};
use crate::syntax::parse_regular_identifier;
use crate::syntax::selector::{parse_compound_selector, parse_selector_function_close_token};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::CssSyntaxKind::{
    CSS_BOGUS_SELECTOR, CSS_COMPOUND_SELECTOR_LIST,
    CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

const PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST_SET: TokenSet<CssSyntaxKind> =
    token_set![MOZANY_KW, WEBKITANY_KW, PAST_KW, CURRENT_KW, FUTURE_KW];

#[inline]
pub(crate) fn is_at_pseudo_class_function_compound_selector_list(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_compound_selector_list(
    p: &mut CssParser,
) -> ParsedSyntax {
    if !is_at_pseudo_class_function_compound_selector_list(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    CssCompoundSelectorList.parse_list(p);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST))
}

struct CssCompoundSelectorList;

impl CssCompoundSelectorList {
    const RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{'], T![')']];
}

impl ParseSeparatedList for CssCompoundSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const LIST_KIND: CssSyntaxKind = CSS_COMPOUND_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_compound_selector(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        p.at(T![')'])
    }

    fn recover(&mut self, p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS_SELECTOR, Self::RECOVERY_SET),
            expect_any_selector,
        )
    }

    fn separating_element_kind(&mut self) -> CssSyntaxKind {
        T![,]
    }
}
