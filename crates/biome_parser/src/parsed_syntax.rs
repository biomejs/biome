use crate::parse_recovery::{ParseRecovery, ParseRecoveryTokenSet, RecoveryResult};
use crate::parsed_syntax::ParsedSyntax::{Absent, Present};
use crate::prelude::*;
use biome_rowan::TextRange;

/// Syntax that is either present in the source tree or absent.
///
/// This type is commonly used as the return type of parse functions with the following types
///
///
/// ## Parse Rule conventions
///
/// * A parse rule must return [ParsedSyntax::Present] if it is able to parse a node or at least parts of it. For example,
///     the `parse_for_statement` should return [ParsedSyntax::Present] for `for (` even tough many of the required children are missing
///     because it is still able to parse parts of the for statement.
/// * A parse rule must return [ParsedSyntax::Absent] if the expected node isn't present in the source code.
///     In most cases, this means if the first expected token isn't present, for example,
///     if the `for` keyword isn't present when parsing a for statement.
///
/// However, it can be possible for rules to recover even if the first token doesn't match. One example
/// is when parsing an assignment target that has an optional default. The rule can recover even
/// if the assignment target is missing as long as the cursor is then positioned at an `=` token.
///
/// The rule must then return [ParsedSyntax::Present] with the partial parsed node.
/// * A parse rule must not eat any tokens when it returns [ParsedSyntax::Absent]
/// * A parse rule must not add any errors when it returns [ParsedSyntax::Absent]
///
/// This is a custom enum over using `Option` because [ParsedSyntax::Absent] values must be handled by the caller.
#[derive(Debug, PartialEq, Eq)]
#[must_use = "this `ParsedSyntax` may be an `Absent` variant, which should be handled"]
pub enum ParsedSyntax {
    /// A syntax that isn't present in the source code. Used when a parse rule can't match the current
    /// token of the parser.
    Absent,

    /// A completed syntax node with all or some of its children.
    Present(CompletedMarker),
}

impl ParsedSyntax {
    /// Converts from `ParsedSyntax` to `Option<CompletedMarker>`.
    ///
    /// Converts `self` into an `Option<CompletedMarker>`, consuming `self`
    #[inline]
    pub fn ok(self) -> Option<CompletedMarker> {
        match self {
            Absent => None,
            Present(marker) => Some(marker),
        }
    }

    /// Calls `op` if the syntax is present and otherwise returns [ParsedSyntax::Absent]
    #[inline]
    pub fn and_then<F>(self, op: F) -> ParsedSyntax
    where
        F: FnOnce(CompletedMarker) -> ParsedSyntax,
    {
        match self {
            Absent => Absent,
            Present(marker) => op(marker),
        }
    }

    /// Calls `op` if the syntax is absent and otherwise returns [ParsedSyntax::Present]
    #[inline]
    pub fn or_else<F>(self, op: F) -> ParsedSyntax
    where
        F: FnOnce() -> ParsedSyntax,
    {
        match self {
            Absent => op(),
            t => t,
        }
    }

    /// Returns `true` if the parsed syntax is [ParsedSyntax::Present]
    #[inline]
    #[must_use]
    pub fn is_present(&self) -> bool {
        matches!(self, Present(_))
    }

    /// Returns `true` if the parsed syntax is [ParsedSyntax::Absent]
    #[inline]
    #[must_use]
    pub fn is_absent(&self) -> bool {
        matches!(self, Absent)
    }

    /// It returns the contained [ParsedSyntax::Present] value, consuming the `self` value
    ///
    /// # Panics
    ///
    ///  Panics if the current syntax is [ParsedSyntax::Absent]
    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> CompletedMarker {
        match self {
            Absent => {
                panic!("Called `unwrap` on an `Absent` syntax");
            }
            Present(marker) => marker,
        }
    }

    /// Returns the contained [ParsedSyntax::Present] value or passed default
    #[inline]
    pub fn unwrap_or(self, default: CompletedMarker) -> CompletedMarker {
        match self {
            Absent => default,
            Present(marker) => marker,
        }
    }

    /// Returns the contained [ParsedSyntax::Present] value or computes it from a clojure.
    #[inline]
    pub fn unwrap_or_else<F>(self, default: F) -> CompletedMarker
    where
        F: FnOnce() -> CompletedMarker,
    {
        match self {
            Absent => default(),
            Present(marker) => marker,
        }
    }

    /// Returns the contained [ParsedSyntax::Present] value, consuming the self value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [ParsedSyntax::Absent] with a custom panic message provided by msg.
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> CompletedMarker {
        match self {
            Present(marker) => marker,
            Absent => panic!("{}", msg),
        }
    }

    /// Maps a [ParsedSyntax::Present] `ParsedSyntax` by applying a function to a contained [ParsedSyntax::Present] value,
    /// leaving an [ParsedSyntax::Absent] value untouched.
    ///
    /// This function can be used to compose the results of two functions.
    pub fn map<F>(self, mapper: F) -> ParsedSyntax
    where
        F: FnOnce(CompletedMarker) -> CompletedMarker,
    {
        match self {
            Absent => Absent,
            Present(element) => Present(mapper(element)),
        }
    }

