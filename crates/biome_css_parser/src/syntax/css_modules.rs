use crate::parser::CssParser;
use biome_css_syntax::{CssSyntaxKind, T, TextRange};
use biome_parser::diagnostic::{ParseDiagnostic, ToDiagnostic, expect_one_of};
use biome_parser::{Parser, TokenSet, token_set};

/// A set of tokens representing the CSS Modules pseudo-classes `:local` and `:global`.
pub(crate) const CSS_MODULES_SCOPE_SET: TokenSet<CssSyntaxKind> = token_set![T![global], T![local]];

pub(crate) const CSS_MODULES_VUE_ENHANCED_SET: TokenSet<CssSyntaxKind> =
    token_set![T![slotted], T![deep]];

/// Generates a parse diagnostic for when the `:local` or `:global` pseudo-classes are not allowed.
///
/// This function returns an error diagnostic indicating that the `:local` or `:global` pseudo-classes
/// are not standard CSS features. It also provides a hint on how to enable
/// parsing of these pseudo-classes by setting the `css.parser.cssModules` option to `true`
/// in the configuration file.
pub(crate) fn local_or_global_not_allowed(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "`:local` and `:global` pseudo-classes are not standard CSS features.",
        range,
    )
        .with_hint(
            "You can enable `:local` and `:global` pseudo-class parsing by setting the `css.parser.cssModules` option to `true` in your configuration file.",
        )
}

/// This function generates a parsing diagnostic for the usage of the
/// `:slotted` and `:deep` pseudo-classes in CSS, which are non-standard CSS features.
pub(crate) fn slotted_or_deep_not_allowed(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "`:slotted` and `:deep` pseudo-classes are not standard CSS features.",
        range,
    )
    .with_hint("These are valid pseudo selectors only when defined inside SFC vue files.")
}

/// This function generates a parsing diagnostic for the usage of the
/// `v-bind()` function in CSS, which is a non-standard CSS feature.
pub(crate) fn v_bind_not_allowed(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("`v-bind()` is not a standard CSS function.", range)
        .with_hint("This is valid only when defined inside SFC vue files.")
}

pub(crate) fn expected_any_css_module_scope(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["global", "local"], range).into_diagnostic(p)
}

/// Generates a parse diagnostic for the `composes` declaration when it is not allowed.
///
/// This function returns an error diagnostic indicating that the `composes` declaration
/// is not a standard CSS feature. It also provides a hint on how to enable parsing of the
/// `composes` declaration by setting the `css.parser.cssModules` option to `true` in the configuration file.
pub(crate) fn composes_not_allowed(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "`composes` declaration is not a standard CSS feature.",
        range,
    )
        .with_hint(
            "You can enable `composes` declaration parsing by setting the `css.parser.cssModules` option to `true` in your configuration file.",
        )
}

/// Generates a parse diagnostic for an expected `composes` import source.
///
/// This function returns a diagnostic error indicating that an `<identifier>` or `<string>`
/// was expected as the source for a `composes` import declaration at the given range in the CSS parser.
pub(crate) fn expected_composes_import_source(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["<identifier>", "<string>"], range).into_diagnostic(p)
}

/// Generates a parse diagnostic for an empty list of classes after `composes`.
///
/// This function returns a diagnostic error indicating that a non-empty list of classes was expected
/// after the `composes` keyword in a CSS Modules declaration, but an empty list was found.
pub(crate) fn expected_classes_list(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "Expected a non-empty list of classes after `composes`.",
        range,
    )
}
