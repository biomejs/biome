mod attribute;
mod nested_selector;
mod pseudo_class;
mod pseudo_element;
pub(crate) mod relative_selector;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::{
    expected_any_sub_selector, expected_compound_selector, expected_identifier, expected_selector,
};
use crate::syntax::selector::attribute::parse_attribute_selector;
use crate::syntax::selector::nested_selector::NestedSelectorList;
use crate::syntax::selector::pseudo_class::parse_pseudo_class_selector;
use crate::syntax::selector::pseudo_element::parse_pseudo_element_selector;
use crate::syntax::{
    is_at_identifier, is_nth_at_identifier, parse_custom_identifier_with_keywords,
    parse_identifier, parse_regular_identifier,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, TextRange, T};
use biome_parser::diagnostic::ToDiagnostic;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{
    ParseRecovery, ParseRecoveryTokenSet, RecoveryError, RecoveryResult,
};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, CompletedMarker, Parser, ParserProgress, TokenSet};

use super::{is_nth_at_metavariable, parse_metavariable};

/// Determines the lexical context for parsing CSS selectors.
///
/// This function is applied when lexing CSS selectors. It decides whether the
/// current context should be treated as a regular context or a selector-specific
/// context. The distinction is important for handling whitespaces, especially
/// around combinators in CSS selectors.
const SELECTOR_LEX_SET: TokenSet<CssSyntaxKind> =
    COMPLEX_SELECTOR_COMBINATOR_SET.union(token_set![T!['{'], T![,], T![')']]);
#[inline]
fn selector_lex_context(p: &mut CssParser) -> CssLexContext {
    // It's an inverted logic for `is_nth_at_selector(p, 1)`.
    if p.nth_at_ts(1, SELECTOR_LEX_SET) {
        CssLexContext::Regular
    } else {
        CssLexContext::Selector
    }
}

pub(crate) struct SelectorList {
    end_kind_ts: TokenSet<CssSyntaxKind>,
    recovery_ts: TokenSet<CssSyntaxKind>,
    is_recovery_disabled: bool,
}

impl Default for SelectorList {
    fn default() -> Self {
        SelectorList {
            end_kind_ts: token_set!(T!['{']),
            recovery_ts: token_set![T!['{']],
            is_recovery_disabled: false,
        }
    }
}

impl SelectorList {
    /// Configures the `SelectorList` with a specified token set to indicate the end of the selector list.
    ///
    /// This method allows setting a custom `TokenSet<CssSyntaxKind>` that determines the tokens
    /// which mark the end of a selector list. It can be used to extend or modify the default
    /// behavior of the selector list parsing.
    pub(crate) fn with_end_kind_ts(mut self, end_kind_ts: TokenSet<CssSyntaxKind>) -> Self {
        self.end_kind_ts = end_kind_ts;
        self
    }

    /// Configures the `SelectorList` with a specified set of tokens for error recovery.
    ///
    /// This method allows setting a custom `TokenSet<CssSyntaxKind>` which is used for
    /// identifying points at which the parser can attempt to recover from errors during
    /// the parsing process. The specified token set represents tokens that the parser will
    /// recognize as potential recovery points.
    pub(crate) fn with_recovery_ts(mut self, recovery_ts: TokenSet<CssSyntaxKind>) -> Self {
        self.recovery_ts = recovery_ts;
        self
    }

    /// Disables error recovery for the selector list parsing.
    ///
    /// By default, the parser might attempt to recover from errors encountered while parsing
    /// the selector list. This method disables such recovery, which can be useful in scenarios
    /// where we want to implement a custom recovery.
    pub(crate) fn disable_recovery(mut self) -> Self {
        self.is_recovery_disabled = true;
        self
    }
}

impl ParseSeparatedList for SelectorList {
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
                &SelectorListParseRecovery::new(self.recovery_ts),
                expected_selector,
            )
        }
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

struct SelectorListParseRecovery {
    recovery_ts: TokenSet<CssSyntaxKind>,
}

impl SelectorListParseRecovery {
    fn new(recovery_ts: TokenSet<CssSyntaxKind>) -> Self {
        SelectorListParseRecovery { recovery_ts }
    }
}

impl ParseRecovery for SelectorListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_SELECTOR;

    /// Determines if the parser is at a point where it can recover from an error
    /// while parsing a selector list.
    ///
    /// This method checks if the parser is currently positioned at a token that
    /// indicates a safe point to resume parsing after encountering an error in a
    /// selector list. The recovery points are identified by `recovery_ts`, a comma,
    /// the start of a new selector, or a preceding line break.
    /// # CSS Examples
    ///
    /// - Recovery at `recovery_ts` (e.g., `{`):
    ///   ```css
    ///   .class1, { }
    ///   /*       ^                   */
    ///   /*       is a recovery point */
    ///   ```
    ///
    /// - Recovery at Comma (`,`):
    ///   ```css
    ///   .class1, "string", div {}
    ///   /*               ^                   */
    ///   /*               is a recovery point */
    ///   ```
    ///
    /// - Recovery at New Selector (`is_nth_at_selector`):
    ///   ```css
    ///   .class1, "string" div {}
    ///   /*                ^                   */
    ///   /*                is a recovery point */
    ///   ```
    ///
    /// - Recovery at Line Break (`has_nth_preceding_line_break`):
    ///   ```css
    ///   .class1 123123
    ///    /*new line is a recovery point*/  color: red;
    ///   }
    ///   ```
    ///
    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(self.recovery_ts)
            || p.at(T![,])
            || is_nth_at_selector(p, 0)
            || p.has_nth_preceding_line_break(1)
    }
}

