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

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&feature.format()),
                r_paren_token.format()
            ])]
        )
    }
}
