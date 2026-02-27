use crate::prelude::*;
use biome_css_syntax::{ScssNestingDeclaration, ScssNestingDeclarationFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssNestingDeclaration;
impl FormatNodeRule<ScssNestingDeclaration> for FormatScssNestingDeclaration {
    fn fmt_fields(&self, node: &ScssNestingDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssNestingDeclarationFields {
            name,
            colon_token,
            value,
            block,
        } = node.as_fields();

        write!(f, [name.format(), colon_token.format()])?;

        if !value.is_empty() {
            write!(f, [space(), value.format()])?;
        }

        write!(f, [space(), block.format()])
    }
}
