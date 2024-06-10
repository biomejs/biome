use crate::prelude::*;
use biome_css_syntax::{CssValueAtRuleGenericProperty, CssValueAtRuleGenericPropertyFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssValueAtRuleGenericProperty;
impl FormatNodeRule<CssValueAtRuleGenericProperty> for FormatCssValueAtRuleGenericProperty {
    fn fmt_fields(
        &self,
        node: &CssValueAtRuleGenericProperty,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssValueAtRuleGenericPropertyFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [name.format(), colon_token.format(), space(), value.format()]
        )
    }
}
