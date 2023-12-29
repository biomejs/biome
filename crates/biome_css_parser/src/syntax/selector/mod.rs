mod attribute;
mod pseudo_class;
mod pseudo_element;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::{
    expected_any_sub_selector, expected_compound_selector, expected_identifier, expected_selector,
};
use crate::syntax::selector::attribute::parse_attribute_selector;
use crate::syntax::selector::pseudo_class::parse_pseudo_class_selector;
use crate::syntax::selector::pseudo_element::parse_pseudo_element_selector;
use crate::syntax::{
    is_at_identifier, parse_custom_identifier_with_keywords, parse_identifier,
    parse_regular_identifier, RULE_RECOVERY_SET,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, TextRange, T};
use biome_parser::diagnostic::ToDiagnostic;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, RecoveryError, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, CompletedMarker, Parser, ParserProgress, TokenSet};

const SELECTOR_RECOVERY_SET: TokenSet<CssSyntaxKind> = RULE_RECOVERY_SET.union(token_set![T![,]]);
const SELECTOR_FUNCTION_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T![')'], T!['{']];

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
    end_kind_ts: TokenSet<CssSyntaxKind>,
    is_recovery_disabled: bool,
}

impl Default for CssSelectorList {
    fn default() -> Self {
        CssSelectorList {
            end_kind_ts: token_set!(T!['{']),
            is_recovery_disabled: false,
        }
    }
}

impl CssSelectorList {
    pub(crate) fn with_end_kind_ts(mut self, end_kind_ts: TokenSet<CssSyntaxKind>) -> Self {
        self.end_kind_ts = end_kind_ts;
        self
    }

    pub(crate) fn with_end_kind(mut self, end_kind: CssSyntaxKind) -> Self {
        self.end_kind_ts = token_set!(end_kind);
        self
    }

    pub(crate) fn disable_recovery(mut self) -> Self {
        self.is_recovery_disabled = true;
        self
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
        p.at_ts(self.end_kind_ts)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        if parsed_element.is_absent() && self.is_recovery_disabled {
            p.error(expected_selector(p, p.cur_range()));
            Err(RecoveryError::RecoveryDisabled)
        } else {
            parsed_element.or_recover(
                p,
                &ParseRecovery::new(CSS_BOGUS_SELECTOR, SELECTOR_RECOVERY_SET)
                    .enable_recovery_on_line_break(),
                expected_selector,
            )
        }
    }
    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

#[inline]
pub(crate) fn is_at_selector(p: &mut CssParser) -> bool {
    is_at_compound_selector(p)
}

#[inline]
pub(crate) fn parse_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_selector(p) {
        return Absent;
    }

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

            p.bump_ts(COMPLEX_SELECTOR_COMBINATOR_SET);
            parse_compound_selector(p).or_add_diagnostic(p, expected_compound_selector);
            left = complex_selector.complete(p, CSS_COMPLEX_SELECTOR)
        } else {
            return Present(left);
        }
    }
}

#[inline]
fn is_at_compound_selector(p: &mut CssParser) -> bool {
    p.at(T![&]) || is_at_simple_selector(p) || p.at_ts(CssSubSelectorList::START_SET)
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
fn is_at_simple_selector(p: &mut CssParser) -> bool {
    is_at_namespace(p) || p.at(T![*]) || is_at_identifier(p)
}

#[inline]
fn parse_simple_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_simple_selector(p) {
        return Absent;
    }

    let namespace = parse_namespace(p);

    if p.at(T![*]) {
        parse_universal_selector(p, namespace)
    } else {
        parse_type_selector(p, namespace)
    }
}

#[inline]
fn is_at_namespace(p: &mut CssParser) -> bool {
    p.at(T![|]) || is_at_namespace_prefix(p) && p.nth_at(1, T![|])
}

#[inline]
fn parse_namespace(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_namespace(p) {
        return Absent;
    }

    let m = p.start();

    // we don't need diagnostic here, because prefix is optional
    parse_namespace_prefix(p).ok();
    p.bump(T![|]);

    Present(m.complete(p, CSS_NAMESPACE))
}

