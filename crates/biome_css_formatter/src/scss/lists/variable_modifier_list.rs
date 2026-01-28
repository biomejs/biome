use crate::prelude::*;
use biome_css_syntax::ScssVariableModifierList;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssVariableModifierList;

impl FormatRule<ScssVariableModifierList> for FormatScssVariableModifierList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &ScssVariableModifierList, f: &mut CssFormatter) -> FormatResult<()> {
        // Format in the correct order: !default, then !global, then unknown/rest
        for modifier in node.iter().filter(|m| m.is_default()) {
            write!(f, [space(), modifier.format()])?;
        }

        for modifier in node.iter().filter(|m| m.is_global()) {
            write!(f, [space(), modifier.format()])?;
        }

        for modifier in node.iter().filter(|m| m.is_unknown()) {
            write!(f, [space(), modifier.format()])?;
        }

        Ok(())
    }
}
