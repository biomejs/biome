use crate::{prelude::*, utils::string_utils::FormatLiteralStringToken};
use biome_css_syntax::{CssString, CssStringFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssString;
impl FormatNodeRule<CssString> for FormatCssString {
    fn fmt_fields(&self, node: &CssString, f: &mut CssFormatter) -> FormatResult<()> {
        let CssStringFields { value_token } = node.as_fields();

        write!(f, [FormatLiteralStringToken::new(&value_token?)])
    }
}
