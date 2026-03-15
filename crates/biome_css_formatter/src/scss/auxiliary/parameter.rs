use biome_css_syntax::{ScssParameter, ScssParameterFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssParameter;

impl FormatNodeRule<ScssParameter> for FormatScssParameter {
    fn fmt_fields(&self, node: &ScssParameter, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssParameterFields {
            name,
            default_value,
            ellipsis_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                default_value.format(),
                ellipsis_token.format()
            ]
        )
    }
}
