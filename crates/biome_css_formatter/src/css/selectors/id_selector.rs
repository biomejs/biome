use crate::prelude::*;
use biome_css_syntax::{CssIdSelector, CssIdSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIdSelector;
impl FormatNodeRule<CssIdSelector> for FormatCssIdSelector {
    fn fmt_fields(&self, node: &CssIdSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssIdSelectorFields { hash_token, name } = node.as_fields();

        write!(f, [hash_token.format(), name.format()])
    }
}
