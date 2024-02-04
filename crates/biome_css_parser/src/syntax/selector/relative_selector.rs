use crate::parser::CssParser;
use crate::syntax::parse_error::expected_relative_selector;
use crate::syntax::selector::{is_nth_at_compound_selector, parse_selector};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryError, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser, TokenSet};

/// Represents a list of relative CSS selectors with support for error recovery.
///
/// This structure is used to parse and manage a list of relative CSS selectors
/// within the context of a larger CSS parsing operation. It includes mechanisms
/// for disabling error recovery if needed. For more details on CSS selectors,
/// see the [CSS Selectors Level 4 Specification](https://www.w3.org/TR/selectors-4/#relative).
///
/// ## Example
///
/// A relative selector might look like `"E > F"` where `E` is an element that is
/// a parent of an element `F`. In the context of this structure, a list might be
/// parsed from a string like `"div > p, ul > li:first-child"`.
pub(crate) struct RelativeSelectorList {
    /// The type of CSS syntax that marks the end of this selector list.
    end_kind: CssSyntaxKind,
    /// Flag indicating whether error recovery is disabled.
    is_recovery_disabled: bool,
}

impl RelativeSelectorList {
    /// Constructs a new `RelativeSelectorList`.
    ///
    /// Initializes the list with a specified end kind, with error recovery enabled by default.
    ///
    /// # Arguments
    ///
    /// * `end_kind` - A `CssSyntaxKind` that indicates the end of the selector list.
    pub(crate) fn new(end_kind: CssSyntaxKind) -> Self {
        RelativeSelectorList {
            end_kind,
            is_recovery_disabled: false,
        }
    }

    /// Disables error recovery for this selector list.
    ///
    /// This method modifies the instance to disable the error recovery mechanism.
    /// It is useful when strict parsing is required without fallbacks.
    pub(crate) fn disable_recovery(mut self) -> Self {
        self.is_recovery_disabled = true;
        self
    }
}

/// Defines error recovery behavior for parsing `RelativeSelectorList`.
///
/// This structure is utilized when the parser encounters errors while parsing
/// a list of relative CSS selectors, allowing it to recover and continue.
pub(crate) struct RelativeSelectorListParseRecovery {
    /// The type of CSS syntax that indicates a potential recovery point.
    end_kind: CssSyntaxKind,
}

impl RelativeSelectorListParseRecovery {
    /// Constructs a new `RelativeSelectorListParseRecovery`.
    ///
    /// Initializes the recovery mechanism with a specified end kind.
    ///
    /// # Arguments
    ///
    /// * `end_kind` - A `CssSyntaxKind` used to determine the recovery point.
    pub(crate) fn new(end_kind: CssSyntaxKind) -> Self {
        RelativeSelectorListParseRecovery { end_kind }
    }
}

impl ParseRecovery for RelativeSelectorListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_SELECTOR;

    /// Determines if the current parser position is a point of recovery.
    ///
    /// Recovery is attempted if the current position matches the `end_kind`,
    /// a comma, or is identified as a relative selector by additional checks.
    /// This ensures parsing can resume smoothly after an error, by identifying
    /// specific tokens or conditions that indicate a safe point for continuation.
    ///
    /// # CSS Examples
    ///
    /// - Recovery at `end_kind` (assuming `end_kind` is `{` for demonstration):
    ///   ```css
    ///   .class1 , 12312312                    {
    ///   /*        ^^^^^^^^ invalid selector   ^ recovery point at end_kind */
    ///   ```
    ///
    /// - Recovery at Comma (`,`) between selectors:
    ///   ```css
    ///   .class1 , 12312312                    , .class3 {
    ///   /*        ^^^^^^^^ invalid selector   ^ recovery point at comma */
    ///   ```
    ///
    /// - Recovery at Relative Selector:
    ///   - When the parser identifies a combinator indicating the start of a new relative selector:
    ///     ```css
    ///     .class1 , 12312312                      > .class2 {
    ///     /*        ^^^^^^^^ invalid selector     ^ recovery point at new relative selector */
    ///     ```
    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(self.end_kind) || p.at(T![,]) || is_at_relative_selector(p)
    }
}

impl ParseSeparatedList for RelativeSelectorList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const LIST_KIND: CssSyntaxKind = CSS_RELATIVE_SELECTOR_LIST;

    fn parse_element(&mut self, p: &mut CssParser) -> ParsedSyntax {
        parse_relative_selector(p)
    }

    fn is_at_list_end(&self, p: &mut CssParser) -> bool {
        p.at(self.end_kind)
    }

    fn recover(&mut self, p: &mut CssParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        if parsed_element.is_absent() && self.is_recovery_disabled {
            Err(RecoveryError::RecoveryDisabled)
        } else {
            parsed_element.or_recover(
                p,
                &RelativeSelectorListParseRecovery::new(self.end_kind),
                expected_relative_selector,
            )
        }
    }

    fn separating_element_kind(&mut self) -> CssSyntaxKind {
        T![,]
    }
}

/// A set of CSS syntax kinds representing combinator types for relative selectors.
///
/// This constant defines a set of combinators used in CSS to establish relationships
/// between selectors. It includes combinators like `>`, `+`, `~`, and `||`, which
/// are used to specify parent-child, adjacent siblings, general siblings, and column
/// combinator selectors, respectively.
const RELATIVE_SELECTOR_COMBINATOR_SET: TokenSet<CssSyntaxKind> =
    token_set![T![>], T![+], T![~], T![||]];

/// Checks if the current position in the parser is at a relative selector combinator.
///
/// This function uses `RELATIVE_SELECTOR_COMBINATOR_SET` to determine if the current
/// position in the CSS parser is at a combinator that can be used in a relative selector.
#[inline]
fn is_at_relative_selector_combinator(p: &mut CssParser) -> bool {
    p.at_ts(RELATIVE_SELECTOR_COMBINATOR_SET)
}

/// Determines if the parser is positioned at a relative selector.
///
/// This function checks if the current position in the CSS parser is at a relative selector
/// by checking for the presence of a combinator or if it's at the start of a compound selector.
#[inline]
pub(crate) fn is_at_relative_selector(p: &mut CssParser) -> bool {
    is_at_relative_selector_combinator(p) || is_nth_at_compound_selector(p, 0)
}

/// Parses a relative selector from the current position in the CSS parser.
///
/// This function attempts to parse a relative selector starting from the current position. If the position
/// does not indicate a relative selector, it returns `Absent`. Otherwise, it parses the selector, adding
/// diagnostics if necessary, and returns `Present` with the parsed syntax.
#[inline]
fn parse_relative_selector(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_relative_selector(p) {
        return Absent;
    }

    let m = p.start();

    // Consume the combinator from the set of relative selector combinators.
    p.eat_ts(RELATIVE_SELECTOR_COMBINATOR_SET);
    // Attempt to parse the selector, adding a diagnostic if it fails.
    parse_selector(p).or_add_diagnostic(p, expected_relative_selector);

    // Mark the end of the parsing operation and return the result.
    Present(m.complete(p, CSS_RELATIVE_SELECTOR))
}
