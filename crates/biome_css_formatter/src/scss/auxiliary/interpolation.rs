use crate::prelude::*;
use biome_css_syntax::{ScssInterpolation, ScssInterpolationFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolation;
impl FormatNodeRule<ScssInterpolation> for FormatScssInterpolation {
    fn fmt_fields(&self, node: &ScssInterpolation, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssInterpolationFields {
            hash_token,
            l_curly_token,
            value,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                hash_token.format(),
                l_curly_token.format(),
                value.format(),
                r_curly_token.format()
            ])]
        )
    }
}
