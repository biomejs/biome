use crate::prelude::*;
use biome_css_syntax::{ScssVariableModifier, ScssVariableModifierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssVariableModifier;

impl FormatNodeRule<ScssVariableModifier> for FormatScssVariableModifier {
    fn fmt_fields(&self, node: &ScssVariableModifier, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssVariableModifierFields { excl_token, value } = node.as_fields();

        write!(f, [excl_token.format(), value.format()])
    }
}
