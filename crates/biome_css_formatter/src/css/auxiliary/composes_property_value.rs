use crate::prelude::*;
use biome_css_syntax::{CssComposesPropertyValue, CssComposesPropertyValueFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComposesPropertyValue;
impl FormatNodeRule<CssComposesPropertyValue> for FormatCssComposesPropertyValue {
    fn fmt_fields(
        &self,
        node: &CssComposesPropertyValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssComposesPropertyValueFields { classes, specifier } = node.as_fields();

        write![f, [classes.format(), specifier.format()]]
    }
}
