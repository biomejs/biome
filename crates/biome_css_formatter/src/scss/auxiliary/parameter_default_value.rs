use biome_css_syntax::{ScssParameterDefaultValue, ScssParameterDefaultValueFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssParameterDefaultValue;

impl FormatNodeRule<ScssParameterDefaultValue> for FormatScssParameterDefaultValue {
    fn fmt_fields(
        &self,
        node: &ScssParameterDefaultValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssParameterDefaultValueFields { colon_token, value } = node.as_fields();

        write!(f, [colon_token.format(), space(), value.format()])
    }
}
