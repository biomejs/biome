use crate::prelude::*;
use biome_css_syntax::{CssSupportsFeatureSelector, CssSupportsFeatureSelectorFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsFeatureSelector;
impl FormatNodeRule<CssSupportsFeatureSelector> for FormatCssSupportsFeatureSelector {
    fn fmt_fields(
        &self,
        node: &CssSupportsFeatureSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssSupportsFeatureSelectorFields {
            selector_token,
            l_paren_token,
            selector,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                selector_token.format(),
                l_paren_token.format(),
                soft_block_indent(&selector.format()),
                r_paren_token.format()
            ])]
        )
    }
}
