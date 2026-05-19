use crate::prelude::*;
use crate::utils::comment_trivia::has_inline_trailing_comment;
use crate::utils::scss_include_keyword_value::is_top_level_include_keyword_parenthesized_value;
use crate::utils::scss_separator_comments::FormatScssSeparatorComments;
use biome_css_syntax::{
    AnyScssExpression, AnyScssExpressionItem, ScssParenthesizedExpression,
    ScssParenthesizedExpressionFields, is_scss_map_outer_parenthesized_value,
    scss_include_keyword_argument_owner, unwrap_single_expression_item,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssParenthesizedExpression;

impl FormatNodeRule<ScssParenthesizedExpression> for FormatScssParenthesizedExpression {
    fn fmt_node(
        &self,
        node: &ScssParenthesizedExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        self.fmt_node_with_scss_separator_comments(node, f)
    }

    fn fmt_fields(
        &self,
        node: &ScssParenthesizedExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        write!(f, [ScssParenthesizedExpressionLayout::new(node)])
    }

    fn fmt_leading_comments(
        &self,
        node: &ScssParenthesizedExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        self.fmt_leading_scss_separator_comments(node, f)
    }
}

/// Formats `(...)` after classifying SCSS-only paren roles like `$arg: (a)`.
#[derive(Debug, Clone, Copy)]
struct ScssParenthesizedExpressionLayout<'a> {
    node: &'a ScssParenthesizedExpression,
}

impl<'a> ScssParenthesizedExpressionLayout<'a> {
    fn new(node: &'a ScssParenthesizedExpression) -> Self {
        Self { node }
    }

    /// Formats the trailing comma inside parentheses that Prettier breaks.
    ///
    /// Examples: `@include mix($arg: (a))`, `key: (value)`.
    fn trailing_comma(&self) -> impl Format<CssFormatContext> + '_ {
        format_with(|f| {
            if self.should_print_trailing_comma() {
                write!(f, [if_group_breaks(&token(","))])
            } else {
                Ok(())
            }
        })
    }

    /// Expands parenthesized map values and include keyword values.
    ///
    /// Examples: `key: (value)`, `@include mix($arg: (a))`.
    fn should_expand(&self) -> bool {
        is_scss_map_outer_parenthesized_value(self.node)
            || is_top_level_include_keyword_parenthesized_value(self.node)
            || self.has_nested_include_parentheses()
            || self.has_include_trailing_comment()
    }

    /// Allows a trailing comma for parentheses Prettier treats as one-item lists.
    ///
    /// Examples: `@include mix($arg: (a))`, `key: (value)`.
    fn should_print_trailing_comma(&self) -> bool {
        self.should_print_map_trailing_comma()
            || self.has_nested_include_parentheses()
            || self.should_print_include_trailing_comma()
    }

    /// Allows the trailing comma in maps such as `key: (value)`.
    fn should_print_map_trailing_comma(&self) -> bool {
        is_scss_map_outer_parenthesized_value(self.node)
            && self
                .node
                .expression()
                .ok()
                .is_some_and(|expression| !expression_owns_list_comma(&expression))
    }

    /// Allows the top-level include comma unless the child list owns commas.
    ///
    /// Example: `$arg: (a)` prints a comma, but `$arg: (a, b)` does not.
    fn should_print_include_trailing_comma(&self) -> bool {
        is_top_level_include_keyword_parenthesized_value(self.node)
            && self
                .node
                .expression()
                .ok()
                .is_some_and(|expression| !expression_owns_list_comma(&expression))
    }

    /// Detects the outer include value in `@include mix($arg: ((a)))`.
    fn has_nested_include_parentheses(&self) -> bool {
        scss_include_keyword_argument_owner(self.node.syntax()).is_some()
            && self.has_nested_parenthesized_item()
    }

    /// Detects include value comments such as `@include mix($arg: (a) /* end */)`.
    fn has_include_trailing_comment(&self) -> bool {
        scss_include_keyword_argument_owner(self.node.syntax()).is_some()
            && has_inline_trailing_comment(self.node.syntax())
    }

    /// Detects the immediate parenthesized child in `((a))`.
    fn has_nested_parenthesized_item(&self) -> bool {
        self.node.expression().ok().is_some_and(|expression| {
            matches!(
                expression,
                AnyScssExpression::ScssParenthesizedExpression(_)
            ) || unwrap_single_expression_item(&expression).is_some_and(|item| {
                matches!(item, AnyScssExpressionItem::ScssParenthesizedExpression(_))
            })
        })
    }
}

impl Format<CssFormatContext> for ScssParenthesizedExpressionLayout<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = self.node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&format_args![expression.format(), self.trailing_comma()]),
                r_paren_token.format()
            ])
            .should_expand(self.should_expand())]
        )
    }
}

/// Lists already print their own item comma in `$arg: (a, b)`.
///
/// Maps only print pair separators, so `key: ((a: b))` still needs the outer
/// scalar comma after the nested map.
fn expression_owns_list_comma(expression: &AnyScssExpression) -> bool {
    matches!(expression, AnyScssExpression::ScssListExpression(_))
        || unwrap_single_expression_item(expression)
            .is_some_and(|item| matches!(item, AnyScssExpressionItem::ScssListExpression(_)))
}
