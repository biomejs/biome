use crate::prelude::*;
use biome_css_syntax::{CssMediaFeatureInParens, CssMediaFeatureInParensFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaFeatureInParens;
impl FormatNodeRule<CssMediaFeatureInParens> for FormatCssMediaFeatureInParens {
    fn fmt_fields(&self, node: &CssMediaFeatureInParens, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaFeatureInParensFields {
            l_paren_token,
            feature,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent_with_maybe_space(&feature.format(), should_insert_space),
                r_paren_token.format()
            ])]
        )
    }
}
