use crate::prelude::*;
use biome_css_syntax::{CssQueryFeaturePlain, CssQueryFeaturePlainFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeaturePlain;
impl FormatNodeRule<CssQueryFeaturePlain> for FormatCssQueryFeaturePlain {
    fn fmt_fields(&self, node: &CssQueryFeaturePlain, f: &mut CssFormatter) -> FormatResult<()> {
        let CssQueryFeaturePlainFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [name.format(), colon_token.format(), space(), value.format()]
        )
    }
}
