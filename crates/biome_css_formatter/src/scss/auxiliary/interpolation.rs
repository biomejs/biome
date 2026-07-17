use crate::prelude::*;
use biome_css_syntax::{
    AnyScssExpression, CssSyntaxKind, CssSyntaxToken, ScssInterpolatedIdentifier,
    ScssInterpolation, ScssInterpolationFields,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolation;
impl FormatNodeRule<ScssInterpolation> for FormatScssInterpolation {
    fn fmt_fields(&self, node: &ScssInterpolation, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssInterpolationFields {
            hash_token,
            l_curly_token,
            value,
            r_curly_token,
        } = node.as_fields();
        let hash_token = hash_token?;
        let l_curly_token = l_curly_token?;
        let value = value?;
        let r_curly_token = r_curly_token?;

        if should_preserve_identifier_interpolation_spacing(node) {
            let has_gap_after_opening = has_source_gap_after_opening_curly(&l_curly_token, &value);
            let has_gap_before_closing =
                has_source_gap_before_closing_curly(&value, &r_curly_token);

            if has_gap_after_opening || has_gap_before_closing {
                let gap_after_opening = format_with(|f| {
                    if has_gap_after_opening {
                        write!(f, [soft_line_break_or_space()])
                    } else {
                        Ok(())
                    }
                });
                let gap_before_closing = format_with(|f| {
                    if has_gap_before_closing {
                        write!(f, [soft_line_break_or_space()])
                    } else {
                        Ok(())
                    }
                });

                return write!(
                    f,
                    [group(&format_args![
                        hash_token.format(),
                        l_curly_token.format(),
                        gap_after_opening,
                        value.format(),
                        gap_before_closing,
                        r_curly_token.format()
                    ])]
                );
            }
        }

        write!(
            f,
            [group(&format_args![
                hash_token.format(),
                l_curly_token.format(),
                value.format(),
                r_curly_token.format()
            ])]
        )
    }
}

/// Preserves source gaps inside selector and property-name interpolation.
///
/// Prettier keeps `.icon-#{ $name}` but prints `value: #{$name}`.
fn should_preserve_identifier_interpolation_spacing(node: &ScssInterpolation) -> bool {
    let Some(identifier) = node
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(ScssInterpolatedIdentifier::cast)
    else {
        return false;
    };

    identifier.syntax().parent().is_some_and(|parent| {
        matches!(
            parent.kind(),
            CssSyntaxKind::CSS_CLASS_SELECTOR
                | CssSyntaxKind::CSS_GENERIC_PROPERTY
                | CssSyntaxKind::CSS_ID_SELECTOR
                | CssSyntaxKind::CSS_TYPE_SELECTOR
                | CssSyntaxKind::SCSS_PLACEHOLDER_SELECTOR
        )
    })
}

/// Detects spacing after `{`.
///
/// Examples: `$value: #{ $name};` and:
///
/// ```scss
/// $value: #{
///   $name};
/// ```
fn has_source_gap_after_opening_curly(
    l_curly_token: &CssSyntaxToken,
    value: &AnyScssExpression,
) -> bool {
    l_curly_token.has_trailing_whitespace()
        || value
            .syntax()
            .first_token()
            .is_some_and(|token| token.has_leading_whitespace_or_newline())
}

/// Detects spacing before `}`.
///
/// Examples: `$value: #{$name };` and:
///
/// ```scss
/// $value: #{$name
/// };
/// ```
fn has_source_gap_before_closing_curly(
    value: &AnyScssExpression,
    r_curly_token: &CssSyntaxToken,
) -> bool {
    value
        .syntax()
        .last_token()
        .is_some_and(|token| token.has_trailing_whitespace())
        || r_curly_token.has_leading_whitespace_or_newline()
}
