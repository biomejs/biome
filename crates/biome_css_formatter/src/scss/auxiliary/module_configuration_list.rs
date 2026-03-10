use crate::prelude::*;
use biome_css_syntax::{ScssModuleConfigurationList, ScssModuleConfigurationListFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssModuleConfigurationList;

impl FormatNodeRule<ScssModuleConfigurationList> for FormatScssModuleConfigurationList {
    fn fmt_fields(
        &self,
        node: &ScssModuleConfigurationList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssModuleConfigurationListFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&items.format()),
                r_paren_token.format()
            ])]
        )
    }
}
