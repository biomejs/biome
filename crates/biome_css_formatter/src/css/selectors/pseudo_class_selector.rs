use crate::prelude::*;
use biome_css_syntax::{CssPseudoClassSelector, CssPseudoClassSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassSelector;
impl FormatNodeRule<CssPseudoClassSelector> for FormatCssPseudoClassSelector {
    fn fmt_fields(&self, node: &CssPseudoClassSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssPseudoClassSelectorFields { colon_token, class } = node.as_fields();

        write!(f, [colon_token.format(), class.format()])
    }
}
