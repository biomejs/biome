use crate::prelude::*;
use biome_css_syntax::{CssPseudoElementSelector, CssPseudoElementSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementSelector;
impl FormatNodeRule<CssPseudoElementSelector> for FormatCssPseudoElementSelector {
    fn fmt_fields(
        &self,
        node: &CssPseudoElementSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoElementSelectorFields {
            double_colon_token,
            element,
        } = node.as_fields();

        write!(f, [double_colon_token.format(), element.format()])
    }
}