/// Determines if the current or nth token in the parser is at the start of a selector.
///
/// This function checks whether the specified position in the CSS parser aligns with
/// the beginning of a compound selector. In CSS, selectors are patterns used to select
/// the elements to which a set of CSS rules apply.
#[inline]
pub(crate) fn is_nth_at_selector(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_compound_selector(p, n) || is_nth_at_metavariable(p, n)
}

/// Parses a CSS selector.
///
/// This function attempts to parse a CSS selector, which may be either a compound selector
/// or a complex selector. Compound selectors are simple, unseparated chains of simple selectors,
/// whereas complex selectors are compound selectors separated by combinators.
///
/// Initially, the function tries to parse a compound selector. If successful, it then checks
/// if this compound selector forms part of a complex selector and continues parsing accordingly.
#[inline]
pub(crate) fn parse_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_nth_at_selector(p, 0) {
        return Absent;
    }
    if is_nth_at_metavariable(p, 0) {
        parse_metavariable(p)
    } else {
        // In CSS, we have compound selectors and complex selectors.
        // Compound selectors are simple, unseparated chains of selectors,
        // while complex selectors are compound selectors separated by combinators.
        // After parsing the compound selector, it then checks if this compound selector is a part of a complex selector.
        parse_compound_selector(p).and_then(|selector| parse_complex_selector(p, selector))
    }
}

const COMPLEX_SELECTOR_COMBINATOR_SET: TokenSet<CssSyntaxKind> =
    token_set![T![>], T![+], T![~], T![||], CSS_SPACE_LITERAL];

/// Checks if the current or nth token in the parser is a complex selector combinator.
///
/// This function determines whether the specified position in the CSS parser is at a
/// complex selector combinator. Complex selector combinators include characters like
/// '>', '+', ' ', and '~', used to define relationships between different elements in CSS.
#[inline]
fn is_nth_at_complex_selector_combinator(p: &mut CssParser, n: usize) -> bool {
    p.nth_at_ts(n, COMPLEX_SELECTOR_COMBINATOR_SET)
}

/// Parses a complex selector in CSS.
///
/// This function attempts to parse a complex selector, which combines compound selectors
/// through combinators like '>', '+', ' ', and '~'. These combinators express relationships
/// between elements in the document tree.
///
/// The function iterates over the tokens, parsing each complex selector combinator and
/// its associated compound selector. Parsing continues until no more complex combinators
/// are found, at which point it returns the completed complex selector.
#[inline]
fn parse_complex_selector(p: &mut CssParser, mut left: CompletedMarker) -> ParsedSyntax {
    let mut progress = ParserProgress::default();

    loop {
        progress.assert_progressing(p);

        if is_nth_at_complex_selector_combinator(p, 0) {
            let complex_selector = left.precede(p);

            p.bump_ts(COMPLEX_SELECTOR_COMBINATOR_SET);
            parse_compound_selector(p).or_add_diagnostic(p, expected_compound_selector);
            left = complex_selector.complete(p, CSS_COMPLEX_SELECTOR)
        } else {
            return Present(left);
        }
    }
}

/// Determines if the current or nth token in the parser is at the start of a compound selector.
///
/// This function checks if the specified position in the CSS parser corresponds to the start
/// of a compound selector. A compound selector combines multiple simple selectors, optionally
/// including a reference combinator ('&').
#[inline]
fn is_nth_at_compound_selector(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, T![&])
        || is_nth_at_simple_selector(p, n)
        || p.nth_at_ts(n, SubSelectorList::START_SET)
}

/// Parses a compound selector in CSS.
///
/// This function attempts to parse a compound selector from the current position in the
/// CSS parser. Compound selectors are combinations of simple selectors and optionally
/// a reference combinator ('&'). They match elements based on multiple conditions.
#[inline]
fn parse_compound_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_nth_at_compound_selector(p, 0) {
        return Absent;
    }

    let m = p.start();

    NestedSelectorList.parse_list(p);
    parse_simple_selector(p).ok(); // We don't need to handle error here because a simple selector is optional
    SubSelectorList.parse_list(p);

    Present(m.complete(p, CSS_COMPOUND_SELECTOR))
}

/// Checks if the current or nth token in the parser is a simple selector.
///
/// This function determines if the specified token position in the CSS parser is at the start
/// of a simple selector. Simple selectors are the basic building blocks of CSS selectors,
/// including type selectors, universal selectors, and attribute selectors.
#[inline]
fn is_nth_at_simple_selector(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_namespace(p, n) || p.nth_at(n, T![*]) || is_nth_at_identifier(p, n)
}

