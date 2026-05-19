use crate::prelude::*;
use biome_css_syntax::{
    AnyScssExpression, CssSyntaxToken, ScssBinaryExpression, ScssBinaryExpressionFields,
    is_in_scss_control_condition_sequence, is_in_scss_parenthesized_expression,
    is_scss_comparison_operator, is_scss_expression_ending_with_interpolation,
    is_scss_expression_starting_with_interpolation, is_scss_parenthesized_expression,
};
use biome_formatter::{format_args, write};
use biome_rowan::SyntaxResult;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssBinaryExpression;

impl FormatNodeRule<ScssBinaryExpression> for FormatScssBinaryExpression {
    fn fmt_fields(&self, node: &ScssBinaryExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssBinaryExpressionFields {
            left,
            operator,
            right,
        } = node.as_fields();
        let left_node = left.as_ref().ok();
        let formatted_right = FormatScssBinaryRightSide::new(node, left_node, &operator, &right);

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
    left: Option<&'a AnyScssExpression>,
    operator: &'a SyntaxResult<CssSyntaxToken>,
    right: &'a SyntaxResult<AnyScssExpression>,
}

impl<'a> FormatScssBinaryRightSide<'a> {
    fn new(
        node: &'a ScssBinaryExpression,
        left: Option<&'a AnyScssExpression>,
        operator: &'a SyntaxResult<CssSyntaxToken>,
        right: &'a SyntaxResult<AnyScssExpression>,
    ) -> Self {
        Self {
            node,
            left,
            operator,
            right,
        }
    }

    fn should_indent(&self) -> bool {
        is_in_scss_parenthesized_expression(self.node)
            || self
                .right
                .as_ref()
                .is_ok_and(is_scss_parenthesized_expression)
    }

    fn should_indent_operator_with_right(&self, spacing: ScssBinaryOperatorSpacing) -> bool {
        self.should_indent() && spacing.keeps_operator_with_right()
    }
}

impl Format<CssFormatContext> for FormatScssBinaryRightSide<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let operator = self.operator.as_ref().ok();
        let right = self.right.as_ref().ok();
        let spacing = ScssBinaryOperatorSpacing::new(self.left, operator, right);
        let gap_before_operator = FormatScssBinaryOperatorGap::before(spacing);
        let gap_after_operator = FormatScssBinaryOperatorGap::after(spacing);

        if self.should_indent_operator_with_right(spacing) {
            return write!(
                f,
                [indent(&format_args![
                    gap_before_operator,
                    self.operator.format(),
                    gap_after_operator,
                    self.right.format()
                ])]
            );
        }

        if self.should_indent() {
            write!(
                f,
                [
                    gap_before_operator,
                    self.operator.format(),
                    indent(&format_args![gap_after_operator, self.right.format()])
                ]
            )
        } else {
            write!(
                f,
                [
                    gap_before_operator,
                    self.operator.format(),
                    gap_after_operator,
                    self.right.format()
                ]
            )
        }
    }
}

/// Tracks operator operands where Prettier preserves no source space.
///
/// Examples: `#{$width}/2px` keeps no space before `/`; `1 >$width` keeps no
/// space after `>` because the source was tight there.
#[derive(Clone, Copy)]
struct ScssBinaryOperatorSpacing<'a> {
    left: Option<&'a AnyScssExpression>,
    operator: Option<&'a CssSyntaxToken>,
    right: Option<&'a AnyScssExpression>,
}

impl<'a> ScssBinaryOperatorSpacing<'a> {
    fn new(
        left: Option<&'a AnyScssExpression>,
        operator: Option<&'a CssSyntaxToken>,
        right: Option<&'a AnyScssExpression>,
    ) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }

    fn keeps_operator_with_right(&self) -> bool {
        !self.is_before_omitted() && self.is_after_omitted()
    }

    /// Returns true for left edges that stay tight inline.
    ///
    /// Examples: `#{$width}/2px`, `10==#{10}`, and `1> $width`.
    fn is_before_omitted(&self) -> bool {
        let (Some(left), Some(operator), Some(_)) = (self.left, self.operator, self.right) else {
            return false;
        };

        (is_scss_expression_ending_with_interpolation(left)
            || (is_scss_comparison_operator(operator) && !is_scss_parenthesized_expression(left)))
            && left.syntax().text_trimmed_range().end() == operator.text_trimmed_range().start()
    }

    /// Returns true for right edges that stay tight inline.
    ///
    /// Examples: `2px/#{$width}` and `1 >$width`.
    fn is_after_omitted(&self) -> bool {
        let (Some(operator), Some(right)) = (self.operator, self.right) else {
            return false;
        };

        (is_scss_expression_starting_with_interpolation(right)
            || is_scss_comparison_operator(operator))
            && operator.text_trimmed_range().end() == right.syntax().text_trimmed_range().start()
    }
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
enum FormatScssBinaryOperatorGap<'a> {
    /// Gap before the operator.
    ///
    /// Example: `#{$width}/2px` has no gap before `/`.
    Before(ScssBinaryOperatorSpacing<'a>),
    /// Gap after the operator.
    ///
    /// Example: `1 >$width` has no gap after `>`.
    After(ScssBinaryOperatorSpacing<'a>),
}

impl<'a> FormatScssBinaryOperatorGap<'a> {
    fn before(spacing: ScssBinaryOperatorSpacing<'a>) -> Self {
        Self::Before(spacing)
    }

    fn after(spacing: ScssBinaryOperatorSpacing<'a>) -> Self {
        Self::After(spacing)
    }
}

impl Format<CssFormatContext> for FormatScssBinaryOperatorGap<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        match self {
            Self::Before(spacing) => {
                if spacing.is_before_omitted() {
                    Ok(())
                } else if spacing.is_after_omitted() {
                    write!(f, [soft_line_break_or_space()])
                } else {
                    write!(f, [space()])
                }
            }
            Self::After(spacing) => {
                if spacing.is_after_omitted() {
                    Ok(())
                } else {
                    write!(f, [soft_line_break_or_space()])
                }
            }
        }
    }
}
