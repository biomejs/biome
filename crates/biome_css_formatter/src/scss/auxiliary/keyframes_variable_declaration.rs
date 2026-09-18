use crate::prelude::*;
use biome_css_syntax::{ScssKeyframesVariableDeclaration, ScssKeyframesVariableDeclarationFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssKeyframesVariableDeclaration;

impl FormatNodeRule<ScssKeyframesVariableDeclaration> for FormatScssKeyframesVariableDeclaration {
    fn fmt_fields(
        &self,
        node: &ScssKeyframesVariableDeclaration,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssKeyframesVariableDeclarationFields { declaration } = node.as_fields();

        write!(f, [declaration.format()])
    }
}
