mod attribute;
mod pseudo_class;
mod pseudo_element;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::{
    expect_any_selector, expect_any_sub_selector, expected_identifier,
};
use crate::syntax::selector::attribute::parse_attribute_selector;
use crate::syntax::selector::pseudo_class::parse_pseudo_class_selector;
use crate::syntax::selector::pseudo_element::parse_pseudo_element_selector;
use crate::syntax::{is_at_identifier, parse_identifier, RULE_RECOVERY_SET};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::diagnostic::expected_token;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, CompletedMarker, Parser, ParserProgress, TokenSet};

const SELECTOR_RECOVERY_SET: TokenSet<CssSyntaxKind> = RULE_RECOVERY_SET.union(token_set![T![,]]);
const SELECTOR_FUNCTION_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T![')'], T!['{']];

#[inline]
fn parse_selector_function_close_token(p: &mut CssParser) {
    let context = selector_lex_context(p);

    if !p.eat_with_context(T![')'], context)
        && ParseRecovery::new(CSS_BOGUS, SELECTOR_FUNCTION_RECOVERY_SET)
            .recover(p)
            .is_err()
    {
        p.error(expected_token(T![')']));
    }
}

const SELECTOR_LEX_SET: TokenSet<CssSyntaxKind> =
    COMPLEX_SELECTOR_COMBINATOR_SET.union(token_set![T!['{'], T![,], T![')']]);
#[inline]
fn selector_lex_context(p: &mut CssParser) -> CssLexContext {
    if SELECTOR_LEX_SET.contains(p.nth(1)) {
        CssLexContext::Regular
    } else {
        CssLexContext::Selector
    }
}

pub(crate) struct CssSelectorList {
    end_kind: CssSyntaxKind,
}

impl Default for CssSelectorList {
    fn default() -> Self {
        CssSelectorList { end_kind: T!['{'] }
    }
}

impl CssSelectorList {
    pub(crate) fn with_end_kind(self, end_kind: CssSyntaxKind) -> Self {
        Self { end_kind }
    }
}

impl ParseSeparatedList for CssSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_selector(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(self.end_kind)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS_SELECTOR, SELECTOR_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expect_any_selector,
        )
    }
    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}
#[inline]
fn parse_selector(p: &mut CssParser) -> ParsedSyntax {
    // In CSS, we have compound selectors and complex selectors.
    // Compound selectors are simple, unseparated chains of selectors,
    // while complex selectors are compound selectors separated by combinators.
    // After parsing the compound selector, it then checks if this compound selector is a part of a complex selector.
    parse_compound_selector(p).and_then(|selector| parse_complex_selector(p, selector))
}

const COMPLEX_SELECTOR_COMBINATOR_SET: TokenSet<CssSyntaxKind> =
    token_set![T![>], T![+], T![~], T![||], CSS_SPACE_LITERAL];
#[inline]
fn is_at_complex_selector_combinator(p: &mut CssParser) -> bool {
    p.at_ts(COMPLEX_SELECTOR_COMBINATOR_SET)
}
#[inline]
fn parse_complex_selector(p: &mut CssParser, mut left: CompletedMarker) -> ParsedSyntax {
    let mut progress = ParserProgress::default();

    loop {
        progress.assert_progressing(p);

        if is_at_complex_selector_combinator(p) {
            let complex_selector = left.precede(p);
            // bump combinator
            p.bump(p.cur());
            parse_compound_selector(p).or_add_diagnostic(p, expect_any_selector);
            left = complex_selector.complete(p, CSS_COMPLEX_SELECTOR)
        } else {
            return Present(left);
        }
    }
}

#[inline]
fn parse_compound_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_compound_selector(p) {
        return Absent;
    }

    let m = p.start();

    p.eat(T![&]);
    parse_simple_selector(p).ok();
    CssSubSelectorList.parse_list(p);

    Present(m.complete(p, CSS_COMPOUND_SELECTOR))
}

#[inline]
fn is_at_compound_selector(p: &mut CssParser) -> bool {
    p.at(T![&]) || is_at_simple_selector(p) || p.at_ts(CssSubSelectorList::START_SET)
}

#[inline]
fn parse_simple_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_simple_selector(p) {
        return Absent;
    }

    match p.cur() {
        T![*] => parse_universal_selector(p),
        _ if is_at_identifier(p) => parse_type_selector(p),
        _ => Absent,
    }
}

#[inline]
fn is_at_simple_selector(p: &mut CssParser) -> bool {
    p.at(T![*]) || is_at_identifier(p)
}

struct CssSubSelectorList;
impl CssSubSelectorList {
    const START_SET: TokenSet<CssSyntaxKind> = token_set![T![#], T![.], T![:], T![::], T!['[']];
}
impl ParseNodeList for CssSubSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const LIST_KIND: CssSyntaxKind = CSS_SUB_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_sub_selector(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        !p.at_ts(Self::START_SET)
    }

    fn recover(&mut self, p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS_SUB_SELECTOR, Self::START_SET),
            expect_any_sub_selector,
        )
    }
}

#[inline]
fn parse_sub_selector(p: &mut CssParser) -> ParsedSyntax {
    match p.cur() {
        T![.] => parse_class_selector(p),
        T![#] => parse_id_selector(p),
        T!['['] => parse_attribute_selector(p),
        T![:] => parse_pseudo_class_selector(p),
        T![::] => parse_pseudo_element_selector(p),
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
    parse_selector_identifier(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_CLASS_SELECTOR))
}

#[inline]
pub(crate) fn parse_id_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![#]) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![#]);
    parse_selector_identifier(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_ID_SELECTOR))
}

#[inline]
pub(crate) fn parse_universal_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![*]) {
        return Absent;
    }

    let m = p.start();

    let context = selector_lex_context(p);
    p.eat_with_context(T![*], context);

    Present(m.complete(p, CSS_UNIVERSAL_SELECTOR))
}

#[inline]
fn parse_type_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    parse_selector_identifier(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, CSS_TYPE_SELECTOR))
}

#[inline]
fn parse_selector_identifier(p: &mut CssParser) -> ParsedSyntax {
    let context = selector_lex_context(p);
    parse_identifier(p, context)
}
