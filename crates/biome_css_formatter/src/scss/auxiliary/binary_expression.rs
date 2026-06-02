use crate::prelude::*;
use biome_css_syntax::{
    AnyScssExpression, CssSyntaxToken, ScssBinaryExpression, ScssBinaryExpressionFields, T,
    is_in_scss_control_condition_sequence, is_in_scss_include_arguments,
    is_in_scss_parenthesized_expression, is_scss_comparison_operator,
    is_scss_expression_ending_with_interpolation, is_scss_expression_starting_with_interpolation,
    is_scss_parenthesized_expression,
};
use biome_formatter::{format_args, write};
use biome_rowan::TextRange;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssBinaryExpression;

impl FormatNodeRule<ScssBinaryExpression> for FormatScssBinaryExpression {
    fn fmt_fields(&self, node: &ScssBinaryExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let left = node.left();
        let formatted_right = FormatScssBinaryRightSide::new(node);

        if is_in_scss_control_condition_sequence(node) {
            write!(f, [left.format(), formatted_right])
        } else {
            write!(f, [group(&format_args![left.format(), formatted_right])])
        }
    }
}

/// Formats the operator and right side of a SCSS binary expression.
///
/// For `1 >$long`, Prettier breaks before `>` and keeps `>` attached to
/// `$long`.
struct FormatScssBinaryRightSide<'a> {
    node: &'a ScssBinaryExpression,
}

impl<'a> FormatScssBinaryRightSide<'a> {
    fn new(node: &'a ScssBinaryExpression) -> Self {
        Self { node }
    }

    fn should_indent(&self) -> bool {
        let ScssBinaryExpressionFields { right, .. } = self.node.as_fields();

        is_in_scss_parenthesized_expression(self.node)
            || right.as_ref().is_ok_and(is_scss_parenthesized_expression)
    }
}

impl Format<CssFormatContext> for FormatScssBinaryRightSide<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssBinaryExpressionFields {
            operator, right, ..
        } = self.node.as_fields();
        let spacing = ScssBinaryOperatorSpacing::new(self.node);
        let gap_before_operator = FormatScssBinaryOperatorGap::before(spacing);
        let gap_after_operator = FormatScssBinaryOperatorGap::after(spacing);
        let should_indent = self.should_indent();

        if should_indent && spacing.keeps_operator_with_right() {
            return write!(
                f,
                [indent(&format_args![
                    gap_before_operator,
                    operator.format(),
                    gap_after_operator,
                    right.format()
                ])]
            );
        }

        if should_indent {
            write!(
                f,
                [
                    gap_before_operator,
                    operator.format(),
                    indent(&format_args![gap_after_operator, right.format()])
                ]
            )
        } else {
            write!(
                f,
                [
                    gap_before_operator,
                    operator.format(),
                    gap_after_operator,
                    right.format()
                ]
            )
        }
    }
}

/// Tracks operator spacing edges where Prettier preserves no source space.
///
/// Examples: `@include transition(min-height ($spacer/2) ease-in-out);` keeps
/// tight `/`, and `1 >$width` keeps no space after `>`.
#[derive(Clone, Copy, Default)]
struct ScssBinaryOperatorSpacing {
    before_omitted: bool,
    after_omitted: bool,
}

impl ScssBinaryOperatorSpacing {
    fn new(node: &ScssBinaryExpression) -> Self {
        let ScssBinaryExpressionFields {
            left: Ok(left),
            operator: Ok(operator),
            right: Ok(right),
        } = node.as_fields()
        else {
            return Self::default();
        };

        let include_slash = is_include_slash_operator(node, &operator);
        let before_omitted = can_omit_before_operator(&left, &operator, include_slash)
            && has_no_source_gap_between(
                left.syntax().text_trimmed_range(),
                operator.text_trimmed_range(),
            );
        let after_omitted =
            can_omit_after_operator(&operator, &right, include_slash, before_omitted)
                && has_no_source_gap_between(
                    operator.text_trimmed_range(),
                    right.syntax().text_trimmed_range(),
                );

        Self {
            before_omitted,
            after_omitted,
        }
    }

    fn keeps_operator_with_right(self) -> bool {
        !self.before_omitted && self.after_omitted
    }
}

/// Returns true for left edges that stay tight inline.
///
/// Examples: `@include mix($spacer/2)`, `#{$width}/2px`, and `1> $width`.
fn can_omit_before_operator(
    left: &AnyScssExpression,
    operator: &CssSyntaxToken,
    include_slash: bool,
) -> bool {
    include_slash
        || is_scss_expression_ending_with_interpolation(left)
        || (is_scss_comparison_operator(operator) && !is_scss_parenthesized_expression(left))
}

/// Returns true for right edges that stay tight inline.
///
/// Examples: `@include mix($spacer/2)` and `1 >$width`. Prettier normalizes
/// `@include mix($spacer /2)` to `$spacer / 2`.
fn can_omit_after_operator(
    operator: &CssSyntaxToken,
    right: &AnyScssExpression,
    include_slash: bool,
    before_omitted: bool,
) -> bool {
    (include_slash && before_omitted)
        || is_scss_expression_starting_with_interpolation(right)
        || is_scss_comparison_operator(operator)
}

/// Returns true for `/` inside `@include mix($spacer/2)`.
fn is_include_slash_operator(node: &ScssBinaryExpression, operator: &CssSyntaxToken) -> bool {
    operator.kind() == T![/] && is_in_scss_include_arguments(node.syntax())
}

/// Returns true when two source ranges touch, as in `$spacer/2`.
fn has_no_source_gap_between(left: TextRange, right: TextRange) -> bool {
    left.end() == right.start()
}

/// Formats one side of a SCSS binary operator.
///
/// Most operators get a space or soft line break. These source edges stay tight:
///
/// ```scss
/// #{$width}/2px
/// 1 >$width
/// ```
#[derive(Clone, Copy)]
enum FormatScssBinaryOperatorGap {
    /// Gap before the operator.
    ///
    /// Example: `#{$width}/2px` has no gap before `/`.
    Before(ScssBinaryOperatorSpacing),
    /// Gap after the operator.
    ///
    /// Example: `1 >$width` has no gap after `>`.
    After(ScssBinaryOperatorSpacing),
}

impl FormatScssBinaryOperatorGap {
    fn before(spacing: ScssBinaryOperatorSpacing) -> Self {
        Self::Before(spacing)
    }

    fn after(spacing: ScssBinaryOperatorSpacing) -> Self {
        Self::After(spacing)
    }
}

impl Format<CssFormatContext> for FormatScssBinaryOperatorGap {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        match self {
            Self::Before(spacing) => {
                if spacing.before_omitted {
                    Ok(())
                } else if spacing.after_omitted {
                    write!(f, [soft_line_break_or_space()])
                } else {
                    write!(f, [space()])
                }
            }
            Self::After(spacing) => {
                if spacing.after_omitted {
                    Ok(())
                } else {
                    write!(f, [soft_line_break_or_space()])
                }
            }
        }
    }
}
