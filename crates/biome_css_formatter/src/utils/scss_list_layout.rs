use crate::prelude::*;
use crate::utils::comment_trivia::has_inline_trailing_comment;
use crate::utils::scss_closing_comments::{
    ClosingCommentSpacing, owns_include_closing_comments, write_include_closing_comments,
};
use biome_css_syntax::{
    ScssExpression, ScssListExpression, ScssListExpressionElementList, ScssListExpressionFields,
    ScssModuleConfiguration, ScssParenthesizedExpression, is_in_scss_include_arguments,
    is_scss_map_key, is_scss_map_outer_parenthesized_value_list,
    scss_include_keyword_argument_owner, single_expression_item, unwrap_single_expression_item,
};
use biome_formatter::{format_args, write};
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

        if is_in_scss_include_arguments(self.node.syntax()) {
            return self.fmt_include_list(&elements, f);
        }

        if is_module_configuration_parenthesized_list_value(self.node) {
            return write!(
                f,
                [group(&format_args![
                    soft_line_break(),
                    elements.format(),
                    token(",")
                ])
                .should_expand(true)]
            );
        }

        if is_scss_map_outer_parenthesized_value_list(self.node) {
            // `key: (a, b)` gets its block indent from the parentheses;
            // the list only forces item breaks and the trailing comma.
            return write!(
                f,
                [group(&format_args![
                    elements.format(),
                    if_group_breaks(&token(","))
                ])
                .should_expand(true)]
            );
        }

        if is_parenthesized_map_key_list(self.node) {
            // `(("a", "b"): value)` already gets indented by the key's
            // parentheses, so the list only controls its own breaks.
            return write!(
                f,
                [group(&format_args![
                    elements.format(),
                    if_group_breaks(&token(","))
                ])]
            );
        }

        if is_parenthesized_list(self.node) {
            // In `(a, b)`, the parentheses own the line break and indent.
            return write!(f, [group(&format_args![elements.format()])]);
        }

        write!(
            f,
            [group(&indent(&format_args![
                soft_line_break(),
                elements.format()
            ]))]
        )
    }

    /// Formats lists inside `@include`, where keyword values own commas/comments.
    ///
    /// Example: `@include mix($arg: (a, b) /* end */)`.
    fn fmt_include_list(
        &self,
        elements: &ScssListExpressionElementList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let should_force_trailing_comma = self.should_force_include_trailing_comma(elements, f);
        let trailing_comma = Self::include_trailing_comma(should_force_trailing_comma);
        let closing_comments = self.include_closing_comments();
        let should_expand = self.should_expand_include_list(elements, f);

        if is_scss_map_outer_parenthesized_value_list(self.node)
            || self.is_parenthesized_include_list()
        {
            // Format the list in `key: (a, b)`. The surrounding parentheses are
            // handled by `FormatScssParenthesizedExpression`.
            write!(
                f,
                [group(&format_args![
                    elements.format(),
                    trailing_comma, closing_comments
                ])
                .should_expand(should_expand)]
            )
        } else {
            write!(
                f,
                [group(&indent(&format_args![
                    soft_line_break(),
                    elements.format(),
                    trailing_comma,
                    closing_comments
                ]))
                .should_expand(should_expand)]
            )
        }
    }

    /// Formats the comma after an include argument list.
    ///
    /// Example: `@include mix($arg: (a))` prints the keyword list comma.
    fn include_trailing_comma(should_force: bool) -> impl Format<CssFormatContext> {
        format_with(move |f| {
            if should_force {
                write!(f, [token(",")])
            } else {
                write!(f, [if_group_breaks(&token(","))])
            }
        })
    }

    /// Formats include-owned comments before the closing `)`.
    ///
    /// Example: `@include mix((a, b) /* end */)`.
    fn include_closing_comments(&self) -> impl Format<CssFormatContext> + '_ {
        format_with(|f| {
            write_include_closing_comments(
                self.node.syntax(),
                ClosingCommentSpacing::SoftLineBreak,
                f,
            )
        })
    }

    /// Forces a comma when include parens or comments own the list shape.
    ///
    /// Examples: `@include mix($arg: (a))`, `@include mix((a) /* end */)`.
    fn should_force_include_trailing_comma(
        &self,
        elements: &ScssListExpressionElementList,
        f: &CssFormatter,
    ) -> bool {
        owns_include_closing_comments(self.node.syntax(), f)
            || self.should_force_include_parenthesized_list_layout(elements)
    }

    /// Expands include keyword lists that need visible item/comment boundaries.
    ///
    /// Examples: `@include mix($arg: (a))`, `@include mix($arg: (a) /* end */)`.
    fn should_expand_include_list(
        &self,
        elements: &ScssListExpressionElementList,
        f: &CssFormatter,
    ) -> bool {
        is_include_keyword_list_value_expanded_by_comments(self.node, f)
            || self.should_force_include_parenthesized_list_layout(elements)
    }

    /// Forces expansion and a comma for include keyword lists in parens.
    ///
    /// Examples: `$arg: (a)`, `$arg: (a,)`.
    fn should_force_include_parenthesized_list_layout(
        &self,
        elements: &ScssListExpressionElementList,
    ) -> bool {
        self.is_parenthesized_include_list()
            && (elements.len() > 0 || elements.trailing_separator().is_some())
    }

    /// Detects the list in `@include mix($arg: (a, b))`.
    fn is_parenthesized_include_list(&self) -> bool {
        let Some(parenthesized) = self
            .node
            .syntax()
            .parent()
            .and_then(ScssParenthesizedExpression::cast)
        else {
            return false;
        };

        scss_include_keyword_argument_owner(parenthesized.syntax()).is_some()
            && is_list_owned_by_parentheses(&parenthesized, self.node)
    }

    /// Checks whether this layout prints include closing comments.
    pub(crate) fn owns_dangling_comments(&self, f: &CssFormatter) -> bool {
        owns_include_closing_comments(self.node.syntax(), f)
    }
}

