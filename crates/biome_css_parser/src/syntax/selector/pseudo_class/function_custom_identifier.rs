use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::{parse_custom_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, TokenSet, token_set};

const PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER_SET: TokenSet<CssSyntaxKind> = token_set![T![state]];

#[inline]
pub(crate) fn is_at_pseudo_class_function_custom_identifier(p: &mut CssParser) -> bool {
    p.at_ts(PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER_SET) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_pseudo_class_function_custom_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_pseudo_class_function_custom_identifier(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).ok();
    p.bump(T!['(']);

    let kind = match parse_custom_identifier(p, CssLexContext::Regular) {
        Present(identifier) => CSS_PSEUDO_CLASS_FUNCTION_CUSTOM_IDENTIFIER,
        Absent => CSS_BOGUS_PSEUDO_CLASS,
    };

    Present(m.complete(p, kind))
}
