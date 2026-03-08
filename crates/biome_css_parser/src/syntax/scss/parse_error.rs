use crate::parser::CssParser;
use biome_console::markup;
use biome_css_syntax::CssSyntaxKind::EOF;
use biome_parser::Parser;
use biome_parser::diagnostic::expected_node;
use biome_parser::prelude::ParseDiagnostic;
use biome_rowan::TextRange;

/// Emits a diagnostic when a SCSS expression is required.
///
/// Example:
/// ```scss
/// $x: 1 + ;
/// ```
#[inline]
pub(crate) fn expected_scss_expression(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_node("SCSS expression", range, p)
}

/// Emits a diagnostic when `...` is used outside function call arguments.
///
/// Example:
/// ```scss
/// $x: $args...;
/// ```
#[inline]
pub(crate) fn scss_ellipsis_not_allowed(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "SCSS arbitrary arguments (`...`) are only allowed in function call arguments.",
        range,
    )
    .with_hint(markup! {
        "Use `...` only for function arguments, for example `fn($args...)`."
    })
}

/// Emits a focused diagnostic when a SCSS variable modifier isn't `default` or
/// `global`.
///
/// Example:
/// ```scss
/// $x: 1 !bad;
/// ```
///
/// Docs: https://sass-lang.com/documentation/variables
#[inline]
pub(crate) fn expected_scss_variable_modifier(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    if p.cur() == EOF {
        p.err_builder(
            "expected 'default', or 'global' but instead the file ends",
            range,
        )
        .with_detail(range, "the file ends here")
    } else {
        p.err_builder(
            format!(
                "expected 'default', or 'global' but instead found `{}`",
                p.cur_text()
            ),
            range,
        )
        .with_hint(format!("Remove {}", p.text(range)))
    }
}
