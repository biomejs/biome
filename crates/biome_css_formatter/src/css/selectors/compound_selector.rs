use crate::prelude::*;
use biome_css_syntax::{CssCompoundSelector, CssCompoundSelectorFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCompoundSelector;
impl FormatNodeRule<CssCompoundSelector> for FormatCssCompoundSelector {
    fn fmt_fields(&self, node: &CssCompoundSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssCompoundSelectorFields {
            nesting_selector_token,
            simple_selector,
            sub_selectors,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                nesting_selector_token.format(),
                simple_selector.format(),
                sub_selectors.format()
            ])]
        )
    }
}
