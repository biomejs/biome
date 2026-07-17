use crate::prelude::*;
use biome_css_syntax::{ScssParentSelectorSuffix, ScssParentSelectorSuffixFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssParentSelectorSuffix;
impl FormatNodeRule<ScssParentSelectorSuffix> for FormatScssParentSelectorSuffix {
    fn fmt_fields(
        &self,
        node: &ScssParentSelectorSuffix,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssParentSelectorSuffixFields { items } = node.as_fields();

        write!(f, [items.format()])
    }
}
