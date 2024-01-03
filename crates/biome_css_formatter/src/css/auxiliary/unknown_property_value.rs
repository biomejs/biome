use crate::prelude::*;
use biome_css_syntax::{CssUnknownPropertyValue, CssUnknownPropertyValueFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownPropertyValue;
impl FormatNodeRule<CssUnknownPropertyValue> for FormatCssUnknownPropertyValue {
    fn fmt_fields(&self, node: &CssUnknownPropertyValue, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUnknownPropertyValueFields {
            css_generic_component_value_list,
        } = node.as_fields();

        write!(f, [css_generic_component_value_list.format()])
    }
}