/// Parses a simple selector in CSS.
///
/// This function attempts to parse a simple selector from the current position in the CSS parser.
/// Simple selectors are the most basic component of CSS selectors, including type selectors,
/// universal selectors, and attribute selectors.
#[inline]
fn parse_simple_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_nth_at_simple_selector(p, 0) {
        return Absent;
    }

    let namespace = parse_namespace(p);

    if p.at(T![*]) {
        parse_universal_selector(p, namespace)
    } else {
        parse_type_selector(p, namespace)
    }
}

/// Determines if the current or nth token in the parser is at a namespace or namespace prefix.
///
/// This function checks if the specified token position in the CSS parser is at a namespace
/// delimiter ('|') or at a namespace prefix. A namespace in CSS is used to differentiate
/// between different types of elements and attributes.
#[inline]
fn is_nth_at_namespace(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, T![|]) || is_nth_at_namespace_prefix(p, n) && p.nth_at(n + 1, T![|])
}

/// Parses a namespace declaration in CSS.
///
/// This function attempts to parse a namespace declaration from the current position
/// in the CSS parser. Namespaces in CSS are used for qualifying element and attribute
/// names by associating them with a namespace, defined by a URI.
#[inline]
fn parse_namespace(p: &mut CssParser) -> ParsedSyntax {
    if !is_nth_at_namespace(p, 0) {
        return Absent;
    }

    let m = p.start();

    // we don't need diagnostic here, because prefix is optional
    parse_namespace_prefix(p).ok();
    p.bump(T![|]);

    Present(m.complete(p, CSS_NAMESPACE))
}

/// Checks if the current or nth token in the parser is a namespace prefix.
///
/// This function determines whether the parser is currently positioned at a namespace
/// prefix, either a universal namespace prefix ('*') or a specific identifier.
/// Namespace prefixes in CSS are used in conjunction with namespace URIs to apply
/// styles to elements in that namespace.
#[inline]
fn is_nth_at_namespace_prefix(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, T![*]) || is_nth_at_identifier(p, n)
}

/// Parses a namespace prefix in CSS.
///
/// This function attempts to parse a namespace prefix, which can be a universal
/// namespace prefix ('*') or a named namespace prefix (an identifier). The namespace
/// prefix is used in CSS selectors to apply styles to elements within a specific namespace.
#[inline]
fn parse_namespace_prefix(p: &mut CssParser) -> ParsedSyntax {
    if !is_nth_at_namespace_prefix(p, 0) {
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

pub(crate) struct SubSelectorList;
impl SubSelectorList {
    pub(crate) const START_SET: TokenSet<CssSyntaxKind> =
        token_set![T![#], T![.], T![:], T![::], T!['[']];
}
impl ParseNodeList for SubSelectorList {
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
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_SUB_SELECTOR, Self::START_SET),
            expected_any_sub_selector,
        )
    }
}

/// Parses a sub-selector in CSS.
///
/// This function is responsible for identifying and parsing different types of sub-selectors
/// based on the current token in the CSS parser. It dispatches to specific parsing functions
/// for class selectors, ID selectors, attribute selectors, pseudo-classes, and pseudo-elements.
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

/// Parses a class selector in CSS.
///
/// This function attempts to parse a class selector, starting with a period ('.')
/// followed by an identifier. Class selectors are used to apply styles to elements
/// with the specified class attribute.
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

/// Parses an ID selector in CSS.
///
/// This function attempts to parse an ID selector, which starts with a hash ('#')
/// followed by an identifier. ID selectors are used to apply styles to a single
/// element with the specified ID attribute.
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

/// Parses a universal selector ('*') in CSS.
///
/// This function attempts to parse a universal selector from the current position
/// in the CSS parser. The universal selector matches elements of any type.
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

/// Parses a type selector (e.g., 'div') in CSS.
///
/// This function attempts to parse a type selector, which matches elements of a specific type,
/// from the current position in the CSS parser. The type selector is typically an identifier
/// representing an HTML element type.
#[inline]
fn parse_type_selector(p: &mut CssParser, namespace: ParsedSyntax) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = namespace.precede(p);

    parse_selector_identifier(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, CSS_TYPE_SELECTOR))
}

/// Parses an identifier within a selector context in CSS.
///
/// This function parses an identifier, which is a fundamental part of various CSS selectors,
/// considering the specific lexical context of selectors. The context affects how the parser
/// interprets tokens, particularly with respect to whitespace handling and combinator parsing.
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

const SELECTOR_FUNCTION_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T![')'], T!['{']];

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
        if let Ok(m) = ParseRecoveryTokenSet::new(CSS_BOGUS, SELECTOR_FUNCTION_RECOVERY_SET)
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

    let range = ParseRecoveryTokenSet::new(CSS_BOGUS, SELECTOR_FUNCTION_RECOVERY_SET)
        .enable_recovery_on_line_break()
        .recover(p)
        .map_or_else(|_| p.cur_range(), |m| m.range(p));

    let diagnostic = error_builder(p, TextRange::new(start, range.end()));
    p.error(diagnostic);
}
