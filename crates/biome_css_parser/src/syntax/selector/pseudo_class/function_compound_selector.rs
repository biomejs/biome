use crate::parser::CssParser;
use crate::syntax::parse_error::{expect_any_selector, expected_identifier};
use crate::syntax::parse_regular_identifier;
use crate::syntax::selector::{parse_compound_selector, parse_selector_function_close_token};
use biome_css_syntax::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

const PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_SET: TokenSet<CssSyntaxKind> =
    token_set![HOST_KW, HOSTCONTEXT_KW];

#[inline]
pub(crate) fn is_at_pseudo_class_function_compound_selector(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_compound_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_compound_selector(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    parse_compound_selector(p).or_add_diagnostic(p, expect_any_selector);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR))
}
