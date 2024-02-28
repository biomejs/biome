use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_grit_syntax::T;
use biome_parser::{token_set, TokenSet};

pub(crate) const ACCESSOR_RECOVERY_SET: TokenSet<GritSyntaxKind> =
    token_set!(T![,], T![')'], T!['}'], T![']']);

pub(crate) const ARG_LIST_RECOVERY_SET: TokenSet<GritSyntaxKind> = token_set!(T![,], T![')']);

pub(crate) const BOOLEAN_VALUE_SET: TokenSet<GritSyntaxKind> = token_set![TRUE_KW, FALSE_KW];

pub(crate) const CODE_SNIPPET_SET: TokenSet<GritSyntaxKind> =
    SUPPORTED_LANGUAGE_SET.union(token_set![GRIT_BACKTICK_SNIPPET, GRIT_RAW_BACKTICK_SNIPPET]);

pub(crate) const CONTAINER_SET: TokenSet<GritSyntaxKind> =
    token_set![GRIT_VARIABLE, GRIT_MAP_ACCESSOR, GRIT_LIST_ACCESSOR];

pub(crate) const ELEMENT_LIST_RECOVERY_SET: TokenSet<GritSyntaxKind> = token_set!(T![,], T!['}']);

pub(crate) const LANGUAGE_NAME_RECOVERY_SET: TokenSet<GritSyntaxKind> = token_set!(T!['('], T![;]);

pub(crate) const LANGUAGE_FLAVOR_RECOVERY_SET: TokenSet<GritSyntaxKind> =
    token_set!(T![')'], T![,], T![;]);

pub(crate) const NOT_SET: TokenSet<GritSyntaxKind> = token_set![NOT_KW, T![!]];

pub(crate) const PATTERN_RECOVERY_SET: TokenSet<GritSyntaxKind> = token_set!(T![')'], EOF);

pub(crate) const PREDICATE_RECOVERY_SET: TokenSet<GritSyntaxKind> = token_set!(T![')']);

pub(crate) const REGEX_SET: TokenSet<GritSyntaxKind> = token_set![GRIT_REGEX, GRIT_SNIPPET_REGEX];

pub(crate) const SUPPORTED_LANGUAGE_SET: TokenSet<GritSyntaxKind> =
    token_set![T![js], T![json], T![css], T![grit], T![html]];

pub(crate) const SUPPORTED_LANGUAGE_FLAVOR_SET: TokenSet<GritSyntaxKind> =
    token_set![T![typescript], T![jsx]];
