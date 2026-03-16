use crate::prelude::*;
use biome_css_syntax::ScssModuleMemberList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssModuleMemberList;

impl FormatRule<ScssModuleMemberList> for FormatScssModuleMemberList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssModuleMemberList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
