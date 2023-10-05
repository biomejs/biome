use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::{is_at_identifier, parse_error, parse_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

#[inline]
pub(crate) fn parse_compound_selector(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();

    let nesting_selector = p.eat(T![&]);

    let simple_selector = parse_simple_selector(p);

    if p.at_ts(SUB_SELECTOR_SET) || nesting_selector || simple_selector.is_present() {
        CssSubSelectorList.parse_list(p);
    } else {
        // If we don't have any selectors, we can't have a compound selector
        m.abandon(p);
        return Absent;
    }

    Present(m.complete(p, CSS_COMPOUND_SELECTOR))
}

#[inline]
pub(crate) fn parse_simple_selector(p: &mut CssParser) -> ParsedSyntax {
    match p.cur() {
        T![*] => parse_universal_selector(p),
        _ if is_at_identifier(p) => parse_type_selector(p),
        _ => Absent,
    }
}

const SUB_SELECTOR_SET: TokenSet<CssSyntaxKind> = token_set![T![#], T![.], T![:], T![::], T!['[']];
struct CssSubSelectorList;
impl ParseNodeList for CssSubSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const LIST_KIND: CssSyntaxKind = CSS_SUB_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_sub_selector(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        !p.at_ts(SUB_SELECTOR_SET)
    }

    fn recover(&mut self, p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS_SUB_SELECTOR, SUB_SELECTOR_SET),
            parse_error::expect_any_sub_selector,
        )
    }
}

#[inline]
pub(crate) fn parse_sub_selector(p: &mut CssParser) -> ParsedSyntax {
    match p.cur() {
        T![.] => parse_class_selector(p),
        T![#] => parse_id_selector(p),
        _ => Absent,
    }
}

#[inline]
pub(crate) fn parse_class_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![.]) {
        return Absent;
    }
    let m = p.start();

    p.bump(T![.]);

    parse_identifier(p, CSS_IDENTIFIER).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_CLASS_SELECTOR))
}

#[inline]
pub(crate) fn parse_id_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![#]) {
        return Absent;
    }
    let m = p.start();

    p.bump(T![#]);

    parse_identifier(p, CSS_IDENTIFIER).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_ID_SELECTOR))
}

#[inline]
pub(crate) fn parse_universal_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![*]) {
        return Absent;
    }
    let m = p.start();

    p.bump(T![*]);

    Present(m.complete(p, CSS_UNIVERSAL_SELECTOR))
}

#[inline]
pub(crate) fn parse_type_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }
    let m = p.start();

    parse_identifier(p, CSS_IDENTIFIER).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_TYPE_SELECTOR))
}
