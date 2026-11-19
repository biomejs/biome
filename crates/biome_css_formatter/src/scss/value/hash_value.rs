use crate::prelude::*;
use biome_css_syntax::{ScssHashValue, ScssHashValueFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssHashValue;

impl FormatNodeRule<ScssHashValue> for FormatScssHashValue {
    fn fmt_fields(&self, node: &ScssHashValue, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssHashValueFields {
            hash_token,
            value_token,
        } = node.as_fields();

        write!(
            f,
            [
                hash_token.format(),
                value_token.format()?.with_text_case(CssCase::Preserve)
            ]
        )
    }
}
