use crate::prelude::*;
use biome_css_syntax::ScssAtRootQueryList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssAtRootQueryList;

impl FormatRule<ScssAtRootQueryList> for FormatScssAtRootQueryList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssAtRootQueryList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.iter().formatted())
            .finish()
    }
}
