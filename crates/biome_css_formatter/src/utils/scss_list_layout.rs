use crate::prelude::*;
use crate::utils::comment_trivia::has_inline_trailing_comment;
use crate::utils::scss_closing_comments::{
    ClosingCommentSpacing, owns_include_closing_comments, write_include_closing_comments,
};
use biome_css_syntax::{
    ScssExpression, ScssExpressionItemList, ScssListExpression, ScssListExpressionElement,
    ScssListExpressionElementList, ScssListExpressionFields, ScssModuleConfiguration,
    ScssParenthesizedExpression, ScssVariableDeclaration, is_in_scss_include_arguments,
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

        if is_scss_map_outer_parenthesized_value_list(self.node) && has_list_shape(&elements) {
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

        if is_compound_variable_list(self.node, &elements) {
            // Print `$buttonConfig:
            //   "save" 50px,
            //   "cancel" 50px;`.
            return write!(
                f,
                [
                    group(&indent(&format_args![soft_line_break(), elements.format()]))
                        .should_expand(true)
                ]
            );
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
        let is_scalar_include_parentheses = self.is_scalar_include_parentheses(elements);
        let trailing_comma = format_with(|f| {
            if should_force_trailing_comma {
                write!(f, [token(",")])
            } else if is_scalar_include_parentheses {
                // Do not turn `2 * ($bar)` into `2 * ($bar,)`.
                Ok(())
            } else {
                write!(f, [if_group_breaks(&token(","))])
            }
        });
        let closing_comments = format_with(|f| self.write_include_closing_comments(f));
        let should_expand = self.should_expand_include_list(elements, f);
        let parenthesized_list = is_scss_map_outer_parenthesized_value_list(self.node)
            || self.is_parenthesized_include_list();
        let content = format_once(|f| {
            if parenthesized_list {
                // `key: (a, b)` gets its indent from the surrounding parentheses.
                write!(f, [elements.format(), trailing_comma, closing_comments])
            } else {
                write!(
                    f,
                    [indent(&format_args![
                        soft_line_break(),
                        elements.format(),
                        trailing_comma,
                        closing_comments
                    ])]
                )
            }
        });

        write!(f, [group(&content).should_expand(should_expand)])
    }

    /// Formats include-owned comments before the closing `)`.
    ///
    /// Example: `@include mix((a, b) /* end */)`.
    fn write_include_closing_comments(&self, f: &mut CssFormatter) -> FormatResult<()> {
        write_include_closing_comments(self.node.syntax(), ClosingCommentSpacing::SoftLineBreak, f)
    }

    /// Forces a comma when include parens or comments own the list shape.
    ///
    /// Examples: `@include mix($arg: (a, b))`, `@include mix((a) /* end */)`.
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
    /// Examples: `@include mix($arg: (a, b))`, `@include mix($arg: (a) /* end */)`.
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
    /// Examples: `$arg: (a, b)`, `$arg: (a,)`.
    fn should_force_include_parenthesized_list_layout(
        &self,
        elements: &ScssListExpressionElementList,
    ) -> bool {
        self.is_parenthesized_include_list() && has_list_shape(elements)
    }

    /// Detects scalar include parentheses such as `2 * ($bar)`.
    fn is_scalar_include_parentheses(&self, elements: &ScssListExpressionElementList) -> bool {
        self.is_parenthesized_include_list() && !has_list_shape(elements)
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

/// Detects `$buttonConfig: "save" 50px, "cancel" 50px;`.
fn is_compound_variable_list(
    node: &ScssListExpression,
    elements: &ScssListExpressionElementList,
) -> bool {
    is_scss_variable_value_list(node) && elements.len() > 1 && has_compound_list_element(elements)
}

/// Checks that the list is the value in `$buttonConfig: "save" 50px, "cancel" 50px;`.
fn is_scss_variable_value_list(node: &ScssListExpression) -> bool {
    node.parent::<ScssExpressionItemList>()
        .and_then(|items| items.parent::<ScssExpression>())
        .and_then(|expression| expression.parent::<ScssVariableDeclaration>())
        .is_some()
}

/// Checks for list items like `"save" 50px`.
fn has_compound_list_element(elements: &ScssListExpressionElementList) -> bool {
    elements
        .iter()
        .any(|element| element.as_ref().is_ok_and(is_compound_list_element))
}

/// Returns `true` for SCSS lists with visible separators.
///
/// `(a, b)` and `(a,)` are lists; `(a)` is scalar parentheses.
pub(crate) fn has_scss_list_shape(node: &ScssListExpression) -> bool {
    has_list_shape(&node.elements())
}

fn has_list_shape(elements: &ScssListExpressionElementList) -> bool {
    elements.len() > 1 || elements.trailing_separator().is_some()
}

/// Checks whether `"save" 50px` has multiple values.
fn is_compound_list_element(element: &ScssListExpressionElement) -> bool {
    element.value().ok().is_some_and(|value| {
        value
            .as_scss_expression()
            .is_some_and(|expression| expression.items().len() > 1)
    })
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
