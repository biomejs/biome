use crate::prelude::*;
use biome_css_syntax::{ScssQualifiedName, ScssQualifiedNameFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssQualifiedName;
impl FormatNodeRule<ScssQualifiedName> for FormatScssQualifiedName {
    fn fmt_fields(&self, node: &ScssQualifiedName, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssQualifiedNameFields {
            module,
            dot_token,
            member,
        } = node.as_fields();

        write!(f, [module.format(), dot_token.format(), member.format()])
    }
}
