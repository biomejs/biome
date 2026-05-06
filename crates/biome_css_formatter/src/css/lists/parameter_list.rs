use crate::prelude::*;
use crate::utils::scss_closing_comments::{
    ClosingCommentSpacing, owns_include_closing_comments, write_include_closing_comments,
};
use crate::utils::scss_separated_list::trailing_separator_for_node;
use biome_css_syntax::{
    AnyCssExpression, CssParameterList, scss_keyword_argument_from_css_expression,
};
use biome_formatter::write;
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssParameterList;
impl FormatRule<CssParameterList> for FormatCssParameterList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssParameterList, f: &mut CssFormatter) -> FormatResult<()> {
        let separated = node
            .format_separated(",")
            .with_trailing_separator(trailing_separator_for_node(node.syntax()));

        let mut is_first = true;
        let mut previous_was_keyword_argument = false;

        for (element, formatted) in node.elements().zip(separated) {
            let element_node = element.node().ok();

            if is_first {
                is_first = false;
            } else {
                write_parameter_separator(element_node, previous_was_keyword_argument, f)?;
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

fn write_parameter_separator(
    parameter: Option<&AnyCssExpression>,
    previous_was_keyword_argument: bool,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    if should_preserve_blank_line_before_parameter(parameter, previous_was_keyword_argument) {
        write!(f, [empty_line()])
    } else {
        write!(f, [soft_line_break_or_space()])
    }
}

fn should_preserve_blank_line_before_parameter(
    parameter: Option<&AnyCssExpression>,
    previous_was_keyword_argument: bool,
) -> bool {
    previous_was_keyword_argument
        && parameter.is_some_and(|node| get_lines_before(node.syntax()) > 1)
}

fn is_scss_keyword_parameter(parameter: Option<&AnyCssExpression>) -> bool {
    parameter.is_some_and(|node| scss_keyword_argument_from_css_expression(node).is_some())
}

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

fn expression_ends_with_arbitrary_argument(node: &AnyCssExpression) -> bool {
    node.as_scss_expression()
        .and_then(|expression| expression.items().iter().last())
        .is_some_and(|item| item.as_scss_arbitrary_argument().is_some())
}
