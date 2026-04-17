use crate::prelude::*;
use biome_css_syntax::{ScssModuleMemberAccess, ScssModuleMemberAccessFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssModuleMemberAccess;
impl FormatNodeRule<ScssModuleMemberAccess> for FormatScssModuleMemberAccess {
    fn fmt_fields(&self, node: &ScssModuleMemberAccess, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssModuleMemberAccessFields {
            module,
            dot_token,
            member,
        } = node.as_fields();

        write!(f, [module.format(), dot_token.format(), member.format()])
    }
}
