use crate::prelude::*;
use biome_css_syntax::{ScssStringText, ScssStringTextFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssStringText;
impl FormatNodeRule<ScssStringText> for FormatScssStringText {
    fn fmt_fields(&self, node: &ScssStringText, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssStringTextFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
