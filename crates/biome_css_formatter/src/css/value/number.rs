use crate::prelude::*;
use crate::utils::custom_property::is_raw_custom_property_component;
use biome_css_syntax::CssNumber;
use biome_formatter::token::number::NumberFormatOptions;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNumber;
impl FormatNodeRule<CssNumber> for FormatCssNumber {
    fn fmt_fields(&self, node: &CssNumber, f: &mut CssFormatter) -> FormatResult<()> {
        if is_raw_custom_property_component(node) {
            return node.value_token()?.format().fmt(f);
        }

        format_number_token(&node.value_token()?, NumberFormatOptions::default()).fmt(f)
    }
}
