use crate::parser::CssParser;
use biome_css_syntax::CssSyntaxKind::CSS_BOGUS;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::Parser;
use biome_parser::parse_recovery::ParseRecovery;

pub(crate) struct AnyQueryParseRecovery;

impl ParseRecovery for AnyQueryParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // Skips malformed or incomplete queries in parentheses until:
        // 1) '{' (start of a declaration block),
        // 3) a line break (new statement boundary).
        p.at(T!['{']) || p.has_preceding_line_break()
    }
}

pub(crate) struct AnyInParensParseRecovery;

impl ParseRecovery for AnyInParensParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // Skips malformed or incomplete queries in parentheses until:
        // 1) '{' (start of a declaration block),
        // 2) ')' (closing this query context), or
        // 3) a line break (new statement boundary).
        p.at(T!['{']) || p.at(T![')']) || p.has_preceding_line_break()
    }
}

pub(crate) struct AnyInParensChainParseRecovery {
    chain_kind: CssSyntaxKind,
}

impl AnyInParensChainParseRecovery {
    pub(crate) fn new(chain_kind: CssSyntaxKind) -> Self {
        Self { chain_kind }
    }
}

impl ParseRecovery for AnyInParensChainParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // Skips malformed or incomplete queries in parentheses until:
        // 1) '{' (start of a declaration block),
        // 2) 'chain_kind' (another query context), or
        // 3) a line break (new statement boundary).
        p.at(T!['{']) || p.at(self.chain_kind) || p.has_preceding_line_break()
    }
}
