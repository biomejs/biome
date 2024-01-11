use crate::prelude::*;
use biome_css_syntax::{CssGenericDelimiter, CssGenericDelimiterFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssGenericDelimiter;
impl FormatNodeRule<CssGenericDelimiter> for FormatCssGenericDelimiter {
    fn fmt_fields(&self, node: &CssGenericDelimiter, f: &mut CssFormatter) -> FormatResult<()> {
        let CssGenericDelimiterFields { value } = node.as_fields();

        write!(f, [value.format()])
    }
}
