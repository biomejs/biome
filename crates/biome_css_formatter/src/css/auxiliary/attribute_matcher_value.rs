use crate::prelude::*;
use biome_css_syntax::{CssAttributeMatcherValue, CssAttributeMatcherValueFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttributeMatcherValue;
impl FormatNodeRule<CssAttributeMatcherValue> for FormatCssAttributeMatcherValue {
    fn fmt_fields(
        &self,
        node: &CssAttributeMatcherValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssAttributeMatcherValueFields { name } = node.as_fields();

        write!(f, [name.format()])
    }
}
