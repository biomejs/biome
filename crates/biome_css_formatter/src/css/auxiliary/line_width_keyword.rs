use crate::prelude::*;
use biome_css_syntax::{CssLineWidthKeyword, CssLineWidthKeywordFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLineWidthKeyword;
impl FormatNodeRule<CssLineWidthKeyword> for FormatCssLineWidthKeyword {
    fn fmt_fields(&self, node: &CssLineWidthKeyword, f: &mut CssFormatter) -> FormatResult<()> {
        let CssLineWidthKeywordFields { keyword } = node.as_fields();

        write!(f, [keyword.format()])
    }
}
