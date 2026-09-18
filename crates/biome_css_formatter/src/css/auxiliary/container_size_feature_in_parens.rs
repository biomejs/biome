use crate::prelude::*;
use biome_css_syntax::{CssContainerSizeFeatureInParens, CssContainerSizeFeatureInParensFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssContainerSizeFeatureInParens;
impl FormatNodeRule<CssContainerSizeFeatureInParens> for FormatCssContainerSizeFeatureInParens {
    fn fmt_fields(
        &self,
        node: &CssContainerSizeFeatureInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssContainerSizeFeatureInParensFields {
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
