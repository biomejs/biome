use crate::prelude::*;
use biome_css_syntax::{ScssParentSelector, ScssParentSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssParentSelector;
impl FormatNodeRule<ScssParentSelector> for FormatScssParentSelector {
    fn fmt_fields(&self, node: &ScssParentSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssParentSelectorFields { amp_token, suffix } = node.as_fields();

        write!(f, [amp_token.format(), suffix.format()])
    }
}
