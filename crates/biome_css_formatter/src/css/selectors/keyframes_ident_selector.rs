use crate::prelude::*;
use biome_css_syntax::{CssKeyframesIdentSelector, CssKeyframesIdentSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssKeyframesIdentSelector;
impl FormatNodeRule<CssKeyframesIdentSelector> for FormatCssKeyframesIdentSelector {
    fn fmt_fields(
        &self,
        node: &CssKeyframesIdentSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssKeyframesIdentSelectorFields { selector } = node.as_fields();

        write!(f, [selector.format()])
    }
}
