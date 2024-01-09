use crate::prelude::*;
use biome_css_syntax::{CssWideKeyword, CssWideKeywordFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssWideKeyword;
impl FormatNodeRule<CssWideKeyword> for FormatCssWideKeyword {
    fn fmt_fields(&self, node: &CssWideKeyword, f: &mut CssFormatter) -> FormatResult<()> {
        let CssWideKeywordFields { value } = node.as_fields();

        write!(f, [value.format()])
    }
}
