use crate::prelude::*;
use biome_css_syntax::{ScssNamespacedVariable, ScssNamespacedVariableFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssNamespacedVariable;

impl FormatNodeRule<ScssNamespacedVariable> for FormatScssNamespacedVariable {
    fn fmt_fields(&self, node: &ScssNamespacedVariable, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssNamespacedVariableFields {
            namespace,
            dot_token,
            name,
        } = node.as_fields();

        write!(f, [namespace.format(), dot_token.format(), name.format()])
    }
}
