use biome_css_syntax::CssFunctionParameterList;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunctionParameterList;

impl FormatRule<CssFunctionParameterList> for FormatCssFunctionParameterList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &CssFunctionParameterList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
