use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::parse_regular_identifier;
use crate::syntax::selector::parse_selector_function_close_token;
use biome_css_syntax::CssSyntaxKind::CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

const PSEUDO_CLASS_FUNCTION_IDENTIFIER_SET: TokenSet<CssSyntaxKind> = token_set![DIR_KW];

#[inline]
pub(crate) fn is_at_pseudo_class_function_identifier(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_IDENTIFIER_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_identifier(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    parse_selector_function_close_token(p);

    Present(m.complete(p, CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER))
}
