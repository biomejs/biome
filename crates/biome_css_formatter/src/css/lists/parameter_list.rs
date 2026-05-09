use crate::prelude::*;
use crate::utils::scss_closing_comments::{
    ClosingCommentSpacing, owns_include_closing_comments, write_include_closing_comments,
};
use biome_css_syntax::{
    AnyCssExpression, CssFunction, CssParameterList, scss_keyword_argument_from_css_expression,
};
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::write;
use biome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssParameterList;
impl FormatRule<CssParameterList> for FormatCssParameterList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssParameterList, f: &mut CssFormatter) -> FormatResult<()> {
        let separated = node.format_separated(",").with_trailing_separator(
            if is_var_function_empty_fallback(node) {
                // `var(--name,)` uses the comma as an empty fallback marker.
                TrailingSeparator::Mandatory
            } else {
                // Prettier removes call trailing commas: `fn(a,)` -> `fn(a)`.
                TrailingSeparator::Omit
            },
        );

        let mut is_first = true;
        let mut previous_was_keyword_argument = false;

        for (element, formatted) in node.elements().zip(separated) {
            let element_node = element.node().ok();

            if is_first {
                is_first = false;
            } else if should_preserve_blank_line_before_parameter(
                element_node,
                previous_was_keyword_argument,
            ) {
                write!(f, [empty_line()])?;
            } else {
                write!(f, [soft_line_break_or_space()])?;
            }

            write!(f, [formatted])?;
            previous_was_keyword_argument = is_scss_keyword_parameter(element_node);
        }

        if should_inline_arbitrary_argument_closing_comment(node, f) {
            write!(
                f,
                [
                    space(),
                    format_dangling_comments(node.syntax()),
                    expand_parent()
                ]
            )
        } else {
            write_include_closing_comments(node.syntax(), ClosingCommentSpacing::Adaptive, f)
        }
    }
}

/// Returns true for `var()` with a source trailing comma.
///
/// CSS uses the comma as an empty fallback marker: `var(--name,)`.
fn is_var_function_empty_fallback(node: &CssParameterList) -> bool {
    node.trailing_separator().is_some() && is_var_function_parameter_list(node)
}

/// Checks whether this parameter list belongs directly to a `var(...)` call.
fn is_var_function_parameter_list(node: &CssParameterList) -> bool {
    node.parent::<CssFunction>()
        .and_then(|function| function.name().ok())
        .and_then(|name| {
            name.as_css_identifier()
                .and_then(|name| name.value_token().ok())
        })
        .is_some_and(|token| token.text_trimmed().eq_ignore_ascii_case("var"))
}

/// Keeps a blank line after SCSS keyword parameters.
///
/// Example: `foo($a: 1,\n\n$b: 2)` keeps the empty line before `$b`.
fn should_preserve_blank_line_before_parameter(
    parameter: Option<&AnyCssExpression>,
    previous_was_keyword_argument: bool,
) -> bool {
    previous_was_keyword_argument
        && parameter.is_some_and(|node| get_lines_before(node.syntax()) > 1)
}

/// Detects SCSS keyword parameters such as `$color: red`.
fn is_scss_keyword_parameter(parameter: Option<&AnyCssExpression>) -> bool {
    parameter.is_some_and(|node| scss_keyword_argument_from_css_expression(node).is_some())
}

/// Keeps a closing line comment attached to a final SCSS spread argument.
///
/// Example: `@include mix($args... // comment\n)`.
fn should_inline_arbitrary_argument_closing_comment(
    node: &CssParameterList,
    f: &CssFormatter,
) -> bool {
    owns_include_closing_comments(node.syntax(), f)
        && f.comments()
            .dangling_comments(node.syntax())
            .iter()
            .all(|comment| comment.kind().is_line())
        && node.iter().last().is_some_and(|parameter| {
            parameter.is_ok_and(|node| expression_ends_with_arbitrary_argument(&node))
        })
}

/// Returns true when an expression ends with a SCSS spread argument: `$args...`.
fn expression_ends_with_arbitrary_argument(node: &AnyCssExpression) -> bool {
    node.as_scss_expression()
        .and_then(|expression| expression.items().iter().last())
        .is_some_and(|item| item.as_scss_arbitrary_argument().is_some())
}
