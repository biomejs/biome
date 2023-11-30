mod at_rule;
mod parse_error;
mod selector;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax;
use crate::syntax::at_rule::{at_at_rule, parse_at_rule};
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::parse_error::{expect_any_value, expected_block};
use crate::syntax::selector::CssSelectorList;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, CompletedMarker, Marker, Parser, ParserProgress, TokenSet};

const RULE_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    token_set![T![#], T![.], T![*], T![ident], T![:], T![::], T!['{']];
const SELECTOR_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{'], T!['}'],];
const BODY_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    SELECTOR_LIST_RECOVERY_SET.union(RULE_RECOVERY_SET);

pub(crate) fn parse_root(p: &mut CssParser) {
    let m = p.start();
    p.eat(UNICODE_BOM);

    parse_rule_list(p);

    m.complete(p, CSS_ROOT);
}

#[inline]
pub(crate) fn parse_rule_list(p: &mut CssParser) {
    let mut progress = ParserProgress::default();

    let rules = p.start();
    while !p.at(EOF) {
        progress.assert_progressing(p);

        if at_at_rule(p) {
            parse_at_rule(p).ok();
        } else {
            parse_rule(p);
        }
    }

    rules.complete(p, CSS_RULE_LIST);
}

#[inline]
pub(crate) fn parse_rule(p: &mut CssParser) -> CompletedMarker {
    let m = p.start();

    CssSelectorList::default().parse_list(p);

    if parse_rule_block(p)
        .or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS_BODY, BODY_RECOVERY_SET),
            expected_block,
        )
        .is_err()
    {
        return m.complete(p, CSS_BOGUS_RULE);
    }

    m.complete(p, CSS_RULE)
}

#[inline]
pub(crate) fn parse_rule_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.expect(T!['{']);
    parse_declaration_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, CSS_BLOCK))
}

pub(crate) fn parse_declaration_list(p: &mut CssParser) {
    if is_at_identifier(p) {
        let m = p.start();

        // first parse declaration item
        parse_declaration_item(p);
        loop {
            if !p.at(T![;]) {
                break;
            }
            p.expect(T![;]);
            parse_declaration_item(p);
        }

        m.complete(p, CSS_DECLARATION_LIST);
    }
}

// CssDeclaration =
// 	name: CssIdentifier | CssCustomProperty
// 	':'
// 	value: AnyCssValue
// 	important: CssDeclarationImportant?
#[inline]
pub(crate) fn parse_declaration_item(p: &mut CssParser) {
    // name
    if parse_regular_identifier(p).is_absent() {
        return;
    }

    p.expect(T![:]);
    loop {
        let any_css_value = parse_any_css_value(p);

        if any_css_value.is_absent() {
            break;
        }
    }
}

#[inline]
pub(crate) fn parse_any_css_value(p: &mut CssParser) -> ParsedSyntax {
    let identifier = parse_regular_identifier(p);
    if identifier.is_present() {
        return identifier;
    }
    let css_string = parse_string(p);
    if css_string.is_present() {
        return css_string;
    }
    // Before css number
    // eat `1px` or `1.0rem`` or  number
    let css_dimension = parse_css_number_or_dimension(p);
    if css_dimension.is_present() {
        return css_dimension;
    }

    // TDOO:
    // 	| CssRatio
    // 	| CssAnyFunction
    // 	| CssCustomProperty

    return Absent;
}

pub(crate) fn parse_css_number_or_dimension(p: &mut CssParser) -> ParsedSyntax {
    if parse_css_number(p).is_present() && p.at(T![ident]) {
        let m = p.start();
        dbg!(p.cur_text());
        parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
        dbg!(p.cur_text());
        return Present(m.complete(p, CSS_DIMENSION));
    }
    Absent
}

#[inline]
pub(crate) fn is_at_identifier(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_identifier(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, T![ident]) || p.nth(n).is_contextual_keyword()
}
#[inline]
pub(crate) fn parse_regular_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_identifier(p, CssLexContext::Regular)
}

#[inline]
pub(crate) fn parse_identifier(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(T![ident], context);
    let identifier = m.complete(p, CSS_IDENTIFIER);

    Present(identifier)
}

#[inline]
pub(crate) fn parse_regular_number(p: &mut CssParser) -> ParsedSyntax {
    parse_number(p, CssLexContext::Regular)
}
#[inline]
pub(crate) fn parse_number(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !p.at(CSS_NUMBER_LITERAL) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(CSS_NUMBER_LITERAL, context);

    Present(m.complete(p, CSS_NUMBER))
}

#[inline]
pub(crate) fn parse_string(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(CSS_STRING_LITERAL) {
        return Absent;
    }

    let m = p.start();

    p.bump(CSS_STRING_LITERAL);

    Present(m.complete(p, CSS_STRING))
}

#[inline]
pub(crate) fn parse_css_number(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(CSS_NUMBER_LITERAL) {
        return Absent;
    }

    let m = p.start();

    p.bump(CSS_NUMBER_LITERAL);

    Present(m.complete(p, CSS_NUMBER))
}
