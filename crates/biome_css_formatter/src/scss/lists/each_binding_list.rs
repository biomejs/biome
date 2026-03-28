use crate::prelude::*;
use biome_css_syntax::ScssEachBindingList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssEachBindingList;

impl FormatRule<ScssEachBindingList> for FormatScssEachBindingList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssEachBindingList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
