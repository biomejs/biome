pub(crate) struct CssParserState {
    /// Indicates that the parser is speculatively parsing a syntax. Speculative parsing means that the
    /// parser tries to parse a syntax as one kind and determines at the end if the assumption was right
    /// by testing if the parser is at a specific token (or has no errors). For this approach to work,
    /// the parser isn't allowed to skip any tokens while doing error recovery because it may then successfully
    /// skip over all invalid tokens, so that it appears as if it was able to parse the syntax correctly.
    ///
    /// Speculative parsing is useful if a syntax is ambiguous and no amount of lookahead (except parsing the whole syntax)
    /// is sufficient to determine what syntax it is. For example, the syntax `(a, b) ...`
    /// in JavaScript is either a parenthesized expression or an arrow expression if `...` is a `=>`.
    /// The challenge is, that it isn't possible to tell which of the two kinds it is until the parser
    /// processed all of `(a, b)`.
    pub(crate) speculative_parsing: bool,
}

impl CssParserState {
    pub fn new() -> Self {
        Self {
            speculative_parsing: false,
        }
    }
}
