use crate::prelude::*;
use biome_css_syntax::{CssLineStyle, CssLineStyleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLineStyle;
impl FormatNodeRule<CssLineStyle> for FormatCssLineStyle {
    fn fmt_fields(&self, node: &CssLineStyle, f: &mut CssFormatter) -> FormatResult<()> {
        let CssLineStyleFields { keyword } = node.as_fields();

        write!(f, [keyword.format()])
    }
}