    /// Returns the kind of the syntax if it is present or [None] otherwise
    #[inline]
    pub fn kind<P>(&self, p: &P) -> Option<P::Kind>
    where
        P: Parser,
    {
        match self {
            Absent => None,
            Present(marker) => Some(marker.kind(p)),
        }
    }

    /// Adds a diagnostic at the current parser position if the syntax is present and return its marker.
    #[inline]
    pub fn add_diagnostic_if_present<P, E, D>(
        self,
        p: &mut P,
        error_builder: E,
    ) -> Option<CompletedMarker>
    where
        P: Parser,
        E: FnOnce(&P, TextRange) -> D,
        D: ToDiagnostic<P>,
    {
        match self {
            Present(syntax) => {
                let range = syntax.range(p);
                let range = TextRange::new(range.start(), range.end());
                let diagnostic = error_builder(p, range);
                p.error(diagnostic);
                Some(syntax)
            }
            Absent => None,
        }
    }

    /// It returns the syntax if present or adds a diagnostic at the current parser position.
    #[inline]
    pub fn or_add_diagnostic<P, E, D>(self, p: &mut P, error_builder: E) -> Option<CompletedMarker>
    where
        P: Parser,
        E: FnOnce(&P, TextRange) -> D,
        D: ToDiagnostic<P>,
    {
        match self {
            Present(syntax) => Some(syntax),
            Absent => {
                let diagnostic = error_builder(p, p.cur_range());
                p.error(diagnostic);
                None
            }
        }
    }

    /// It creates and returns a marker preceding this parsed syntax if it is present or starts
    /// a new marker and adds an error to the current parser position.
    /// See [CompletedMarker.precede]
    #[inline]
    pub fn precede_or_add_diagnostic<P, E, D>(self, p: &mut P, error_builder: E) -> Marker
    where
        P: Parser,
        E: FnOnce(&P, TextRange) -> D,
        D: ToDiagnostic<P>,
    {
        match self {
            Present(completed) => completed.precede(p),
            Absent => {
                let diagnostic = error_builder(p, p.cur_range());
                p.error(diagnostic);
                p.start()
            }
        }
    }

    /// Creates a new marker that precedes this syntax or starts a new marker
    #[inline]
    pub fn precede<P>(self, p: &mut P) -> Marker
    where
        P: Parser,
    {
        match self {
            Present(marker) => marker.precede(p),
            Absent => p.start(),
        }
    }

    /// Returns this Syntax if it is present in the source text or tries to recover the
    /// parser if the syntax is absent. The recovery...
    ///
    /// * eats all unexpected tokens into a `Bogus*` node until the parser reaches one
    ///   of the "safe tokens" configured in the [ParseRecoveryTokenSet].
    /// * creates an error using the passed in error builder and adds it to the parsing diagnostics.
    ///
    /// The error recovery can fail if the parser is located at the EOF token or if the parser
    /// is already at a valid position according to the [ParseRecoveryTokenSet].
    pub fn or_recover_with_token_set<P, E>(
        self,
        p: &mut P,
        recovery: &ParseRecoveryTokenSet<P::Kind>,
        error_builder: E,
    ) -> RecoveryResult
    where
        P: Parser,
        E: FnOnce(&P, TextRange) -> ParseDiagnostic,
    {
        match self {
            Present(syntax) => Ok(syntax),
            Absent => match recovery.recover(p) {
                Ok(recovered) => {
                    let diagnostic = error_builder(p, recovered.range(p));
                    p.error(diagnostic);
                    Ok(recovered)
                }

                Err(recovery_error) => {
                    let diagnostic = error_builder(p, p.cur_range());
                    p.error(diagnostic);
                    Err(recovery_error)
                }
            },
        }
    }

    /// Returns this Syntax if it is present in the source text or tries to recover the
    /// parser if the syntax is absent. The recovery...
    ///
    /// * eats all unexpected tokens into a `Bogus*` node until the parser reaches one
    ///   of the "safe tokens" configured in the [ParseRecovery].
    /// * creates an error using the passed in error builder and adds it to the parsing diagnostics.
    ///
    /// The error recovery can fail if the parser is located at the EOF token or if the parser
    /// is already at a valid position according to the [ParseRecovery].
    pub fn or_recover<'source, P, E, R>(
        self,
        p: &mut P,
        recovery: &R,
        error_builder: E,
    ) -> RecoveryResult
    where
        P: Parser,
        R: ParseRecovery<Parser<'source> = P>,
        E: FnOnce(&P, TextRange) -> ParseDiagnostic,
    {
        match self {
            Present(syntax) => Ok(syntax),
            Absent => match recovery.recover(p) {
                Ok(recovered) => {
                    let diagnostic = error_builder(p, recovered.range(p));
                    p.error(diagnostic);
                    Ok(recovered)
                }

                Err(recovery_error) => {
                    let diagnostic = error_builder(p, p.cur_range());
                    p.error(diagnostic);
                    Err(recovery_error)
                }
            },
        }
    }
}

impl From<CompletedMarker> for ParsedSyntax {
    fn from(marker: CompletedMarker) -> Self {
        Present(marker)
    }
}

impl From<Option<CompletedMarker>> for ParsedSyntax {
    fn from(option: Option<CompletedMarker>) -> Self {
        match option {
            Some(completed) => Present(completed),
            None => Absent,
        }
    }
}
