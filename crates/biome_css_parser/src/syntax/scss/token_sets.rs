use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::{TokenSet, token_set};

pub(crate) const END_OF_SCSS_EXPRESSION_TOKEN_SET: TokenSet<CssSyntaxKind> =
    token_set![T![')'], T![;], T!['}']];
pub(crate) const SCSS_NESTING_VALUE_END_SET: TokenSet<CssSyntaxKind> =
    token_set![T!['{'], T![;], T!['}'], T![!], EOF];
pub(crate) const SCSS_VARIABLE_MODIFIER_LIST_END_SET: TokenSet<CssSyntaxKind> =
    token_set![T![;], T!['}'], EOF];

pub(crate) const SCSS_STATEMENT_START_SET: TokenSet<CssSyntaxKind> = token_set![
    T![@],
    T![$],
    T![&],
    T![.],
    T![#],
    T![:],
    T![::],
    T!['['],
    T![*],
    T![|],
    T![>],
    T![+],
    T![~],
    T![||],
];

pub(crate) const SCSS_IDENT_CONTINUATION_SET: TokenSet<CssSyntaxKind> = token_set![
    T![.],
    T![#],
    T![:],
    T![::],
    T!['['],
    T![|],
    T![>],
    T![+],
    T![~],
    T![||],
];
