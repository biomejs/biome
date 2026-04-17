use crate::prelude::*;
use biome_css_syntax::{ScssParentSelectorValue, ScssParentSelectorValueFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssParentSelectorValue;
impl FormatNodeRule<ScssParentSelectorValue> for FormatScssParentSelectorValue {
    fn fmt_fields(&self, node: &ScssParentSelectorValue, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssParentSelectorValueFields { amp_token } = node.as_fields();

        write!(f, [amp_token.format()])
    }
}