/// Detects `@include mix($arg: (a, b) /* end */)`, where comments force expansion.
fn is_include_keyword_list_value_expanded_by_comments(
    node: &ScssListExpression,
    f: &CssFormatter,
) -> bool {
    let has_direct_trailing_comment = has_inline_trailing_comment(node.syntax());
    let has_expression_trailing_comment = node
        .syntax()
        .parent()
        .and_then(ScssExpression::cast)
        .is_some_and(|expression| has_inline_trailing_comment(expression.syntax()));
    let Some(parenthesized) = node
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(ScssParenthesizedExpression::cast)
    else {
        let Some(keyword_argument) = scss_include_keyword_argument_owner(node.syntax()) else {
            return false;
        };

        return has_direct_trailing_comment
            || has_expression_trailing_comment
            || f.comments()
                .has_dangling_comments(keyword_argument.syntax());
    };

    let has_trailing_comment = has_inline_trailing_comment(parenthesized.syntax());
    let Some(keyword_argument) = scss_include_keyword_argument_owner(parenthesized.syntax()) else {
        return false;
    };
    let has_keyword_closing_comments = f
        .comments()
        .has_dangling_comments(keyword_argument.syntax());

    has_trailing_comment || has_keyword_closing_comments
}

/// Detects the list in `@use "x" with ($family: (a, b))`.
fn is_module_configuration_parenthesized_list_value(node: &ScssListExpression) -> bool {
    let Some(parenthesized) = node
        .syntax()
        .parent()
        .and_then(ScssParenthesizedExpression::cast)
    else {
        return false;
    };

    if !is_list_owned_by_parentheses(&parenthesized, node) {
        return false;
    }

    parenthesized
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(ScssModuleConfiguration::cast)
        .and_then(|configuration| configuration.value().ok())
        .is_some_and(|value| {
            value.syntax() == parenthesized.syntax()
                || single_expression_item(&value)
                    .is_some_and(|item| item.syntax() == parenthesized.syntax())
        })
}

/// Detects the list key in `(("a", "b"): value)`.
fn is_parenthesized_map_key_list(node: &ScssListExpression) -> bool {
    node.syntax()
        .parent()
        .and_then(ScssParenthesizedExpression::cast)
        .is_some_and(|parenthesized| {
            is_scss_map_key(&parenthesized) && is_list_owned_by_parentheses(&parenthesized, node)
        })
}

/// Detects a list directly wrapped by parentheses, such as `(a, b)`.
fn is_parenthesized_list(node: &ScssListExpression) -> bool {
    node.syntax()
        .parent()
        .and_then(ScssParenthesizedExpression::cast)
        .is_some_and(|parenthesized| is_list_owned_by_parentheses(&parenthesized, node))
}

/// Detects the list owned by parentheses in `(a, b)`.
fn is_list_owned_by_parentheses(
    parenthesized: &ScssParenthesizedExpression,
    node: &ScssListExpression,
) -> bool {
    parenthesized.expression().ok().is_some_and(|expression| {
        expression
            .as_scss_list_expression()
            .is_some_and(|list| list == node)
            || unwrap_single_expression_item(&expression)
                .and_then(|item| item.as_scss_list_expression().cloned())
                .is_some_and(|list| &list == node)
    })
}
