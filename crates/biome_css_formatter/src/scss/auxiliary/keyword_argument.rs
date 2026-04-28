use crate::prelude::*;
use crate::utils::scss_keyword_argument_layout::ScssKeywordArgumentLayout;
use biome_css_syntax::ScssKeywordArgument;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssKeywordArgument;
impl FormatNodeRule<ScssKeywordArgument> for FormatScssKeywordArgument {
    fn fmt_fields(&self, node: &ScssKeywordArgument, f: &mut CssFormatter) -> FormatResult<()> {
        ScssKeywordArgumentLayout::new(node).fmt(f)
    }
}