#[inline]
fn is_at_namespace_prefix(p: &mut CssParser) -> bool {
    p.at(T![*]) || is_at_identifier(p)
}

#[inline]
fn parse_namespace_prefix(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_namespace_prefix(p) {
        return Absent;
    }

    let m = p.start();

    let kind = if p.eat(T![*]) {
        CSS_UNIVERSAL_NAMESPACE_PREFIX
    } else {
        // we don't need to check if the identifier is valid, because we already did that
        parse_regular_identifier(p).ok();
        CSS_NAMED_NAMESPACE_PREFIX
    };

    Present(m.complete(p, kind))
}

pub(crate) struct CssSubSelectorList;
impl CssSubSelectorList {
    pub(crate) const START_SET: TokenSet<CssSyntaxKind> =
        token_set![T![#], T![.], T![:], T![::], T!['[']];
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
            expected_any_sub_selector,
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
    parse_selector_custom_identifier(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_CLASS_SELECTOR))
}

#[inline]
pub(crate) fn parse_id_selector(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![#]) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![#]);
    parse_selector_custom_identifier(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_ID_SELECTOR))
}

#[inline]
pub(crate) fn parse_universal_selector(p: &mut CssParser, namespace: ParsedSyntax) -> ParsedSyntax {
    if !p.at(T![*]) {
        return Absent;
    }

    let m = namespace.precede(p);

    let context = selector_lex_context(p);
    p.eat_with_context(T![*], context);

    Present(m.complete(p, CSS_UNIVERSAL_SELECTOR))
}

#[inline]
fn parse_type_selector(p: &mut CssParser, namespace: ParsedSyntax) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = namespace.precede(p);

    parse_selector_identifier(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_TYPE_SELECTOR))
}

#[inline]
fn parse_selector_identifier(p: &mut CssParser) -> ParsedSyntax {
    let context = selector_lex_context(p);
    parse_identifier(p, context)
}

/// Custom identifiers are used for class names and ids in selectors and are
/// case-sensitive. These are distinguished from regular identifiers in
/// selectors that are case-insensitive for safety in preserving the casing.
#[inline]
fn parse_selector_custom_identifier(p: &mut CssParser) -> ParsedSyntax {
    let context = selector_lex_context(p);
    // Class and ID selectors are technically `<ident>` _and_ case-sensitive.
    // To handle this, we use `<custom-ident>` instead, but also have to allow
    // the CSS-wide keywords to include selectors like `.inherit`, which is
    // valid as a regular ident.
    parse_custom_identifier_with_keywords(p, context, true)
}

#[inline]
pub(crate) fn eat_or_recover_selector_function_close_token<'a, E, D>(
    p: &mut CssParser<'a>,
    parameter: CompletedMarker,
    error_builder: E,
) -> bool
where
    E: FnOnce(&CssParser, TextRange) -> D,
    D: ToDiagnostic<CssParser<'a>>,
{
    let context = selector_lex_context(p);

    if p.eat_with_context(T![')'], context) {
        true
    } else {
        if let Ok(m) = ParseRecovery::new(CSS_BOGUS, SELECTOR_FUNCTION_RECOVERY_SET)
            .enable_recovery_on_line_break()
            .recover(p)
        {
            let diagnostic = error_builder(
                p,
                TextRange::new(parameter.range(p).start(), m.range(p).end()),
            );
            p.error(diagnostic);
        }

        let context = selector_lex_context(p);
        p.expect_with_context(T![')'], context);

        false
    }
}

#[inline]
pub(crate) fn recover_selector_function_parameter<'a, E, D>(p: &mut CssParser<'a>, error_builder: E)
where
    E: FnOnce(&CssParser, TextRange) -> D,
    D: ToDiagnostic<CssParser<'a>>,
{
    let start = p.cur_range().start();

    let range = ParseRecovery::new(CSS_BOGUS, SELECTOR_FUNCTION_RECOVERY_SET)
        .enable_recovery_on_line_break()
        .recover(p)
        .map(|m| m.range(p))
        .unwrap_or_else(|_| p.cur_range());

    let diagnostic = error_builder(p, TextRange::new(start, range.end()));
    p.error(diagnostic);
}
