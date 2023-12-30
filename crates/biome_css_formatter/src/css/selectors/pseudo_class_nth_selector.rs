use crate::prelude::*;
use biome_css_syntax::{CssPseudoClassNthSelector, CssPseudoClassNthSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassNthSelector;
impl FormatNodeRule<CssPseudoClassNthSelector> for FormatCssPseudoClassNthSelector {
    fn fmt_fields(
        &self,
        node: &CssPseudoClassNthSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoClassNthSelectorFields { nth, of_selector } = node.as_fields();

        write!(f, [group(&nth.format())])?;

        if of_selector.is_some() {
            write!(f, [space(), of_selector.format()])?;
        }

        Ok(())
    }
}
