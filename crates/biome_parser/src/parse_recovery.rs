use crate::prelude::*;
use biome_rowan::SyntaxKind;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum RecoveryError {
    /// Recovery failed because the parser reached the end of file
    Eof,

    /// Recovery failed because it didn't eat any tokens. Meaning, the parser is already in a recovered state.
    /// This is an error because:
    /// a) It shouldn't create a completed marker wrapping no tokens
    /// b) This results in an infinite-loop if the recovery is used inside a while loop. For example,
    ///    it's common that list parsing also recovers at the end of a statement or block. However, list elements
    ///    don't start with a `;` or `}` which is why parsing, for example, an array element fails again and
    ///    the array expression triggers another recovery. Handling this as an error ensures that list parsing
    ///    rules break out of the loop the same way as they would at the EOF.
    AlreadyRecovered,

    /// Returned if there's an unexpected token and the parser is speculatively parsing a syntax.
    /// Error-recovery is disabled when doing speculative parsing because it can then make the impression
    /// that the parser was able to correctly parse a syntax when, in fact, it only skipped over the tokens.
    ///
    /// For example the syntax `(a, b, c) ...` in JavaScript can either be a parenthesized expression
    /// or an arrow function, depending on what kind of token `...` is. Thus, the parer's only option is
    /// to make the following assumption:
    /// "Let's assume `(a, b, c) ...`" is an arrow function expression"
    ///
    /// The parser then tries to parse `(a, b, c)` as an arrow function parameters and validates that `...`
    /// indeed is the `=>`. The parser rewinds and re-parses the syntax as a parenthesized expression
    /// if it turns out that `...` isn't the `=>` token or if any element in `(a, b, c)` isn't a valid parameter (for example `5 + 3` isn't valid).
    RecoveryDisabled,
}

impl Error for RecoveryError {}

impl Display for RecoveryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RecoveryError::Eof => write!(f, "EOF"),
            RecoveryError::AlreadyRecovered => write!(f, "already recovered"),
            RecoveryError::RecoveryDisabled => write!(f, "recovery disabled"),
        }
    }
}

pub type RecoveryResult = Result<CompletedMarker, RecoveryError>;

/// Recovers the parser by finding a token/point (depending on the configuration) from where
/// the caller knows how to proceed parsing. The recovery wraps all the skipped tokens inside a `Bogus` node.
/// A safe recovery point for an array element could by finding the next `,` or `]`.
pub struct ParseRecoveryTokenSet<K: SyntaxKind> {
    node_kind: K,
    recovery_set: TokenSet<K>,
    line_break: bool,
}

impl<K: SyntaxKind> ParseRecoveryTokenSet<K> {
    /// Creates a new parse recovery that eats all tokens until it finds any token in the passed recovery set.
    pub fn new(node_kind: K, recovery_set: TokenSet<K>) -> Self {
        Self {
            node_kind,
            recovery_set,
            line_break: false,
        }
    }

    /// Enable recovery on line breaks
    pub fn enable_recovery_on_line_break(mut self) -> Self {
        self.line_break = true;
        self
    }

    // TODO: Add a `recover_until` which recovers until the parser reached a token inside of the recovery set
    // or the passed in `parse_*` rule was able to successfully parse an element.

    /// Tries to recover by parsing all tokens into an `Bogus*` node until the parser finds any token
    /// specified in the recovery set, the EOF, or a line break (depending on configuration).
    /// Returns `Ok(bogus_node)` if recovery was successful, and `Err(RecoveryError::Eof)` if the parser
    /// is at the end of the file (before starting recovery).
    pub fn recover<P>(&self, p: &mut P) -> RecoveryResult
    where
        P: Parser<Kind = K>,
    {
        if p.at(P::Kind::EOF) {
            return Err(RecoveryError::Eof);
        }

        if self.is_at_recovered(p) {
            return Err(RecoveryError::AlreadyRecovered);
        }

        if p.is_speculative_parsing() {
            return Err(RecoveryError::RecoveryDisabled);
        }

        let m = p.start();

        while !(self.is_at_recovered(p) || p.at(P::Kind::EOF)) {
            p.bump_any();
        }

        Ok(m.complete(p, self.node_kind))
    }

    #[inline]
    fn is_at_recovered<P>(&self, p: &P) -> bool
    where
        P: Parser<Kind = K>,
    {
        p.at_ts(self.recovery_set) || (self.line_break && p.has_preceding_line_break())
    }
}

/// Recovers the parser by finding a token/point (depending on the configuration) from where
/// the caller knows how to proceed parsing. The recovery wraps all the skipped tokens inside a `Bogus` node.
pub trait ParseRecovery {
    type Kind: SyntaxKind;
    type Parser<'source>: Parser<Kind = Self::Kind>;

    /// The kind of the recovered node
    const RECOVERED_KIND: Self::Kind;

    /// Checks if the parser is in a recovered state.
    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool;

    /// Tries to recover by parsing all tokens into an `Bogus*` node until the parser finds any token
    /// specified in the recovery set, the EOF, or a line break (depending on configuration).
    /// Returns `Ok(bogus_node)` if recovery was successful, and `Err(RecoveryError::Eof)` if the parser
    /// is at the end of the file (before starting recovery).
    fn recover(&self, p: &mut Self::Parser<'_>) -> RecoveryResult {
        if p.at(Self::Kind::EOF) {
            return Err(RecoveryError::Eof);
        }

        if self.is_at_recovered(p) {
            return Err(RecoveryError::AlreadyRecovered);
        }

        if p.is_speculative_parsing() {
            return Err(RecoveryError::RecoveryDisabled);
        }

        let m = p.start();

        while !(self.is_at_recovered(p) || p.at(Self::Kind::EOF)) {
            p.bump_any();
        }

        Ok(m.complete(p, Self::RECOVERED_KIND))
    }
}
