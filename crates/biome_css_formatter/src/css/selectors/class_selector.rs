use crate::prelude::*;
use biome_css_syntax::{CssClassSelector, CssClassSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssClassSelector;
impl FormatNodeRule<CssClassSelector> for FormatCssClassSelector {
    fn fmt_fields(&self, node: &CssClassSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssClassSelectorFields { dot_token, name } = node.as_fields();

        write!(f, [dot_token.format(), name.format()])
    }
}
