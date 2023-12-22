use crate::prelude::*;
use biome_css_syntax::{CssPseudoClassOfNthSelector, CssPseudoClassOfNthSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassOfNthSelector;
impl FormatNodeRule<CssPseudoClassOfNthSelector> for FormatCssPseudoClassOfNthSelector {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassOfNthSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassOfNthSelectorFields {
            of_token,
            selectors,
        } = node.as_fields();

        write!(f, [of_token.format(), space(), selectors.format()])
    }
}
