use crate::prelude::*;
use biome_css_syntax::{CssPageSelector, CssPageSelectorFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPageSelector;
impl FormatNodeRule<CssPageSelector> for FormatCssPageSelector {
    fn fmt_fields(&self, node: &CssPageSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssPageSelectorFields { ty, pseudos } = node.as_fields();

        write!(f, [group(&format_args![ty.format(), pseudos.format()])])
    }
}
