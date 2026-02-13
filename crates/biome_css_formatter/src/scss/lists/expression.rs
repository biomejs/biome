use crate::prelude::*;
use crate::utils::component_value_list::write_component_value_list;
use crate::verbatim::format_suppressed_node;
use biome_css_syntax::ScssExpression;
use biome_formatter::{CstFormatContext, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssExpression;

impl FormatRule<ScssExpression> for FormatScssExpression {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        if f.context().comments().is_suppressed(node.syntax())
            || f.context().comments().is_global_suppressed(node.syntax())
        {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        write_component_value_list(node, f)
    }
}
