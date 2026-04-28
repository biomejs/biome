use crate::prelude::*;
use crate::utils::scss_context::is_in_scss_include_arguments;
use crate::utils::scss_expression::scss_keyword_argument_from_css_expression;
use crate::utils::scss_separated_list::trailing_separator_for_node;
use biome_css_syntax::{AnyCssExpression, CssParameterList};
use biome_formatter::write;
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssParameterList;
impl FormatRule<CssParameterList> for FormatCssParameterList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssParameterList, f: &mut CssFormatter) -> FormatResult<()> {
        if !is_in_scss_include_arguments(node.syntax()) {
            let separator = soft_line_break_or_space();
            let mut joiner = f.join_with(&separator);

            for formatted in node.format_separated(",") {
                joiner.entry(&formatted);
            }

            return joiner.finish();
        }

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

        Ok(())
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
