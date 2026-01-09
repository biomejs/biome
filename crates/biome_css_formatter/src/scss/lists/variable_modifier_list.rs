use crate::prelude::*;
use biome_css_syntax::ScssVariableModifierList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssVariableModifierList;
impl FormatRule<ScssVariableModifierList> for FormatScssVariableModifierList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &ScssVariableModifierList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
