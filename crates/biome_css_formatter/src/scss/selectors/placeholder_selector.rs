use crate::prelude::*;
use biome_css_syntax::{ScssPlaceholderSelector, ScssPlaceholderSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssPlaceholderSelector;
impl FormatNodeRule<ScssPlaceholderSelector> for FormatScssPlaceholderSelector {
    fn fmt_fields(&self, node: &ScssPlaceholderSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssPlaceholderSelectorFields {
            percent_token,
            name,
        } = node.as_fields();

        write!(f, [percent_token.format(), name.format()])
    }
}
