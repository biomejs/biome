use crate::parser::CssParser;
use biome_css_syntax::{CssSyntaxKind, T, TextRange};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::{Parser, TokenSet, token_set};

/// A set of tokens representing the Vue SFC scoped CSS selectors `:deep` and `:slotted`.
///
/// Note: `:global` is shared with CSS Modules and handled separately.
pub(crate) const VUE_SCOPED_CSS_SET: TokenSet<CssSyntaxKind> = token_set![T![deep], T![slotted]];

/// Generates a parse diagnostic for when Vue SFC scoped CSS selectors are not allowed.
pub(crate) fn vue_scoped_css_not_allowed(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "`:deep` and `:slotted` are Vue SFC scoped CSS selectors, not standard CSS features.",
        range,
    )
    .with_hint(
        "You can enable Vue SFC scoped CSS parsing by setting the `css.parser.vueScopedCss` option to `true` in your configuration file.",
    )
}
