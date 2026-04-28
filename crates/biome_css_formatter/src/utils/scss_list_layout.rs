use crate::prelude::*;
use crate::utils::scss_context::is_in_scss_include_arguments;
use crate::utils::scss_expression::{
    include_keyword_argument_before_argument_list, unwrap_single_expression_item,
};
use crate::utils::scss_map::scss_map_context;
use biome_css_syntax::{
    ScssListExpression, ScssListExpressionElementList, ScssListExpressionFields,
    ScssParenthesizedExpression,
};
use biome_formatter::{GroupId, format_args, write};
use biome_rowan::{AstNode, AstSeparatedList};

/// Layout for SCSS list expressions.
pub(crate) struct ScssListLayout<'a> {
    node: &'a ScssListExpression,
}

impl<'a> ScssListLayout<'a> {
    pub(crate) fn new(node: &'a ScssListExpression) -> Self {
        Self { node }
    }

    pub(crate) fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssListExpressionFields { elements } = self.node.as_fields();

        if !is_in_scss_include_arguments(self.node.syntax()) {
            if scss_map_context(self.node)
                .is_some_and(|context| context.is_outer_parenthesized_value_list)
            {
                let group_id = f.group_id("scss_list_expression");
                let comma = token(",");
                let trailing_comma = if_group_breaks(&comma).with_group_id(Some(group_id));

                return write!(
                    f,
                    [group(&indent(&format_args![
                        soft_line_break(),
                        elements.format(),
                        trailing_comma
                    ]))
                    .with_group_id(Some(group_id))]
                );
            }

            return write!(
                f,
                [group(&indent(&format_args![
                    soft_line_break(),
                    elements.format()
                ]))]
            );
        }

        let group_id = f.group_id("scss_list_expression");
        let list_layout = IncludeListLayout::new(self.node, &elements);
        let trailing_comma = format_with(|f| list_layout.write_trailing_comma(group_id, f));

        if scss_map_context(self.node)
            .is_some_and(|context| context.is_outer_parenthesized_value_list)
            || list_layout.is_keyword_parenthesized_list
        {
            // Format the list in `key: (a, b)`. The surrounding parentheses are
            // handled by `FormatScssParenthesizedExpression`.
            write!(
                f,
                [group(&format_args![elements.format(), trailing_comma])
                    .with_group_id(Some(group_id))
                    .should_expand(list_layout.should_expand)]
            )
        } else {
            write!(
                f,
                [group(&indent(&format_args![
                    soft_line_break(),
                    elements.format(),
                    trailing_comma
                ]))
                .with_group_id(Some(group_id))
                .should_expand(list_layout.should_expand)]
            )
        }
    }
}

/// Prettier-style decisions for lists inside include arguments.
#[derive(Debug, Clone, Copy)]
struct IncludeListLayout {
    is_keyword_parenthesized_list: bool,
    force_trailing_comma: bool,
    should_expand: bool,
}

impl IncludeListLayout {
    fn new(node: &ScssListExpression, elements: &ScssListExpressionElementList) -> Self {
        let is_keyword_parenthesized_list =
            is_direct_include_keyword_parenthesized_list_value(node);
        let keeps_source_trailing_separator =
            elements.trailing_separator().is_some() && is_keyword_parenthesized_list;

        Self {
            is_keyword_parenthesized_list,
            force_trailing_comma: keeps_source_trailing_separator,
            should_expand: keeps_source_trailing_separator,
        }
    }

    fn write_trailing_comma(self, group_id: GroupId, f: &mut CssFormatter) -> FormatResult<()> {
        if self.force_trailing_comma {
            write!(f, [token(",")])
        } else {
            write!(
                f,
                [if_group_breaks(&token(",")).with_group_id(Some(group_id))]
            )
        }
    }
}

/// Returns `true` for the list in `@include mix($arg: (a, b))`.
fn is_direct_include_keyword_parenthesized_list_value(node: &ScssListExpression) -> bool {
    let Some(parenthesized) = node
        .syntax()
        .parent()
        .and_then(ScssParenthesizedExpression::cast)
    else {
        return false;
    };

    if include_keyword_argument_before_argument_list(parenthesized.syntax()).is_none() {
        return false;
    }

    parenthesized.expression().ok().is_some_and(|expression| {
        expression
            .as_scss_list_expression()
            .is_some_and(|list| list == node)
            || unwrap_single_expression_item(&expression)
                .and_then(|item| item.as_scss_list_expression().cloned())
                .is_some_and(|list| &list == node)
    })
}
