use crate::prelude::*;
use crate::utils::scss_control_condition::ScssControlConditionLayout;
use biome_css_syntax::{
    AnyScssExpression, AnyScssExpressionItem, ScssBinaryExpression, ScssElseClause, ScssExpression,
    ScssIfAtRule, ScssIfAtRuleFields, single_expression_item,
};
use biome_formatter::{GroupId, format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssIfAtRule;

impl FormatNodeRule<ScssIfAtRule> for FormatScssIfAtRule {
    fn fmt_fields(&self, node: &ScssIfAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssIfAtRuleFields {
            if_token,
            condition,
            block,
            else_clause,
        } = node.as_fields();

        let condition = condition?;
        let header_group_id = f.group_id("scss_if_header");
        let is_else_if = node
            .syntax()
            .parent()
            .is_some_and(|parent| ScssElseClause::can_cast(parent.kind()));
        let should_split_after_if = is_else_if && is_else_if_condition_split_after_if(&condition);
        let condition_layout = ScssControlConditionLayout::from_condition(&condition);
        let block_separator = format_with(|f| {
            if condition_layout.should_keep_block_on_same_line() {
                write!(f, [space()])
            } else {
                write!(f, [soft_line_break_or_space()])
            }
        });

        write!(f, [if_token.format()])?;

        if !should_split_after_if {
            write!(f, [space()])?;
        }

        write!(
            f,
            [
                group(&format_args![
                    FormatScssIfConditionHeader {
                        condition: &condition.format(),
                        header_group_id,
                        should_split_after_if,
                        condition_layout,
                    },
                    block_separator
                ])
                .with_group_id(Some(header_group_id)),
                block.format()
            ]
        )?;

        if let Some(else_clause) = else_clause {
            if f.comments().has_leading_comments(else_clause.syntax()) {
                write!(f, [hard_line_break(), else_clause.format()])?;
            } else {
                write!(f, [space(), else_clause.format()])?;
            }
        }

        Ok(())
    }
}

/// Formats the condition after `@if` or `@else if`.
///
/// Prettier breaks after `if` for `@else if $a == ...`, but keeps unary
/// conditions such as `@else if not $a == ...` grouped after `not`.
struct FormatScssIfConditionHeader<'a> {
    condition: &'a dyn Format<CssFormatContext>,
    header_group_id: GroupId,
    should_split_after_if: bool,
    condition_layout: ScssControlConditionLayout,
}

impl Format<CssFormatContext> for FormatScssIfConditionHeader<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        if self.should_split_after_if {
            write!(
                f,
                [indent(&format_args![
                    soft_line_break_or_space(),
                    self.condition
                ])]
            )
        } else if self.condition_layout.should_indent_condition() {
            write!(
                f,
                [indent_if_group_breaks(
                    &format_args![self.condition],
                    self.header_group_id
                )]
            )
        } else {
            write!(f, [self.condition])
        }
    }
}

/// Detects `@else if $a == ...`, where Prettier breaks after `if`.
///
/// `@else if not $a == ...` stays grouped after `not`.
fn is_else_if_condition_split_after_if(condition: &ScssExpression) -> bool {
    match single_expression_item(condition) {
        Some(AnyScssExpressionItem::ScssBinaryExpression(binary)) => {
            !is_binary_chain_started_by_unary_expression(&binary)
        }
        _ => false,
    }
}

/// Detects a unary left edge, such as `not $a == $b`.
fn is_binary_chain_started_by_unary_expression(binary: &ScssBinaryExpression) -> bool {
    match binary.left().ok() {
        Some(AnyScssExpression::ScssBinaryExpression(left)) => {
            is_binary_chain_started_by_unary_expression(&left)
        }
        Some(AnyScssExpression::ScssUnaryExpression(_)) => true,
        _ => false,
    }
}
