use crate::prelude::*;
use biome_css_syntax::{ScssAtRootSelector, ScssAtRootSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssAtRootSelector;

impl FormatNodeRule<ScssAtRootSelector> for FormatScssAtRootSelector {
    fn fmt_fields(&self, node: &ScssAtRootSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssAtRootSelectorFields { selector } = node.as_fields();

        write!(f, [selector.format()])
    }
}
