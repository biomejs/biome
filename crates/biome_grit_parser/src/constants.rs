use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_grit_syntax::T;
use biome_parser::{token_set, TokenSet};

// See the precedence rules defined in the Grit grammar:
//   https://github.com/getgrit/tree-sitter-gritql/blob/main/grammar.js#L7
pub(crate) const PRECEDENCE_NOT: isize = 0;
pub(crate) const PRECEDENCE_PATTERN_AS: isize = 10;
pub(crate) const PRECEDENCE_MUL: isize = 8;
pub(crate) const PRECEDENCE_DIV: isize = 8;
pub(crate) const PRECEDENCE_MOD: isize = 8;
pub(crate) const PRECEDENCE_ADD: isize = 7;
pub(crate) const PRECEDENCE_SUB: isize = 7;
pub(crate) const PRECEDENCE_REWRITE: isize = 3;
pub(crate) const PRECEDENCE_ACCUMULATE: isize = 3;
pub(crate) const PRECEDENCE_PATTERN_LIMIT: isize = 1;
pub(crate) const PRECEDENCE_PATTERN_WHERE: isize = 1;
pub(crate) const PRECEDENCE_PATTERN: isize = -20;

// Recovery sets.
pub(crate) const ARG_LIST_RECOVERY_SET: TokenSet<GritSyntaxKind> = token_set!(T![,], T![')']);

pub(crate) const DEFINITION_LIST_RECOVERY_SET: TokenSet<GritSyntaxKind> = token_set!(EOF);

pub(crate) const ELEMENT_LIST_RECOVERY_SET: TokenSet<GritSyntaxKind> = token_set!(T![,], T!['}']);

pub(crate) const PATTERN_RECOVERY_SET: TokenSet<GritSyntaxKind> = token_set!(T![')'], T!['}'], EOF);

pub(crate) const PATTERN_ELSE_RECOVERY_SET: TokenSet<GritSyntaxKind> =
    token_set!(T![')'], T!['}'], UNTIL_KW, EOF);

pub(crate) const PATTERN_LIST_RECOVERY_SET: TokenSet<GritSyntaxKind> = token_set!(T![,], T![']']);

pub(crate) const PATTERN_UNTIL_RECOVERY_SET: TokenSet<GritSyntaxKind> =
    token_set!(T![')'], T!['}'], UNTIL_KW, EOF);

pub(crate) const PREDICATE_RECOVERY_SET: TokenSet<GritSyntaxKind> =
    token_set!(T![')'], T!['}'], T![,], ELSE_KW);

// Other sets.
pub(crate) const BOOLEAN_VALUE_SET: TokenSet<GritSyntaxKind> = token_set![TRUE_KW, FALSE_KW];

pub(crate) const CODE_SNIPPET_SET: TokenSet<GritSyntaxKind> =
    SUPPORTED_LANGUAGE_SET.union(token_set![GRIT_BACKTICK_SNIPPET, GRIT_RAW_BACKTICK_SNIPPET]);

pub(crate) const CONTAINER_SET: TokenSet<GritSyntaxKind> =
    token_set![GRIT_VARIABLE, GRIT_MAP_ACCESSOR, GRIT_LIST_ACCESSOR];

pub(crate) const NOT_SET: TokenSet<GritSyntaxKind> = token_set![NOT_KW, T![!]];

pub(crate) const REGEX_SET: TokenSet<GritSyntaxKind> = token_set![GRIT_REGEX, GRIT_SNIPPET_REGEX];

// Engine names we can parse for formatting purposes
pub(crate) const SUPPORTED_ENGINE_SET: TokenSet<GritSyntaxKind> =
    token_set![T![biome], T![marzano]];

pub(crate) const SUPPORTED_LANGUAGE_SET: TokenSet<GritSyntaxKind> =
    token_set![T![js], T![json], T![css], T![grit], T![html]];

pub(crate) const SUPPORTED_LANGUAGE_SET_STR: &[&str] = &["js", "json", "css", "grit", "html"];

pub(crate) const SUPPORTED_LANGUAGE_FLAVOR_SET: TokenSet<GritSyntaxKind> =
    token_set![T![typescript], T![jsx]];

pub(crate) const SUPPORTED_LANGUAGE_FLAVOR_SET_STR: &[&str] = &["typescript", "jsx"];
