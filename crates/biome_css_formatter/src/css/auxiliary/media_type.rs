use crate::prelude::*;
use biome_css_syntax::{CssMediaType, CssMediaTypeFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaType;
impl FormatNodeRule<CssMediaType> for FormatCssMediaType {
    fn fmt_fields(&self, node: &CssMediaType, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaTypeFields { value } = node.as_fields();

        write!(f, [value.format()])
    }
}
